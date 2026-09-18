use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::value::Value;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::time::Instant;

pub fn start_repl() {
    println!("{}", "=========================================================".bright_blue());
    println!("{}", "   🇮🇩 Selamat Datang di Widya-Lang Interactive REPL v0.1.0".bright_cyan().bold());
    println!("{}", "   Ketik kode Widya atau '.bantuan' untuk perintah khusus.".white());
    println!("{}", "   Ketik '.keluar' atau tekan Ctrl+C untuk keluar.".white());
    println!("{}", "=========================================================".bright_blue());

    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Gagal menginisialisasi editor REPL: {}", e);
            return;
        }
    };

    let mut interpreter = Interpreter::new();
    let mut buffer = String::new();
    let mut open_braces: usize = 0;
    let mut show_time = false;
    let mut show_type = false;

    loop {
        let prompt = if buffer.is_empty() {
            "widya> ".bright_green().bold().to_string()
        } else {
            "...    ".bright_yellow().to_string()
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim();

                if buffer.is_empty() {
                    if trimmed == ".keluar" || trimmed == "exit" || trimmed == "quit" {
                        println!("{}", "Sampai jumpa! 👋 Terima kasih telah menggunakan Widya-Lang.".bright_green());
                        break;
                    } else if trimmed == ".bantuan" || trimmed == "help" {
                        print_help();
                        continue;
                    } else if trimmed == ".bersih" || trimmed == "clear" {
                        print!("\x1B[2J\x1B[1;1H");
                        continue;
                    } else if trimmed == ".waktu" {
                        show_time = !show_time;
                        let status = if show_time { "diaktifkan".bright_green() } else { "dinonaktifkan".bright_red() };
                        println!("⏱️  Pengukuran waktu eksekusi: {}", status);
                        continue;
                    } else if trimmed == ".tipe" {
                        show_type = !show_type;
                        let status = if show_type { "diaktifkan".bright_green() } else { "dinonaktifkan".bright_red() };
                        println!("🏷️  Penampil tipe data ekspresi: {}", status);
                        continue;
                    } else if trimmed == ".lingkungan" || trimmed == ".vars" {
                        print_environment(&interpreter);
                        continue;
                    } else if trimmed == ".contoh" {
                        print_examples();
                        continue;
                    } else if trimmed.is_empty() {
                        continue;
                    }
                }

                let _ = rl.add_history_entry(line.as_str());

                // Count braces to handle multiline input
                for c in line.chars() {
                    if c == '{' || c == '(' || c == '[' {
                        open_braces += 1;
                    } else if c == '}' || c == ')' || c == ']' {
                        open_braces = open_braces.saturating_sub(1);
                    }
                }

                buffer.push_str(&line);
                buffer.push('\n');

                if open_braces > 0 {
                    continue;
                }

                // Execute buffer
                let source = buffer.clone();
                buffer.clear();
                open_braces = 0;

                let start_time = Instant::now();

                let mut lexer = Lexer::new(&source);
                let tokens = match lexer.scan_tokens() {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("{}", e.format_dengan_sumber(&source).bright_red());
                        continue;
                    }
                };

                let mut parser = Parser::new(tokens);
                let program = match parser.parse() {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("{}", e.format_dengan_sumber(&source).bright_red());
                        continue;
                    }
                };

                match interpreter.interpret(&program) {
                    Ok(Value::Nil) => {
                        if show_time {
                            let duration = start_time.elapsed();
                            println!("{} {:?}", "⏱️  Waktu eksekusi:".bright_black(), duration);
                        }
                    }
                    Ok(val) => {
                        let duration = start_time.elapsed();
                        if show_type {
                            let type_name = val.type_name();
                            println!("{} {} {}", "=>".bright_cyan().bold(), val.to_debug_repr().bright_yellow(), format!("({})", type_name).bright_black());
                        } else {
                            println!("{} {}", "=>".bright_cyan().bold(), val.to_debug_repr().bright_yellow());
                        }
                        if show_time {
                            println!("{} {:?}", "⏱️  Waktu eksekusi:".bright_black(), duration);
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e.format_dengan_sumber(&source).bright_red());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                if !buffer.is_empty() {
                    buffer.clear();
                    open_braces = 0;
                    println!("{}", "(masukan dibatalkan)".yellow());
                } else {
                    println!("{}", "\nSampai jumpa! 👋".bright_green());
                    break;
                }
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "\nSampai jumpa! 👋".bright_green());
                break;
            }
            Err(err) => {
                eprintln!("Galat REPL: {:?}", err);
                break;
            }
        }
    }
}

fn print_environment(interpreter: &Interpreter) {
    println!("\n{}", "--- Variabel & Lingkungan Aktif ---".bright_cyan().bold());
    let bindings = interpreter.get_environment_bindings();
    if bindings.is_empty() {
        println!("{}", "  (Belum ada variabel atau fungsi lokal yang didefinisikan)".bright_black());
    } else {
        for (name, val) in bindings {
            println!("  {} {} = {} {}", "•".bright_blue(), name.bright_yellow().bold(), val.to_debug_repr(), format!("({})", val.type_name()).bright_black());
        }
    }
    println!();
}

fn print_examples() {
    println!("\n{}", "--- Contoh Kode Widya-Lang ---".bright_cyan().bold());
    println!("{}", "1. Deklarasi Variabel & List:".bright_yellow());
    println!("   misal skor = [85, 90, 95]");
    println!("   misal rata = skor.rata_rata()\n");

    println!("{}", "2. Fungsi & Rekursi:".bright_yellow());
    println!("   fungsi faktorial(n) {{");
    println!("       jika n <= 1 {{ kembalikan 1 }}");
    println!("       kembalikan n * faktorial(n - 1)");
    println!("   }}");
    println!("   faktorial(5)\n");

    println!("{}", "3. Pencocokan Pola (Pola):".bright_yellow());
    println!("   misal status = 200");
    println!("   pola status {{");
    println!("       200 => \"Berhasil!\",");
    println!("       404 => \"Tidak Ditemukan\",");
    println!("       _ => \"Status Lain\"");
    println!("   }}\n");
}

fn print_help() {
    println!("\n{}", "--- Bantuan Perintah Widya-Lang REPL ---".bright_cyan().bold());
    println!("  {:<16} - Menampilkan menu bantuan ini", ".bantuan".bright_yellow());
    println!("  {:<16} - Membersihkan layar terminal", ".bersih".bright_yellow());
    println!("  {:<16} - Menampilkan contoh sintaks dan fitur", ".contoh".bright_yellow());
    println!("  {:<16} - Menampilkan variabel dalam lingkungan sesi", ".lingkungan".bright_yellow());
    println!("  {:<16} - Mengaktifkan/menonaktifkan info tipe ekspresi", ".tipe".bright_yellow());
    println!("  {:<16} - Mengaktifkan/menonaktifkan timer eksekusi", ".waktu".bright_yellow());
    println!("  {:<16} - Keluar dari sesi REPL", ".keluar".bright_yellow());
    println!();
}
