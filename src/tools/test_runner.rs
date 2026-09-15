use colored::*;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use crate::ast::Stmt;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn jalankan_test_suite(path: &PathBuf) {
    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Gagal membaca berkas '{}': {}", "❌".bright_red(), path.display(), e);
            return;
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Galat Pemindai Token: {}", e);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Galat Parser: {}", e);
            return;
        }
    };

    println!("\n{}", "🧪 Menjalankan Unit Test Suite (Widya Test Runner)...".bright_cyan().bold());
    println!("📂 Berkas Target: {}\n", path.display().to_string().bright_yellow());

    let mut test_count = 0;
    let mut pass_count = 0;
    let mut fail_count = 0;
    let start_total = Instant::now();

    // Setup base environment with all module functions & structs
    let mut base_interp = Interpreter::new();
    for stmt in &program.statements {
        if let Stmt::FunctionDecl { attributes, .. } = stmt {
            let is_test = attributes.iter().any(|a| a.name == "uji" || a.name == "test");
            if !is_test {
                let _ = base_interp.execute(stmt);
            }
        } else if !matches!(stmt, Stmt::Expression(_)) {
            let _ = base_interp.execute(stmt);
        }
    }

    for stmt in &program.statements {
        if let Stmt::FunctionDecl { name, body, attributes, .. } = stmt {
            let is_test = attributes.iter().any(|a| a.name == "uji" || a.name == "test");
            if is_test {
                test_count += 1;
                print!("  uji {} ... ", name.bright_white().bold());
                
                let mut interp = Interpreter::new();
                interp.environment = base_interp.environment.clone();
                interp.globals = base_interp.globals.clone();
                let start_test = Instant::now();
                let mut failed = false;
                let mut err_msg = String::new();

                for s in body {
                    if let Err(e) = interp.execute(s) {
                        failed = true;
                        err_msg = format!("{}", e);
                        break;
                    }
                }

                let duration = start_test.elapsed();
                if failed {
                    fail_count += 1;
                    println!("{} ({:.2?})", "GAGAL / FAIL".bright_red().bold(), duration);
                    println!("    └─ {}", err_msg.bright_red());
                } else {
                    pass_count += 1;
                    println!("{} ({:.2?})", "LOLOS / OK".bright_green().bold(), duration);
                }
            }
        }
    }

    let total_dur = start_total.elapsed();
    println!("\n{}", "=========================================================".bright_cyan());
    if test_count == 0 {
        println!("ℹ️  Tidak ada fungsi bertanda #[uji] atau #[test] ditemukan.");
    } else if fail_count == 0 {
        println!("🎉 {} Seluruh {} pengujian berhasil lolos dalam {:.2?}!", "HASIL: SEMPURNA".bright_green().bold(), pass_count, total_dur);
    } else {
        println!("⚠️  {} {} lolos, {} gagal dari {} pengujian ({:.2?}).", "HASIL: ADA GALAT".bright_red().bold(), pass_count, fail_count, test_count, total_dur);
    }
    println!("{}\n", "=========================================================".bright_cyan());
}
