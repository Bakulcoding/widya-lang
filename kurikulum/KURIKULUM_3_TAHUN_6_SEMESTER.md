# 🎓 Kurikulum Komprehensif 3 Tahun (6 Semester) Widya-Lang
**Program Keahlian:** Rekayasa Perangkat Lunak, Informatika, & Sistem Komputasi  
**Target:** Jenjang SMA / SMK / Madrasah Aliyah (Kelas 10, 11, dan 12)  
**Total Durasi:** 6 Semester (3 Tahun Penuh) — 96 Pertemuan (192 Jam Pelajaran)

---

## 🗺️ Peta Jalan Kurikulum 6 Semester (3 Tahun)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 🟢 TAHUN 1: FONDASI ALGORITMA, SINTAKS DWIBASA, & SISTEM TIPE               │
├─────────────────────────────────────────────────────────────────────────────┤
│ • Semester 1: Berpikir Komputasional, Sintaks Intuitif, & Modularitas       │
│ • Semester 2: Sistem Tipe Hindley-Milner, Generics, & Struktur Data Koleksi │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ 🟡 TAHUN 2: PEMODELAN BERORIENTASI OBJEK, DATABASE, & MULTI-TARGET          │
├─────────────────────────────────────────────────────────────────────────────┤
│ • Semester 3: OOP, Trait (Sifat), Algebraic Data Types, & Pattern Matching  │
│ • Semester 4: Native C FFI, SQLite3 Engine, REST API Backend, & WebAssembly │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ 🔵 TAHUN 3: MOBILE DEV, AUDIT KEAMANAN, KOMPILER, & PROYEK CAPSTONE         │
├─────────────────────────────────────────────────────────────────────────────┤
│ • Semester 5: Aplikasi Mobile Android/iOS & Audit Keamanan Siber Statis     │
│ • Semester 6: Full Self-Hosting Compiler Pipeline & Capstone Industry Ready │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📑 Akses Modul Detail per Semester:
- 📘 **Semester 1:** [semester_1_dasar_algoritma.md](semester_1_dasar_algoritma.md) — *Fondasi Algoritma & Pemrograman Modular (Pertemuan 1 - 16)*
- 📘 **Semester 2:** [semester_2_sistem_tipe_koleksi.md](semester_2_sistem_tipe_koleksi.md) — *Sistem Tipe Hindley-Milner & Struktur Data Koleksi (Pertemuan 17 - 32)*
- 📘 **Semester 3:** [semester_3_oop_adt_pattern_matching.md](semester_3_oop_adt_pattern_matching.md) — *OOP, Traits, ADT Enum, & Pattern Matching (Pertemuan 33 - 48)*
- 📘 **Semester 4:** [semester_4_ffi_database_web_wasm.md](semester_4_ffi_database_web_wasm.md) — *C FFI, SQLite3 Database, REST API, & WebAssembly (Pertemuan 49 - 64)*
- 📘 **Semester 5:** [semester_5_mobile_keamanan_siber.md](semester_5_mobile_keamanan_siber.md) — *Mobile App Android/iOS & Audit Keamanan Siber Statis (Pertemuan 65 - 80)*
- 📘 **Semester 6:** [semester_6_compiler_capstone_industri.md](semester_6_compiler_capstone_industri.md) — *Compiler Pipeline, Bootstrap, & Capstone Project (Pertemuan 81 - 96)*

---

## 📚 Semester 1: Fondasi Berpikir Komputasional & Pemrograman Modular (Kelas 10 - Ganjil)
*Fokus: Logika Terstruktur, Input-Output, Percabangan, Perulangan, & Fungsi Mandiri*

