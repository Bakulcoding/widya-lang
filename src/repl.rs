use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::value::Value;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub fn start_repl() {
    println!("{}", "================================================".bright_blue());
    println!("{}", "   🇮🇩 Selamat Datang di Widya-Lang REPL v0.1.0".bright_cyan().bold());
    println!("{}", "   Ketik kode Widya atau '.bantuan' untuk bantuan.".white());
    println!("{}", "   Ketik '.keluar' atau tekan Ctrl+C untuk keluar.".white());
    println!("{}", "================================================".bright_blue());

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
                        println!("{}", "Sampai jumpa! 👋".bright_green());
                        break;
                    } else if trimmed == ".bantuan" || trimmed == "help" {
                        print_help();
                        continue;
                    } else if trimmed == ".bersih" || trimmed == "clear" {
                        print!("\x1B[2J\x1B[1;1H");
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
                    Ok(Value::Nil) => {}
                    Ok(val) => {
                        println!("{} {}", "=>".bright_cyan().bold(), val.to_debug_repr().bright_yellow());
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
                    println!("{}", "(dibatalkan)".yellow());
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

fn print_help() {
    println!("\n{}", "--- Bantuan Widya-Lang REPL ---".bright_cyan().bold());
    println!("  {}  - Menampilkan menu bantuan ini", ".bantuan".bright_yellow());
    println!("  {}   - Membersihkan layar terminal", ".bersih".bright_yellow());
    println!("  {}   - Keluar dari sesi REPL", ".keluar".bright_yellow());
    println!("\nContoh Sintaks Widya:");
    println!("  misal nama = \"Widya\"");
    println!("  cetak(\"Halo, \" + nama + \"!\")");
    println!("  fungsi kuadrat(x) {{ kembalikan x * x }}");
    println!("  kuadrat(5)\n");
}
