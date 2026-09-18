# 📘 Silabus & Modul Detail Semester 4 (Kelas 11 Genap)
## Tema: Native FFI, Database Engine, & Web/WASM (Pertemuan 49 - 64)

---

### 📅 Pertemuan 49: Konsep Foreign Function Interface (FFI) & C ABI
- **Tujuan Pembelajaran:** Siswa memahami bagaimana Widya-Lang menjembatani pemanggilan pustaka biner native C tanpa perantara lambat.
- **Teori:** Application Binary Interface (ABI), representasi pointer memori, konversi tipe primitif C ke Widya.

---

### 📅 Pertemuan 50: Menggunakan Modul Bawaan C FFI Widya-Lang
- **Tujuan Pembelajaran:** Mengakses pustaka C tingkat rendah sistem operasi untuk operasi waktu dan komputasi presisi tinggi.

---

### 📅 Pertemuan 51: Integrasi Database Relasional SQLite3 (`stdlib::sqlite`)
- **Tujuan Pembelajaran:** Membuka koneksi file database lokal `sekolah.db` dan mengeksekusi perintah pembuatan tabel (DDL).
- **Contoh Program (`p51_sqlite_init.wya`):**
```widya
gunakan stdlib::sqlite;

misal db = "sekolah.db";
misal ddl = "CREATE TABLE IF NOT EXISTS siswa (nis TEXT PRIMARY KEY, nama TEXT, kelas TEXT);";

sqlite::eksekusi(db, ddl);
cetak("✅ Database dan tabel 'siswa' berhasil disiapkan!");
```
- **Latihan Soal:** Buat tabel `presensi` dengan kolom `id`, `nis`, `status`, dan `tanggal`.

---

### 📅 Pertemuan 52: Operasi Manipulasi Data DML (`INSERT`, `UPDATE`, `DELETE`)
- **Tujuan Pembelajaran:** Menulis fungsi pembungkus (*wrapper*) untuk menambah, mengubah, dan menghapus record database secara terstruktur.
- **Contoh Program (`p52_sqlite_crud.wya`):**
```widya
gunakan stdlib::sqlite;

fungsi tambah_siswa(nis: String, nama: String, kelas: String) {
    misal sql = "INSERT OR REPLACE INTO siswa VALUES ('" + nis + "', '" + nama + "', '" + kelas + "');";
    sqlite::eksekusi("sekolah.db", sql);
    cetak("Data tersimpan:", nama);
}

tambah_siswa("2026001", "Aisyah Zahra", "XI RPL 1");
tambah_siswa("2026002", "Bima Sakti", "XI RPL 1");
```

---

### 📅 Pertemuan 53: Relasi Antar Tabel & Kueri SQL JOIN di Widya-Lang
- **Tujuan Pembelajaran:** Menghubungkan tabel master `siswa` dengan tabel data `presensi` melalui relasi kunci asing (*Foreign Key*).
- **Contoh Program (`p53_sql_join.wya`):**
```widya
// Menggabungkan data nama siswa dan status kehadirannya
misal kueri_join = "SELECT s.nama, p.status, p.tanggal FROM siswa s INNER JOIN presensi p ON s.nis = p.nis;";
```

---

### 📅 Pertemuan 54: Transaksi Database ACID & Rollback
- **Tujuan Pembelajaran:** Menjamin integritas data saat eksekusi banyak query sekaligus (Atomicity, Consistency, Isolation, Durability).

---

### 📅 Pertemuan 55: Kriptografi Industri (`stdlib::kripto`): Hashing SHA-256
- **Tujuan Pembelajaran:** Mengamankan kredensial pengguna menggunakan algoritma kriptografi hash satu arah standar NIST.
- **Contoh Program (`p55_sha256.wya`):**
```widya
gunakan stdlib::kripto;

misal kata_sandi = "KatasandiRahasia2026!";
misal hash_sandi = kripto::sha256(kata_sandi);

cetak("Kata Sandi Asli :", kata_sandi);
cetak("Hash SHA-256    :", hash_sandi);
```

---

### 📅 Pertemuan 56: Otentikasi Password dengan Argon2 & HMAC Signing
- **Tujuan Pembelajaran:** Mencegah serangan *brute-force* dan verifikasi integritas pesan berbasis kunci simetris (*HMAC*).

---

### 📅 Pertemuan 57: 🎯 Ujian Tengah Semester (UTS) — Layanan CRUD Terotentikasi Hash
- **Tugas Praktik Mandiri:** Siswa membuat modul login pengguna: memverifikasi NIS dan kata sandi yang telah di-hash SHA-256 pada database SQLite.

---

### 📅 Pertemuan 58: Arsitektur REST API & Protokol HTTP
- **Tujuan Pembelajaran:** Memahami cara kerja Client-Server, HTTP Request Methods (`GET`, `POST`, `PUT`, `DELETE`), Request Header, Payload, dan Status Code.

---

### 📅 Pertemuan 59: Serialisasi & Deserialisasi Format JSON
- **Tujuan Pembelajaran:** Mengonversi data Struct Widya-Lang menjadi string berformat standar JSON untuk pertukaran data web.
- **Contoh Program (`p59_json.wya`):**
```widya
struktur ResponApi {
    status: Int,
    pesan: String
}

fungsi ke_json(resp: ResponApi) -> String {
    kembalikan "{\"status\":" + resp.status + ",\"pesan\":\"" + resp.pesan + "\"}";
}

misal respon = ResponApi { status: 200, pesan: "Presensi berhasil dicatat" };
cetak(ke_json(respon));
```

---

### 📅 Pertemuan 60: Membangun Endpoint Backend REST API Presensi
- **Tujuan Pembelajaran:** Membuat backend service yang menerima payload presensi JSON dari aplikasi klien dan menyimpannya ke database SQLite.

---

### 📅 Pertemuan 61: Pengenalan Kompilasi Multi-Target: WebAssembly (WASM)
- **Tujuan Pembelajaran:** Mengompilasi program logika Widya menjadi biner `.wasm` yang dapat dieksekusi di browser web klien dengan performa *near-native*.
- **Perintah CLI:**
```bash
widya build-wasm src/kalkulator_web.wya -o wasm_output.wasm
```

---

### 📅 Pertemuan 62: Integrasi WebAssembly dengan Antarmuka HTML5 & CSS3
- **Tujuan Pembelajaran:** Menghubungkan fungsi komputasi WASM Widya dengan antarmuka web interaktif di folder `www/`.

---

### 📅 Pertemuan 63: Optimasi Binary Size & Latency Eksekusi WASM
- **Tujuan Pembelajaran:** Menghasilkan payload WASM berukuran kecil dan efisien untuk loading halaman secepat kilat.

---

### 📅 Pertemuan 64: 🏆 Ujian Akhir Semester (UAS) — Portal Web Presensi Siswa Terpadu
- **Tugas Proyek Akhir:** Membangun aplikasi web presensi siswa lengkap: Frontend interaktif terhubung dengan komputasi WASM Widya dan Backend REST API berpenyimpanan database SQLite3.
