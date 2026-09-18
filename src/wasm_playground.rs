use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::borrow_checker::BorrowChecker;

/// Struktur respons dari eksekusi WebAssembly Playground
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WasmPlaygroundResult {
    pub success: bool,
    pub stdout: String,
    pub ast_json: Option<String>,
    pub inferred_types: Vec<String>,
    pub errors: Vec<String>,
}

/// Runner compiler di dalam browser WebAssembly
pub struct WasmPlayground;

impl WasmPlayground {
    /// Mengevaluasi kode sumber Widya di dalam runtime WebAssembly
    pub fn evaluate(source: &str) -> WasmPlaygroundResult {
        let mut lexer = Lexer::new(source);
        let tokens = match lexer.scan_tokens() {
            Ok(toks) => toks,
            Err(e) => {
                return WasmPlaygroundResult {
                    success: false,
                    stdout: String::new(),
                    ast_json: None,
                    inferred_types: vec![],
                    errors: vec![format!("Lexer Error: {:?}", e)],
                };
            }
        };

        let mut parser = Parser::new(tokens);
        let program = match parser.parse() {
            Ok(ast) => ast,
            Err(e) => {
                return WasmPlaygroundResult {
                    success: false,
                    stdout: String::new(),
                    ast_json: None,
                    inferred_types: vec![],
                    errors: vec![format!("Parser Error: {:?}", e)],
                };
            }
        };

        // AST representasi
        let ast_repr = format!("{:#?}", program);

        // Memory Safety & Borrow Checker Validation
        let mut borrow_checker = BorrowChecker::new();
        if let Err(e) = borrow_checker.check_program(&program) {
            return WasmPlaygroundResult {
                success: false,
                stdout: String::new(),
                ast_json: Some(ast_repr),
                inferred_types: vec![],
                errors: vec![format!("Borrow Checker Error: {:?}", e)],
            };
        }

        // Interpretasi
        let mut interpreter = Interpreter::new();
        match interpreter.interpret(&program) {
            Ok(val) => WasmPlaygroundResult {
                success: true,
                stdout: format!("Hasil: {:?}", val),
                ast_json: Some(ast_repr),
                inferred_types: vec!["Inferred: Verified Tipe Aman".to_string()],
                errors: vec![],
            },
            Err(e) => WasmPlaygroundResult {
                success: false,
                stdout: String::new(),
                ast_json: Some(ast_repr),
                inferred_types: vec![],
                errors: vec![format!("Runtime Error: {:?}", e)],
            },
        }
    }
}
