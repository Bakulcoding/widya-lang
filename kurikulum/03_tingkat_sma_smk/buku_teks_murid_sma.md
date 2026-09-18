# 📖 Buku Teks Murid: Rekayasa Perangkat Lunak & Sistem Terapan (Tingkat SMA / SMK)

Selamat datang di Buku Teks Siswa **Rekayasa Perangkat Lunak Modern** dengan **Widya-Lang**. Buku ini ditujukan untuk siswa SMA/SMK (Peminatan Informatika, RPL, SIJA) guna membekali kompetensi pemrograman tingkat industri global.

---

## 📑 Daftar Isi & Modul Keahlian

1. **Bab 1: Sistem Tipe Statis Hindley-Milner & Generic Programming**
2. **Bab 2: Pemodelan Data: Struktur (`struktur`) & Sifat/Traits (`sifat`)**
3. **Bab 3: Aljabar Tipe Data (ADT) & Pencocokan Pola Komprehensif (`cocokkan`)**
4. **Bab 4: Integrasi Basis Data Relasional Tingkat Sistem (SQLite3 C FFI)**
5. **Bab 5: Arsitektur Backend REST API, Token Security, & Audit Keamanan Kode**

---

## ⚡ Bab 1: Sistem Tipe Statis Hindley-Milner & Generics

Widya-Lang menggunakan algoritma inferensi tipe **Hindley-Milner**. Artinya, kompiler secara otomatis mengetahui dan memverifikasi keabsahan tipe data sejak tahap kompilasi (*compile-time*) tanpa memaksa programmer menulis deklarasi tipe berulang-ulang (*zero boilerplate*).

### 💡 Pemrograman Generic (`<T>`)
Generics memungkinkan kita menulis fungsi atau struktur data yang dapat bekerja secara aman pada berbagai jenis tipe data tanpa mengorbankan performa (*monomorphization*).

```widya
// Fungsi generic untuk menukar posisi tuple dari sembarang tipe T
fungsi balik_posisi<T>(a: T, b: T) -> (T, T) {
    kembalikan (b, a);
}

misal (x, y) = balik_posisi(100, 200); // T diinferensi sebagai Int
misal (s1, s2) = balik_posisi("Depan", "Belakang"); // T diinferensi sebagai String
```

---

## ⚡ Bab 2: Pemodelan Data: Struktur (`struktur`) & Sifat (`sifat`)

Dalam paradigma rekayasa modern, kita memisahkan definisi representasi data (*data shape*) dari kontrak perilaku (*behavior contracts*).

- **`struktur`**: Mendefinisikan atribut atau properti data.
- **`sifat` (Trait / Interface)**: Mendefinisikan protokol atau fungsi yang harus dipenuhi oleh suatu entitas.

```widya
sifat DapatDivalidasi {
    fungsi cek_validitas(&diri) -> Bool;
}

struktur Pengguna {
    username: String,
    umur: Int
}

implementasi DapatDivalidasi untuk Pengguna {
    fungsi cek_validitas(&diri) -> Bool {
        jika diri.umur >= 17 {
            kembalikan benar;
        } lainnya {
            kembalikan salah;
        }
    }
}
```

---

## ⚡ Bab 3: Algebraic Data Types (ADT) & Pattern Matching

Hindley-Milner memungkian representasi tipe data bervarian (*Sum Types*) melalui kata kunci `pilihan`. Hal ini mengeliminasi masalah `NullPointerException` atau *undefined behavior* yang sering terjadi di bahasa pemrograman klasik.

```widya
pilihan StatusPembayaran {
    Tertunda,
    Lunas(Int, String), // Nominal, ID Transaksi
    Dibatalkan(String)  // Alasan pembatalan
}

fungsi proses_notifikasi(status: StatusPembayaran) {
    cocokkan status {
        StatusPembayaran::Tertunda => cetak("⏳ Menunggu pembayaran dari siswa..."),
        StatusPembayaran::Lunas(nominal, id_trx) => {
            cetak("✅ Pembayaran Lunas Rp", nominal, "dengan Trx ID:", id_trx);
        },
        StatusPembayaran::Dibatalkan(alasan) => {
            cetak("❌ Pembayaran Dibatalkan karena:", alasan);
        }
    }
}
```

---

## ⚡ Bab 4: Integrasi Basis Data Sistem (SQLite3 C FFI)

Widya-Lang menyediakan antarmuka langsung ke pustaka C SQLite3 melalui `stdlib::sqlite`. Siswa diajarkan bagaimana melakukan konektivitas basis data relasional, pembuatan tabel (*DDL*), dan manipulasi data (*DML*).

```widya
gunakan stdlib::sqlite;

// Menyiapkan tabel siswa di database
sqlite::eksekusi("sekolah.db", "CREATE TABLE IF NOT EXISTS siswa (id INTEGER PRIMARY KEY, nis TEXT, nama TEXT);");

// Menyisipkan data
sqlite::eksekusi("sekolah.db", "INSERT INTO siswa (nis, nama) VALUES ('2026001', 'Ahmad Dani');");
```

---

## ⚡ Bab 5: Arsitektur Backend API & Audit Keamanan (`widya audit`)

Pada industri nyata, kode tidak hanya harus berjalan benar, tetapi juga harus **aman dari serangan siber**.

### 🔒 Praktik Audit Keamanan Bawaan Widya
Siswa dilatih untuk selalu menjalankan alat inspeksi AST:
```bash
# Menjalankan scanner audit statis untuk mendeteksi kerentanan SQLi & hardcoded secret
widya audit program.wya
```

### 📋 Tugas Proyek Capstone:
Bangun sistem backend REST API untuk presensi sekolah yang dilengkapi:
1. Skema entitas siswa & kehadiran.
2. Endpoint API berformat JSON.
3. Otentikasi token rahasia.
4. Lolos uji `widya test` dan `widya audit` tanpa temuan kerentanan.
