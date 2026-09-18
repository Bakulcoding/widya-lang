# Changelog

Semua perubahan penting di Widya-Lang akan didokumentasikan dalam file ini.

Format didasarkan pada [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), dan proyek ini menggunakan [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0-beta.1] - 2026-09-18

### 🎉 Public Beta Preview & Full Self-Hosting Release

#### 🚀 Fitur Baru & Arsitektur Utama
- **100% Full Self-Hosting Compiler Pipeline**: Kompiler Widya (`compiler_self_hosted/`) mampu mengompilasi dirinya sendiri melalui 3-stage bootstrap deterministik (`widya bootstrap`).
- **Sistem Tipe Statis Hindley-Milner**: Type inference otomatis tanpa boilerplate tipe manual, Algebraic Data Types (ADT), Pattern Matching (`cocokkan`), dan Traits / Type Classes (`sifat`, `implementasi`).
- **Multi-Target Compilation**:
  - LLVM IR (`.ll`) backend & Native OS Binaries
  - WebAssembly (WASM) & Interactive Web Playground
  - WebGPU Compute Shader (`.wgsl`)
  - Linux Kernel eBPF (`.bpf.c`)
- **Mobile Engine Bawaan**: Direct scaffold & kompilasi untuk **Android** (APK/AAB + Gradle) dan **iOS** (Xcode Project + Swift Bridge) tanpa ketergantungan framework luar.
- **Audit Keamanan Statis (AST Linter)**: Detektor rahasia/API key yang bocor, analisis pencegahan SQL Injection (SQLi), dan proteksi XSS bawaan via `widya audit`.
- **Ekosistem & Developer Experience**:
  - Language Server Protocol (LSP) bawaan (`widya lsp`) untuk editor modern & VS Code Extension.
  - Debug Adapter Protocol (DAP) bawaan (`widya dap`) untuk breakpointing & variable inspection.
  - Interactive REPL (`widya repl`) dengan history dan syntax highlighting.
  - Widya Package Manager (WPM) dengan verifikasi integritas SHA-256 (`widya.lock`).
  - Safe Memory Profiler & Tarjan Cycle Detector.
  - Standard Library C FFI: SQLite3 engine & Industrial Cryptography (AES-GCM, Argon2, Ed25519, ChaCha20-Poly1305, HMAC, SHA-256).

#### 📚 Dokumentasi & Edukasi
- Rilis buku panduan resmi **`PANDUAN.md`** mencakup 18 bab lengkap dari dasar hingga topik lanjutan.
- Peluncuran portal web interaktif & WebAssembly Playground di `www/`.
- 54+ contoh kode praktis di folder `contoh/`.
- 111 Unit & Integration Test Suites lulus 100%.

---

## [0.1.0] - 2026-09-17

### 🏗️ Production Infrastructure & Foundations

#### 🚀 Fitur Baru - Production Infrastructure
- **Sharding Engine** - Horizontal scaling dengan consistent hashing ring (100+ virtual nodes/node).
- **Distributed Consensus** - Algoritma Raft & Paxos untuk leader election & log replication.
- **Multi-Node Replication** - Master-slave, master-master, quorum-based dengan conflict resolution.
- **Performance Benchmarks** - Comprehensive suite untuk latency, throughput, SQL, replication.
- **CI/CD Pipeline** - Full GitHub Actions workflow dengan multi-stage pipeline.
- **Docker Deployment** - Multi-stage builds, Compose untuk cluster production.
- **System Call Interface** - POSIX subset system calls untuk pengembangan Widya OS.
- **VFS Layer** - Virtual Filesystem dengan mount points & inode management.
- **SQL JOIN Support** - Operasi INNER, LEFT, RIGHT, FULL JOIN di parser.

#### 🏗️ Database Engine (WidyaDB)
- WAL (Write-Ahead Logging) dengan protokol ARIES.
- B+Tree dengan bulk load & range scans.
- Query Optimizer dengan cost model.
- Execution engine dengan vectorized processing.

#### 🔧 OS Primitives (WidyaOS)
- Kernel threading dengan preemptive scheduling.
- Memory management (page allocator, slab allocator).
- Synchronization primitives (Spinlock, Mutex, Semaphore).
- Interrupt handling (IDT, PIC, Exception handler).
- Filesystem VFS layer dengan mount point management.

---

## [0.0.9] - 2026-09-10

### 🚀 Initial Exploration Release
- Web & Desktop Applications foundation.
- GPU Computing & WebGPU initial experiment.
- Network Programming (WebSocket, TCP/IP).
- Cross-platform compilation pipeline foundations.