| Pertemuan | Topik Pembelajaran | Teori & Konsep | Praktik & Studi Kasus |
|:---:|---|---|---|
| **01 - 02** | Pengenalan Widya-Lang & Lingkungan Belajar | Instalasi CLI, VS Code Extension, dan Eksekusi skrip `.wya` | Program Biodata Diri & Perkenalan Konsol |
| **03 - 04** | Variabel (`misal`), Konstanta, & Tipe Primitif | Integer, Float, String, Boolean, Immutability | Kalkulator Konversi Suhu & Geometri |
| **05 - 06** | Operator Matematika & Logika Relasional | `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `>`, `<`, `&&`, `\|\|` | Evaluasi Kelulusan Siswa & Diskon Belanja |
| **07** | Percabangan Logika Majemuk | `jika`, `lainnya jika`, dan `lainnya` | Simulasi Tarif Listrik & Tiket Kereta |
| **08** | **Ujian Tengah Semester (UTS)** | Evaluasi Teori Logika & Praktik Dasar | Pembuatan Aplikasi Tiket Bioskop Interaktif |
| **09 - 10** | Struktur Pengulangan (`untuk` & `selama`) | Iterasi rentang `1..n`, kontrol loop (`berhenti`, `lanjut`) | Tabel Perkalian Matriks & Deret Fibonacci |
| **11 - 12** | Modularitas Kode & Fungsi (`fungsi`) | Parameter, Return Value, Scope (Lokal vs Global) | Library Rumus Fisika & Matematika |
| **13 - 14** | Fungsi Rekursif & Stack Execution | Call Stack, Base Case, Rekursi Sederhana | Faktorial Angka & Menara Hanoi |
| **15 - 16** | **Ujian Akhir Semester (UAS)** | Evaluasi Pemrograman Modular | **Proyek 1:** Mini Game Terminal *"Petualangan Ksatria"* |

---

## 📚 Semester 2: Sistem Tipe Hindley-Milner & Struktur Data Koleksi (Kelas 10 - Genap)
*Fokus: Type Inference Matematis, Generics, Larik, Tuple, & Algoritma Pengurutan*

| Pertemuan | Topik Pembelajaran | Teori & Konsep | Praktik & Studi Kasus |
|:---:|---|---|---|
| **17 - 18** | Teori Tipe Statis & Hindley-Milner (HM) | Compile-time vs Runtime Safety, Type Reconstruction | Verifikasi Tipe tanpa Deklarasi Manual |
| **19 - 20** | Pemrograman Generic (`<T>`) | Abstraksi tipe data umum tanpa degradasi performa | Fungsi Penukar Posisi (Generic Swap) & Identity |
| **21 - 22** | Tuple & Koleksi Data Heterogen | Pengelompokan data berpasangan `(T1, T2)` | Representasi Titik Koordinat GPS & Nilai Rapor |
| **23 - 24** | Struktur Data Larik / Array Dinamis | Indeks array, alokasi memori, boundary checking | Manajemen Daftar Nilai & Kehadiran Kelas |
| **25** | **Ujian Tengah Semester (UTS)** | Evaluasi Pemrograman Generics & Koleksi | Implementasi Struktur Data Antrean (*Queue*) |
| **26 - 27** | Algoritma Pencarian (*Search Algorithms*) | Linear Search vs Binary Search (Pencarian Biner) | Pencarian NIS Siswa dalam Database Sekolah |
| **28 - 29** | Algoritma Pengurutan (*Sorting Algorithms*) | Bubble Sort, Insertion Sort, & Merge Sort sederhana | Pengurutan Ranking Paralel Siswa |
| **30 - 31** | Optimasi & Profiler Memori Awal | Deteksi siklus referensi (*Tarjan Cycle Detector*) | Profiling Penggunaan RAM pada Array 10.000 Elemen |
| **32** | **Ujian Akhir Semester (UAS)** | Evaluasi Algoritma Koleksi | **Proyek 2:** Sistem Pemeringkatan Siswa & Ekspor Data |

---

## 📚 Semester 3: Pemodelan Berorientasi Objek & Aljabar Tipe Data (Kelas 11 - Ganjil)
*Fokus: `struktur`, `sifat` (Traits), ADT `pilihan`, & Exhaustive Pattern Matching*

| Pertemuan | Topik Pembelajaran | Teori & Konsep | Praktik & Studi Kasus |
|:---:|---|---|---|
| **33 - 34** | Pemodelan Data dengan `struktur` (Structs) | Enkapsulasi data entitas, inisialisasi struct | Entitas Data Master Siswa & Guru |
| **35 - 36** | Metode Instance & Referensi Diri (`&diri`) | Metode anggota, pemutakhiran state objek | Perhitungan IPK Otomatis pada Objek Mahasiswa |
| **37 - 38** | Abstraksi Kontrak dengan `sifat` (Traits) | Polimorfisme dinamis & statis, Trait bounds | Sifat `DapatDicetak` dan `DapatDivalidasi` |
| **39 - 40** | Algebraic Data Types (ADT) dengan `pilihan` | Sum Types vs Product Types, Eliminasi Null Pointer | Model State: `Opsi<T>` (`Ada(T)` / `Kosong`) |
| **41** | **Ujian Tengah Semester (UTS)** | Pemodelan Sistem Berbasis ADT & Structs | Desain Skema Pembayaran SPP Sekolah |
| **42 - 44** | Pencocokan Pola Mendalam (`cocokkan`) | Pattern matching dengan destructuring & guard clause | Penanganan State Mesin ATM & Transaksi Finansial |
| **45 - 46** | Penanganan Galat Modern (*Safe Error Handling*) | Result Type `Hasil<T, E>` vs Exception Klasik | Parser File Konfigurasi Tanpa Crash |
| **47 - 48** | **Ujian Akhir Semester (UAS)** | Rekayasa Polimorfisme & ADT | **Proyek 3:** Simulator Transaksi Bank & Manajemen Rekening |

---

## 📚 Semester 4: Native FFI, Database Engine, & Web/WASM (Kelas 11 - Genap)
*Fokus: Foreign Function Interface (C ABI), SQLite3 FFI, Backend REST API, & WASM*

| Pertemuan | Topik Pembelajaran | Teori & Konsep | Praktik & Studi Kasus |
|:---:|---|---|---|
| **49 - 50** | Pengenalan C Foreign Function Interface (FFI) | Memanggil pustaka dinamis C (`.dll`/`.so`) dari Widya | Integrasi Pustaka Native C Matematika & Waktu |
| **51 - 52** | Integrasi Database SQLite3 (`stdlib::sqlite`) | Pembuatan Tabel DDL, Query DML (`INSERT`, `SELECT`) | Database Presensi Siswa `presensi.db` |
| **53 - 54** | Relasi Tabel Database & SQL JOIN di Widya | Foreign Key, One-to-Many, Transaksi Database ACID | Sistem Perpustakaan Sekolah (Buku & Peminjaman) |
| **55 - 56** | Kriptografi Industri (`stdlib::kripto`) | Enkripsi AES-GCM, Hash SHA-256, Argon2 Password | Sistem Enkripsi Kata Sandi & Token Keamanan |
| **57** | **Ujian Tengah Semester (UTS)** | Integrasi C FFI & Relasi Database | Service CRUD Database Berotentikasi Hash |
| **58 - 60** | Arsitektur REST API & Parsing JSON | HTTP Handlers, Routing, Serialisasi JSON | Backend API Katalog Buku Sekolah |
| **61 - 62** | Kompilasi Multi-Target: WebAssembly (WASM) | Kompilasi ke `.wasm` & integrasi HTML/JavaScript | Live Web Playground & Eksekusi di Browser |
| **63 - 64** | **Ujian Akhir Semester (UAS)** | Full-Stack Web & Database Integration | **Proyek 4:** Aplikasi Web Presensi Siswa Berbasis SQLite & API |

---

## 📚 Semester 5: Mobile App Development & Audit Keamanan Siber (Kelas 12 - Ganjil)
*Fokus: Mobile Engine (Android & iOS), Static Security AST Linter, & Defensive Coding*

| Pertemuan | Topik Pembelajaran | Teori & Konsep | Praktik & Studi Kasus |
|:---:|---|---|---|
| **65 - 66** | Arsitektur Mobile Engine Widya-Lang | Perancah proyek otomatis via `widya mobile-init` | Generate Template Aplikasi Android & iOS |
| **67 - 69** | Desain Antarmuka Mobile Deklaratif | Widget Layout, State Management, Interaksi Tombol | Aplikasi Mobile Kasir Kantin & Kartu Pelajar Digital |
| **70 - 71** | Integrasi SQLite & Penyimpanan Lokal Mobile | Sinkronisasi data offline-to-online pada smartphone | Aplikasi Catatan Harian Siswa Terenkripsi |
| **72** | **Ujian Tengah Semester (UTS)** | Uji Kompilasi & Simulasi Mobile APK | Build APK Android Presensi Siswa |
| **73 - 75** | Keamanan Siber: Deteksi Kerentanan Statis | OWASP Top 10, SQL Injection (SQLi), XSS, Secret Leaks | Simulasi Peretasan & Eksploitasi Kode Rentan |
| **76 - 78** | Audit Keamanan Otomatis via `widya audit` | AST Inspection, Pencegahan Hardcoded Token | Remediasi & Patching Kode hingga Skor Keamanan 100% |
| **79 - 80** | **Ujian Akhir Semester (UAS)** | Mobile Deployment & Security Hardening | **Proyek 5:** Aplikasi Mobile Kartu Pelajar Digital Bebas Celah Keamanan |

---

## 📚 Semester 6: Kompiler Self-Hosting & Proyek Capstone Industri (Kelas 12 - Genap)
*Fokus: Compiler Architecture, 3-Stage Bootstrap, Multi-Target LLVM/eBPF, & Capstone Project*

| Pertemuan | Topik Pembelajaran | Teori & Konsep | Praktik & Studi Kasus |
|:---:|---|---|---|
| **81 - 82** | Arsitektur Kompiler Self-Hosting | Analisis Kode Sumber di `compiler_self_hosted/` | Membaca Lexer & Tokenizer dalam Bahasa Widya |
| **83 - 84** | 3-Stage Deterministic Bootstrap | Proses `widya bootstrap` (Stage-0 $\rightarrow$ Stage-1 $\rightarrow$ Stage-2) | Verifikasi Kesamaan Biner (Bit-for-Bit Determinism) |
| **85 - 86** | Multi-Target Backend: LLVM IR & eBPF | Menghasilkan kode `.ll` dan Linux Kernel Sandbox | Optimasi Kompilasi O3 & Kernel Packet Filter |
| **87 - 88** | WPM Package Manager & Lockfile Integrity | Pengelolaan modul, dependensi, verifikasi SHA-256 | Menerbitkan Pustaka (*Publish Library*) ke Ekosistem |
| **89 - 90** | Automated Unit Testing & CI/CD Pipeline | Menulis Test Suites dengan `widya test`, GitHub Actions | Otomatisasi Uji Kualitas Kode (CI Pipeline) |
| **91 - 94** | Pengerjaan Proyek Akhir Kelompok (Capstone) | Kolaborasi tim, arsitektur enterprise skala besar | Bimbingan Intensif Proyek Akhir Sekolah |
| **95 - 96** | **Sidang Uji Kompetensi Keahlian (UKK) & Pameran Karya** | Presentasi Produk, Demo Aplikasi, & Audit Kode | **Pameran Karya Perangkat Lunak Nasional** |

---

## 🏆 Standar Kompetensi Kelulusan 3 Tahun (SKL)

Setelah menuntaskan kurikulum 6 semester ini, lulusan memiliki portofolio dan kompetensi:
1. **Junior Systems & Application Programmer**: Menguasai konsep algoritma, memori, dan inferensi tipe modern setara standar industri global (Rust/Go/Swift/TypeScript).
2. **Database & Backend Engineer**: Mampu merancang REST API terproteksi, database relasional SQLite, dan integritas transaksi ACID.
3. **Mobile & Web Developer**: Mengembangkan aplikasi lintas platform untuk Android, iOS, dan WebAssembly.
4. **Security-Aware Developer**: Terbiasa menulis kode defensif yang lolos audit keamanan statis siber (*Zero Secret Leak, Anti-SQLi, Anti-XSS*).
5. **Kesiapan Sertifikasi & Dunia Kerja**: Memiliki portofolio GitHub aktif, pemahaman CI/CD, dan siap bekerja atau melanjutkan ke perguruan tinggi jurusan Teknik Informatika / Ilmu Komputer.
