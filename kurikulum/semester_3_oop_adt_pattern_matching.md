# 📘 Silabus & Modul Detail Semester 3 (Kelas 11 Ganjil)
## Tema: Pemodelan Berorientasi Objek & Aljabar Tipe Data (Pertemuan 33 - 48)

---

### 📅 Pertemuan 33: Paradigma Pemodelan Data & Konsep `struktur` (Structs)
- **Tujuan Pembelajaran:** Siswa mampu merancang tipe data komposit bentukan sendiri (*user-defined types*) untuk merepresentasikan entitas nyata.
- **Teori & Konsep:** Pengelompokan field/atribut, tipe data heterogen terstruktur, alokasi memori struct.
- **Contoh Program (`p33_struktur_dasar.wya`):**
```widya
struktur BukuPerpustakaan {
    isbn: String,
    judul: String,
    penulis: String,
    stok: Int
}

misal buku1 = BukuPerpustakaan {
    isbn: "978-602-001",
    judul: "Algoritma Pemrograman Modern",
    penulis: "Prof. Widya",
    stok: 5
};

cetak("Judul Buku :", buku1.judul);
cetak("Penulis    :", buku1.penulis);
cetak("Stok Ada   :", buku1.stok, "eksemplar");
```
- **Latihan Soal:** Buat struktur data `Siswa` dengan field `nis`, `nama`, dan `kelas`.

---

### 📅 Pertemuan 34: Enkapsulasi Data & Nilai Immutability pada Struct
- **Tujuan Pembelajaran:** Memahami keamanan manipulasi atribut struct dan teknik pembentukan salinan data terbarui (*functional update*).
- **Contoh Program (`p34_immutability.wya`):**
```widya
struktur AkunPengguna {
    username: String,
    poin: Int
}

fungsi tambah_poin(akun: AkunPengguna, bonus: Int) -> AkunPengguna {
    kembalikan AkunPengguna {
        username: akun.username,
        poin: akun.poin + bonus
    };
}
```

---

### 📅 Pertemuan 35: Metode Instance & Referensi Diri (`&diri`)
- **Tujuan Pembelajaran:** Mengaitkan fungsi/metode langsung ke dalam struct dengan blok `implementasi`.
- **Contoh Program (`p35_metode_struct.wya`):**
```widya
struktur Tabungan {
    nomor_rekening: String,
    saldo: Int
}

implementasi Tabungan {
    fungsi cek_saldo(&diri) -> Int {
        kembalikan diri.saldo;
    }
    
    fungsi setor(&diri, jumlah: Int) -> Int {
        kembalikan diri.saldo + jumlah;
    }
}

misal tabunganku = Tabungan { nomor_rekening: "001-22-333", saldo: 500000 };
cetak("Saldo Awal     : Rp", tabunganku.cek_saldo());
cetak("Setelah Setor  : Rp", tabunganku.setor(150000));
```

---

### 📅 Pertemuan 36: Abstraksi Antarmuka dengan `sifat` (Traits / Interfaces)
- **Tujuan Pembelajaran:** Mendefinisikan kontrak perilaku (*behavior contract*) yang wajib diwujudkan oleh struct.
- **Contoh Program (`p36_sifat.wya`):**
```widya
sifat DapatDihitungPajak {
    fungsi hitung_pajak(&diri) -> Int;
}

struktur Laptop {
    merk: String,
    harga: Int
}

implementasi DapatDihitungPajak untuk Laptop {
    fungsi hitung_pajak(&diri) -> Int {
        kembalikan diri.harga * 11 / 100; // PPN 11%
    }
}

misal l = Laptop { merk: "WidyaBook Pro", harga: 15000000 };
cetak("Pajak Barang:", l.hitung_pajak());
```

---

### 📅 Pertemuan 37: Polimorfisme Statis & Trait Bounds
- **Tujuan Pembelajaran:** Membuat fungsi generic yang dibatasi hanya untuk struct yang mengimplementasikan trait tertentu.

---

### 📅 Pertemuan 38: Algebraic Data Types (ADT) dengan `pilihan` (Enums)
- **Tujuan Pembelajaran:** Memodelkan data ber-varian (*Sum Types*) di mana setiap varian dapat membawa data payload sendiri.
- **Contoh Program (`p38_enum_adt.wya`):**
```widya
pilihan StatusPesanan {
    Dipesan,
    Dikirim(String), // Nomor Resi
    Selesai,
    Dibatalkan(String) // Alasan Batal
}

misal pesanan_budi = StatusPesanan::Dikirim("EXP-9928172");
```

