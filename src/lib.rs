pub mod ast;
pub mod compiler;
pub mod environment;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod stdlib;
pub mod token;
pub mod value;
pub mod lsp;
pub mod studio;
pub mod pm;
pub mod llvm;
pub mod wasm;
pub mod gpu;
pub mod ebpf;
pub mod borrow_checker;
pub mod tools;

use borrow_checker::BorrowChecker;
use error::Galat;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use value::Value;

/// Menjalankan kode sumber Widya-Lang dari string dengan validasi keamanan memori
pub fn jalankan(sumber: &str) -> Result<Value, Galat> {
    let mut lexer = Lexer::new(sumber);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    // Static Memory Safety & Borrow Checking Pass
    let mut borrow_checker = BorrowChecker::new();
    borrow_checker.check_program(&program)?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program)
}
