# 📘 Silabus & Modul Detail Semester 4 (Kelas 11 Genap)
## Tema: Native FFI, Database Engine, & Web/WASM (16 Pertemuan)

---

### 📅 Pertemuan 49: Konsep Foreign Function Interface (FFI) & C ABI
- **Tujuan:** Memahami bagaimana Widya-Lang berinteraksi dengan library native C (`.dll`, `.so`, `.dylib`) secara langsung dan berkecepatan tinggi.

---

### 📅 Pertemuan 50: Menggunakan Modul Bawaan FFI Widya-Lang
- **Tujuan:** Memanggil fungsi sistem operasi dan manipulasi buffer memory native C.

---

### 📅 Pertemuan 51: Integrasi Database Relasional SQLite3 (`stdlib::sqlite`)
- **Tujuan:** Membuka koneksi database lokal `sekolah.db` dan mengeksekusi perintah DDL (`CREATE TABLE`).
- **Contoh Program (`p51_sqlite_init.wya`):**
```widya
gunakan stdlib::sqlite;

misal db_file = "sekolah.db";
misal sql = "CREATE TABLE IF NOT EXISTS siswa (nis TEXT PRIMARY KEY, nama TEXT, kelas TEXT);";

sqlite::eksekusi(db_file, sql);
cetak("✅ Database dan tabel 'siswa' berhasil disiapkan!");
```

---

### 📅 Pertemuan 52: Operasi Manipulasi Data DML (`INSERT`, `UPDATE`, `DELETE`)
- **Tujuan:** Menyimpan, memperbarui, dan menghapus record siswa dalam database SQLite.
- **Contoh Program (`p52_sqlite_crud.wya`):**
```widya
gunakan stdlib::sqlite;

fungsi tambah_siswa(nis: String, nama: String, kelas: String) {
    misal query = "INSERT INTO siswa VALUES ('" + nis + "', '" + nama + "', '" + kelas + "');";
    sqlite::eksekusi("sekolah.db", query);
    cetak("Data siswa berhasil ditambahkan:", nama);
}

tambah_siswa("2026001", "Aisyah Zahra", "XI RPL 1");
```

---

### 📅 Pertemuan 53: Relasi Antar Tabel & Kueri SQL JOIN
- **Tujuan:** Menghubungkan tabel master `siswa` dengan tabel transaksi `presensi` menggunakan `INNER JOIN`.

---

### 📅 Pertemuan 54: Transaksi Database ACID & Rollback
- **Tujuan:** Menjamin integritas data perbankan/kehadiran agar tidak terjadi inkonsistensi saat kegagalan listrik atau error.

---

### 📅 Pertemuan 55: Kriptografi Industri (`stdlib::kripto`): Hashing SHA-256
- **Tujuan:** Mengamankan data sensitif seperti password dengan fungsi hash satu arah tak terbalikkan (*one-way hash*).
- **Contoh Program (`p55_sha256.wya`):**
```widya
gunakan stdlib::kripto;

misal password_polos = "rahasiaSekolah123";
misal password_hash = kripto::sha256(password_polos);

cetak("Password Terenkripsi Hash:", password_hash);
```

---

### 📅 Pertemuan 56: Otentikasi Password dengan Algoritma Argon2 & HMAC
- **Tujuan:** Mencegah serangan *rainbow table* menggunakan key derivation function standar industri.

---

### 📅 Pertemuan 57: 🎯 Ujian Tengah Semester (UTS) — Layanan CRUD Terotentikasi Hash
- **Tugas Praktik:** Siswa membangun modul autentikasi login siswa berbasis SQLite dengan verifikasi password hash SHA-256.

---

### 📅 Pertemuan 58: Arsitektur REST API & Protokol HTTP
- **Tujuan:** Memahami metode HTTP (`GET`, `POST`, `PUT`, `DELETE`), Request Header, Payload, dan Status Code (200, 400, 401, 404, 500).

---

### 📅 Pertemuan 59: Serialisasi & Deserialisasi Format JSON
- **Tujuan:** Mengonversi data Struct Widya-Lang menjadi format standar JSON dan sebaliknya.
- **Contoh Program (`p59_json.wya`):**
```widya
struktur ResponApi {
    status: Int,
    pesan: String
}

fungsi format_json(resp: ResponApi) -> String {
    kembalikan "{\"status\":" + resp.status + ",\"pesan\":\"" + resp.pesan + "\"}";
}
```

---

### 📅 Pertemuan 60: Membangun Endpoint Backend REST API Presensi
- **Tujuan:** Merancang API handler yang memproses data check-in kehadiran siswa melalui format data JSON.

---

### 📅 Pertemuan 61: Pengenalan Kompilasi Multi-Target: WebAssembly (WASM)
- **Tujuan:** Mengompilasi program logika Widya-Lang menjadi biner `.wasm` yang dapat dieksekusi di browser web klien tanpa instalasi biner lokal.

---

### 📅 Pertemuan 62: Integrasi WebAssembly dengan Antarmuka HTML5 & CSS3
- **Tujuan:** Menghubungkan fungsi komputasi WASM Widya dengan elemen UI Website di folder `www/`.

---

### 📅 Pertemuan 63: Optimasi Binary Size & Latency WASM
- **Tujuan:** Menghasilkan payload WASM berukuran di bawah 100KB untuk loading halaman secepat kilat.

---

### 📅 Pertemuan 64: 🏆 Ujian Akhir Semester (UAS) — Portal Web Presensi Siswa Terpadu
- **Tugas Akhir:** Membangun aplikasi web lengkap: Frontend interaktif terhubung dengan WASM Widya dan Backend REST API berpenyimpanan SQLite3.
