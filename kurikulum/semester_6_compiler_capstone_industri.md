# 📘 Silabus & Modul Detail Semester 6 (Kelas 12 Genap)
## Tema: Compiler Pipeline, Self-Hosting, & Capstone Project (Pertemuan 81 - 96)

---

### 📅 Pertemuan 81: Anatomi Arsitektur Kompiler Modern
- **Tujuan Pembelajaran:** Siswa memahami aliran pemrosesan kompiler: Teks Mentah $\rightarrow$ Token (Lexing) $\rightarrow$ AST (Parsing) $\rightarrow$ Typed AST (Type Checking) $\rightarrow$ IR $\rightarrow$ Machine Code / Native Executable.

---

### 📅 Pertemuan 82: Analisis Kode Kompiler Self-Hosting (`compiler_self_hosted/`)
- **Tujuan Pembelajaran:** Mempelajari bagaimana kompiler Widya ditulis seluruhnya menggunakan bahasa Widya sendiri (`lexer.widya`, `parser.widya`, `ast.widya`).

---

### 📅 Pertemuan 83: 3-Stage Deterministic Bootstrap (`widya bootstrap`)
- **Tujuan Pembelajaran:** Mempraktikkan proses kompilasi bootstrap 3 tahap (Stage-0 $\rightarrow$ Stage-1 $\rightarrow$ Stage-2) dan memverifikasi kesamaan bit biner (*Bit-for-Bit Determinism*).
- **Perintah CLI:**
```bash
widya bootstrap
```

---

### 📅 Pertemuan 84: Backend Multi-Target: LLVM IR Code Generation (`.ll`)
- **Tujuan Pembelajaran:** Memahami emit instruksi tingkat rendah LLVM dan optimasi tingkat mesin (`-O1`, `-O2`, `-O3`).
- **Perintah CLI:**
```bash
widya build-llvm src/program.wya -o llvm_output.ll
```

---

### 📅 Pertemuan 85: Backend Kernel Sandbox: Linux eBPF (`.bpf.c`)
- **Tujuan Pembelajaran:** Memahami kompilasi program ke dalam kernel Linux untuk inspeksi paket jaringan tingkat tinggi dan keamanan sistem operasi.

---

### 📅 Pertemuan 86: Backend GPU Compute Shader: WebGPU WGSL (`.wgsl`)
- **Tujuan Pembelajaran:** Mengompilasi kode matematika paralel untuk dieksekusi langsung pada chip kartu grafis (GPU).

---

### 📅 Pertemuan 87: Manajemen Dependensi Industri: WPM & `widya.lock`
- **Tujuan Pembelajaran:** Memahami cara kerja package manager, resolusi dependensi semver, dan verifikasi checksum SHA-256 untuk mencegah serangan rantai pasok (*supply-chain attack*).

---

### 📅 Pertemuan 88: Standardisasi Otomatisasi Uji: `widya test` & Code Coverage
- **Tujuan Pembelajaran:** Menulis unit test dan integration test suite untuk menjamin tidak adanya regresi bug pada software enterprise.
- **Perintah CLI:**
```bash
widya test
```

---

### 📅 Pertemuan 89: Integrasi CI/CD Pipeline (GitHub Actions)
- **Tujuan Pembelajaran:** Mengotomatiskan proses build, test, audit, dan packaging setiap kali kode di-push ke git repository.

---

### 📅 Pertemuan 90: Penyusunan Dokumen Spesifikasi Rekayasa Perangkat Lunak (SRS)
- **Tujuan Pembelajaran:** Merancang dokumen teknis arsitektur proyek Capstone akhir sekolah secara profesional berstandar industri.

---

### 📅 Pertemuan 91 - 94: Bimbingan Intensif Pengerjaan Proyek Capstone Kelompok
- **Pilihan Topik Proyek Capstone:**
  1. *Smart School ERP System* (Web, Mobile, SQLite, API, Multi-User).
  2. *Distributed IoT Monitoring Engine* (Sensor Data, eBPF Network, Web Dashboard).
  3. *Secure Mobile Banking & Point of Sale (POS) Enterprise*.

---

### 📅 Pertemuan 95: Simulasi Sidang Teknis & Code Review Antar Tim
- **Tujuan Pembelajaran:** Melatih kemampuan presentasi teknis, argumentasi arsitektur perangkat lunak, dan *peer-review* kode antarsiswa.

---

### 📅 Pertemuan 96: 🏆 Sidang Uji Kompetensi Keahlian (UKK) & Pameran Karya Software Nasional
- **Parameter Penilaian Kelulusan:**
  1. Kelayakan Arsitektur Kode & Desain Sistem Berbasis Widya-Lang.
  2. Hasil Uji Otomatis (`widya test` 100% Passed).
  3. Sertifikasi Keamanan (`widya audit` 100% Clean).
  4. Demo Produk Aplikasi Live (Web / Mobile / Desktop).
