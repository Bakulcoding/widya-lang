# 📘 Silabus & Modul Detail Semester 3 (Kelas 11 Ganjil)
## Tema: Pemodelan Berorientasi Objek & Aljabar Tipe Data (16 Pertemuan)

---

### 📅 Pertemuan 33: Paradigma Pemodelan Data & Konsep `struktur` (Structs)
- **Tujuan:** Merepresentasikan entitas dunia nyata ke dalam tipe data bentukan.
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

cetak("Buku:", buku1.judul, "oleh", buku1.penulis);
```

---

### 📅 Pertemuan 34: Enkapsulasi & Immutability pada Atribut Struct
- **Tujuan:** Memahami proteksi nilai atribut dan pembuatan struct baru saat mutasi data.

---

### 📅 Pertemuan 35: Metode Instance & Kata Kunci Referensi Diri (`&diri`)
- **Tujuan:** Menambahkan fungsionalitas perilaku (*behavior*) langsung ke dalam struct.
- **Contoh Program (`p35_metode_struct.wya`):**
```widya
struktur Tabungan {
    nomor_rekening: String,
    saldo: Int
}

implementasi Tabungan {
    fungsi setor(&diri, jumlah: Int) -> Int {
        kembalikan diri.saldo + jumlah;
    }
}
```

---

### 📅 Pertemuan 36: Abstraksi Antarmuka dengan `sifat` (Traits / Interfaces)
- **Tujuan:** Mendefinisikan kontrak fungsi yang dapat diimplementasikan oleh banyak struct.
- **Contoh Program (`p36_sifat.wya`):**
```widya
sifat DapatDihitungPajak {
    fungsi hitung_pajak(&diri) -> Int;
}

struktur ProdukElektronik {
    nama: String,
    harga: Int
}

implementasi DapatDihitungPajak untuk ProdukElektronik {
    fungsi hitung_pajak(&diri) -> Int {
        kembalikan diri.harga * 11 / 100; // PPN 11%
    }
}
```

---

### 📅 Pertemuan 37: Polimorfisme Statis & Dynamic Dispatch
- **Tujuan:** Memahami bagaimana satu method antarmuka dapat merespons objek yang berbeda.

---

### 📅 Pertemuan 38: Algebraic Data Types (ADT) dengan `pilihan` (Enums)
- **Tujuan:** Memodelkan data ber-varian (*Sum Types*) untuk mengatasi keterbatasan tipe primitif.
- **Contoh Program (`p38_enum_adt.wya`):**
```widya
pilihan StatusPembayaran {
    BelumBayar,
    MenungguVerifikasi(String), // Bank pengirim
    Lunas(Int)                  // Nominal lunas
}
```

---

### 📅 Pertemuan 39: Menghilangkan Risiko Null Pointer dengan `pilihan Opsi<T>`
- **Tujuan:** Memahami desain bebas crash: `Ada(T)` atau `Kosong`.
- **Contoh Program (`p39_opsi.wya`):**
```widya
pilihan Opsi<T> {
    Ada(T),
    Kosong
}

fungsi cari_user_by_id(id: Int) -> Opsi<String> {
    jika id == 1 {
        kembalikan Opsi::Ada("Admin Sekolah");
    }
    kembalikan Opsi::Kosong;
}
```

---

### 📅 Pertemuan 40: Pencocokan Pola Komprehensif (`cocokkan`)
- **Tujuan:** Menggunakan pattern matching terstruktur dengan validasi kompiler wajib lengkap (*exhaustiveness check*).
- **Contoh Program (`p40_cocokkan.wya`):**
```widya
misal status = StatusPembayaran::Lunas(150000);

cocokkan status {
    StatusPembayaran::BelumBayar => cetak("Tagihan belum dibayar!"),
    StatusPembayaran::MenungguVerifikasi(bank) => cetak("Sedang diverifikasi di bank:", bank),
    StatusPembayaran::Lunas(nominal) => cetak("Pembayaran LUNAS sebesar Rp", nominal)
}
```

---

### 📅 Pertemuan 41: 🎯 Ujian Tengah Semester (UTS) — Desain State Mesin Transaksi
- **Tugas Praktik:** Memodelkan siklus hidup transaksi tiket online (Dipesan, Dibayar, Dibatalkan, Digunakan) dengan ADT Enum & Pencocokan Pola.

---

### 📅 Pertemuan 42: Pattern Matching dengan Pola Nilai & Guard Clause
- **Tujuan:** Menambahkan kondisi ekspresi kondisional `jika` di dalam cabang `cocokkan`.

---

### 📅 Pertemuan 43: Safe Error Handling dengan Tipe `pilihan Hasil<T, E>`
- **Tujuan:** Penanganan galat tanpa panic runtime: `Sukses(T)` dan `Gagal(E)`.
- **Contoh Program (`p43_hasil_error.wya`):**
```widya
pilihan Hasil<T, E> {
    Sukses(T),
    Gagal(E)
}

fungsi bagi_angka(a: Int, b: Int) -> Hasil<Int, String> {
    jika b == 0 {
        kembalikan Hasil::Gagal("Pembagian dengan angka nol tidak diizinkan!");
    }
    kembalikan Hasil::Sukses(a / b);
}
```

---

### 📅 Pertemuan 44: Pengomposisian Struct & Hubungan Agregasi Objek
- **Tujuan:** Menggabungkan struct di dalam struct (misal: `Siswa` memiliki struct `Alamat` dan struct `Wali`).

---

### 📅 Pertemuan 45: Pembuatan Library Parser File Teks Sederhana
- **Tujuan:** Membaca dan mem-parsing format CSV ke dalam struct Widya.

---

### 📅 Pertemuan 46: Arsitektur Simulator Sistem Perbankan
- **Tujuan:** Merancang modul nasabah, mutasi rekening, dan validasi transaksi transfer antar akun.

---

### 📅 Pertemuan 47 - 48: 🏆 Ujian Akhir Semester (UAS) — Simulator Transaksi Bank Modern
- **Tugas Akhir:** Aplikasi perbankan terminal dengan autentikasi PIN, mutasi saldo, pencatatan histori transaksi menggunakan Structs, Traits, dan penanganan galat Result Type.