---

### 📅 Pertemuan 39: Menghilangkan Risiko Null Pointer Crash dengan `pilihan Opsi<T>`
- **Tujuan Pembelajaran:** Memahami arsitektur bebas *NullPointerException* menggunakan enum standar `Ada(T)` dan `Kosong`.
- **Contoh Program (`p39_opsi.wya`):**
```widya
pilihan Opsi<T> {
    Ada(T),
    Kosong
}

fungsi cari_siswa_by_nis(nis: String) -> Opsi<String> {
    jika nis == "202601" {
        kembalikan Opsi::Ada("Kusuma Wardani");
    }
    kembalikan Opsi::Kosong;
}
```

---

### 📅 Pertemuan 40: Pencocokan Pola Komprehensif (`cocokkan`)
- **Tujuan Pembelajaran:** Mengevaluasi seluruh varian enum dengan verifikasi kompiler wajib lengkap (*exhaustiveness check*).
- **Contoh Program (`p40_cocokkan.wya`):**
```widya
misal status = StatusPesanan::Dikirim("EXP-9928172");

cocokkan status {
    StatusPesanan::Dipesan => cetak("Pesanan sedang disiapkan di gudang."),
    StatusPesanan::Dikirim(resi) => cetak("Pesanan sedang dalam kurir dengan Resi:", resi),
    StatusPesanan::Selesai => cetak("Pesanan telah diterima pelanggan."),
    StatusPesanan::Dibatalkan(alasan) => cetak("Pesanan batal karena:", alasan)
}
```

---

### 📅 Pertemuan 41: 🎯 Ujian Tengah Semester (UTS) — Desain State Mesin Transaksi
- **Tugas Praktik Mandiri:** Siswa merancang state machine alur pembayaran SPP sekolah (Tertunda, Terverifikasi, Lunas, Ditolak) menggunakan ADT `pilihan` dan pencocokan pola `cocokkan`.

---

### 📅 Pertemuan 42: Pattern Matching Tingkat Lanjut dengan Guard Clause (`jika`)
- **Tujuan:** Menambahkan ekspresi filter tambahan langsung di dalam cabang kecocokan.
- **Contoh Program (`p42_guard_clause.wya`):**
```widya
misal skor = 85;

cocokkan skor {
    n jika n >= 90 => cetak("Grade: Sangat Baik"),
    n jika n >= 75 => cetak("Grade: Lulus KKM"),
    _ => cetak("Grade: Belajar Lebih Giat")
}
```

---

### 📅 Pertemuan 43: Safe Error Handling dengan Tipe `pilihan Hasil<T, E>`
- **Tujuan Pembelajaran:** Menangani potensi kegagalan sistem tanpa mematikan program secara paksa (*panic*).
- **Contoh Program (`p43_hasil_error.wya`):**
```widya
pilihan Hasil<T, E> {
    Sukses(T),
    Gagal(E)
}

fungsi kalkulasi_bagi(pembilang: Int, penyebut: Int) -> Hasil<Int, String> {
    jika penyebut == 0 {
        kembalikan Hasil::Gagal("Penyebut bernilai nol tidak terdefinisi!");
    }
    kembalikan Hasil::Sukses(pembilang / penyebut);
}
```

---

### 📅 Pertemuan 44: Komposisi Objek & Relasi Entitas Majemuk
- **Tujuan:** Memodelkan hubungan antar struct (misal struct `Sekolah` menaungi daftar struct `Siswa` dan struct `Guru`).

---

### 📅 Pertemuan 45: Pembuatan Modul Parser Format Data Teks
- **Tujuan:** Mengonversi baris string berpemisah koma (CSV) menjadi koleksi struct `[Siswa]` secara otomatis.

---

### 📅 Pertemuan 46: Perancangan Arsitektur Simulator Sistem Perbankan
- **Tujuan:** Merancang modul nasabah, mutasi rekening, verifikasi PIN, dan pelaporan buku tabungan.

---

### 📅 Pertemuan 47 - 48: 🏆 Ujian Akhir Semester (UAS) — Simulator Transaksi Bank Modern
- **Tugas Proyek Akhir:** Membangun aplikasi perbankan terminal terpadu:
  1. Manajemen akun nasabah (`struktur Nasabah`).
  2. Trait `LayananTransaksi` untuk setor, tarik, dan transfer saldo.
  3. Status hasil transaksi menggunakan `Hasil<String, String>` bebas runtime exception.
