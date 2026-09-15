use colored::*;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use crate::ast::Stmt;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn jalankan_benchmark(path: &PathBuf) {
    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas: {}", e);
            return;
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Galat Pemindai: {}", e);
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

    println!("\n{}", "⏱️  Menjalankan Widya Micro-Benchmark Suite...".bright_cyan().bold());
    println!("📂 Target: {}\n", path.display().to_string().bright_yellow());

    let iter_count: u32 = 10_000;
    let mut bench_found = false;

    // Setup base environment with all module functions & structs
    let mut base_interp = Interpreter::new();
    for stmt in &program.statements {
        if let Stmt::FunctionDecl { attributes, .. } = stmt {
            let is_bench = attributes.iter().any(|a| a.name == "tolak_ukur" || a.name == "bench");
            if !is_bench {
                let _ = base_interp.execute(stmt);
            }
        } else if !matches!(stmt, Stmt::Expression(_)) {
            let _ = base_interp.execute(stmt);
        }
    }

    for stmt in &program.statements {
        if let Stmt::FunctionDecl { name, body, attributes, .. } = stmt {
            let is_bench = attributes.iter().any(|a| a.name == "tolak_ukur" || a.name == "bench");
            if is_bench {
                bench_found = true;
                print!("  tolak_ukur {} ({} iterasi) ... ", name.bright_white().bold(), iter_count);

                let mut interp = Interpreter::new();
                interp.environment = base_interp.environment.clone();
                interp.globals = base_interp.globals.clone();
                let start = Instant::now();
                for _ in 0..iter_count {
                    for s in body {
                        let _ = interp.execute(s);
                    }
                }
                let total_time = start.elapsed();
                let ns_per_op = total_time.as_nanos() as f64 / iter_count as f64;
                let ops_per_sec = 1_000_000_000.0 / ns_per_op.max(1.0);

                println!(
                    "{} ({:.2} ns/op | {:.0} ops/detik)",
                    "SELESAI".bright_green().bold(),
                    ns_per_op,
                    ops_per_sec
                );
            }
        }
    }

    if !bench_found {
        println!("ℹ️  Tidak ada fungsi bertanda #[tolak_ukur] atau #[bench] ditemukan.");
    }
    println!("\n{}", "=========================================================".bright_cyan());
}
