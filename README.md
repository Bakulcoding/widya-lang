<div align="center">

# ⚡ Widya-Lang
### *Bahasa Pemrograman Modern, Tipe Aman, Performa Tinggi, dan Full Self-Hosting.*

[![Version](https://img.shields.io/badge/version-1.0.0--beta.1-blue.svg)](https://github.com/Bakulcoding/widya-lang/releases/tag/v1.0.0-beta.1)
[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-green.svg)](#lisensi)
[![Tests](https://img.shields.io/badge/tests-111%20passed%20(100%25)-success.svg)](#pengujian)
[![Self-Hosting](https://img.shields.io/badge/self--hosting-100%25%20stage--2-purple.svg)](#full-self-hosting)

[Buku Panduan Lengkap](DOKUMENTASI_KOMPREHENSIF.md) • [Web Playground](https://bakulcoding.github.io/widya-lang/www/) • [Ekstensi VS Code](editors/vscode/) • [Contoh Kode](contoh/)

</div>

---

## 🌟 Tentang Widya-Lang

**Widya-Lang** adalah bahasa pemrograman tingkat sistem dan aplikasi yang menggabungkan ekspresivitas sintaks dwibahasa (Nusantara & Internasional), inferensi tipe statis matematis **Hindley-Milner**, manajemen memori aman tanpa jeda Garbage Collector besar (*Safe Reference Counting + Tarjan Cycle Detection*), dan multi-target compiler modern (Native, LLVM, WASM, GPU WGSL, Linux eBPF).

---

## 🚀 Fitur & Keunggulan Utama

- 🧠 **Sistem Tipe Hindley-Milner**: Inferensi tipe otomatis tanpa boilerplate, Aljabar Tipe Data (ADT), Pattern Matching, dan Traits / Type Classes.
- 🏗️ **100% Full Self-Hosting**: Kompiler Widya ditulis seluruhnya dalam bahasa Widya (`compiler_self_hosted/`) dengan 3-Stage Bootstrap deterministik (`widya bootstrap`).
- ⚡ **Multi-Target Native Compiler**: Menghasilkan biner Native OS executable, LLVM IR (`.ll`), WebAssembly (`.wasm`), WebGPU Compute Shader (`.wgsl`), dan Linux Kernel eBPF (`.bpf.c`).
- 📱 **Mobile Engine Bawaan**: Mendukung kompilasi dan perancah proyek langsung untuk **Android (APK/AAB)** dan **iOS (Xcode/Swift Bridge)** tanpa dependensi framework berat luar.
- 🔒 **Audit Keamanan Statis (AST Linter)**: Detektor kebocoran Secret/API Key, pencegahan SQL Injection (SQLi), dan proteksi XSS bawaan via perintah `widya audit`.
- 🛠️ **All-in-One Toolchain**: Language Server Protocol (LSP), Debug Adapter Protocol (DAP), Memory Profiler, dan WPM Package Manager dengan `widya.lock` SHA-256 integrity check.
- 🗄️ **Pustaka Standar C FFI**: Binding resmi tingkat tinggi untuk **SQLite3 Engine** dan **Kriptografi** (SHA-256, HMAC, Base64, Secure Tokens).

---

## 💻 Sintaks Dasar

Widya-Lang mendukung kata kunci berbahasa Indonesia maupun sintaks internasional standar:

```widya
// Contoh 1: Fungsi Generics & Inferensi Tipe Hindley-Milner
fungsi tambah<T>(a: T, b: T) -> T {
    kembalikan a + b;
}

misal hasil = tambah(10, 25);
cetak("Hasil:", hasil);

// Contoh 2: Pencocokan Pola (Pattern Matching)
pilihan StatusRespon {
    Sukses(String),
    Galat(Int, String)
}

misal status = StatusRespon::Sukses("Data berhasil diproses");

cocokkan status {
    StatusRespon::Sukses(pesan) => cetak("OK:", pesan),
    StatusRespon::Galat(kode, err) => cetak("Error [", kode, "]:", err),
    _ => cetak("Status tidak dikenal")
}
```

---

## ⚡ Cara Instalasi & Menjalankan

### 🐧 Di Linux (Ubuntu, Debian, Fedora, Arch, dll.)
```bash
# 1. Kloning repositori
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang

# 2. Jalankan skrip instalasi otomatis (memerlukan Rust/Cargo)
chmod +x install.sh
./install.sh

# 3. Verifikasi instalasi
widya --version
```
> *Alternatif manual:* Jalankan `cargo install --path .` untuk memasang biner `widya` langsung ke `~/.cargo/bin/widya`.

---

### 🍎 Di macOS (Apple Silicon M1/M2/M3 & Intel)
```bash
# 1. Kloning repositori
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang

# 2. Jalankan skrip instalasi otomatis
chmod +x install.sh
./install.sh

# 3. Verifikasi instalasi
widya --version
```

---

### 🪟 Di Windows (PowerShell / Command Prompt)
```powershell
# 1. Buka PowerShell dan kloning repositori
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang

# 2. Jalankan skrip instalasi otomatis Windows
.\install.ps1

# 3. Buka jendela terminal baru lalu verifikasi instalasi
widya --version
```
> *Alternatif manual:* Jalankan `cargo build --release`, lalu tambahkan folder `target\release` ke variabel `PATH` lingkungan Windows Anda.

---

## 🛠️ Perintah CLI Utama

```bash
# Menjalankan skrip langsung
widya run contoh/01_halo_dunia.wya

# Kompilasi ke biner mandiri (.exe / ELF)
widya compile app.wya --output app.exe

# Memverifikasi Full Self-Hosting Compiler
widya bootstrap

# Menjalankan audit keamanan AST linter
widya audit .

# Memeriksa profil memori & deteksi siklus referensi
widya profile app.wya

# Menjalankan Language Server Protocol (LSP) untuk IDE
widya lsp

# Menjalankan server Debug Adapter Protocol (DAP)
widya dap

# Mengompilasi WebAssembly untuk browser
widya wasm app.wya --output app.wasm

# Mengompilasi aplikasi mobile Android / iOS
widya mobile bangun app.wya --target android
```

---

## 📁 Struktur Direktori Repositori

```text
widya-lang/
├── src/                    # Source code kompiler utama & runtime
│   ├── compiler/           # Codegen, monomorphization, type-aware optimizations
│   ├── typesystem.rs       # Hindley-Milner Type Inferrer & Unification
│   ├── stdlib_ffi/         # Pustaka C FFI (SQLite3, Cryptography)
│   ├── dap.rs              # Debug Adapter Protocol Engine
│   ├── lsp.rs              # Language Server Protocol Engine
│   ├── profiler.rs         # Memory Profiler & Tarjan Cycle Detector
│   ├── security_tools.rs   # Static Security AST Linter (SQLi, XSS, Secrets)
│   └── wasm_playground.rs  # WASM Compiler Engine
├── compiler_self_hosted/   # Kompiler mandiri ditulis 100% dalam Widya (.widya)
├── editors/                # Ekstensi resmi editor VS Code
│   └── vscode/             # TextMate Grammar, LSP client, DAP contribution
├── www/                    # Website resmi & Interactive WebAssembly Playground
├── tests/                  # 111 Unit & Integration Test Suites
├── contoh/                 # Kumpulan contoh aplikasi resmi
└── DOKUMENTASI_KOMPREHENSIF.md # Buku panduan 18 bab lengkap
```

---

## 🧪 Pengujian (Test Suite)

Seluruh komponen compiler dan ekosistem diuji secara otomatis:

```bash
cargo test
```
**Status Saat Ini:** ✅ **111 / 111 Tests Passed (100%)** — 0 Errors, 0 Warnings.

---

## 📖 Dokumentasi Lengkap

Untuk panduan mendalam tentang arsitektur, sintaks, sistem operasi, basis data terdistribusi, dan tutorial bertahap, silakan baca:
👉 **[DOKUMENTASI_KOMPREHENSIF.md](DOKUMENTASI_KOMPREHENSIF.md)**

---

## 📄 Lisensi

Widya-Lang dirilis di bawah lisensi open source [MIT](LICENSE) / Apache-2.0.

<div align="center">
  <sub>Dibangun dengan dedikasi untuk kedaulatan teknologi dan standar rekayasa perangkat lunak modern.</sub>
</div>
