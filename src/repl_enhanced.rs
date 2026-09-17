use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::value::Value;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::time::Instant;

pub struct ReplConfig {
    pub show_time: bool,
    pub show_types: bool,
}

impl Default for ReplConfig {
    fn default() -> Self {
        Self {
            show_time: false,
            show_types: false,
        }
    }
}

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
    let mut config = ReplConfig::default();

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
                    if handle_repl_command(trimmed, &mut interpreter, &config) {
                        continue;
                    }
                    
                    if trimmed.is_empty() {
                        continue;
                    }
                }

                let _ = rl.add_history_entry(line.as_str());

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

                let source = buffer.clone();
                buffer.clear();
                open_braces = 0;

                let start = Instant::now();
                
                let mut lexer = Lexer::new(&source);
                let tokens = match lexer.scan_tokens() {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("{}", format_pretty_error(&e, &source));
                        continue;
                    }
                };

                let mut parser = Parser::new(tokens);
                let program = match parser.parse() {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("{}", format_pretty_error(&e, &source));
                        continue;
                    }
                };

                match interpreter.interpret(&program) {
                    Ok(Value::Nil) => {}
                    Ok(val) => {
                        let type_info = if config.show_types {
                            format!(" [{}]", val.type_name()).bright_black().to_string()
                        } else {
                            String::new()
                        };
                        
                        let time_info = if config.show_time {
                            let elapsed = start.elapsed();
                            format!(" ({:.2?})", elapsed).bright_black().to_string()
                        } else {
                            String::new()
                        };
                        
                        println!("{} {}{}{}", 
                            "=>".bright_cyan().bold(), 
                            val.to_debug_repr().bright_yellow(),
                            type_info,
                            time_info
                        );
                    }
                    Err(e) => {
                        eprintln!("{}", format_pretty_error(&e, &source));
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "^C".yellow());
                buffer.clear();
                open_braces = 0;
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "Sampai jumpa! 👋".bright_green());
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }
}

fn handle_repl_command(input: &str, interpreter: &mut Interpreter, config: &ReplConfig) -> bool {
    match input {
        ".keluar" | "exit" | "quit" => {
            println!("{}", "Sampai jumpa! 👋".bright_green());
            std::process::exit(0);
        }
        ".bantuan" | "help" => {
            print_help();
        }
        ".bersih" | "clear" => {
            print!("\x1B[2J\x1B[1;1H");
        }
        ".waktu" | ".time" => {
            println!("{} Mode waktu {}", 
                "⏱️".bright_cyan(),
                if config.show_time { "dimatikan" } else { "diaktifkan" }
            );
            // Toggle would need mutable config
        }
        ".tipe" | ".type" => {
            println!("{} Mode tipe {}", 
                "📝".bright_cyan(),
                if config.show_types { "dimatikan" } else { "diaktifkan" }
            );
        }
        ".contoh" | ".examples" => {
            print_examples();
        }
        ".versi" | ".version" => {
            println!("Widya-Lang v0.1.0");
            println!("Rust {}", rustc_version_runtime::version());
        }
        ".lingkungan" | ".env" => {
            show_environment(interpreter);
        }
        _ => return false,
    }
    true
}

fn print_help() {
    println!("\n{}", "📚 Bantuan Widya-Lang REPL".bright_cyan().bold());
    println!("{}", "─".repeat(40).bright_black());
    
    println!("\n{}", "Perintah REPL:".bright_yellow().bold());
    println!("  {}  Tampilkan bantuan ini", ".bantuan".bright_green());
    println!("  {}       Keluar dari REPL", ".keluar".bright_green());
    println!("  {}       Bersihkan layar", ".bersih".bright_green());
    println!("  {}        Tampilkan waktu eksekusi", ".waktu".bright_green());
    println!("  {}         Tampilkan tipe nilai", ".tipe".bright_green());
    println!("  {}      Tampilkan contoh kode", ".contoh".bright_green());
    println!("  {}      Tampilkan versi", ".versi".bright_green());
    println!("  {}          Tampilkan variabel yang didefinisikan", ".lingkungan".bright_green());
    
    println!("\n{}", "Kata Kunci:".bright_yellow().bold());
    println!("  {}      Buat variabel atau konstanta", "var".bright_green());
    println!("  {}       Definisikan fungsi", "fungsi".bright_green());
    println!("  {}        Kondisional if", "jika".bright_green());
    println!("  {}      Kondisional else", "kalau_tidak".bright_green());
    println!("  {}       Perulangan for", "untuk".bright_green());
    println!("  {}    Perulangan while", "selama".bright_green());
    println!("  {}  Kembalikan nilai dari fungsi", "kembalikan".bright_green());
    println!("  {}       Cetak ke layar", "cetak".bright_green());
    
    println!("\n{}", "Tipe Data:".bright_yellow().bold());
    println!("  {}      Angka bulat atau desimal", "Angka".bright_cyan());
    println!("  {}       Teks dalam tanda kutip", "Teks".bright_cyan());
    println!("  {}     Benar atau Salah", "Boolean".bright_cyan());
    println!("  {}      Daftar nilai", "Daftar".bright_cyan());
    println!("  {}        Pasangan kunci-nilai", "Peta".bright_cyan());
    
    println!("\n{}", "Operator:".bright_yellow().bold());
    println!("  {}         Penugasan", "=".bright_magenta());
    println!("  {}      Penjumlahan", "+".bright_magenta());
    println!("  {}      Pengurangan", "-".bright_magenta());
    println!("  {}      Perkalian", "*".bright_magenta());
    println!("  {}        Pembagian", "/".bright_magenta());
    println!("  {}      Modulo/Sisa", "%".bright_magenta());
    println!("  {}   Sama dengan", "==".bright_magenta());
    println!("  {} Tidak sama dengan", "!=".bright_magenta());
    println!("  {}   Lebih besar", ">".bright_magenta());
    println!("  {}      Lebih kecil", "<".bright_magenta());
    
    println!("\n{}", "Tips:".bright_yellow().bold());
    println!("  • Ketik kode langsung dan tekan Enter");
    println!("  • Gunakan Tab untuk auto-complete (coming soon)");
    println!("  • Gunakan Ctrl+C untuk membatalkan");
    println!("  • Gunakan Ctrl+D untuk keluar");
    println!();
}

