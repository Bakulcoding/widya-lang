# 📘 Silabus & Modul Detail Semester 5 (Kelas 12 Ganjil)
## Tema: Mobile App Development & Audit Keamanan Siber (Pertemuan 65 - 80)

---

### 📅 Pertemuan 65: Arsitektur Mobile Engine Bawaan Widya-Lang
- **Tujuan Pembelajaran:** Siswa memahami bagaimana engine compiler Widya mentranslasi logika kode langsung ke layer native Android (NDK/Gradle) dan iOS (Xcode/Swift Bridge) tanpa ketergantungan framework pihak ketiga.

---

### 📅 Pertemuan 66: Perancangan Proyek Mobile dengan `widya mobile-init`
- **Tujuan Pembelajaran:** Menggunakan perintah scaffold otomatis untuk membentuk struktur folder proyek aplikasi mobile.
- **Perintah CLI:**
```bash
widya mobile-init KartuPelajarDigital --package com.sekolah.kartupelajar
```

---

### 📅 Pertemuan 67: Desain Antarmuka Deklaratif Mobile (Widgets & Layouts)
- **Tujuan Pembelajaran:** Menyusun hirarki visual antarmuka: Kolom, Baris, Tombol, Kotak Teks, dan Gambar secara deklaratif.
- **Contoh Program (`p67_mobile_ui.wya`):**
```widya
struktur TampilanKartu {
    nis: String,
    nama_siswa: String,
    jurusan: String
}

fungsi format_kartu(k: TampilanKartu) -> String {
    kembalikan "[KARTU PELAJAR] " + k.nama_siswa + " (" + k.jurusan + ") - NIS: " + k.nis;
}
```

---

### 📅 Pertemuan 68: Manajemen State & Event Handling Tombol Mobile
- **Tujuan Pembelajaran:** Mengelola status reaktif (*reactive state*) saat tombol ditekan atau input teks dimasukkan pengguna di layar smartphone.

---

### 📅 Pertemuan 69: Navigasi Antar Halaman & Routing Mobile
- **Tujuan Pembelajaran:** Mengatur alur transisi layar dari Halaman Login $\rightarrow$ Dashboard Siswa $\rightarrow$ Profil.

---

### 📅 Pertemuan 70: Penyimpanan Lokal Mobile & Cache Offline (SQLite Mobile)
- **Tujuan Pembelajaran:** Mengimplementasikan database lokal di perangkat smartphone agar aplikasi tetap dapat bekerja saat koneksi internet terputus (*offline-first design*).

---

### 📅 Pertemuan 71: Sinkronisasi Data Latar Belakang (Background Sync)
- **Tujuan Pembelajaran:** Mengirim rekaman presensi offline yang tersimpan di memori smartphone ke server pusat ketika koneksi internet aktif kembali.

---

### 📅 Pertemuan 72: 🎯 Ujian Tengah Semester (UTS) — Build Prototype APK Android
- **Tugas Praktik Mandiri:** Mengompilasi dan menguji jalannya file APK Kartu Pelajar Digital pada emulator atau perangkat fisik Android.

---

### 📅 Pertemuan 73: Pengantar Keamanan Siber & OWASP Top 10
- **Tujuan Pembelajaran:** Mempelajari 10 celah kerentanan aplikasi perangkat lunak paling umum dan berbahaya di dunia industri siber modern.

---

### 📅 Pertemuan 74: Analisis & Eksploitasi Serangan SQL Injection (SQLi)
- **Tujuan Pembelajaran:** Memahami bagaimana penyerang menyusupkan instruksi SQL berbahaya melalui input form yang tidak disanitasi.

---

### 📅 Pertemuan 75: Bahaya Hardcoded Secrets & Kebocoran Token / API Key
- **Tujuan Pembelajaran:** Memahami risiko bencana keamanan jika password database atau API key ditulis langsung di dalam source code publik.

---

### 📅 Pertemuan 76: Audit Keamanan Statis Otomatis via `widya audit`
- **Tujuan Pembelajaran:** Menjalankan AST Linter bawaan Widya untuk mendeteksi celah keamanan secara instan sebelum aplikasi dirilis.
- **Perintah CLI:**
```bash
widya audit src/aplikasi_mobile.wya
```

---

### 📅 Pertemuan 77: Teknik Sanitasi Parameterized Query & Anti-XSS
- **Tujuan Pembelajaran:** Memperbaiki (*remediasi*) kode yang rentan menjadi 100% aman dan tahan terhadap serangan siber.

---

### 📅 Pertemuan 78: Enkripsi Data Sensitif di Sisi Klien (Client-Side Storage Encryption)
- **Tujuan Pembelajaran:** Melindungi cache lokal pada smartphone menggunakan enkripsi algoritma simetris AES-256 GCM.

---

### 📅 Pertemuan 79 - 80: 🏆 Ujian Akhir Semester (UAS) — Aplikasi Mobile Bebas Celah Keamanan
- **Tugas Proyek Akhir:** Membangun aplikasi mobile presensi dan kartu pelajar digital yang lulus audit statis `widya audit` dengan skor keamanan 100% (Zero Secret Leak, Zero SQLi, Zero XSS).
