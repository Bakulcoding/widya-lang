use colored::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use crate::ast::Stmt;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn jalankan_test_suite(path: &PathBuf) {
    let mut files_to_test = Vec::new();

    if path.is_dir() {
        collect_wya_files(path, &mut files_to_test);
    } else if path.exists() {
        files_to_test.push(path.clone());
    } else {
        // Fallback: check tests/ or contoh/
        let tests_dir = PathBuf::from("tests");
        if tests_dir.exists() && tests_dir.is_dir() {
            collect_wya_files(&tests_dir, &mut files_to_test);
        }
        if files_to_test.is_empty() {
            let contoh_dir = PathBuf::from("contoh");
            if contoh_dir.exists() && contoh_dir.is_dir() {
                collect_wya_files(&contoh_dir, &mut files_to_test);
            }
        }
        if files_to_test.is_empty() {
            eprintln!("{} Path '{}' tidak ditemukan dan tidak ada berkas .wya ditemukan", "❌".bright_red(), path.display());
            return;
        }
    }

    println!("\n{}", "🧪 Menjalankan Unit Test Suite (Widya Test Runner)...".bright_cyan().bold());
    println!("📂 Target: {} ({} berkas ditemukan)\n", path.display().to_string().bright_yellow(), files_to_test.len());

    let mut total_tests = 0;
    let mut total_passed = 0;
    let mut total_failed = 0;
    let start_total = Instant::now();

    for file in &files_to_test {
        let (tests, passed, failed) = jalankan_satu_berkas(file);
        total_tests += tests;
        total_passed += passed;
        total_failed += failed;
    }

    let total_dur = start_total.elapsed();
    println!("\n{}", "=========================================================".bright_cyan());
    if total_tests == 0 {
        println!("ℹ️  Tidak ada fungsi bertanda #[uji] atau #[test] ditemukan pada seluruh berkas.");
    } else if total_failed == 0 {
        println!("🎉 {} Seluruh {} pengujian berhasil lolos dalam {:.2?}!", "HASIL: SEMPURNA".bright_green().bold(), total_passed, total_dur);
    } else {
        println!("⚠️  {} {} lolos, {} gagal dari {} pengujian ({:.2?}).", "HASIL: ADA GALAT".bright_red().bold(), total_passed, total_failed, total_tests, total_dur);
    }
    println!("{}\n", "=========================================================".bright_cyan());
}

fn collect_wya_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_wya_files(&path, files);
            } else if path.extension().is_some_and(|ext| ext == "wya") {
                files.push(path);
            }
        }
    }
}

fn jalankan_satu_berkas(path: &Path) -> (usize, usize, usize) {
    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Gagal membaca berkas '{}': {}", "❌".bright_red(), path.display(), e);
            return (0, 0, 0);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Galat Pemindai Token di '{}': {}", path.display(), e);
            return (0, 0, 0);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Galat Parser di '{}': {}", path.display(), e);
            return (0, 0, 0);
        }
    };

    let mut test_count = 0;
    let mut pass_count = 0;
    let mut fail_count = 0;

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

    let mut printed_header = false;

    for stmt in &program.statements {
        if let Stmt::FunctionDecl { name, body, attributes, .. } = stmt {
            let is_test = attributes.iter().any(|a| a.name == "uji" || a.name == "test");
            if is_test {
                if !printed_header {
                    println!("📄 Berkas: {}", path.display().to_string().bright_cyan());
                    printed_header = true;
                }
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

    (test_count, pass_count, fail_count)
}