fn print_examples() {
    println!("\n{}", "📝 Contoh Kode Widya".bright_cyan().bold());
    println!("{}", "─".repeat(40).bright_black());
    
    println!("\n{}", "1. Variabel:".bright_yellow());
    println!("   {}", r#"nama = "Widya""#.bright_green());
    println!("   {}", r#"umur = 25"#.bright_green());
    println!("   {}", r#"cetak(nama, umur)"#.bright_green());
    
    println!("\n{}", "2. Fungsi:".bright_yellow());
    println!("   {}", r#"fungsi tambah(a, b) {"#.bright_green());
    println!("   {}", r#"    kembalikan a + b"#.bright_green());
    println!("   {}", r#"}"#.bright_green());
    println!("   {}", r#"cetak(tambah(5, 3))"#.bright_green());
    
    println!("\n{}", "3. Kondisi:".bright_yellow());
    println!("   {}", r#"nilai = 85"#.bright_green());
    println!("   {}", r#"jika nilai >= 80 {"#.bright_green());
    println!("   {}", r#"    cetak("Lulus")"#.bright_green());
    println!("   {}", r#"} kalau_tidak {"#.bright_green());
    println!("   {}", r#"    cetak("Tidak Lulus")"#.bright_green());
    println!("   {}", r#"}"#.bright_green());
    
    println!("\n{}", "4. Perulangan:".bright_yellow());
    println!("   {}", r#"untuk i dalam rentang(1, 6) {"#.bright_green());
    println!("   {}", r#"    cetak(i)"#.bright_green());
    println!("   {}", r#"}"#.bright_green());
    
    println!("\n{}", "5. Daftar:".bright_yellow());
    println!("   {}", r#"buah = ["apel", "jeruk", "mangga"]"#.bright_green());
    println!("   {}", r#"cetak(buah[0])"#.bright_green());
    
    println!("\n{}", "6. Peta:".bright_yellow());
    println!("   {}", r#"orang = {"nama": "Budi", "umur": 30}"#.bright_green());
    println!("   {}", r#"cetak(orang["nama"])"#.bright_green());
    
    println!();
}

fn show_environment(interpreter: &Interpreter) {
    println!("\n{}", "📦 Lingkungan Variabel".bright_cyan().bold());
    println!("{}", "─".repeat(40).bright_black());
    
    let vars = interpreter.get_global_variables();
    
    if vars.is_empty() {
        println!("{}", "  (kosong)".bright_black().italic());
    } else {
        for (name, value) in vars {
            println!("  {} {} {} {}", 
                name.bright_green(),
                "=".bright_black(),
                value.to_debug_repr().bright_yellow(),
                format!("[{}]", value.type_name()).bright_black()
            );
        }
    }
    
    println!();
}

fn format_pretty_error(error: &crate::error::Galat, source: &str) -> String {
    use crate::error::Galat;
    
    let (line, col, msg) = match error {
        Galat::Sintaks { baris, kolom, pesan } | 
        Galat::Runtime { baris, kolom, pesan } => (*baris, *kolom, pesan.clone()),
        _ => (1, 1, format!("{}", error)),
    };
    
    let mut output = String::new();
    output.push_str(&format!("\n❌ {}\n\n", msg.bright_red().bold()));
    
    let lines: Vec<&str> = source.lines().collect();
    if line > 0 && line <= lines.len() {
        let line_num = line;
        let col_num = col;
        
        if line_num > 1 {
            output.push_str(&format!("   {} | {}\n", line_num - 1, lines[line_num - 2].bright_black()));
        }
        
        output.push_str(&format!("   {} | {}\n", line_num, lines[line_num - 1]));
        output.push_str(&format!("     {}{}\n", 
            " ".repeat(col_num.max(1) - 1),
            "^".repeat(1).bright_red()
        ));
        
        if line_num < lines.len() {
            output.push_str(&format!("   {} | {}\n", line_num + 1, lines[line_num].bright_black()));
        }
    }
    
    let suggestion = get_error_suggestion(&msg);
    if let Some(sugg) = suggestion {
        output.push_str(&format!("\n💡 Saran: {}\n", sugg.bright_yellow()));
    }
    
    output
}

fn get_error_suggestion(error_msg: &str) -> Option<String> {
    if error_msg.contains("Expected") {
        if error_msg.contains("expression") {
            return Some("Tambahkan nilai setelah operator. Contoh: x = 10".to_string());
        }
        if error_msg.contains("')'") {
            return Some("Tutup kurung dengan ')'. Contoh: fungsi(a, b)".to_string());
        }
        if error_msg.contains("'}'") {
            return Some("Tutup blok dengan '}'. Contoh: jika x { ... }".to_string());
        }
    }
    
    if error_msg.contains("Undefined variable") {
        return Some("Pastikan variabel sudah didefinisikan. Contoh: nama = \"Widya\"".to_string());
    }
    
    if error_msg.contains("Undefined function") {
        return Some("Pastikan fungsi sudah didefinisikan. Contoh: fungsi hitung() { ... }".to_string());
    }
    
    None
}
