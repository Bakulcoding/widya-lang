use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
        }
    }
    
    pub fn emoji(&self) -> &'static str {
        match self {
            Severity::Critical => "🔴",
            Severity::High => "🟠",
            Severity::Medium => "🟡",
            Severity::Low => "🟢",
            Severity::Info => "🔵",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vulnerability {
    pub id: String,
    pub severity: Severity,
    pub category: String,
    pub title: String,
    pub description: String,
    pub line: usize,
    pub column: usize,
    pub code: String,
    pub recommendation: String,
    pub cwe: Option<String>,
}

pub struct SecretScanner {
    patterns: HashMap<String, (Regex, Severity)>,
}

impl SecretScanner {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        patterns.insert(
            "AWS_ACCESS_KEY".to_string(),
            (Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(), Severity::Critical),
        );
        
        patterns.insert(
            "AWS_SECRET_KEY".to_string(),
            (Regex::new(r#"aws_secret_access_key\s*=\s*['"]?[A-Za-z0-9/+=]{40}['"]?"#).unwrap(), Severity::Critical),
        );
        
        patterns.insert(
            "JWT_TOKEN".to_string(),
            (Regex::new(r"eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.?[A-Za-z0-9_.+-]*").unwrap(), Severity::High),
        );
        
        patterns.insert(
            "PRIVATE_KEY".to_string(),
            (Regex::new(r"-----BEGIN RSA PRIVATE KEY-----").unwrap(), Severity::Critical),
        );
        
        patterns.insert(
            "DATABASE_URL".to_string(),
            (Regex::new(r"(mongodb|postgres|mysql)://[a-zA-Z0-9:@/._-]+").unwrap(), Severity::High),
        );
        
        patterns.insert(
            "HARDCODED_PASSWORD".to_string(),
            (Regex::new(r#"password\s*=\s*['"]([^'"]{6,})['"]"#).unwrap(), Severity::High),
        );
        
        patterns.insert(
            "SLACK_TOKEN".to_string(),
            (Regex::new(r"xox[baprs]-[0-9]{10,13}-[0-9]{10,13}[a-zA-Z0-9-]*").unwrap(), Severity::Critical),
        );
        
        patterns.insert(
            "GITHUB_TOKEN".to_string(),
            (Regex::new(r"ghp_[A-Za-z0-9_]{36}").unwrap(), Severity::Critical),
        );
        
        patterns.insert(
            "API_KEY".to_string(),
            (Regex::new(r#"api[_-]?key\s*[:=]\s*['"]?[a-zA-Z0-9_-]{32,}['"]?"#).unwrap(), Severity::High),
        );
        
        Self { patterns }
    }
    
    pub fn scan(&self, source: &str, filename: &str) -> Vec<Vulnerability> {
        let mut findings = Vec::new();
        
        for (name, (regex, severity)) in &self.patterns {
            for (line_num, line) in source.lines().enumerate() {
                for mat in regex.find_iter(line) {
                    findings.push(Vulnerability {
                        id: format!("SEC-{:04}", findings.len() + 1),
                        severity: severity.clone(),
                        category: "Secret Detection".to_string(),
                        title: format!("Potential {} detected", name),
                        description: format!(
                            "Found what appears to be a {} in file {}: {}",
                            name.to_lowercase(),
                            filename,
                            self.mask_secret(mat.as_str())
                        ),
                        line: line_num + 1,
                        column: mat.start(),
                        code: line.trim().to_string(),
                        recommendation: match name.as_str() {
                            "PRIVATE_KEY" => "Move private keys to secure key management system (e.g., AWS KMS, Vault)".to_string(),
                            "DATABASE_URL" => "Move database credentials to environment variables".to_string(),
                            "HARDCODED_PASSWORD" => "Move passwords to environment variables or secret manager".to_string(),
                            _ => "Move secrets to environment variables or dedicated secret management system".to_string(),
                        },
                        cwe: Some("CWE-798: Use of Hard-coded Credentials".to_string()),
                    });
                }
            }
        }
        
        findings
    }
    
    fn mask_secret(&self, secret: &str) -> String {
        if secret.len() <= 4 {
            "*".repeat(secret.len())
        } else {
            format!("{}...{}", &secret[..4], &secret[secret.len()-4..])
        }
    }
}

pub struct SqlInjectionChecker;

impl SqlInjectionChecker {
    pub fn new() -> Self {
        Self
    }
    
    pub fn scan(&self, source: &str) -> Vec<Vulnerability> {
        let mut findings = Vec::new();
        
        let dangerous_patterns = vec![
            (r#"(?i)SELECT\s+.*\s+FROM\s+.*WHERE\s+.*\+"#, "Potential SQL injection: string concatenation in SELECT"),
            (r#"(?i)INSERT\s+INTO\s+.*VALUES.*\+"#, "Potential SQL injection: string concatenation in INSERT"),
            (r#"(?i)UPDATE\s+.*SET.*WHERE.*\+"#, "Potential SQL injection: string concatenation in UPDATE"),
            (r#"(?i)DELETE\s+FROM.*WHERE.*\+"#, "Potential SQL injection: string concatenation in DELETE"),
            (r#"(?i)query\s*=\s*".*\+.*""#, "String concatenation in query construction"),
        ];
        
        for (pattern_str, message) in dangerous_patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                for (line_num, line) in source.lines().enumerate() {
                    if regex.is_match(line) {
                        findings.push(Vulnerability {
                            id: format!("SEC-SQL-{:04}", findings.len() + 1),
                            severity: Severity::Critical,
                            category: "SQL Injection".to_string(),
                            title: message.to_string(),
                            description: format!("Detected potential SQL injection: {}", message),
                            line: line_num + 1,
                            column: 0,
                            code: line.trim().to_string(),
                            recommendation: "Use parameterized queries or prepared statements".to_string(),
                            cwe: Some("CWE-89: SQL Injection".to_string()),
                        });
                    }
                }
            }
        }
        
        findings
    }
}

pub struct XssChecker;

impl XssChecker {
    pub fn new() -> Self {
        Self
    }
    
    pub fn scan(&self, source: &str) -> Vec<Vulnerability> {
        let mut findings = Vec::new();
        
        let dangerous_patterns = vec![
            (r#"(?i)innerHTML\s*=\s*.*"# , "Potential XSS: direct assignment to innerHTML"),
            (r#"(?i)eval\s*\(.*"# , "Potential XSS: use of eval"),
            (r#"(?i)document\.write\s*\(.*"# , "Potential XSS: use of document.write"),
            (r#"<.*\{\{.*\}\}.*>"#, "Potential XSS: unescaped template variable"),
        ];
        
        for (pattern_str, message) in dangerous_patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                for (line_num, line) in source.lines().enumerate() {
                    if regex.is_match(line) {
                        findings.push(Vulnerability {
                            id: format!("SEC-XSS-{:04}", findings.len() + 1),
                            severity: Severity::High,
                            category: "Cross-Site Scripting (XSS)".to_string(),
                            title: message.to_string(),
                            description: format!("Detected potential XSS vulnerability: {}", message),
                            line: line_num + 1,
                            column: 0,
                            code: line.trim().to_string(),
                            recommendation: "Use proper output encoding/escaping".to_string(),
                            cwe: Some("CWE-79: Cross-site Scripting".to_string()),
                        });
                    }
                }
            }
        }
        
        findings
    }
}

pub struct SecurityLinter {
    secret_scanner: SecretScanner,
    sql_checker: SqlInjectionChecker,
    xss_checker: XssChecker,
}

impl SecurityLinter {
    pub fn new() -> Self {
        Self {
            secret_scanner: SecretScanner::new(),
            sql_checker: SqlInjectionChecker::new(),
            xss_checker: XssChecker::new(),
        }
    }
    
    pub fn scan(&self, source: &str, filename: &str) -> Vec<Vulnerability> {
        let mut findings = Vec::new();
        
        findings.extend(self.secret_scanner.scan(source, filename));
        findings.extend(self.sql_checker.scan(source));
        findings.extend(self.xss_checker.scan(source));
        
        findings.sort_by(|a, b| {
            match (&a.severity, &b.severity) {
                (Severity::Critical, Severity::Critical) => a.line.cmp(&b.line),
                (Severity::Critical, _) => std::cmp::Ordering::Less,
                (_, Severity::Critical) => std::cmp::Ordering::Greater,
                _ => a.line.cmp(&b.line),
            }
        });
        
        findings
    }
    
    pub fn format_report(&self, findings: &[Vulnerability]) -> String {
        let mut output = String::new();
        
        output.push_str("\n🛡️ Security Scan Report\n");
        output.push_str(&"═".repeat(50));
        output.push('\n');
        
        if findings.is_empty() {
            output.push_str("✅ No security issues found\n");
            return output;
        }
        
        let critical = findings.iter().filter(|f| matches!(f.severity, Severity::Critical)).count();
        let high = findings.iter().filter(|f| matches!(f.severity, Severity::High)).count();
        let medium = findings.iter().filter(|f| matches!(f.severity, Severity::Medium)).count();
        let low = findings.iter().filter(|f| matches!(f.severity, Severity::Low)).count();
        
        output.push_str(&format!(
            "Summary: {} Critical, {} High, {} Medium, {} Low\n\n",
            critical, high, medium, low
        ));
        
        for finding in findings {
            output.push_str(&format!(
                "{} [{}] {} - Line {}\n",
                finding.severity.emoji(),
                finding.severity.as_str(),
                finding.title,
                finding.line
            ));
            output.push_str(&format!("   Category: {}\n", finding.category));
            output.push_str(&format!("   Code: {}\n", finding.code));
            output.push_str(&format!("   Recommendation: {}\n", finding.recommendation));
            
            if let Some(cwe) = &finding.cwe {
                output.push_str(&format!("   {}\n", cwe));
            }
            
            output.push('\n');
        }
        
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_secret_detection() {
        let source = r#"
        var api_key = "sk-1234567890abcdefghijklmnopqrst"
        "#;
        
        let scanner = SecretScanner::new();
        let findings = scanner.scan(source, "test.wya");
        
        assert!(!findings.is_empty());
        assert!(findings[0].title.contains("API_KEY"));
    }
    
    #[test]
    fn test_sql_injection_detection() {
        let source = r#"
        query = "SELECT * FROM users WHERE id = " + userId
        "#;
        
        let checker = SqlInjectionChecker::new();
        let findings = checker.scan(source);
        
        assert!(!findings.is_empty());
        assert!(findings[0].title.contains("SQL injection"));
    }
    
    #[test]
    fn test_linter_report() {
        let source = r#"
        password = "secret123"
        api_key = "sk-1234567890abcdefghijklmnopqrst"
        "#;
        
        let linter = SecurityLinter::new();
        let findings = linter.scan(source, "test.wya");
        let report = linter.format_report(&findings);
        
        assert!(report.contains("Security Scan Report"));
        assert!(!findings.is_empty());
    }
}
