# 📖 Modul Pembelajaran Lengkap Widya-Lang — Tingkat SD (Kelas 4 - 6)
**Penulis / Pengembang:** Tim Kurikulum Widya-Lang  
**Sasaran:** Siswa Kelas 4, 5, dan 6 SD / MI  
**Pendekatan:** Berpikir Komputasional (*Computational Thinking*), Bahasa Ibu, & *Learning by Doing*

---

## 📑 Daftar Isi Modul (16 Pertemuan)

- [Pertemuan 01: Halo Dunia & Komputer Sahabat Kita](#pertemuan-01-halo-dunia--komputer-sahabat-kita)
- [Pertemuan 02: Menghias Cerita dengan Banyak Baris Teks](#pertemuan-02-menghias-cerita-dengan-banyak-baris-teks)
- [Pertemuan 03: Kotak Rahasia Bernama Variabel (`misal`)](#pertemuan-03-kotak-rahasia-bernama-variabel-misal)
- [Pertemuan 04: Membedakan Angka dan Tulisan (Tipe Data Dasar)](#pertemuan-04-membedakan-angka-dan-tulisan-tipe-data-dasar)
- [Pertemuan 05: Berhitung Matematika (+ dan -)](#pertemuan-05-berhitung-matematika--dan--)
- [Pertemuan 06: Perkalian dan Pembagian (* dan /)](#pertemuan-06-perkalian-dan-pembagian--dan-)
- [Pertemuan 07: Simulasi Kasir Warung Sekolah](#pertemuan-07-simulasi-kasir-warung-sekolah)
- [Pertemuan 08: Ujian Tengah Semester — Kuis Interaktif Biodata Diri](#pertemuan-08-ujian-tengah-semester--kuis-interaktif-biodata-diri)
- [Pertemuan 09: Pengambilan Keputusan Sederhana (`jika` dan `lainnya`)](#pertemuan-09-pengambilan-keputusan-sederhana-jika-dan-lainnya)
- [Pertemuan 10: Lampu Lalu Lintas & Keputusan Bertingkat](#pertemuan-10-lampu-lalu-lintas--keputusan-bertingkat)
- [Pertemuan 11: Memeriksa Kelulusan Nilai Siswa](#pertemuan-11-memeriksa-kelulusan-nilai-siswa)
- [Pertemuan 12: Robot yang Mengulang Tugas (`untuk`)](#pertemuan-12-robot-yang-mengulang-tugas-untuk)
- [Pertemuan 13: Tabel Perkalian Ajaib](#pertemuan-13-tabel-perkalian-ajaib)
- [Pertemuan 14: Hitung Mundur Roket Menuju Bintang](#pertemuan-14-hitung-mundur-roket-menuju-bintang)
- [Pertemuan 15: Merancang Skenario Game Cerita Pilihan](#pertemuan-15-merancang-skenario-game-cerita-pilihan)
- [Pertemuan 16: Proyek Akhir — Game Petualangan Mencari Harta Karun](#pertemuan-16-proyek-akhir--game-petualangan-mencari-harta-karun)

---

## Pertemuan 01: Halo Dunia & Komputer Sahabat Kita

### 🎯 Tujuan Pembelajaran
- Siswa memahami bahwa komputer bekerja atas dasar instruksi (perintah tertulis).
- Siswa dapat menulis program pertama untuk menampilkan pesan teks ke layar.

### 💡 Konsep Materi
Di Widya-Lang, kata ajaib untuk memerintahkan komputer berbicara di layar adalah **`cetak`**.
Setiap kalimat harus dibungkus oleh tanda petik ganda (`"..."`) dan diakhiri dengan titik koma (`;`).

### 💻 Contoh Program (`p01_halo.wya`)
```widya
cetak("Halo Dunia!");
cetak("Saya siap menjadi pembuat kode masa depan!");
```

### ✍️ Latihan Soal & Tugas Praktik
1. **Latihan 1:** Buatlah program yang menampilkan:
   - Baris 1: Nama lengkapmu
   - Baris 2: Makanan kesukaanmu
   - Baris 3: Warna favoritmu
2. **Kuis Pilihan:**
   Manakah penulisan perintah `cetak` yang benar?
   - A. `cetak Halo Dunia;`
   - B. `cetak("Halo Dunia!");`
   - C. `cetak[Halo Dunia]`
   *(Kunci: B)*

---

## Pertemuan 02: Menghias Cerita dengan Banyak Baris Teks

### 🎯 Tujuan Pembelajaran
- Siswa dapat menyusun beberapa instruksi berurutan untuk membentuk puisi atau cerita pendek.

### 💻 Contoh Program (`p02_puisi.wya`)
```widya
cetak("=================================");
cetak("       BINTANG KECIL DI LANGIT   ");
cetak("=================================");
cetak("Bintang kecil di langit yang biru");
cetak("Amat banyak menghias angkasa");
cetak("Aku ingin terbang dan menari");
cetak("Jauh tinggi ke tempat kau berada");
```

### ✍️ Latihan Soal & Tugas Praktik
- Buatlah sebuah kartu ucapan ulang tahun untuk teman sebangkumu menggunakan karakter pembatas seperti `*` atau `=` agar terlihat indah!

---

## Pertemuan 03: Kotak Rahasia Bernama Variabel (`misal`)

### 🎯 Tujuan Pembelajaran
- Memahami konsep variabel sebagai wadah berlabel untuk menyimpan informasi.
- Menggunakan kata kunci **`misal`**.

### 💡 Konsep Materi
Bayangkan kotak berlabel:
- Label `nama_kucing` berisi `"Meong"`.
- Label `umur_kucing` berisi `2`.

### 💻 Contoh Program (`p03_variabel.wya`)
```widya
misal nama_hewan = "Kucing Oren";
misal warna_bulu = "Kuning Belang";

cetak("Hewan Peliharaan Saya:");
cetak("Nama :", nama_hewan);
cetak("Warna:", warna_bulu);
```

### ✍️ Latihan Soal
1. Buat variabel `nama_sekolah` dan isi dengan nama SD tempat kamu belajar.
2. Buat variabel `nama_guru` dan cetak salam untuk gurumu!

---

## Pertemuan 04: Membedakan Angka dan Tulisan (Tipe Data Dasar)

### 🎯 Tujuan Pembelajaran
- Membedakan data teks (*String* / diapit tanda petik) dan angka (*Integer* / tanpa tanda petik).

### 💡 Konsep Materi
- `"10"` (dengan petik) adalah **tulisan/teks** (tidak bisa dihitung matematika).
- `10` (tanpa petik) adalah **angka nyata** (bisa ditambah atau dikurang).

### 💻 Contoh Program (`p04_tipe_data.wya`)
```widya
misal nama_buku = "Petualangan Si Kancil";
misal jumlah_halaman = 48;
misal harga = 25000;

cetak("Judul Buku     :", nama_buku);
cetak("Jumlah Halaman :", jumlah_halaman);
cetak("Harga          : Rp", harga);
```

---

## Pertemuan 05: Berhitung Matematika (+ dan -)

### 🎯 Tujuan Pembelajaran
- Menggunakan operator penjumlahan (`+`) dan pengurangan (`-`) dalam program komputer.

### 💻 Contoh Program (`p05_tambah_kurang.wya`)
```widya
misal kelereng_budi = 20;
misal kelereng_diberi_ayah = 15;
misal kelereng_hilang = 5;

misal total_kelereng = kelereng_budi + kelereng_diberi_ayah - kelereng_hilang;

cetak("Kelereng Awal    :", kelereng_budi);
cetak("Diberi Ayah      :", kelereng_diberi_ayah);
cetak("Hilang saat main :", kelereng_hilang);
cetak("-----------------------------");
cetak("Sisa Kelereng Sekarang:", total_kelereng);
```

---

## Pertemuan 06: Perkalian dan Pembagian (* dan /)

### 🎯 Tujuan Pembelajaran
- Menggunakan operator bintang (`*`) untuk perkalian dan garis miring (`/`) untuk pembagian.

### 💻 Contoh Program (`p06_kali_bagi.wya`)
```widya
// Menghitung luas persegi panjang
misal panjang = 10;
misal lebar = 6;
misal luas = panjang * lebar;

cetak("Panjang Persegi :", panjang, "cm");
cetak("Lebar Persegi   :", lebar, "cm");
cetak("Luas Persegi    :", luas, "cm persegi");

// Membagi donat ke teman
misal total_donat = 24;
misal jumlah_anak = 4;
misal donat_per_anak = total_donat / jumlah_anak;

cetak("Setiap anak mendapat:", donat_per_anak, "donat");
```

---

## Pertemuan 07: Simulasi Kasir Warung Sekolah

### 🎯 Tujuan Pembelajaran
- Menggabungkan variabel, teks, perkalian, dan pengurangan dalam studi kasus belanja.

### 💻 Contoh Program (`p07_kasir_warung.wya`)
```widya
misal uang_bawaan = 20000;
misal harga_pensil = 3000;
misal beli_pensil = 2;
misal harga_buku = 5000;
misal beli_buku = 1;

misal total_pensil = harga_pensil * beli_pensil;
misal total_buku = harga_buku * beli_buku;
misal total_bayar = total_pensil + total_buku;
misal uang_kembalian = uang_bawaan - total_bayar;

cetak("=== NOTA PEMBELIAN KOPERASI SEKOLAH ===");
cetak("2 Pensil : Rp", total_pensil);
cetak("1 Buku   : Rp", total_buku);
cetak("Total    : Rp", total_bayar);
cetak("Bayar    : Rp", uang_bawaan);
cetak("Kembali  : Rp", uang_kembalian);
```

---

## Pertemuan 08: Ujian Tengah Semester — Kuis Interaktif Biodata Diri

### 🎯 Penilaian Praktik
Siswa membuat proyek mandiri `uts_biodata.wya` dengan ketentuan:
1. Menyimpan data diri dalam 5 variabel berbeda.
2. Menghitung tahun lahir berdasarkan umur saat ini.
3. Mencetak tabel profil yang rapi di layar konsol.

---

## Pertemuan 09: Pengambilan Keputusan Sederhana (`jika` dan `lainnya`)

### 🎯 Tujuan Pembelajaran
- Memahami konsep logika kondisional di mana komputer dapat memilih jalur perintah.

### 💻 Contoh Program (`p09_keputusan.wya`)
```widya
misal cuaca = "hujan";

cetak("Melihat ke luar jendela...");

jika cuaca == "hujan" {
    cetak("🌧️ Hari ini hujan! Bawa payung ya!");
} lainnya {
    cetak("☀️ Hari cerah! Selamat bermain di taman!");
}
```

---

## Pertemuan 10: Lampu Lalu Lintas & Keputusan Bertingkat

### 🎯 Tujuan Pembelajaran
- Menggunakan `jika`, `lainnya jika`, dan `lainnya` untuk 3 kondisi atau lebih.

### 💻 Contoh Program (`p10_lampu_lalin.wya`)
```widya
misal warna = "kuning";

jika warna == "merah" {
    cetak("🛑 Berhenti!");
} lainnya jika warna == "kuning" {
    cetak("⚠️ Hati-hati, bersiap pelan!");
} lainnya jika warna == "hijau" {
    cetak("🟢 Silakan jalan!");
} lainnya {
    cetak("❓ Lampu mati atau rusak.");
}
```

---

## Pertemuan 11: Memeriksa Kelulusan Nilai Siswa

### 🎯 Tujuan Pembelajaran
- Membandingkan angka menggunakan operator relasional (`>=`, `<=`, `>`).

### 💻 Contoh Program (`p11_nilai_ujian.wya`)
```widya
misal kkm = 75;
misal nilai_dani = 82;

cetak("Nilai Siswa :", nilai_dani);
cetak("Batas KKM   :", kkm);

jika nilai_dani >= kkm {
    cetak("🎉 Selamat! Kamu Lulus KKM dengan Hebat!");
} lainnya {
    cetak("📚 Tetap Semangat! Belajar lagi untuk remedial ya.");
}
```

---

## Pertemuan 12: Robot yang Mengulang Tugas (`untuk`)

### 🎯 Tujuan Pembelajaran
- Memahami konsep perulangan (*looping*) menggunakan rentang bilangan `1..n`.

### 💻 Contoh Program (`p12_perulangan.wya`)
```widya
cetak("Lari keliling lapangan:");

untuk putaran dalam 1..6 {
    cetak("Pelari melewati putaran ke-", putaran);
}

cetak("🏁 Selesai! Kamu telah menyelesaikan 5 putaran!");
```

---

## Pertemuan 13: Tabel Perkalian Ajaib

### 🎯 Tujuan Pembelajaran
- Menggabungkan perulangan dan perkalian untuk mencetak tabel matematika otomatis.

### 💻 Contoh Program (`p13_tabel_perkalian.wya`)
```widya
misal angka_dasar = 7;
cetak("=== TABEL PERKALIAN", angka_dasar, "===");

untuk i dalam 1..11 {
    misal hasil = angka_dasar * i;
    cetak(angka_dasar, "x", i, "=", hasil);
}
```

---

## Pertemuan 14: Hitung Mundur Roket Menuju Bintang

### 🎯 Tujuan Pembelajaran
- Membuat efek hitungan maju dan mundur dengan struktur perulangan teratur.

### 💻 Contoh Program (`p14_roket.wya`)
```widya
cetak("🚀 PERSIAPAN PELUNCURAN ROKET WIDYA-1 🚀");

untuk detik dalam 1..6 {
    cetak("T-minus detik ke-", detik);
}

cetak("💥 IGNITION!");
cetak("🚀🚀 ROKET MELESAT KE LUAR ANGKASA! 🌌");
```

---

## Pertemuan 15: Merancang Skenario Game Cerita Pilihan

### 🎯 Tujuan Pembelajaran
- Menyusun alur pohon keputusan (*decision tree*) di atas kertas sebelum menulis kode game cerita interaktif.

---

## Pertemuan 16: Proyek Akhir — Game Petualangan Mencari Harta Karun

### 🎯 Tujuan Pembelajaran
- Membangun aplikasi lengkap berformat mini-game berbasis percabangan keputusan.

### 💻 Contoh Program Lengkap (`p16_game_harta_karun.wya`)
```widya
cetak("============================================");
cetak("🏰 PETUALANGAN MENCARI HARTA KARUN WIDYA 🏰");
cetak("============================================");

misal nama_pemain = "Ksatria Danu";
misal jalur_pilihan = "kanan"; // Pilihan: "kiri", "kanan", "lurus"
misal tebakan_kunci = 7;

cetak("Pahlawan", nama_pemain, "tiba di gerbang gua ajaib.");
cetak("Kamu melangkah ke arah:", jalur_pilihan);

jika jalur_pilihan == "kiri" {
    cetak("🦇 Kamu masuk ke gua kelelawar, namun berhasil menemukan 50 koin perak!");
} lainnya jika jalur_pilihan == "kanan" {
    cetak("🚪 Kamu menemukan Pintu Emas Kerajaan!");
    jika tebakan_kunci == 7 {
        cetak("✨ Kunci Ajaib Cocok! Kamu menemukan Mahkota Raja!");
    } lainnya {
        cetak("❌ Kunci salah, pintu tetap tertutup rapat.");
    }
} lainnya {
    cetak("🌲 Kamu berjalan memutar dan kembali ke pintu masuk hutan.");
}

cetak("============================================");
cetak("🎉 Permainan Selesai! Terima kasih telah bermain!");
```
