use colored::*;
use std::fs;
use std::path::PathBuf;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn format_berkas(path: &PathBuf, overwrite: bool) {
    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Gagal membaca berkas '{}': {}", "❌".bright_red(), path.display(), e);
            return;
        }
    };

    println!("🎨 {} Memformat kode sumber '{}'...", "Widya Formatter:".bright_cyan().bold(), path.display().to_string().bright_yellow());

    let mut formatted = String::new();
    let lines = sumber.lines();
    let mut indent_level: usize = 0;

    for raw_line in lines {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            formatted.push('\n');
            continue;
        }

        // Adjust indent for closing brace
        let leading_close = trimmed.starts_with('}') || trimmed.starts_with(']');
        let effective_indent = if leading_close && indent_level > 0 {
            indent_level - 1
        } else {
            indent_level
        };

        let indent_str = "    ".repeat(effective_indent);
        formatted.push_str(&indent_str);
        formatted.push_str(trimmed);
        formatted.push('\n');

        // Count braces to adjust subsequent indentation
        let open_count = trimmed.chars().filter(|c| *c == '{' || *c == '[').count();
        let close_count = trimmed.chars().filter(|c| *c == '}' || *c == ']').count();

        if open_count > close_count {
            indent_level += open_count - close_count;
        } else if close_count > open_count {
            indent_level = indent_level.saturating_sub(close_count - open_count);
        }
    }

    if overwrite {
        if let Err(e) = fs::write(path, &formatted) {
            eprintln!("Gagal menyimpan pemformatan ke berkas: {}", e);
        } else {
            println!("✅ Berkas berhasil diformat dan disimpan ulang secara konsisten.");
        }
    } else {
        println!("{}", "=== HASIL PEMFORMATAN STANDAR WIDYA ===".bright_green().bold());
        println!("{}", formatted);
    }
}

pub fn periksa_linter(path: &PathBuf) {
    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas: {}", e);
            return;
        }
    };

    println!("🔍 {} Memeriksa potensi galat & best-practices pada '{}'...", "Widya Linter:".bright_cyan().bold(), path.display().to_string().bright_yellow());

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

    let mut warnings_found = 0;
    for stmt in &program.statements {
        if let crate::ast::Stmt::VarDecl { name, span, initializer, .. } = stmt {
            if name.starts_with('_') {
                continue;
            }
            if initializer.is_none() {
                println!("  {} [Baris {}, Kolom {}] Variabel '{}' dideklarasikan tanpa inisialisasi awal", "Peringatan:".bright_yellow(), span.line, span.column, name);
                warnings_found += 1;
            }
        }
    }

    if warnings_found == 0 {
        println!("{}", "✨ Kode bersih sempurna! Tidak ditemukan masalah linter.".bright_green().bold());
    } else {
        println!("⚠️  Ditemukan {} peringatan gaya kode.", warnings_found);
    }
}
