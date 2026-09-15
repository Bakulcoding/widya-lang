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
| 12 | `widya studio --port 9000` | **Widya Studio IDE** — IDE level industri 20+ fitur (Multi-tab, Debugger, Command Palette, Kompilasi 6 target UI, Git, AI) | Serve `http://localhost:9000` → GUI penuh di browser |
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

## 12. `widya studio` — Widya Studio IDE Level Industri (20+ Fitur)

Widya Studio adalah **IDE mandiri berbasis browser** (server lokal HTTP + UI Monaco di dalam satu binary Rust). Buka browser → GUI penuh tanpa install apa pun.

```bash
# Jalankan di port default 8080
widya studio

# Atau port khusus
widya studio --port 9000

# Lalu buka di browser
#   http://localhost:9000
```

> Adress hanya `127.0.0.1` (localhost) demi keamanan. Rate limiter bawaan **120 request/menit** (per client); request ke-121 ditolak dengan HTTP **429**.

### 12.1 Alur Memulai
1. Jalankan `widya studio --port 9000`.
2. Buka `http://localhost:9000` → halaman **Welcome Dashboard** muncul dengan 4 menu utama:
   - **Mulai Mengedit** (buat file baru)
   - **Buka Folder Workspace** (ubah direktori kerja)
   - **Buka Contoh** (168+ file contoh `contoh/`)
   - **Modul Industri** (80+ modul per sektor)
3. Tekan **F5** untuk menjalankan kode tab aktif, **Ctrl+Shift+P** untuk Command Palette (35+ aksi).

### 12.2 Fitur Inti Editor
- **Multi-Tab Monaco Editor**: buka banyak file sekaligus; penanda `*` = belum disimpan (dirty); `Ctrl+W` tutup tab; `Ctrl+Tab` / `Ctrl+Shift+Tab` pindah tab.
- **Autosave**: tiap 5 detik setelah ada perubahan kebijakan; **Save** `Ctrl+S`, **Save As** `Ctrl+Shift+S`.
- **Inline Error Marker**: tiap 500 ms editor mengetik → diagnostik LSP → garis merah + ikon margin + panel **Masalah** dengan jumlah error.
- **Command Palette** `Ctrl+Shift+P`: 35+ aksi (simpan, format, build 6 target, debug, tema, git commit, inisialisasi proyek, sisip snippet, generate dokumentasi, dan lain-lain).
- **Pengaturan** `Ctrl+,`: tema (Catppuccin Dark/Latte, VS Dark/Light), ukuran font, tab size, minimap, word wrap, auto format on save, AI on/off — tersimpan di `localStorage`.

### 12.3 Sidebar Explorer & Modul
- **Kiri-atas (Explorer)**: pohon workspace rekursif (folder, file `.wya` biru, `.md` abu, `.json` kuning, `.rs` oranye). Klik file → buka tab. Grup **Contoh** berisi 168+ `contoh/*.wya`.
- **Kiri-tengah (Modul)**: 12+ kategori industri (AI & ML, Fintech, Medis, Otomotif SCADA, WidyaDB Data, Mesh Networking, Hardware SGX/CUDA/FPGA, OS/Kernel, Cloud Microservice, dan lain-lain) — total **80+ modul**. Klik modul → buka tab.
- **Kiri-bawah (Git)**: status modified/untracked/staged, isi pesan → **Commit** (`/api/git/commit`), riwayat log `git log`.
- **Kiri-bawah (Debug)**: panel breakpoint & variabel watch.

### 12.4 Menjalankan, Menguji, Mengompilasi
- **F5 Jalankan** → interpreter Widya; hasil di panel **Output** (termasuk capture semua `cetak()`/`tulis()`).
- **Cargo Test (`cargo test --all`)**: tombol 🧪 di header → tabel nama test per baris (hijau = lulus) + ringkasan jumlah passed/failed + log lengkap.
- **Build dropdown (⚙️)**: 6 target kompilasi langsung dari UI —
  - `native` (EXE), `rust` (source), `llvm` (IR), `wasm` (+ HTML runner), `wgsl` (WebGPU shader), `ebpf` (kernel probe).
  - Hasil muncul di panel **Kompilasi** lengkap dengan tombol **Download Output**.
- **Terminal REPL**: panel bawah → prompt `widya>`, state interpreter persisten antar-request; `↑/↓` riwayat; hasil + variabel aktif; tombol reset.
- **Mini Debugger**: klik nomor baris di gutter = set/selepas **breakpoint**; `Alt+F5` mulai, `F10` step over, `Shift+F5` stop. Watch variabel di panel Debug.

