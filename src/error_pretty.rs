use std::collections::HashMap;

pub struct ErrorFormatter {
    source: String,
    source_lines: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PrettyError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub error_code: String,
    pub suggestion: Option<String>,
    pub documentation_url: Option<String>,
    pub example: Option<String>,
}

impl ErrorFormatter {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            source_lines: source.lines().map(|s| s.to_string()).collect(),
        }
    }
    
    pub fn format_error(&self, error: &PrettyError) -> String {
        let mut output = String::new();
        
        output.push_str(&self.format_header(&error.error_code, &error.message));
        output.push_str(&self.format_source_context(error.line, error.column));
        
        if let Some(suggestion) = &error.suggestion {
            output.push_str(&self.format_suggestion(suggestion));
        }
        
        if let Some(example) = &error.example {
            output.push_str(&self.format_example(example));
        }
        
        if let Some(doc_url) = &error.documentation_url {
            output.push_str(&self.format_documentation(doc_url));
        }
        
        output
    }
    
    fn format_header(&self, code: &str, message: &str) -> String {
        format!("\n❌ Error [{}]: {}\n\n", code, message)
    }
    
    fn format_source_context(&self, line: usize, column: usize) -> String {
        let mut output = String::new();
        
        if line == 0 || line > self.source_lines.len() {
            return output;
        }
        
        let line_idx = line - 1;
        let context_lines = 2;
        
        let start = line_idx.saturating_sub(context_lines);
        let end = (line_idx + context_lines + 1).min(self.source_lines.len());
        
        let line_num_width = end.to_string().len();
        
        for i in start..end {
            let line_num = i + 1;
            let prefix = if i == line_idx {
                "→ "
            } else {
                "  "
            };
            
            output.push_str(&format!(
                "  {}{:>width$} | {}\n",
                prefix,
                line_num,
                self.source_lines[i],
                width = line_num_width
            ));
            
            if i == line_idx {
                let underline = self.create_underline(column, self.source_lines[i].len());
                output.push_str(&format!(
                    "  {}{}   | {}\n",
                    " ".repeat(line_num_width),
                    " ".repeat(3),
                    underline
                ));
            }
        }
        
        output.push('\n');
        output
    }
    
    fn create_underline(&self, column: usize, line_len: usize) -> String {
        if column == 0 || column > line_len {
            return "^".to_string();
        }
        
        let spaces = column - 1;
        format!("{}^", " ".repeat(spaces))
    }
    
    fn format_suggestion(&self, suggestion: &str) -> String {
        format!("💡 Saran: {}\n\n", suggestion)
    }
    
    fn format_example(&self, example: &str) -> String {
        format!("📝 Contoh benar:\n   {}\n\n", example)
    }
    
    fn format_documentation(&self, url: &str) -> String {
        format!("📖 Dokumentasi: {}\n", url)
    }
}

pub struct ErrorSuggestions;

