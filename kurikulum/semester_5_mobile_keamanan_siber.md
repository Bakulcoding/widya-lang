# 📘 Silabus & Modul Detail Semester 5 (Kelas 12 Ganjil)
## Tema: Mobile App Development & Audit Keamanan Siber (16 Pertemuan)

---

### 📅 Pertemuan 65: Arsitektur Mobile Engine Bawaan Widya-Lang
- **Tujuan:** Memahami bagaimana kompiler Widya mentranslasi logika kode langsung ke layer native Android (NDK/Gradle) dan iOS (Xcode/Swift Bridge) tanpa ketergantungan framework pihak ketiga.

---

### 📅 Pertemuan 66: Perancangan Proyek Mobile dengan `widya mobile-init`
- **Tujuan:** Menggunakan perintah scaffold otomatis untuk membentuk struktur folder aplikasi mobile.
- **Perintah CLI:**
```bash
widya mobile-init KartuPelajarDigital --package com.sekolah.kartupelajar
```

---

### 📅 Pertemuan 67: Desain Antarmuka Deklaratif Mobile (Widgets & Layouts)
- **Tujuan:** Menyusun hirarki visual antarmuka: Kolom, Baris, Tombol, Kotak Teks, dan Gambar.
- **Contoh Program (`p67_mobile_ui.wya`):**
```widya
struktur TampilanKartu {
    nama_siswa: String,
    jurusan: String,
    foto_url: String
}
```

---

### 📅 Pertemuan 68: Manajemen State & Event Handling Tombol Mobile
- **Tujuan:** Mengelola status reaktif (*reactive state*) saat tombol ditekan atau input teks dimasukkan pengguna.

---

### 📅 Pertemuan 69: Navigasi Antar Halaman & Routing Mobile
- **Tujuan:** Mengatur transisi layar dari Halaman Login $\rightarrow$ Dashboard Siswa $\rightarrow$ Profil.

---

### 📅 Pertemuan 70: Penyimpanan Lokal Mobile & Cache Offline (SQLite Mobile)
- **Tujuan:** Mengimplementasikan database lokal di perangkat smartphone agar aplikasi tetap dapat bekerja saat tidak ada sinyal internet (*offline-first design*).

---

### 📅 Pertemuan 71: Sinkronisasi Data Latar Belakang (Background Sync)
- **Tujuan:** Mengirim data rekaman presensi offline ke server pusat saat koneksi internet kembali aktif.

---

### 📅 Pertemuan 72: 🎯 Ujian Tengah Semester (UTS) — Build Prototype APK Android
- **Tugas Praktik:** Mengompilasi dan menguji jalannya file APK Kartu Pelajar Digital pada emulator atau perangkat fisik Android.

---

### 📅 Pertemuan 73: Pengantar Keamanan Siber & OWASP Top 10
- **Tujuan:** Mempelajari 10 celah kerentanan aplikasi perangkat lunak paling umum dan berbahaya di dunia industri.

---

### 📅 Pertemuan 74: Analisis & Eksploitasi Serangan SQL Injection (SQLi)
- **Tujuan:** Memahami bagaimana penyerang menyusupkan kode SQL berbahaya melalui input form yang tidak disanitasi.

---

### 📅 Pertemuan 75: Bahaya Hardcoded Secrets & Kebocoran Token / API Key
- **Tujuan:** Memahami risiko bencana keamanan jika password database atau API key ditulis langsung di dalam source code publik.

---

### 📅 Pertemuan 76: Audit Keamanan Statis Otomatis via `widya audit`
- **Tujuan:** Menjalankan AST Linter bawaan Widya untuk mendeteksi celah keamanan secara instan sebelum aplikasi dirilis.
- **Perintah CLI:**
```bash
widya audit src/aplikasi_mobile.wya
```

---

### 📅 Pertemuan 77: Teknik Sanitasi Parameterized Query & Anti-XSS
- **Tujuan:** Memperbaiki (*remediasi*) kode yang rentan menjadi 100% aman dan tahan serangan siber.

---

### 📅 Pertemuan 78: Enkripsi Data Sensitif di Sisi Klien (Client-Side Storage Encryption)
- **Tujuan:** Melindungi cache lokal pada smartphone menggunakan AES-256 GCM.

---

### 📅 Pertemuan 79 - 80: 🏆 Ujian Akhir Semester (UAS) — Aplikasi Mobile Bebas Celah Keamanan
- **Tugas Akhir:** Membangun aplikasi mobile presensi dan kartu pelajar digital yang lulus audit statis `widya audit` dengan skor keamanan 100% (Zero Secret Leak, Zero SQLi, Zero XSS).
