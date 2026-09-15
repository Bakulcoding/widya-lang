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
        None => {
            if let Some(berkas) = cli.berkas {
                jalankan_berkas(&berkas, cli.profil.as_deref());
            } else {
                start_repl();
            }
        }
    }
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
