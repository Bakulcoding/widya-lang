# 📘 Silabus & Modul Detail Semester 6 (Kelas 12 Genap)
## Tema: Compiler Pipeline, Self-Hosting, & Proyek Capstone Industri (16 Pertemuan)

---

### 📅 Pertemuan 81: Anatomi Arsitektur Kompiler Modern
- **Tujuan:** Memahami aliran data dari teks mentah $\rightarrow$ Token $\rightarrow$ Abstract Syntax Tree (AST) $\rightarrow$ Typed AST $\rightarrow$ Intermediate Representation (IR) $\rightarrow$ Machine Code.

---

### 📅 Pertemuan 82: Analisis Kode Kompiler Self-Hosting (`compiler_self_hosted/`)
- **Tujuan:** Mempelajari bagaimana kompiler Widya ditulis seluruhnya menggunakan bahasa Widya sendiri (`lexer.widya`, `parser.widya`, `ast.widya`).

---

### 📅 Pertemuan 83: 3-Stage Deterministic Bootstrap (`widya bootstrap`)
- **Tujuan:** Mempraktikkan proses kompilasi bootstrap 3 tahap (Stage-0 $\rightarrow$ Stage-1 $\rightarrow$ Stage-2) dan memverifikasi kesamaan bit biner (Deterministic Builds).

---

### 📅 Pertemuan 84: Backend Multi-Target: LLVM IR Code Generation (`.ll`)
- **Tujuan:** Memahami emit instruksi tingkat rendah LLVM dan optimasi tingkat mesin (`-O1`, `-O2`, `-O3`).

---

### 📅 Pertemuan 85: Backend Kernel Sandbox: Linux eBPF (`.bpf.c`)
- **Tujuan:** Memahami kompilasi program ke dalam kernel Linux untuk inspeksi paket jaringan tingkat tinggi dan keamanan sistem operasi.

---

### 📅 Pertemuan 86: Backend GPU Compute Shader: WebGPU WGSL (`.wgsl`)
- **Tujuan:** Mengompilasi kode matematika paralel untuk dieksekusi langsung pada chip kartu grafis (GPU).

---

### 📅 Pertemuan 87: Manajemen Dependensi Industri: WPM & `widya.lock`
- **Tujuan:** Memahami cara kerja package manager, resolusi dependensi semver, dan verifikasi checksum SHA-256 untuk mencegah serangan *supply-chain attack*.

---

### 📅 Pertemuan 88: Standardisasi Otomatisasi Uji: `widya test` & Code Coverage
- **Tujuan:** Menulis unit test dan integration test suite untuk menjamin tidak adanya regresi bug pada software enterprise.

---

### 📅 Pertemuan 89: Integrasi CI/CD Pipeline (GitHub Actions)
- **Tujuan:** Mengotomatiskan proses build, test, audit, dan packaging setiap kali kode di-push ke git repository.

---

### 📅 Pertemuan 90: Penyusunan Dokumen Spesifikasi Rekayasa Perangkat Lunak (SRS)
- **Tujuan:** Merancang dokumen teknis arsitektur proyek Capstone akhir sekolah secara profesional.

---

### 📅 Pertemuan 91 - 94: Bimbingan Intensif Pengerjaan Proyek Capstone Kelompok
- **Topik Pilihan Proyek Capstone:**
  1. *Smart School ERP System* (Web, Mobile, SQLite, API, Multi-User).
  2. *Distributed IoT Monitoring Engine* (Sensor Data, eBPF Network, Web Dashboard).
  3. *Secure Mobile Banking & Point of Sale (POS) Enterprise*.

---

### 📅 Pertemuan 95: Simulasi Sidang Teknis & Code Review Antar Tim
- **Tujuan:** Melatih kemampuan presentasi teknis, argumentasi arsitektur perangkat lunak, dan *peer-review* kode.

---

### 📅 Pertemuan 96: 🏆 Sidang Uji Kompetensi Keahlian (UKK) & Pameran Karya Software Nasional
- **Penilaian Akhir:**
  1. Kelayakan Arsitektur Kode & Desain Sistem.
  2. Hasil Uji Otomatis (`widya test` 100% Passed).
  3. Sertifikasi Keamanan (`widya audit` 100% Clean).
  4. Demo Produk Aplikasi Live (Web/Mobile/Desktop).
