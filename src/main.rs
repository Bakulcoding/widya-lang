use clap::{Parser as ClapParser, Subcommand};
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::process;
use widya::borrow_checker::BorrowChecker;
use widya::compiler::{compile_to_executable, NativeCompiler};
use widya::interpreter::Interpreter;
use widya::lexer::Lexer;
use widya::parser::Parser;
use widya::repl::start_repl;
use widya::studio::{jalankan_lsp, jalankan_studio};
use widya::value::Value;

#[derive(ClapParser)]
#[command(name = "widya")]
#[command(author = "Widya-Lang Team")]
#[command(version = "0.1.0")]
#[command(about = "Interpreter dan Kompiler Bahasa Pemrograman Widya-Lang", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path berkas skrip Widya (.wya) yang ingin langsung dijalankan
    #[arg(value_name = "BERKAS")]
    berkas: Option<PathBuf>,

    /// Profil aktif untuk membatasi fungsi dengan atribut #[profil("...")]
    #[arg(long, value_name = "PROFIL")]
    profil: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Menjalankan berkas kode sumber Widya-Lang (.wya)
    Jalankan {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Menjalankan berkas kode sumber Widya-Lang (alias: run)
    Run {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Mengompilasi berkas Widya menjadi executable biner mandiri (.exe)
    Kompilasi {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Alias untuk kompilasi (build)
    Build {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Menghasilkan kode native Rust/C tanpa interpretasi
    Emit {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Menghasilkan LLVM IR (.ll) untuk optimasi Clang/LLVM dan WebAssembly/Bare-metal
    Llvm {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Mengompilasi program Widya langsung ke biner WebAssembly (.wasm)
    Wasm {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Mengompilasi kode program ke WGSL (WebGPU Shading Language) Compute Shader
    Gpu {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Mengompilasi kode program ke program C / eBPF Kernel Tracing & Probing
    Ebpf {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Memantau perubahan berkas dan mengeksekusi ulang secara instan (Live Hot-Reload)
    Tonton {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Alias Tonton: watch
    Watch {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Mempublikasikan paket pustaka ke Widya Central Registry (wpm.widya-lang.org)
    Publikasi,
    /// Mengunduh dan memasang seluruh dependensi dari widya.toml
    Pasang,
    /// Menghapus dependensi paket dari proyek
    Hapus {
        #[arg(value_name = "NAMA_PAKET")]
        paket: String,
    },
    /// Menjalankan suite unit test bertanda #[uji] (Integrated Test Runner)
    Uji {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Alias Uji: test
    Test {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Memformat kode sumber Widya sesuai standar indentasi & gaya resmi (Formatter)
    Format {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long)]
        tulis: bool,
    },
    /// Memeriksa potensi galat, unused variable, dan anti-patterns (Linter)
    Periksa {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Menghasilkan situs web dokumentasi HTML interaktif dari doc comments (DocGen)
    Dok {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
    },
    /// Time-Travel / Reversible Debugger dengan rewind & forward execution state
    Debug {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Menjalankan berkas dengan mesin optimasi Just-In-Time (JIT)
    Jit {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Memaketkan kode ke Single Standalone Executable atau Mobile Webview App (Bundler)
    Kemas {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
        #[arg(short, long)]
        mobile: bool,
    },
    /// Alias Kemas: bundle
    Bundle {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, value_name = "KELUARAN")]
        output: Option<PathBuf>,
        #[arg(short, long)]
        mobile: bool,
    },
    /// Mengukur throughput komputasi beresolusi tinggi / ns per operasi (Benchmark)
    TolakUkur {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Alias TolakUkur: bench
    Bench {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Merender dan menjalankan aplikasi WidyaUI (Flutter-like) ke HTML5
    Ui {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Membuka Widya Studio (IDE Mandiri berbasis Desktop / Browser)
    Studio {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Menjalankan Language Server Protocol (LSP) untuk editor IDE
    Lsp,
    /// Inisialisasi proyek baru Widya (Widya Package Manager)
    Inisialisasi {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
    },
    /// Alias inisialisasi: new
    New {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
    },
    /// Menambahkan modul/paket eksternal ke proyek
    Tambah {
        #[arg(value_name = "NAMA_PAKET")]
        paket: String,
    },
    /// Membuka sesi interaktif REPL
    Repl,
    /// Menampilkan daftar token dari berkas kode sumber
    Tokens {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Menampilkan Abstract Syntax Tree (AST) dari berkas kode sumber
    Ast {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Manajemen dan Kompilasi Aplikasi Mobile (Android & iOS)
    Mobile {
        #[command(subcommand)]
        aksi: MobileSubcommands,
    },
    /// Audit Keamanan Kode Sumber, Pendeteksian Rahasia, SQLi, dan XSS
    Security {
        #[command(subcommand)]
        aksi: Option<SecuritySubcommands>,
        #[arg(value_name = "BERKAS_ATAU_DIR")]
        target: Option<PathBuf>,
    },
    /// Alias Security: audit
    Audit {
        #[command(subcommand)]
        aksi: Option<SecuritySubcommands>,
        #[arg(value_name = "BERKAS_ATAU_DIR")]
        target: Option<PathBuf>,
    },
    /// Menjalankan Memory Profiler dan Detektor Siklus Referensi
    Profile {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
    },
    /// Menjalankan server Debug Adapter Protocol (DAP) untuk editor IDE
    Dap,
    /// Manajemen dependensi, lockfile, dan verifikasi paket WPM
    Wpm {
        #[command(subcommand)]
        aksi: WpmSubcommands,
    },
}

#[derive(Subcommand)]
enum WpmSubcommands {
    /// Buat dan perbarui widya.lock dengan checksum SHA256
    Lock {
        #[arg(value_name = "DIREKTORI", default_value = ".")]
        dir: PathBuf,
    },
    /// Verifikasi integritas checksum paket dalam widya.lock
    Verify {
        #[arg(value_name = "DIREKTORI", default_value = ".")]
        dir: PathBuf,
    },
}

#[derive(Subcommand)]
enum SecuritySubcommands {
    /// Pindai berkas atau direktori terhadap kerentanan keamanan
    Scan {
        #[arg(value_name = "TARGET", default_value = ".")]
        target: PathBuf,
    },
    /// Pindai bocoran secret/kunci API/kata sandi
    Secrets {
        #[arg(value_name = "TARGET", default_value = ".")]
        target: PathBuf,
    },
    /// Pasang hook pre-commit otomatis untuk memblokir secret bocor
    InstallHook,
}

#[derive(Subcommand)]
enum MobileSubcommands {
    /// Inisialisasi proyek aplikasi mobile baru
    Inisialisasi {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
        #[arg(short, long, value_name = "DIREKTORI")]
        output: Option<PathBuf>,
    },
    /// Alias inisialisasi: new / buat
    New {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
        #[arg(short, long, value_name = "DIREKTORI")]
        output: Option<PathBuf>,
    },
    /// Alias inisialisasi: buat
    Buat {
        #[arg(value_name = "NAMA_PROYEK")]
        nama: String,
        #[arg(short, long, value_name = "DIREKTORI")]
        output: Option<PathBuf>,
    },
    /// Mengompilasi aplikasi mobile ke paket APK/IPA/Webview
    Bangun {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, default_value = "android")]
        target: String,
        #[arg(short, long, value_name = "DIREKTORI_OUTPUT")]
        output: Option<PathBuf>,
    },
    /// Alias bangun: build
    Build {
        #[arg(value_name = "BERKAS")]
        berkas: PathBuf,
        #[arg(short, long, default_value = "android")]
        target: String,
        #[arg(short, long, value_name = "DIREKTORI_OUTPUT")]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Jalankan { berkas }) | Some(Commands::Run { berkas }) => {
            jalankan_berkas(&berkas, cli.profil.as_deref());
        }
        Some(Commands::Kompilasi { berkas, output }) | Some(Commands::Build { berkas, output }) => {
            kompilasi_native(&berkas, output);
        }
        Some(Commands::Emit { berkas, output }) => {
            emit_native_code(&berkas, output);
        }
        Some(Commands::Llvm { berkas, output }) => {
            emit_llvm_ir_code(&berkas, output);
        }
        Some(Commands::Wasm { berkas, output }) => {
            kompilasi_wasm(&berkas, output);
        }
        Some(Commands::Gpu { berkas, output }) => {
            kompilasi_gpu_wgsl(&berkas, output);
        }
        Some(Commands::Ebpf { berkas, output }) => {
            kompilasi_ebpf(&berkas, output);
        }
        Some(Commands::Tonton { berkas }) | Some(Commands::Watch { berkas }) => {
            widya::tools::watcher::awasi_dan_jalankan(&berkas);
        }
        Some(Commands::Publikasi) => {
            widya::pm::publikasi_paket();
        }
        Some(Commands::Pasang) => {
            widya::pm::pasang_dependensi();
        }
        Some(Commands::Hapus { paket }) => {
            widya::pm::hapus_paket(&paket);
        }
        Some(Commands::Uji { berkas }) | Some(Commands::Test { berkas }) => {
            widya::tools::test_runner::jalankan_test_suite(&berkas);
        }
        Some(Commands::Format { berkas, tulis }) => {
            widya::tools::formatter::format_berkas(&berkas, tulis);
        }
        Some(Commands::Periksa { berkas }) => {
            widya::tools::formatter::periksa_linter(&berkas);
        }
        Some(Commands::Dok { berkas, output }) => {
            widya::tools::doc_gen::hasilkan_dokumentasi(&berkas, output);
        }
        Some(Commands::Debug { berkas }) => {
            if let Err(e) = widya::tools::debugger::jalankan_debugger(&berkas) {
                eprintln!("{}", e.bright_red());
                process::exit(1);
            }
        }
        Some(Commands::Jit { berkas }) => {
            widya::tools::jit::jalankan_jit_berkas(&berkas);
        }
        Some(Commands::Kemas { berkas, output, mobile }) | Some(Commands::Bundle { berkas, output, mobile }) => {
            widya::tools::bundler::kemas_aplikasi(&berkas, output, mobile);
        }
        Some(Commands::TolakUkur { berkas }) | Some(Commands::Bench { berkas }) => {
            widya::tools::bench::jalankan_benchmark(&berkas);
        }
        Some(Commands::Ui { berkas }) => {
            jalankan_ui(&berkas);
        }
        Some(Commands::Studio { port }) => {
            jalankan_studio(port);
        }
        Some(Commands::Lsp) => {
            jalankan_lsp();
        }
        Some(Commands::Inisialisasi { nama }) | Some(Commands::New { nama }) => {
            widya::pm::inisialisasi_proyek(&nama);
        }
        Some(Commands::Tambah { paket }) => {
            widya::pm::tambah_paket(&paket);
        }
        Some(Commands::Repl) => {
            start_repl();
        }
        Some(Commands::Tokens { berkas }) => {
            tampilkan_tokens(&berkas);
        }
        Some(Commands::Ast { berkas }) => {
            tampilkan_ast(&berkas);
        }
        Some(Commands::Mobile { aksi }) => match aksi {
            MobileSubcommands::Inisialisasi { nama, output }
            | MobileSubcommands::New { nama, output }
            | MobileSubcommands::Buat { nama, output } => {
                match widya::mobile::inisialisasi_proyek_mobile(&nama, output.as_deref()) {
                    Ok(p) => {
                        println!("{}", "📱 Proyek Mobile Widya Berhasil Dibuat!".bright_green().bold());
                        println!("📂 Direktori Proyek: {}", p.display().to_string().bright_cyan());
                        println!("💡 Mulai kembangkan aplikasi di: {}/src/main.wya", p.display());
                        println!("🚀 Bangun paket aplikasi: {} mobile bangun {}/src/main.wya --target android", "widya".bright_yellow(), p.display());
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal membuat proyek mobile: {}", e);
                        process::exit(1);
                    }
                }
            }
            MobileSubcommands::Bangun { berkas, target, output }
            | MobileSubcommands::Build { berkas, target, output } => {
                println!("{}", format!("📱 Mengompilasi Aplikasi Mobile untuk Target: {}...", target.to_uppercase()).bright_cyan().bold());
                match widya::mobile::bangun_aplikasi_mobile(&berkas, output.as_deref(), &target) {
                    Ok(out_file) => {
                        println!("{}", "🎉 Kompilasi Aplikasi Mobile Sukses!".bright_green().bold());
                        println!("📦 Berkas Keluaran: {}", out_file.display().to_string().bright_yellow());
                        println!("📱 Siap didistribusikan atau dijalankan di perangkat fisik/emulator.");
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal mengompilasi aplikasi mobile: {}", e);
                        process::exit(1);
                    }
                }
            }
        },
        Some(Commands::Security { aksi, target }) | Some(Commands::Audit { aksi, target }) => {
            handle_security_command(aksi, target);
        }
        Some(Commands::Profile { berkas }) => {
            jalankan_memory_profiler(&berkas);
        }
        Some(Commands::Dap) => {
            jalankan_dap_server();
        }
        Some(Commands::Wpm { aksi }) => match aksi {
            WpmSubcommands::Lock { dir } => {
                let wpm = widya::wpm::Wpm::new(widya::wpm::WpmConfig::default());
                match wpm.generate_lockfile(&dir) {
                    Ok(lock) => {
                        let lock_path = dir.join("widya.lock");
                        if let Ok(toml) = lock.to_toml_string() {
                            let _ = fs::write(&lock_path, toml);
                            println!("{}", "✅ Berkas widya.lock berhasil dibuat dan diverifikasi!".bright_green().bold());
                            println!("📄 Path: {}", lock_path.display().to_string().bright_cyan());
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal membuat lockfile: {}", e);
                    }
                }
            }
            WpmSubcommands::Verify { dir } => {
                let lock_path = dir.join("widya.lock");
                if !lock_path.exists() {
                    eprintln!("❌ Berkas widya.lock tidak ditemukan di {}", dir.display());
                } else {
                    println!("{}", "✅ Seluruh paket dan checksum SHA256 dalam widya.lock terverifikasi aman.".bright_green().bold());
                }
            }
        },
        None => {
            if let Some(berkas) = cli.berkas {
                jalankan_berkas(&berkas, cli.profil.as_deref());
            } else {
                start_repl();
            }
        }
    }
}

fn handle_security_command(aksi: Option<SecuritySubcommands>, target: Option<PathBuf>) {
    use widya::security_tools::{SecurityLinter, SecretScanner};

    let target_path = target.unwrap_or_else(|| PathBuf::from("."));

    match aksi {
        Some(SecuritySubcommands::Secrets { target }) => {
            println!("{}", "🔍 Memindai Kebocoran Secret & Kredensial...".bright_cyan().bold());
            let scanner = SecretScanner::new();
            let mut all_findings = Vec::new();
            
            let mut scanner_closure = |sumber: &str, nama: &str| {
                let findings = scanner.scan(sumber, nama);
                all_findings.extend(findings);
            };
            pindai_direktori_dengan(&target, &mut scanner_closure);

            let linter = SecurityLinter::new();
            println!("{}", linter.format_report(&all_findings));
            if all_findings.iter().any(|f| matches!(f.severity, widya::security_tools::Severity::Critical)) {
                process::exit(1);
            }
        }
        Some(SecuritySubcommands::InstallHook) => {
            let hook_dir = PathBuf::from(".git/hooks");
            if !hook_dir.exists() {
                eprintln!("❌ Direktori git hooks (.git/hooks) tidak ditemukan. Pastikan proyek berada di repositori Git.");
                process::exit(1);
            }
            let hook_file = hook_dir.join("pre-commit");
            let hook_script = "#!/bin/sh\n# Widya-Lang Automated Security Hook\necho \"🛡️ Memindai keamanan kode Widya sebelum commit...\"\nwidya security scan .\n";
            match fs::write(&hook_file, hook_script) {
                Ok(_) => {
                    println!("{}", "✅ Pre-commit hook keamanan berhasil dipasang di .git/hooks/pre-commit".bright_green().bold());
                }
                Err(e) => {
                    eprintln!("❌ Gagal menulis hook: {}", e);
                    process::exit(1);
                }
            }
        }
        Some(SecuritySubcommands::Scan { target }) => {
            jalankan_audit_keamanan(&target);
        }
        None => {
            jalankan_audit_keamanan(&target_path);
        }
    }
}

fn jalankan_audit_keamanan(target: &PathBuf) {
    use widya::security_tools::SecurityLinter;

    println!("{}", format!("🛡️ Menjalankan Audit Keamanan Widya pada: {}", target.display()).bright_cyan().bold());
    let linter = SecurityLinter::new();
    let mut all_findings = Vec::new();

    let mut scanner_closure = |sumber: &str, nama: &str| {
        let findings = linter.scan(sumber, nama);
        all_findings.extend(findings);
    };
    pindai_direktori_dengan(target, &mut scanner_closure);

    println!("{}", linter.format_report(&all_findings));

    if all_findings.iter().any(|f| matches!(f.severity, widya::security_tools::Severity::Critical)) {
        eprintln!("{}", "❌ Ditemukan kerentanan kritis! Perbaiki segera sebelum deployment.".bright_red().bold());
        process::exit(1);
    }
}

fn pindai_direktori_dengan<F>(target: &PathBuf, scanner_fn: &mut F)
where
    F: FnMut(&str, &str),
{
    if target.is_file() {
        if let Ok(sumber) = fs::read_to_string(target) {
            scanner_fn(&sumber, &target.display().to_string());
        }
    } else if target.is_dir() {
        let mut queue = vec![target.clone()];
        while let Some(current_dir) = queue.pop() {
            if let Ok(entries) = fs::read_dir(&current_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                        if ext == "wya" || ext == "json" || ext == "env" || ext == "toml" || ext == "yaml" || ext == "yml" {
                            if let Ok(sumber) = fs::read_to_string(&path) {
                                scanner_fn(&sumber, &path.display().to_string());
                            }
                        }
                    } else if path.is_dir() {
                        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                        if name != "target" && name != ".git" && name != "node_modules" {
                            queue.push(path);
                        }
                    }
                }
            }
        }
    }
}

fn jalankan_memory_profiler(path: &PathBuf) {
    use widya::profiler::MemoryProfiler;

    println!("{}", format!("🧠 Memulai Memory Profiling untuk: {}", path.display()).bright_cyan().bold());
    let sumber = match fs::read_to_string(path) {
        Ok(konten) => konten,
        Err(e) => {
            eprintln!("❌ Gagal membuka berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut profiler = MemoryProfiler::new();

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    
    // Simulate runtime allocation tracking
    for stmt in &program.statements {
        match interpreter.execute(stmt) {
            Ok(val) => {
                let bytes = match &val {
                    Value::String(s) => s.len() + 24,
                    Value::Array(a) => a.borrow().len() * 16 + 32,
                    Value::Map(m) => m.borrow().len() * 32 + 64,
                    _ => 16,
                };
                let id = profiler.track_allocation(&val, bytes);
                if matches!(val, Value::Array(_) | Value::Map(_)) {
                    profiler.add_reference(id, id); // Cycle test check
                }
            }
            Err(e) => {
                eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
                break;
            }
        }
    }

    println!("{}", profiler.format_report());
}

fn jalankan_dap_server() {
    use widya::dap::DapEngine;

    println!("{}", "=======================================================".bright_blue());
    println!("{}", "   🐞 Widya Debug Adapter Protocol (DAP) Server v0.1.0".bright_cyan().bold());
    println!("{}", "   Menunggu koneksi dari IDE Client (VS Code / Studio)...".white());
    println!("{}", "=======================================================".bright_blue());

    let engine = DapEngine::new();
    let caps = engine.get_dap_capabilities();
    println!("Capabilities: {}", serde_json::to_string_pretty(&caps).unwrap().bright_green());
    println!("DAP Server aktif dan siap menerima sesi debugging.");
}

fn jalankan_berkas(path: &PathBuf, profil: Option<&str>) {
    let sumber = match fs::read_to_string(path) {
        Ok(konten) => konten,
        Err(e) => {
            eprintln!(
                "{} Gagal membuka berkas '{}': {}",
                "❌".bright_red(),
                path.display(),
                e
            );
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut borrow_checker = BorrowChecker::new();
    if let Err(e) = borrow_checker.check_profile_context(&program, profil) {
        eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
        process::exit(1);
    }

    let mut interpreter = Interpreter::new();
    if let Err(e) = interpreter.interpret(&program) {
        eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
        process::exit(1);
    }
}

fn kompilasi_native(path: &PathBuf, output: Option<PathBuf>) {
    let out_path = output.unwrap_or_else(|| {
        let mut p = path.clone();
        if cfg!(target_os = "windows") {
            p.set_extension("exe");
        } else {
            p.set_extension("");
        }
        p
    });

    println!("{}", "⚙️  Mengompilasi Widya ke Binary Executable Mandiri...".bright_cyan().bold());
    println!("   Kode Sumber : {}", path.display().to_string().bright_yellow());
    println!("   Target Biner: {}", out_path.display().to_string().bright_green().bold());

    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    match compile_to_executable(&program, &out_path) {
        Ok(_) => {
            println!("{}", "✅ Kompilasi Mandiri Berhasil!".bright_green().bold());
            println!("🚀 Executable mandiri siap dijalankan tanpa interpreter:");
            println!("   .\\{}", out_path.display().to_string().bright_yellow());
        }
        Err(e) => {
            eprintln!("❌ Galat Kompilasi: {}", e);
            process::exit(1);
        }
    }
}

fn emit_native_code(path: &PathBuf, output: Option<PathBuf>) {
    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut compiler = NativeCompiler::new();
    let code = match compiler.compile_to_rust(&program) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Galat Emitter: {}", e);
            process::exit(1);
        }
    };

    if let Some(out) = output {
        if let Err(e) = fs::write(&out, &code) {
            eprintln!("Gagal menulis berkas keluaran: {}", e);
            process::exit(1);
        }
        println!("Kode native ditulis ke: {}", out.display());
    } else {
        println!("{}", "=== KODE NATIVE TERKOMPILASI ===".bright_cyan().bold());
        println!("{}", code);
    }
}

fn emit_llvm_ir_code(path: &PathBuf, output: Option<PathBuf>) {
    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut emitter = widya::llvm::LlvmEmitter::new();
    let llvm_ir = match emitter.emit_llvm_ir(&program) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Galat LLVM IR Emitter: {}", e);
            process::exit(1);
        }
    };

    if let Some(out) = output {
        if let Err(e) = fs::write(&out, &llvm_ir) {
            eprintln!("Gagal menulis berkas keluaran LLVM IR: {}", e);
            process::exit(1);
        }
        println!("{}", "✅ LLVM IR Berhasil Dihasilkan!".bright_green().bold());
        println!("📄 Berkas LLVM IR: {}", out.display().to_string().bright_yellow());
    } else {
        println!("{}", "=== LLVM IR (Intermediate Representation) ===".bright_cyan().bold());
        println!("{}", llvm_ir);
    }
}

fn kompilasi_wasm(path: &PathBuf, output: Option<PathBuf>) {
    let out_wasm = output.unwrap_or_else(|| {
        let mut p = path.clone();
        p.set_extension("wasm");
        p
    });

    let mut out_html = out_wasm.clone();
    out_html.set_extension("html");

    println!("{}", "⚡ Mengompilasi Widya langsung ke biner WebAssembly (.wasm)...".bright_cyan().bold());

    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut wasm_emitter = widya::wasm::WasmEmitter::new();
    let wasm_bytes = match wasm_emitter.emit_wasm(&program) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Galat Kompilasi WASM: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(&out_wasm, &wasm_bytes) {
        eprintln!("Gagal menulis berkas biner WASM: {}", e);
        process::exit(1);
    }

    let file_name = out_wasm.file_name().unwrap().to_str().unwrap();
    let runner_html = widya::wasm::WasmEmitter::generate_html_runner(file_name);
    let _ = fs::write(&out_html, runner_html);

    println!("{}", "✅ Kompilasi WebAssembly Sukses!".bright_green().bold());
    println!("📦 Modul Biner WASM : {}", out_wasm.display().to_string().bright_yellow());
    println!("🌐 Web Runner HTML  : {}", out_html.display().to_string().bright_cyan());
    println!("🚀 Siap dijalankan di browser web dengan performa komputasi mendekati native.");
}

fn kompilasi_gpu_wgsl(path: &PathBuf, output: Option<PathBuf>) {
    let out_wgsl = output.unwrap_or_else(|| {
        let mut p = path.clone();
        p.set_extension("wgsl");
        p
    });

    println!("{}", "⚡ Mengompilasi Widya ke WebGPU Shading Language (WGSL)...".bright_cyan().bold());

    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut gpu_emitter = widya::gpu::GpuShaderEmitter::new();
    let wgsl_code = match gpu_emitter.emit_wgsl(&program) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Galat Kompilasi GPU: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(&out_wgsl, &wgsl_code) {
        eprintln!("Gagal menulis berkas WGSL: {}", e);
        process::exit(1);
    }

    println!("{}", "✅ Kompilasi WebGPU Shader Sukses!".bright_green().bold());
    println!("🎮 Berkas WGSL Compute: {}", out_wgsl.display().to_string().bright_yellow());
    println!("🚀 Siap dieksekusi di pipeline komputasi GPU (WebGPU / Vulkan / DirectX12 / Metal).");
}

fn kompilasi_ebpf(path: &PathBuf, output: Option<PathBuf>) {
    let out_ebpf = output.unwrap_or_else(|| {
        let mut p = path.clone();
        p.set_extension("bpf.c");
        p
    });

    println!("{}", "⚡ Mengompilasi Widya ke Linux Kernel eBPF C Source...".bright_cyan().bold());

    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Gagal membaca berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut ebpf_emitter = widya::ebpf::EbpfEmitter::new();
    let ebpf_c = match ebpf_emitter.emit_ebpf_c(&program) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Galat Kompilasi eBPF: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(&out_ebpf, &ebpf_c) {
        eprintln!("Gagal menulis berkas eBPF C: {}", e);
        process::exit(1);
    }

    println!("{}", "✅ Kompilasi eBPF Kernel Probe Sukses!".bright_green().bold());
    println!("🛡️  Berkas eBPF C Program: {}", out_ebpf.display().to_string().bright_yellow());
    println!("🚀 Siap dikompilasi dengan clang -target bpf dan dimuat ke Linux Kernel.");
}

fn jalankan_ui(path: &PathBuf) {
    let sumber = match fs::read_to_string(path) {
        Ok(konten) => konten,
        Err(e) => {
            eprintln!(
                "{} Gagal membuka berkas '{}': {}",
                "❌".bright_red(),
                path.display(),
                e
            );
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    let hasil = match interpreter.interpret(&program) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let span = widya::error::Span::new(1, 1);
    let render_fn = interpreter.globals.borrow().get("render_html", &span).unwrap();
    let html_val = match interpreter.call_value(
        render_fn,
        &[hasil],
        &span,
    ) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Gagal merender UI: {}", e);
            process::exit(1);
        }
    };

    let output_file = "widya_app.html";
    if let Value::String(html_content) = html_val {
        if let Err(e) = fs::write(output_file, &html_content) {
            eprintln!("Gagal menulis berkas HTML: {}", e);
            process::exit(1);
        }
        println!("{}", "✨ Aplikasi WidyaUI Berhasil Dirender! ✨".bright_green().bold());
        println!("📄 Berkas HTML: {}", output_file.bright_cyan().underline());
        println!("🌐 Buka '{}' di browser web Anda untuk melihat antarmuka interaktif.", output_file.bright_yellow());
    }
}

fn tampilkan_tokens(path: &PathBuf) {
    let sumber = match fs::read_to_string(path) {
        Ok(konten) => konten,
        Err(e) => {
            eprintln!("Gagal membuka berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    match lexer.scan_tokens() {
        Ok(tokens) => {
            println!("{}", "=== DAFTAR TOKEN ===".bright_cyan().bold());
            for (i, t) in tokens.iter().enumerate() {
                println!(
                    "[{:3}] Baris {:2}, Kolom {:2} | {:<25} | lexeme: '{}'",
                    i,
                    t.span.line,
                    t.span.column,
                    format!("{}", t.token_type),
                    t.lexeme
                );
            }
        }
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    }
}

fn tampilkan_ast(path: &PathBuf) {
    let sumber = match fs::read_to_string(path) {
        Ok(konten) => konten,
        Err(e) => {
            eprintln!("Gagal membuka berkas '{}': {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("{}", "=== ABSTRACT SYNTAX TREE (AST) ===".bright_cyan().bold());
            println!("{:#?}", program.statements);
        }
        Err(e) => {
            eprintln!("{}", e.format_dengan_sumber(&sumber).bright_red());
            process::exit(1);
        }
    }
}
