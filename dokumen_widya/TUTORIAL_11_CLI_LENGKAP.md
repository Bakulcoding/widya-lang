# Tutorial 11 — CLI Widya Lengkap (12 Perintah Resmi)

**Target**: Menguasai SEMUA perintah `widya` dari interpretasi langsung sampai packaging bundle app.
**Level**: Menengah ⭐⭐
**Waktu**: ~20 menit
**File sumber**: [src/main.rs](../src/main.rs) (Parser clap CLI) | [README.md](../README.md)

---

## Daftar Perintah Resmi CLI Widya

Jalankan tanpa argumen untuk buka REPL, atau `widya --bantuan` untuk lihat help lengkap:

| No | Perintah | Sinopsis | Target Output |
|----|----------|----------|---------------|
| 1 | `widya jalankan FILE.wya` | **Interpreter** — jalankan skrip tanpa compile | Output stdout + exit code |
| 2 | `widya kompilasi FILE.wya -o OUT.exe` | **Native Binary** — ELF / PE / Mach-O | .exe (Windows) / ELF (Linux) / Mach-O (macOS) |
| 3 | `widya rust FILE.wya -o out.rs` | **Compile Target Rust** (transpile) | File `*.rs` siap `cargo build` |
| 4 | `widya llvm FILE.wya -o out.ll` | **LLVM IR** (optimasi LTO/Clang) | File `*.ll` LLVM Intermediate Representation |
| 5 | `widya wasm FILE.wya -o app.wasm` | **WebAssembly** (wasm32) + HTML runner | app.wasm + app.html |
| 6 | `widya gpu FILE.wya -o shader.wgsl` | **WebGPU Compute Shader (WGSL)** | File `*.wgsl` siap jalan di WebGPU API |
| 7 | `widya ebpf FILE.wya -o probe.bpf.c` | **Linux Kernel eBPF C Source** | File `*.bpf.c` siap compile clang -target bpf |
| 8 | `widya` (tanpa argumen) | **REPL (Interactive Shell)** | Prompt `Widya>` loop sampai ketik `keluar` |
| 9 | `widya repl` | **REPL** alias perintah no 8 | Sama di atas |
| 10 | `widya uji FILE.wya` | **Test Runner** — scan file untuk assertion & hitung coverage | Output TAP-style laporan + exit code 0 bila semua pass |
| 11 | `widya lsp --stdio` | **Language Server Protocol** (untuk VSCode/Neovim) | JSON-RPC stdio stdin/stdout |
| 12 | `widya studio FILE.wya` | **Mini Studio IDE** (browser-based GUI preview) | Open `adegan_widya_3d.html` + live preview |
| 13 | `widya --versi` | Cek versi Widya saat ini | Print banner ASCII versi |
| 14 | `widya --bantuan` | Print panduan help semua perintah | Daftar flag + contoh |

---

## 1. `widya jalankan` — Interpreter Utama (Paling Sering Dipakai)

```bash
# Basic
widya jalankan contoh\01_halo_dunia.wya

# Shortcut alias (run = jalankan — keduanya bekerja)
widya run contoh\168_semua_layer_100persen.wya

# Lewati parameter POSISIONAL ke skrip: $ARGS[0], $ARGS[1], dll
widya jalankan app.wya arg1 arg2 arg3
```

Pola exit code convention:
- **0** = Sukses (semua `pastikan()` lulus, tidak ada panic)
- **1** = Galat parse / runtime panic / assertion gagal
- **2** = File tidak ditemukan

---

## 2. `widya kompilasi -o out.exe` — Native EXE Mandiri

Target platform **sesuai OS Widya di-build**:
- Windows → **PE x64 widya.exe** (link ke MSVC / MinGW runtime)
- Linux → **ELF x64 widya** (glibc 2.27+ atau musl static)
- macOS → **Mach-O AArch64 widya** (Apple Silicon M1/M2/M3/M4 Universal)

```bash
# Compile fibonacci.wya → fibonacci.exe (mandiri, tanpa widya runtime)
widya kompilasi contoh\02_fibonacci.wya -o fibonacci.exe

# Jalankan binary hasil kompilasi
.\fibonacci.exe

# Kompilasi mode RELEASE + strip symbol (lebih kecil)
widya kompilasi app.wya -o app.exe --release --strip
```

---

## 3. `widya rust FILE -o out.rs` — Transpile ke Rust Source

