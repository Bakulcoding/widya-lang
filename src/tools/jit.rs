use crate::ast::*;
use crate::error::Galat;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::time::Instant;

pub struct JitEngine {
    pub optimizations_enabled: bool,
}

impl JitEngine {
    pub fn new() -> Self {
        Self {
            optimizations_enabled: true,
        }
    }

    pub fn execute_jit(&self, source: &str) -> Result<f64, Galat> {
        let start = Instant::now();

        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let mut program = parser.parse()?;

        // Perform AST optimizations before execution
        if self.optimizations_enabled {
            self.optimize_program(&mut program);
        }

        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program)?;

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        Ok(elapsed)
    }

    fn optimize_program(&self, program: &mut Program) {
        for stmt in &mut program.statements {
            self.optimize_stmt(stmt);
        }
    }

    fn optimize_stmt(&self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Expression(expr) => self.optimize_expr(expr),
            Stmt::VarDecl { initializer: Some(expr), .. } => self.optimize_expr(expr),
            Stmt::FunctionDecl { body, .. } => {
                for s in body {
                    self.optimize_stmt(s);
                }
            }
            Stmt::Block(stmts, _) => {
                for s in stmts {
                    self.optimize_stmt(s);
                }
            }
            _ => {}
        }
    }

    fn optimize_expr(&self, expr: &mut Expr) {
        // Constant folding for binary arithmetic
        if let Expr::Binary { left, op, right, span } = expr {
            self.optimize_expr(left);
            self.optimize_expr(right);

            if let (Expr::Number(a, _), Expr::Number(b, _)) = (&**left, &**right) {
                let folded_val = match op {
                    BinaryOp::Add => Some(a + b),
                    BinaryOp::Subtract => Some(a - b),
                    BinaryOp::Multiply => Some(a * b),
                    BinaryOp::Divide if *b != 0.0 => Some(a / b),
                    _ => None,
                };

                if let Some(val) = folded_val {
                    *expr = Expr::Number(val, span.clone());
                }
            }
        }
    }
}

pub fn jalankan_jit_berkas(path: &std::path::PathBuf) {
    println!("⚡ Menjalankan skrip dengan Widya JIT Optimizing Engine: {}", path.display());
    let sumber = match std::fs::read_to_string(path) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("❌ Gagal membaca berkas: {}", e);
            return;
        }
    };

    let jit = JitEngine::new();
    match jit.execute_jit(&sumber) {
        Ok(durasi) => {
            println!("🚀 Eksekusi JIT Selesai dalam {:.2} ms!", durasi);
        }
        Err(e) => {
            eprintln!("❌ Galat JIT Runtime: {}", e);
        }
    }
}
