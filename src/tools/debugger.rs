use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use colored::*;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;

#[derive(Debug, Clone)]
pub struct ExecutionSnapshot {
    pub step_index: usize,
    pub line_number: usize,
    pub source_line: String,
    pub variables: HashMap<String, String>,
    pub stdout_output: Vec<String>,
}

pub struct TimeTravelDebugger {
    pub snapshots: Vec<ExecutionSnapshot>,
    pub current_step: usize,
    pub source_lines: Vec<String>,
}

impl TimeTravelDebugger {
    pub fn new(source: &str) -> Self {
        let lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        Self {
            snapshots: Vec::new(),
            current_step: 0,
            source_lines: lines,
        }
    }

    pub fn record_snapshot(&mut self, line: usize, vars: HashMap<String, String>, out: Vec<String>) {
        let src = if line > 0 && line <= self.source_lines.len() {
            self.source_lines[line - 1].clone()
        } else {
            String::new()
        };

        let step_idx = self.snapshots.len();
        self.snapshots.push(ExecutionSnapshot {
            step_index: step_idx,
            line_number: line,
            source_line: src,
            variables: vars,
            stdout_output: out,
        });
    }

    pub fn render_current_state(&self) {
        if self.snapshots.is_empty() {
            println!("{}", "Belum ada rekaman eksekusi (program kosong atau gagal diurai).".yellow());
            return;
        }

        let snap = &self.snapshots[self.current_step];
        println!("\n{}", "================================================================".cyan());
        println!(
            " {} | Langkah [{}/{}] | Baris Sumber: {}",
            "WIDYA TIME-TRAVEL DEBUGGER".bold().bright_magenta(),
            (snap.step_index + 1).to_string().yellow(),
            self.snapshots.len().to_string().yellow(),
            snap.line_number.to_string().green()
        );
        println!("{}", "================================================================".cyan());

        // Print surrounding source code
        let start_line = snap.line_number.saturating_sub(3);
        let end_line = (snap.line_number + 3).min(self.source_lines.len());

        println!("{}", "--- [ KODE SUMBER ] ---".bright_blue());
        for l in start_line..=end_line {
            if l == 0 || l > self.source_lines.len() { continue; }
            let line_str = &self.source_lines[l - 1];
            if l == snap.line_number {
                println!(" {} {:3} | {}", "->".bold().bright_red(), l.to_string().cyan(), line_str.bold().bright_white());
            } else {
                println!("    {:3} | {}", l.to_string().dimmed(), line_str.dimmed());
            }
        }

        // Print Variable States
        println!("\n{}", "--- [ KEADAAN VARIABEL (ENVIRONMENT) ] ---".bright_green());
        if snap.variables.is_empty() {
            println!("  (Tidak ada variabel dalam scope)");
        } else {
            for (k, v) in &snap.variables {
                println!("  * {}: {}", k.bold().bright_yellow(), v.cyan());
            }
        }

        // Print Captured Stdout
        if !snap.stdout_output.is_empty() {
            println!("\n{}", "--- [ OUTPUT TERAKHIR ] ---".bright_yellow());
            for out in &snap.stdout_output {
                println!("  > {}", out.dimmed());
            }
        }
        println!("{}", "================================================================".cyan());
        println!(
            "Navigasi: [{}] Maju 1 Langkah | [{}] Mundur 1 Langkah | [{}] Mulai Awal | [{}] Lompat ke Akhir | [{}] Keluar",
            "s/next".bold().green(),
            "b/prev".bold().yellow(),
            "r/restart".bold().blue(),
            "e/end".bold().magenta(),
            "q/quit".bold().red()
        );
    }
}

pub fn jalankan_debugger(berkas: &PathBuf) -> Result<(), String> {
    let kode = fs::read_to_string(berkas)
        .map_err(|e| format!("Gagal membaca berkas sumber '{:?}': {}", berkas, e))?;

    let mut lexer = Lexer::new(&kode);
    let tokens = lexer.scan_tokens()
        .map_err(|e| format!("Galat Lexer: {:?}", e))?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()
        .map_err(|e| format!("Galat Parser: {:?}", e))?;

    // Jalankan interpreter dan rekam snapshot
    let mut interp = Interpreter::new();
    let mut debugger = TimeTravelDebugger::new(&kode);

    // Initial snapshot
    debugger.record_snapshot(1, HashMap::new(), Vec::new());

    // Eksekusi statement by statement untuk merekam state
    for stmt in &program.statements {
        let _ = interp.execute(stmt);
        
        // Ekstrak variabel dari current environment
        let mut vars_map = HashMap::new();
        for (k, v) in interp.environment.borrow().get_all_local() {
            vars_map.insert(k, v.to_string_repr());
        }

        let line_num = debugger.snapshots.len() + 1;
        debugger.record_snapshot(line_num.min(debugger.source_lines.len().max(1)), vars_map, Vec::new());
    }

    println!("{}", "=== MEMULAI SESI WIDYA REVERSIBLE TIME-TRAVEL DEBUGGER ===".bold().green());
    println!("Total langkah eksekusi tercatat: {}", debugger.snapshots.len().to_string().yellow());

    debugger.current_step = 0;

    loop {
        debugger.render_current_state();
        print!("\n{} ", "widya-debug>".bold().bright_magenta());
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let cmd = input.trim().to_lowercase();
        match cmd.as_str() {
            "s" | "step" | "next" | "n" => {
                if debugger.current_step + 1 < debugger.snapshots.len() {
                    debugger.current_step += 1;
                } else {
                    println!("{}", "Sudah di langkah eksekusi terakhir!".yellow());
                }
            }
            "b" | "back" | "prev" | "p" => {
                if debugger.current_step > 0 {
                    debugger.current_step -= 1;
                } else {
                    println!("{}", "Sudah di langkah paling awal (mulai)!".yellow());
                }
            }
            "r" | "restart" => {
                debugger.current_step = 0;
                println!("{}", "Kembali ke langkah pertama.".cyan());
            }
            "e" | "end" => {
                if !debugger.snapshots.is_empty() {
                    debugger.current_step = debugger.snapshots.len() - 1;
                }
                println!("{}", "Lompat ke langkah terakhir.".magenta());
            }
            "q" | "quit" | "keluar" => {
                println!("{}", "Keluar dari Time-Travel Debugger.".green());
                break;
            }
            _ => {
                if let Ok(jump_step) = cmd.parse::<usize>() {
                    if jump_step > 0 && jump_step <= debugger.snapshots.len() {
                        debugger.current_step = jump_step - 1;
                    } else {
                        println!("{}", "Nomor langkah tidak valid!".red());
                    }
                } else {
                    println!("{}", "Perintah tidak dikenali. Gunakan: s (step), b (back), r (restart), e (end), q (quit), atau nomor langkah.".red());
                }
            }
        }
    }

    Ok(())
}