impl ErrorSuggestions {
    pub fn get_suggestion(error_type: &str, context: Option<&str>) -> Option<(String, Option<String>, Option<String>)> {
        match error_type {
            "expected_expression" => Some((
                "Expected expression after operator or keyword".to_string(),
                Some("Tambahkan nilai setelah operator".to_string()),
                Some("x = 10  atau  fungsi(param)".to_string()),
            )),
            
            "expected_closing_paren" => Some((
                "Missing closing parenthesis ')'".to_string(),
                Some("Tutup kurung buka dengan ')'".to_string()),
                Some("fungsi(a, b)".to_string()),
            )),
            
            "expected_closing_brace" => Some((
                "Missing closing brace '}'".to_string(),
                Some("Tutup blok kode dengan '}'".to_string()),
                Some("jika x { ... }".to_string()),
            )),
            
            "expected_closing_bracket" => Some((
                "Missing closing bracket ']'".to_string(),
                Some("Tutup array dengan ']'".to_string()),
                Some("[1, 2, 3]".to_string()),
            )),
            
            "undefined_variable" => {
                if let Some(var_name) = context {
                    Some((
                        format!("Variable '{}' is not defined", var_name),
                        Some(format!("Definisikan variabel terlebih dahulu: var {} = nilai", var_name)),
                        Some(format!("var {} = 10", var_name)),
                    ))
                } else {
                    Some((
                        "Variable is not defined".to_string(),
                        Some("Definisikan variabel sebelum digunakan".to_string()),
                        Some("var nama = \"Widya\"".to_string()),
                    ))
                }
            },
            
            "undefined_function" => {
                if let Some(func_name) = context {
                    Some((
                        format!("Function '{}' is not defined", func_name),
                        Some(format!("Definisikan fungsi terlebih dahulu: fungsi {}() {{ ... }}", func_name)),
                        Some(format!("fungsi {}() {{ kembalikan 1 }}", func_name)),
                    ))
                } else {
                    Some((
                        "Function is not defined".to_string(),
                        Some("Definisikan fungsi sebelum digunakan".to_string()),
                        Some("fungsi hitung() { kembalikan 1 }".to_string()),
                    ))
                }
            },
            
            "type_mismatch" => Some((
                "Type mismatch in operation or assignment".to_string(),
                Some("Pastikan tipe data sesuai".to_string()),
                Some("angka + angka  atau  teks + teks".to_string()),
            )),
            
            "invalid_assignment" => Some((
                "Cannot assign to this expression".to_string(),
                Some("Assignment hanya bisa ke variabel".to_string()),
                Some("var x = 10  (bukan  10 = x)".to_string()),
            )),
            
            "invalid_operator" => Some((
                "Invalid operator for this type".to_string(),
                Some("Gunakan operator yang sesuai dengan tipe data".to_string()),
                Some("Angka: + - * /, Teks: +, Boolean: dan atau".to_string()),
            )),
            
            "missing_function_body" => Some((
                "Function has no body".to_string(),
                Some("Tambahkan body fungsi dalam { }".to_string()),
                Some("fungsi nama() { kembalikan nilai }".to_string()),
            )),
            
            "invalid_parameter" => Some((
                "Invalid function parameter".to_string(),
                Some("Parameter harus berupa nama variabel".to_string()),
                Some("fungsi hitung(a, b) { ... }".to_string()),
            )),
            
            "division_by_zero" => Some((
                "Division by zero".to_string(),
                Some("Periksa pembagi sebelum operasi".to_string()),
                Some("jika b != 0 { hasil = a / b }".to_string()),
            )),
            
            "index_out_of_bounds" => Some((
                "Index out of bounds".to_string(),
                Some("Gunakan index yang valid (0 sampai panjang-1)".to_string()),
                Some("arr[0]  atau  arr[panjang(arr) - 1]".to_string()),
            )),
            
            _ => None,
        }
    }
    
    pub fn get_error_code(error_type: &str) -> String {
        match error_type {
            "expected_expression" => "E001".to_string(),
            "expected_closing_paren" => "E002".to_string(),
            "expected_closing_brace" => "E003".to_string(),
            "expected_closing_bracket" => "E004".to_string(),
            "undefined_variable" => "E101".to_string(),
            "undefined_function" => "E102".to_string(),
            "type_mismatch" => "E201".to_string(),
            "invalid_assignment" => "E202".to_string(),
            "invalid_operator" => "E203".to_string(),
            "missing_function_body" => "E301".to_string(),
            "invalid_parameter" => "E302".to_string(),
            "division_by_zero" => "E401".to_string(),
            "index_out_of_bounds" => "E402".to_string(),
            _ => "E000".to_string(),
        }
    }
    
    pub fn get_documentation_url(error_code: &str) -> String {
        format!("https://widya-lang.org/docs/errors/{}", error_code)
    }
}

pub fn format_pretty_error(
    error_type: &str,
    message: &str,
    line: usize,
    column: usize,
    source: &str,
    context: Option<&str>,
) -> String {
    let formatter = ErrorFormatter::new(source);
    
    let (suggestion, example, _): (Option<String>, Option<String>, Option<String>) = 
        ErrorSuggestions::get_suggestion(error_type, context)
            .map(|(s, e, ex)| (Some(s), e, ex))
            .unwrap_or((None, None, None));
    
    let error_code = ErrorSuggestions::get_error_code(error_type);
    let doc_url = ErrorSuggestions::get_documentation_url(&error_code);
    
    let pretty_error = PrettyError {
        message: message.to_string(),
        line,
        column,
        error_code,
        suggestion,
        example,
        documentation_url: Some(doc_url),
    };
    
    formatter.format_error(&pretty_error)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_formatter() {
        let source = "var x =\nvar y = 10";
        let formatter = ErrorFormatter::new(source);
        
        let error = PrettyError {
            message: "Expected expression after '='".to_string(),
            line: 1,
            column: 8,
            error_code: "E001".to_string(),
            suggestion: Some("Tambahkan nilai setelah '='".to_string()),
            example: Some("var x = 10".to_string()),
            documentation_url: Some("https://widya-lang.org/docs/errors/E001".to_string()),
        };
        
        let output = formatter.format_error(&error);
        assert!(output.contains("E001"));
        assert!(output.contains("Expected expression"));
    }
    
    #[test]
    fn test_error_suggestions() {
        let result = ErrorSuggestions::get_suggestion("undefined_variable", Some("x"));
        assert!(result.is_some());
        
        let (suggestion, example, _) = result.unwrap();
        assert!(suggestion.contains("x"));
        assert!(example.is_some());
    }
}
