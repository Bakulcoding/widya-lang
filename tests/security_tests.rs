use widya::security_tools::{SecretScanner, SecurityLinter, Severity, SqlInjectionChecker, XssChecker};

#[test]
fn test_security_secret_scanner_detects_credentials() {
    let source = r#"
    misal aws_key = "AKIA1234567890ABCDEF"
    misal db_uri = "postgres://user:password123@localhost:5432/mydb"
    misal api_secret = "ghp_1234567890abcdefghijklmnopqrstuvwxyz"
    "#;

    let scanner = SecretScanner::new();
    let findings = scanner.scan(source, "config.wya");

    assert!(!findings.is_empty(), "Should detect hardcoded secrets");
    let titles: Vec<String> = findings.iter().map(|f| f.title.clone()).collect();
    assert!(titles.iter().any(|t| t.contains("AWS_ACCESS_KEY")));
    assert!(titles.iter().any(|t| t.contains("DATABASE_URL")));
    assert!(titles.iter().any(|t| t.contains("GITHUB_TOKEN")));
}

#[test]
fn test_security_sql_injection_detector() {
    let source = r#"
    misal query = "SELECT * FROM users WHERE id = " + user_input
    misal delete_stmt = "DELETE FROM items WHERE code = " + code
    "#;

    let checker = SqlInjectionChecker::new();
    let findings = checker.scan(source);

    assert_eq!(findings.len(), 2);
    assert_eq!(findings[0].category, "SQL Injection");
    assert!(matches!(findings[0].severity, Severity::Critical));
}

#[test]
fn test_security_xss_detector() {
    let source = r#"
    elemen.innerHTML = "<div>" + user_submitted_name + "</div>"
    eval("alert(" + input + ")")
    "#;

    let checker = XssChecker::new();
    let findings = checker.scan(source);

    assert_eq!(findings.len(), 2);
    assert_eq!(findings[0].category, "Cross-Site Scripting (XSS)");
}

#[test]
fn test_security_linter_full_report() {
    let source = r#"
    misal pass = "password = 'admin123456'"
    misal q = "SELECT * FROM data WHERE x = " + x
    "#;

    let linter = SecurityLinter::new();
    let findings = linter.scan(source, "main.wya");
    let report = linter.format_report(&findings);

    assert!(report.contains("🛡️ Security Scan Report"));
    assert!(report.contains("Summary:"));
    assert!(!findings.is_empty());
}
