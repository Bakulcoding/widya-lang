# Tutorial 12 — Kompilasi 6 Target Cross-Platform (Rust/LLVM/Wasm/WGSL/eBPF/Native)

**Target**: Bisa mengompilasi SATU file `.wya` menjadi 6 output berbeda untuk 6 platform berbeda, tanpa mengubah sebaris kode sumber.
**Level**: Menengah ⭐⭐
**Waktu**: ~25 menit
**File contoh**: [12_kompiler_mandiri.wya](../contoh/12_kompiler_mandiri.wya) | [src/compiler/mod.rs](../src/compiler/mod.rs)

---

## Pendahuluan: 6 Target Kompilasi Widya

Widya menggunakan **Single Source Multi Target (SSMT)** paradigm. Satu kode sumber = bisa jadi 6 target:

| No | Target | CLI Flag | Output File | Kegunaan Utama | Arsitektur Target |
|----|--------|----------|-------------|----------------|------------------|
| T1 | **Native Binary** | `kompilasi -o app.exe` | PE / ELF / Mach-O | Aplikasi mandiri production | x86_64 / AArch64 / i686 sesuai build host |
| T2 | **Rust Source** | `rust -o out.rs` | `*.rs` | Audit keamanan, integrasi crates.io | Semua target Rust toolchain (600+) |
| T3 | **LLVM IR** | `llvm -o out.ll` | `*.ll` | Optimasi LTO Clang, Formal Verif | Semua target LLVM (arm/riscv/mips/x86/wasm dll) |
| T4 | **WebAssembly 32-bit** | `wasm -o app.wasm` | `app.wasm` + `app.html` | Web app, WASI edge server, plugin sandbox | wasm32-unknown-unknown / wasm32-wasi |
| T5 | **WebGPU Shader (WGSL)** | `gpu -o shader.wgsl` | `*.wgsl` | GPU compute shader (AI inference, matmul, vector) | Semua GPU support WebGPU (NVIDIA, AMD, ARM Mali, Intel Iris) |
| T6 | **Linux Kernel eBPF** | `ebpf -o probe.bpf.c` | `*.bpf.c` | Kernel probe, observability, tracing, firewall Cilium | Linux x86_64 / AArch64 kernel ≥ 4.15 BPF JIT enabled |

---

## Contoh Kasus: Satu Kode Sumber, Enam Output

Mari kita pakai kode sumber **SATU FILE** `contoh/02_fibonacci.wya` lalu generate ke 6 output berbeda:

```widya
// ===================== contoh/02_fibonacci.wya =====================
cetak("=== Demo Fibonacci 6 Target Cross Compile ===");

fungsi fib(n) {
    jika n < 2 { kembalikan n };
    kembalikan fib(n-1) + fib(n-2);
}

misal n_test = 15;
cetak("Fibonacci ke-" + ke_teks(n_test) + " = " + ke_teks(fib(n_test)));
cetak("=== SELESAI ===");
```

### T1 — Kompilasi Native Binary (.exe / ELF)
```bash
# Windows → PE x64
widya kompilasi contoh\02_fibonacci.wya -o bin\fibonacci.exe

# Linux → ELF x64 (jika host Linux)
widya kompilasi contoh/02_fibonacci.wya -o bin/fibonacci_linux

# Ukuran: ~350 KB (termasuk interpreter + stdlib, mode release strip ~120 KB)
.\bin\fibonacci.exe
# Output: Fibonacci ke-15 = 610
```

### T2 — Transpile ke Rust Source
```bash
widya rust contoh\02_fibonacci.wya -o gen\fibonacci_gen.rs

# Selanjutnya bisa:
# - Review line-by-line oleh tim Rust safety
# - Tambah dependency crate: tokio, serde, reqwest dll
# - Cross-compile ke 600+ target Rust (misal RISC-V 64):
rustup target add riscv64gc-unknown-linux-gnu
cargo build --release --target riscv64gc-unknown-linux-gnu
```

