# 📖 Modul Pembelajaran Lengkap Widya-Lang — Tingkat SMA/SMK (Kelas 10 - 12)
**Penulis / Pengembang:** Tim Kurikulum Widya-Lang  
**Sasaran:** Siswa Kelas 10, 11, dan 12 SMA/SMK (Peminatan Informatika, Rekayasa Perangkat Lunak, SIJA)  
**Pendekatan:** *Industrial Software Engineering*, Hindley-Milner Type System, ADT, C FFI, dan Keamanan Siber

---

## 📑 Daftar Isi Modul (16 Pertemuan)

- [Pertemuan 01: Arsitektur Kompiler Modern & Self-Hosting Pipeline](#pertemuan-01-arsitektur-kompiler-modern--self-hosting-pipeline)
- [Pertemuan 02: Inferensi Tipe Hindley-Milner & Compile-Time Safety](#pertemuan-02-inferensi-tipe-hindley-milner--compile-time-safety)
- [Pertemuan 03: Generic Programming (`<T>`) Tanpa Overhead](#pertemuan-03-generic-programming-t-tanpa-overhead)
- [Pertemuan 04: Pemodelan Data dengan `struktur` (Structs)](#pertemuan-04-pemodelan-data-dengan-struktur-structs)
- [Pertemuan 05: Abstraksi Perilaku dengan `sifat` (Traits / Interfaces)](#pertemuan-05-abstraksi-perilaku-dengan-sifat-traits--interfaces)
- [Pertemuan 06: Algebraic Data Types (ADT) dengan `pilihan` (Enums)](#pertemuan-06-algebraic-data-types-adt-dengan-pilihan-enums)
- [Pertemuan 07: Pattern Matching Komprehensif (`cocokkan`)](#pertemuan-07-pattern-matching-komprehensif-cocokkan)
- [Pertemuan 08: Ujian Tengah Semester — Library Struktur Data Generic](#pertemuan-08-ujian-tengah-semester--library-struktur-data-generic)
- [Pertemuan 09: Pustaka Standar C Foreign Function Interface (FFI)](#pertemuan-09-pustaka-standar-c-foreign-function-interface-ffi)
- [Pertemuan 10: Integrasi Basis Data Relasional SQLite3](#pertemuan-10-integrasi-basis-data-relasional-sqlite3)
- [Pertemuan 11: Kriptografi Terapan & Keamanan Token (SHA-256 / HMAC)](#pertemuan-11-kriptografi-terapan--keamanan-token-sha-256--hmac)
- [Pertemuan 12: Audit Keamanan Statis & AST Linter (`widya audit`)](#pertemuan-12-audit-keamanan-statis--ast-linter-widya-audit)
- [Pertemuan 13: Arsitektur REST API & Parsing JSON](#pertemuan-13-arsitektur-rest-api--parsing-json)
- [Pertemuan 14: Pengembangan Mobile Terintegrasi (Android / iOS)](#pertemuan-14-pengembangan-mobile-terintegrasi-android--ios)
- [Pertemuan 15: Unit Testing & Automated Test Suites (`widya test`)](#pertemuan-15-unit-testing--automated-test-suites-widya-test)
- [Pertemuan 16: Proyek Akhir — Sistem Backend API Presensi Sekolah Berbasis DB & Security Audit](#pertemuan-16-proyek-akhir--sistem-backend-api-presensi-sekolah-berbasis-db--security-audit)

---

## Pertemuan 01: Arsitektur Kompiler Modern & Self-Hosting Pipeline

### 🎯 Tujuan Pembelajaran
- Memahami tahapan kompilasi: Lexer $\rightarrow$ Parser $\rightarrow$ Type Checker $\rightarrow$ Code Generator (LLVM/C/WASM).

---

## Pertemuan 02: Inferensi Tipe Hindley-Milner & Compile-Time Safety

### 💻 Contoh Program (`p02_hindley_milner.wya`)
```widya
// Kompiler secara matematis menyimpulkan tipe data secara deterministik
misal id_siswa = 1001;              // Int
misal nama_siswa = "Kusuma Wardani"; // String
misal aktif = benar;                 // Bool

cetak("Tipe terinferensi otomatis dan terverifikasi aman.");
```

---

## Pertemuan 03: Generic Programming (`<T>`) Tanpa Overhead

### 💻 Contoh Program (`p03_generics.wya`)
```widya
fungsi tukar_posisi<T>(a: T, b: T) -> (T, T) {
    kembalikan (b, a);
}

misal (x, y) = tukar_posisi(10, 20);
misal (s1, s2) = tukar_posisi("Depan", "Belakang");

cetak("x =", x, ", y =", y);
cetak("s1 =", s1, ", s2 =", s2);
```

---

## Pertemuan 04: Pemodelan Data dengan `struktur` (Structs)

### 💻 Contoh Program (`p04_struktur.wya`)
```widya
struktur Siswa {
    nis: String,
    nama: String,
    jurusan: String,
    skor_akademik: Int
}

misal data_siswa = Siswa {
    nis: "20261101",
    nama: "Bayu Pratama",
    jurusan: "Rekayasa Perangkat Lunak",
    skor_akademik: 92
};

cetak("Siswa:", data_siswa.nama, "-", data_siswa.jurusan);
```

---

## Pertemuan 05: Abstraksi Perilaku dengan `sifat` (Traits / Interfaces)

### 💻 Contoh Program (`p05_sifat_traits.wya`)
```widya
sifat Ringkasan {
    fungsi dapatkan_ringkasan(&diri) -> String;
}

struktur Pengguna {
    nama: String,
    peran: String
}

implementasi Ringkasan untuk Pengguna {
    fungsi dapatkan_ringkasan(&diri) -> String {
        kembalikan diri.nama + " (" + diri.peran + ")";
    }
}

misal u = Pengguna { nama: "Admin Sekolah", peran: "Superuser" };
cetak(u.dapatkan_ringkasan());
```

---

## Pertemuan 06: Algebraic Data Types (ADT) dengan `pilihan` (Enums)

### 💻 Contoh Program (`p06_adt_enum.wya`)
```widya
pilihan StatusKoneksi {
    Terhubung(String),
    Terputus,
    Galat(Int, String)
}

misal status = StatusKoneksi::Terhubung("192.168.1.100");
```

---

## Pertemuan 07: Pattern Matching Komprehensif (`cocokkan`)

### 💻 Contoh Program (`p07_pattern_matching.wya`)
```widya
pilihan ResponServer {
    Sukses(String),
    Galat(Int, String)
}

fungsi evaluasi_respon(resp: ResponServer) {
    cocokkan resp {
        ResponServer::Sukses(data) => cetak("✅ Respon 200:", data),
        ResponServer::Galat(kode, err) => cetak("❌ Error [", kode, "]:", err)
    }
}

evaluasi_respon(ResponServer::Sukses("Data berhasil disinkronisasi"));
```

---

## Pertemuan 08: Ujian Tengah Semester — Library Struktur Data Generic

Siswa merancang modul *Stack* Generic (`Tumpukan<T>`) menggunakan `struktur` dan `sifat`.

---

## Pertemuan 09: Pustaka Standar C Foreign Function Interface (FFI)

Mempelajari pemanggilan fungsi biner native C dari Widya-Lang untuk performa tingkat tinggi.

---

## Pertemuan 10: Integrasi Basis Data Relasional SQLite3

### 💻 Contoh Program (`p10_sqlite.wya`)
```widya
gunakan stdlib::sqlite;

sqlite::eksekusi("sekolah.db", "CREATE TABLE IF NOT EXISTS siswa (nis TEXT PRIMARY KEY, nama TEXT);");
sqlite::eksekusi("sekolah.db", "INSERT OR REPLACE INTO siswa VALUES ('202601', 'Dewi Lestari');");
cetak("✅ Database SQLite siap digunakan!");
```

---

## Pertemuan 11: Kriptografi Terapan & Keamanan Token (SHA-256 / HMAC)

### 💻 Contoh Program (`p11_kripto.wya`)
```widya
gunakan stdlib::kripto;

misal token_asli = "SECRET_TOKEN_SEKOLAH_2026";
misal hash = kripto::sha256(token_asli);
cetak("Token Asli:", token_asli);
cetak("SHA-256   :", hash);
```

---

## Pertemuan 12: Audit Keamanan Statis & AST Linter (`widya audit`)

Mendeteksi kebocoran hardcoded secret, proteksi XSS, dan injeksi SQL menggunakan CLI `widya audit`.

---

## Pertemuan 13: Arsitektur REST API & Parsing JSON

### 💻 Contoh Program (`p13_rest_api.wya`)
```widya
struktur PermintaanPresensi {
    nis: String,
    waktu: String
}

fungsi handle_presensi(req: PermintaanPresensi) -> String {
    kembalikan "{\"status\": 200, \"msg\": \"Presensi " + req.nis + " tercatat\"}";
}

misal payload = PermintaanPresensi { nis: "2026001", waktu: "07:05:00" };
cetak("API Response:", handle_presensi(payload));
```

---

## Pertemuan 14: Pengembangan Mobile Terintegrasi (Android / iOS)

Menggunakan generator antarmuka mobile `widya mobile-init` untuk kompilasi lintas platform.

---

## Pertemuan 15: Unit Testing & Automated Test Suites (`widya test`)

Menulis skenario pengujian unit otomatis untuk memastikan fungsi bebas dari regresi logika.

---

## Pertemuan 16: Proyek Akhir — Sistem Backend API Presensi Sekolah Berbasis DB & Security Audit

### 💻 Proyek Lengkap (`p16_proyek_akhir_sma.wya`)
```widya
gunakan stdlib::sqlite;

struktur RekamanPresensi {
    nis: String,
    status: String,
    waktu: String
}

fungsi simpan_presensi_ke_db(p: RekamanPresensi) -> Bool {
    sqlite::eksekusi("presensi_sekolah.db", "CREATE TABLE IF NOT EXISTS presensi (nis TEXT, status TEXT, waktu TEXT);");
    misal sql = "INSERT INTO presensi (nis, status, waktu) VALUES ('" + p.nis + "', '" + p.status + "', '" + p.waktu + "');";
    sqlite::eksekusi("presensi_sekolah.db", sql);
    kembalikan benar;
}

cetak("============================================");
cetak("🚀 PRODUCTION SERVICE PRESENSI SEKOLAH 🚀");
cetak("============================================");

misal data = RekamanPresensi {
    nis: "2026101",
    status: "HADIR",
    waktu: "2026-09-18 07:15:00"
};

jika simpan_presensi_ke_db(data) {
    cetak("✅ Data presensi siswa", data.nis, "berhasil dicatat ke database SQLite!");
}

cetak("============================================");
cetak("Audit keamanan: Jalankan `widya audit p16_proyek_akhir_sma.wya`");
```