### 12.5 AI, Snippet, dan Produktivitas Lain
- **AI Autocomplete** `Ctrl+Space`: 3–6 saran offline berbasis modul `ai_rag_stack` (di-hide; kualitas saran ≥3). `Tab` terima saran.
- **Insert Snippet** (Command Palette → “Insert: Sisipkan Snippet Widya”): pilih template dari `vscode-extension/snippets/snippets.json` → disisipkan di kursor.
- **Generate Dokumentasi HTML**: palette action "Project: Generate Dokumentasi HTML" memindai komentar `///` pada file aktif → ringkasan fungsi/struktur/enum + download `.html`.
- **Format Dokumen** `Alt+Shift+F`: buang tab → 4 spasi, indentasi `{}` otomatis, rapikan spasi berlebih.
- **Split Editor**: `Ctrl+\` (kanan) / `Ctrl+Shift+\` (bawah) untuk 2 instansi Monaco.
- **Zen Mode** `Ctrl+K Z`: sembunyikan header/sidebar/panel bawah; **ESC 2x** keluar.
- **Cari di Proyek** `Ctrl+Shift+F`: mencari rekursif di SELURUH workspace (backend `/api/search`, lompati `.git`/`node_modules`/`target`), dikelompokkan per file; klik hasil → langsung ke baris. `Ganti Semua` bekerja di tab terbuka.
- **Quick Open** `Ctrl+P`: cari cepat file contoh + tab.
- **Inisialisasi Proyek** (palette → "Project: Inisialisasi Proyek Baru"): buat folder berisi `Widya.toml`, `sumber/utama.wya`, `README.md`.
- **8 Panel Bawah**: Output · Masalah · Terminal REPL · Cargo Test · Kompilasi · Debug Console · Aktivitas Git · AI Saran.

### 12.6 Keamanan & Validasi
- Semua endpoint di-rate-limit (120/menit) — cek sisa kuota: `GET /api/rate/status`.
- Path traversal dicegah (`safe_join`): akses file di luar workspace ditolak.
- Tidak ada dependency eksternal runtime: UI + server dalam satu binary (`widya.exe`).

### 12.7 Daftar Endpoint API Lokal (untuk Integrator)
| Method | Path | Fungsi |
|---|---|---|
| GET | `/` | Halaman IDE (HTML+JS+CSS inline) |
| POST | `/api/run` | Jalankan kode Widya → `{sukses, hasil, output[]}` |
| POST | `/api/check` | Diagnostik sintaks (LSP inline) → `[{baris, kolom, pesan}]` |
| GET | `/api/contoh` | Daftar 168+ file contoh |
| POST | `/api/format` | Format ulang kode |
| POST | `/api/doc/generate` | Dokumentasi HTML dari komentar `///` |
| POST | `/api/compile/{native\|rust\|llvm\|wasm\|wgsl\|ebpf}` | Kompilasi 6 target |
| POST | `/api/cargo/test` | Jalankan `cargo test --all` |
| POST | `/api/repl/eval` + `GET /api/repl/vars` + `POST /api/repl/reset` | REPL stateful |
| POST | `/api/debug/breakpoints` + `/api/debug/step` | Mini debugger |
| POST | `/api/ai/autocomplete` | Saran AI offline |
| GET | `/api/file/list` · POST `/api/file/{simpan\|baca\|hapus}` | CRUD file workspace |
| POST | `/api/folder/buat` · `POST /api/workspace/set` | Kelola folder/workspace |
| GET | `/api/modul` · `/api/snippets` · `/api/dashboard` | Katalog modul, snippet, statistik |
| POST | `/api/git/{status\|commit\|log}` | Source control mini |
| POST | `/api/proyek/init` | Inisialisasi proyek |
| POST | `/api/search` | Cari rekursif di workspace |
| GET | `/api/rate/status` | Sisa kuota rate limit |

### 12.8 Pintasan Keyboard (35+ Resmi)
| Pintasan | Aksi | Pintasan | Aksi |
|---|---|---|---|
| `F5` | Jalankan tab aktif | `Alt+F5` | Mulai Debug |
| `F9` | Toggle breakpoint | `F10` | Debug step over |
| `Shift+F5` | Stop debug | `Ctrl+S` | Simpan |
| `Ctrl+Shift+S` | Save As | `Ctrl+N` | File baru |
| `Ctrl+P` | Quick Open | `Ctrl+Shift+P` | Command Palette |
| `Ctrl+,` | Pengaturan | `Ctrl+\`` | Toggle panel bawah |
| `Alt+Shift+F` | Format dokumen | `Ctrl+K Z` | Zen mode |
| `Ctrl+Shift+E` | Explorer | `Ctrl+Shift+G` | Git |
| `Ctrl+Shift+D` | Debug sidebar | `Ctrl+Shift+F` | Cari di proyek |
| `Ctrl+\` | Split kanan | `Ctrl+Space` | AI autocomplete |
| `Ctrl+W` | Tutup tab | `Ctrl+Tab` | Pindah tab |
| `Ctrl+Shift+Tab` | Tab sebelumnya | `ESC` | Tutup modal / keluar zen |

### 12.9 Troubleshooting
- **Halaman tidak terbuka** → pastikan hanya satu instansi; port dipakai → ganti `--port 9000`. Server bind `127.0.0.1` saja.
- **`429 Too Many Requests`** → tunggu hingga menit berikutnya atau cek `/api/rate/status`.
- **File tidak muncul di Explorer** → klik ikon folder di header lalu masukkan path workspace (misal `C:\widya-lang`).
- **Output kosong saat run** → pastikan memakai `cetak(...)` (bukan `tulis`, yang juga tertangkap; keduanya kini direkam di `output[]`).
- **AI tidak muncul** → aktifkan toggle AI di Pengaturan (`Ctrl+,`) → tombol "AI Autocomplete Aktif".

---

## 13. Flag Global Lengkap

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