### T3 — LLVM Intermediate Representation (untuk Optimasi Clang/LTO)
```bash
widya llvm contoh\02_fibonacci.wya -o gen\fibonacci.ll

# Optimasi LTO O3
opt -O3 -enable-loop-unroll -vectorize-loops gen\fibonacci.ll -o gen\fibonacci_opt.bc

# Generate native final object
llc -filetype=obj -march=x86-64 -mattr=avx2 gen\fibonacci_opt.bc -o gen\fibonacci_avx2.o

# Link ke exe final (dengan glibc)
clang gen\fibonacci_avx2.o -o bin\fibonacci_avx2_optimized.exe

# Hasil: ~2-3x lebih cepat untuk loop berat karena AVX2 auto-vectorization LLVM
```

### T4 — WebAssembly + HTML Runner (Web Browser / WASI)
```bash
widya wasm contoh\02_fibonacci.wya -o gen\app_fib.wasm
# Otomatis buat gen\app_fib.html (JS glue untuk load wasm + call export)

# Jalankan via browser atau wasmtime (WASI preview1):
wasmtime gen\app_fib.wasm
# Output via WASI stdout: Fibonacci ke-15 = 610
```

### T5 — WebGPU WGSL Compute Shader (AI / Vector Math)
Gunakan kode shader-friendly:
```widya
// contoh/25_gpu_compute_shader.wya — hitung MatMul 256x256 di GPU
misal A = [[0.0; 256]; 256];
misal B = [[0.0; 256]; 256];
misal i = 0;
selama i < 256 { A[i][i] = 1.0; B[i][i] = 2.0; i = i+1; }
fungsi matmul_gpu(A, B) {
    misal C = [[0.0;256];256];
    // Workgroup size 64,1,1 — mapping ke workgroup WGSL
    kembalikan C;
}
```

Compile:
```bash
widya gpu contoh\25_gpu_compute_shader.wya -o gen\matmul256.wgsl

# File gen\matmul256.wgsl berisi:
# - binding group 0 for buffers
# - @compute @workgroup_size(64,1,1) fn main()
# Validasi via naga CLI (WGPU validator):
naga gen\matmul256.wgsl    # → "Success" jika WGSL valid
```

### T6 — eBPF Linux Kernel Probe (Observability / Tracing)
```bash
# Gunakan contoh yang sudah include pattern tracepoint:
widya ebpf contoh\27_raft_dan_ebpf.wya -o gen\execve_trace.bpf.c

# Compile dengan Clang BPF target + vmlinux.h:
clang -O2 -target bpf -c gen\execve_trace.bpf.c -I/usr/include/bpf -o gen\execve_trace.bpf.o

# Load via bpftool:
sudo bpftool prog load gen\execve_trace.bpf.o /sys/fs/bpf/widya_exec_trace
sudo bpftool prog tracelog
# → Output tracepoint sys_enter_execve tertulis log setiap kali ada execve()
```

---

## Cross Compilation via Target Rust Native

Jika butuh EXE untuk OS lain dari OS host:
```bash
# Host = Windows, target = Linux Musl static
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

# Host = Linux, target = Apple Silicon (butuh linker SDK)
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

---

## 🔑 Best Practice 6 Target

1. **Fungsi yang pure & no-side-effect = jalan optimal di 6 target**. Hindari `baca_berkas()` jika mau jalankan di eBPF kernel space (tidak ada filesystem).
2. **Profil target via `#[profil]`**:
   ```widya
   #[profil("edge")]  // constraint: Wasm mem ≤ 64MB, stack ≤ 16KB, no float jika non-FPU
   fungsi filter_suhu(raw_readings) { ... }
   ```
3. **Selalu test native terlebih dahulu sebelum cross compile** — debug error lebih cepat di host.
4. **Pola fallback**: Jika CUDA tidak ada → CPU SIMD; jika WASM tidak ada → interpretasi.

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 11 — CLI Lengkap](./TUTORIAL_11_CLI_LENGKAP.md) |
➡️ **[Tutorial 13 — Cargo Test Suite Rust](./TUTORIAL_13_CARGO_TEST.md)**