Untuk keperluan:
- Audit kode oleh reviewer Rust team
- Integrasi dengan library Rust third-party (lebih mudah dari FFI)
- Pakai crates.io ecosystem

```bash
widya rust app_saya.wya -o app_saya_generated.rs

# lalu compile lagi dengan Rust toolchain asli untuk cross-platform optimal
cargo build --release --target aarch64-unknown-linux-gnu
```

---

## 4. `widya llvm FILE -o out.ll` — LLVM IR Optimizable

Cocok untuk:
- Link Time Optimization (LTO) Clang/LLVM tingkat lanjut
- Static analyzer / formal verification
- Patch IR sebelum JIT

```bash
widya llvm contoh\14_mesin_database.wya -o db_engine.ll

# Lanjut optimasi dengan opt (LLVM optimizer) + Clang jadi native
opt -O3 db_engine.ll -o db_engine_opt.bc
llc -filetype=obj db_engine_opt.bc -o db_engine.o
clang db_engine.o -o db_engine  # link jadi ELF native
```

---

## 5. `widya wasm FILE -o app.wasm` — Web Full Stack

Menghasilkan **dua file**:
- `app.wasm` — binary wasm32-unknown-unknown
- `app.html` — runner HTML + JS glue untuk load wasm

```bash
widya wasm contoh\09_aplikasi_flutter.wya -o app.wasm
# Otomatis juga membuat app.html di path sama

# Coba buka app.html di browser (Chrome/Edge/Firefox)
# Atau serve via HTTP server lokal:
python -m http.server 8080
# lalu akses http://localhost:8080/app.html
```

---

## 6. `widya gpu FILE -o shader.wgsl` — WebGPU Compute Shader

Untuk AI inferensi edge, komputasi vektor / matrik berbasis GPU tanpa install CUDA:

```bash
widya gpu contoh\25_gpu_compute_shader.wya -o matmul_256x256.wgsl

# Jalankan shader via WebGPU API:
# - navigator.gpu.requestAdapter()
# - createComputePipeline(matmul_256x256.wgsl source)
```

---

## 7. `widya ebpf FILE -o probe.bpf.c` — Linux Kernel Probe

Untuk DevOps, observability, tracing system call, network firewall:

```bash
widya ebpf contoh\27_raft_dan_ebpf.wya -o syscall_audit.bpf.c

# Compile dengan clang BPF target
clang -O2 -target bpf -c syscall_audit.bpf.c -o syscall_audit.bpf.o

# Load ke kernel via libbpf / bpftool
bpftool prog load syscall_audit.bpf.o /sys/fs/bpf/syscall_audit
```

---

## 8. `widya` — REPL (Interactive Shell)

Jika tidak ada argumen apapun → masuk mode interaktif:

```
Widya-Lang REPL v1.0.0 (ketik `keluar` untuk berhenti)
Widya> misal a = 5
nilai: 5
Widya> a * a + 10
nilai: 35
Widya> cetak("Saya suka", huruf_besar("widya"))
Saya suka WIDYA
Widya> struktur PersegiPanjang { p, l, fungsi luas(){ kembalikan ini.p * ini.l } }
struktur: PersegiPanjang
Widya> PersegiPanjang(3,5).luas()
nilai: 15
Widya> keluar
👋 Sampai jumpa di Widya-Lang!
```

---

## 9. `widya lsp --stdio` — VSCode Extension Support

Folder `vscode-extension/` sudah menyediakan package.json untuk install:
1. Di VSCode → Extensions → Install from VSIX → pilih folder `vscode-extension/`
2. Buka file `.wya` → syntax highlighting aktif
3. Command `widya lsp --stdio` otomatis dijalankan VSCode di belakang layar → autocomplete, hover, go-to-definition.

---

## 10. Flag Global Lengkap

```bash
widya --versi          # Cetak versi
widya --bantuan        # Print help lengkap semua subcommand
widya -v run app.wya   # Mode verbose — cetak AST, bytecode, trace eksekusi
widya --no-stdlib run app.wya   # Jangan load stdlib default (cocok untuk kernel/embedded)
widya --profil="edge" run app.wya   # Terapkan profile "edge" (default "server")
```

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 10 — Unit Test & Benchmark](./TUTORIAL_10_UJI_BENCHMARK.md) |
➡️ **[Tutorial 12 — Kompilasi 6 Target Cross-Platform](./TUTORIAL_12_KOMPILASI_6_TARGET.md)**
