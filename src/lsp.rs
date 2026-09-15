use crate::error::Galat;
use crate::lexer::Lexer;
use crate::parser::Parser;
use serde_json::json;

pub struct WidyaLsp;

impl WidyaLsp {
    pub fn get_diagnostics(source: &str) -> Vec<serde_json::Value> {
        let mut diagnostics = Vec::new();

        // 1. Lexing phase check
        let mut lexer = Lexer::new(source);
        let tokens = match lexer.scan_tokens() {
            Ok(toks) => toks,
            Err(e) => {
                let (line, col, msg) = match e {
                    Galat::Sintaks { baris, kolom, pesan } | Galat::Runtime { baris, kolom, pesan } => (baris, kolom, pesan),
                    _ => (1, 1, format!("{}", e)),
                };
                diagnostics.push(json!({
                    "line": line,
                    "col": col,
                    "message": format!("Sintaks Lexer: {}", msg),
                    "severity": "error"
                }));
                return diagnostics;
            }
        };

        // 2. Parsing phase check
        let mut parser = Parser::new(tokens);
        if let Err(e) = parser.parse() {
            let (line, col, msg) = match e {
                Galat::Sintaks { baris, kolom, pesan } | Galat::Runtime { baris, kolom, pesan } => (baris, kolom, pesan),
                _ => (1, 1, format!("{}", e)),
            };
            diagnostics.push(json!({
                "line": line,
                "col": col,
                "message": format!("Sintaks Parser: {}", msg),
                "severity": "error"
            }));
        }

        diagnostics
    }
}
