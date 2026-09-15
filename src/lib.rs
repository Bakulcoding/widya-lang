pub mod ast;
pub mod compiler;
pub mod environment;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod stdlib;
pub mod token;
pub mod value;
pub mod lsp;
pub mod studio;
pub mod pm;
pub mod llvm;
pub mod wasm;
pub mod gpu;
pub mod ebpf;
pub mod borrow_checker;
pub mod tools;

use borrow_checker::BorrowChecker;
use error::Galat;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use serde_json::json;
use value::Value;

#[derive(Debug, Clone)]
pub struct CompileTarget {
    pub id: &'static str,
    pub nama: &'static str,
    pub deskripsi: &'static str,
    pub ekstensi_keluaran: &'static str,
    pub kategori: &'static str,
    pub tersedia: bool,
}

pub fn compile_targets() -> Vec<CompileTarget> {
    vec![
        CompileTarget {
            id: "native",
            nama: "Native Executable",
            deskripsi: "Kompilasi ke biner executable native OS target (Windows/Linux/macOS)",
            ekstensi_keluaran: if cfg!(windows) { "exe" } else { "out" },
            kategori: "Native/System",
            tersedia: true,
        },
        CompileTarget {
            id: "rust",
            nama: "Rust Source Code",
            deskripsi: "Transpile Widya-Lang ke source code Rust murni (kompilable dengan rustc)",
            ekstensi_keluaran: "rs",
            kategori: "Source Emitter",
            tersedia: true,
        },
        CompileTarget {
            id: "llvm",
            nama: "LLVM IR",
            deskripsi: "Emit LLVM Intermediate Representation untuk optimasi Clang/LLVM",
            ekstensi_keluaran: "ll",
            kategori: "Intermediate Representation",
            tersedia: true,
        },
        CompileTarget {
            id: "wasm",
            nama: "WebAssembly Binary",
            deskripsi: "Kompilasi ke bytecode WebAssembly (.wasm) + HTML runner untuk browser",
            ekstensi_keluaran: "wasm",
            kategori: "Web/Browser",
            tersedia: true,
        },
        CompileTarget {
            id: "wgsl",
            nama: "WebGPU WGSL Compute Shader",
            deskripsi: "Transpile Widya-Lang ke WebGPU Shading Language (WGSL) Compute",
            ekstensi_keluaran: "wgsl",
            kategori: "GPU/WebGPU",
            tersedia: true,
        },
        CompileTarget {
            id: "ebpf",
            nama: "Linux eBPF C Program",
            deskripsi: "Emit kode C untuk eBPF kernel tracing/networking (clang -target bpf)",
            ekstensi_keluaran: "bpf.c",
            kategori: "Kernel/Observability",
            tersedia: true,
        },
    ]
}

pub fn compile_targets_json() -> String {
    let ct = compile_targets();
    let items: Vec<serde_json::Value> = ct.iter().map(|t| {
        json!({
            "id": t.id,
            "nama": t.nama,
            "deskripsi": t.deskripsi,
            "ekstensi_keluaran": t.ekstensi_keluaran,
            "kategori": t.kategori,
            "tersedia": t.tersedia
        })
    }).collect();
    json!({
        "sukses": true,
        "jumlah": items.len(),
        "target": items
    }).to_string()
}

pub fn compile_target_by_id(id: &str) -> Option<CompileTarget> {
    compile_targets().into_iter().find(|t| t.id == id)
}

/// Menjalankan kode sumber Widya-Lang dari string dengan validasi keamanan memori
pub fn jalankan(sumber: &str) -> Result<Value, Galat> {
    let mut lexer = Lexer::new(sumber);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    // Static Memory Safety & Borrow Checking Pass
    let mut borrow_checker = BorrowChecker::new();
    borrow_checker.check_program(&program)?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program)
}
