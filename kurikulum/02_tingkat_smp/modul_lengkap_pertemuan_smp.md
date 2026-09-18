# 📖 Modul Pembelajaran Lengkap Widya-Lang — Tingkat SMP (Kelas 7 - 9)
**Penulis / Pengembang:** Tim Kurikulum Widya-Lang  
**Sasaran:** Siswa Kelas 7, 8, dan 9 SMP / MTs  
**Pendekatan:** Algoritma Terstruktur, Modularitas, Struktur Data Koleksi, & Logika Terapan

---

## 📑 Daftar Isi Modul (16 Pertemuan)

- [Pertemuan 01: Review Dasar & Filosofi Modularitas](#pertemuan-01-review-dasar--filosofi-modularitas)
- [Pertemuan 02: Membuat Fungsi Pertama (`fungsi` dan `kembalikan`)](#pertemuan-02-membuat-fungsi-pertama-fungsi-dan-kembalikan)
- [Pertemuan 03: Parameter & Nilai Kembalian Jamak](#pertemuan-03-parameter--nilai-kembalian-jamak)
- [Pertemuan 04: Ruang Lingkup Variabel (Local vs Global Scope)](#pertemuan-04-ruang-lingkup-variabel-local-vs-global-scope)
- [Pertemuan 05: Modul Matematika: Konversi Suhu & Luas Bidang](#pertemuan-05-modul-matematika-konversi-suhu--luas-bidang)
- [Pertemuan 06: Pengenalan Struktur Data Larik / Array (`[T]`)](#pertemuan-06-pengenalan-struktur-data-larik--array-t)
- [Pertemuan 07: Mengakses dan Memodifikasi Elemen Larik](#pertemuan-07-mengakses-dan-memodifikasi-elemen-larik)
- [Pertemuan 08: Ujian Tengah Semester — Analisis Data Statistik Sederhana](#pertemuan-08-ujian-tengah-semester--analisis-data-statistik-sederhana)
- [Pertemuan 09: Iterasi Larik Menggunakan `untuk ... dalam ...`](#pertemuan-09-iterasi-larik-menggunakan-untuk--dalam-)
- [Pertemuan 10: Algoritma Pencarian Linier (*Linear Search*)](#pertemuan-10-algoritma-pencarian-linier-linear-search)
- [Pertemuan 11: Algoritma Pengurutan Nilai (*Bubble Sort*)](#pertemuan-11-algoritma-pengurutan-nilai-bubble-sort)
- [Pertemuan 12: Menemukan Nilai Ekstrem (Minimum & Maksimum)](#pertemuan-12-menemukan-nilai-ekstrem-minimum--maksimum)
- [Pertemuan 13: Logika Game Interaktif: Tebak Angka Komputer](#pertemuan-13-logika-game-interaktif-tebak-angka-komputer)
- [Pertemuan 14: Pola Pencocokan Dasar (`cocokkan`)](#pertemuan-14-pola-pencocokan-dasar-cocokkan)
- [Pertemuan 15: Perancangan Sistem Mini Kasir & Diskon Member](#pertemuan-15-perancangan-sistem-mini-kasir--diskon-member)
- [Pertemuan 16: Proyek Akhir — Sistem Aplikasi Kasir Kantin Sekolah](#pertemuan-16-proyek-akhir--sistem-aplikasi-kasir-kantin-sekolah)

---

## Pertemuan 01: Review Dasar & Filosofi Modularitas

### 🎯 Tujuan Pembelajaran
- Memahami mengapa kode program skala menengah membutuhkan pemecahan blok (modularitas).

---

## Pertemuan 02: Membuat Fungsi Pertama (`fungsi` dan `kembalikan`)

### 🎯 Tujuan Pembelajaran
- Membuat dan memanggil fungsi sederhana dengan nilai kembalian.

### 💻 Contoh Program (`p02_fungsi_dasar.wya`)
```widya
fungsi kuadrat(x: Int) -> Int {
    kembalikan x * x;
}

misal angka = 9;
misal hasil = kuadrat(angka);
cetak("Kuadrat dari", angka, "adalah:", hasil);
```

### ✍️ Latihan Soal
- Buat fungsi `hitung_keliling_persegi(sisi: Int) -> Int` yang mengembalikan `4 * sisi`.

---

## Pertemuan 03: Parameter & Nilai Kembalian Jamak

### 🎯 Tujuan Pembelajaran
- Mengirim lebih dari satu parameter dan mengembalikan nilai komputasi.

### 💻 Contoh Program (`p03_param_jamak.wya`)
```widya
fungsi hitung_luas_balok(panjang: Int, lebar: Int, tinggi: Int) -> Int {
    misal pl = panjang * lebar;
    misal pt = panjang * tinggi;
    misal lt = lebar * tinggi;
    kembalikan 2 * (pl + pt + lt);
}

misal l_balok = hitung_luas_balok(5, 3, 2);
cetak("Luas Permukaan Balok:", l_balok, "cm²");
```

---

## Pertemuan 04: Ruang Lingkup Variabel (Local vs Global Scope)

### 🎯 Tujuan Pembelajaran
- Membedakan variabel yang dideklarasikan di dalam blok fungsi dengan variabel di luar fungsi.

---

## Pertemuan 05: Modul Matematika: Konversi Suhu & Luas Bidang

### 💻 Contoh Program (`p05_konversi_suhu.wya`)
```widya
fungsi celcius_ke_fahrenheit(c: Int) -> Int {
    kembalikan (c * 9 / 5) + 32;
}

fungsi celcius_ke_kelvin(c: Int) -> Int {
    kembalikan c + 273;
}

misal suhu_ruang = 25;
cetak("Suhu Celcius    :", suhu_ruang, "C");
cetak("Suhu Fahrenheit :", celcius_ke_fahrenheit(suhu_ruang), "F");
cetak("Suhu Kelvin     :", celcius_ke_kelvin(suhu_ruang), "K");
```

---

## Pertemuan 06: Pengenalan Struktur Data Larik / Array (`[T]`)

### 🎯 Tujuan Pembelajaran
- Menyimpan sekumpulan data sejenis dalam satu wadah berindeks.

### 💻 Contoh Program (`p06_larik_dasar.wya`)
```widya
misal nilai_tugas = [75, 80, 95, 88, 92];
cetak("Daftar Nilai Tugas:", nilai_tugas);
cetak("Nilai Pertama (indeks 0):", nilai_tugas[0]);
cetak("Nilai Ketiga  (indeks 2):", nilai_tugas[2]);
```

---

## Pertemuan 07: Mengakses dan Memodifikasi Elemen Larik

### 💻 Contoh Program (`p07_modifikasi_larik.wya`)
```widya
misal keranjang = [10000, 20000, 15000];
cetak("Sebelum diubah:", keranjang);

// Mengupdate nilai barang ke-2
keranjang[1] = 25000;
cetak("Setelah diubah :", keranjang);
```

---

## Pertemuan 08: Ujian Tengah Semester — Analisis Data Statistik Sederhana

Siswa mengimplementasikan program statistik yang menghitung jumlah total nilai siswa dalam array dan mencari rata-rata nilai kelas secara mandiri.

---

## Pertemuan 09: Iterasi Larik Menggunakan `untuk ... dalam ...`

### 💻 Contoh Program (`p09_iterasi_larik.wya`)
```widya
misal data_absen = [80, 90, 85, 70, 95];
misal total_nilai = 0;

untuk nilai dalam data_absen {
    total_nilai = total_nilai + nilai;
}

misal rata_rata = total_nilai / 5;
cetak("Total Nilai Kumulatif:", total_nilai);
cetak("Rata-rata Kelas      :", rata_rata);
```

---

## Pertemuan 10: Algoritma Pencarian Linier (*Linear Search*)

### 💻 Contoh Program (`p10_linear_search.wya`)
```widya
fungsi cari_angka(koleksi: [Int], target: Int) -> Bool {
    untuk item dalam koleksi {
        jika item == target {
            kembalikan benar;
        }
    }
    kembalikan salah;
}

misal data_nis = [101, 105, 110, 125, 130];
misal dicari = 110;

jika cari_angka(data_nis, dicari) {
    cetak("✅ Data NIS", dicari, "ditemukan dalam sistem!");
} lainnya {
    cetak("❌ Data NIS", dicari, "tidak ditemukan.");
}
```

---

## Pertemuan 11: Algoritma Pengurutan Nilai (*Bubble Sort*)

### 💻 Contoh Program (`p11_bubble_sort.wya`)
```widya
fungsi urutkan(arr: [Int]) -> [Int] {
    misal data = arr;
    misal n = 5;
    untuk i dalam 0..n {
        untuk j dalam 0..(n - 1) {
            jika data[j] > data[j + 1] {
                misal temp = data[j];
                data[j] = data[j + 1];
                data[j + 1] = temp;
            }
        }
    }
    kembalikan data;
}

misal acak = [64, 34, 25, 12, 22];
cetak("Sebelum diurutkan:", acak);
misal rapi = urutkan(acak);
cetak("Setelah diurutkan :", rapi);
```

---

## Pertemuan 12: Menemukan Nilai Ekstrem (Minimum & Maksimum)

### 💻 Contoh Program (`p12_min_max.wya`)
```widya
misal skor_ujian = [45, 98, 77, 62, 89];
misal skor_tertinggi = skor_ujian[0];
misal skor_terendah = skor_ujian[0];

untuk skor dalam skor_ujian {
    jika skor > skor_tertinggi {
        skor_tertinggi = skor;
    }
    jika skor < skor_terendah {
        skor_terendah = skor;
    }
}

cetak("Skor Tertinggi:", skor_tertinggi);
cetak("Skor Terendah :", skor_terendah);
```

---

## Pertemuan 13: Logika Game Interaktif: Tebak Angka Komputer

### 💻 Contoh Program (`p13_tebak_angka.wya`)
```widya
misal angka_rahasia = 42;
misal riwayat_tebakan = [20, 50, 40, 42];

untuk tebakan dalam riwayat_tebakan {
    cetak("Mencoba tebakan:", tebakan);
    jika tebakan == angka_rahasia {
        cetak("🎯 Tepat Sasaran! Angka rahasia adalah", angka_rahasia);
    } lainnya jika tebakan < angka_rahasia {
        cetak("🔼 Terlalu kecil!");
    } lainnya {
        cetak("🔽 Terlalu besar!");
    }
}
```

---

## Pertemuan 14: Pola Pencocokan Dasar (`cocokkan`)

### 💻 Contoh Program (`p14_cocokkan.wya`)
```widya
misal kode_hari = 3;

cocokkan kode_hari {
    1 => cetak("Hari Senin: Upacara Bendera"),
    2 => cetak("Hari Selasa: Praktikum Lab"),
    3 => cetak("Hari Rabu: Olahraga & Seni"),
    _ => cetak("Hari Lainnya: Belajar Mandiri")
}
```

---

## Pertemuan 15: Perancangan Sistem Mini Kasir & Diskon Member

Merancang alur kalkulasi kasir:
1. Menghitung total belanjaan dari list harga.
2. Menghitung potongan diskon (Siswa = 10%, Guru = 15%).
3. Mengembalikan kembalian uang.

---

## Pertemuan 16: Proyek Akhir — Sistem Aplikasi Kasir Kantin Sekolah

### 💻 Kode Program Lengkap (`p16_kasir_final.wya`)
```widya
fungsi hitung_diskon_member(total: Int, role: String) -> Int {
    jika role == "guru" {
        kembalikan total * 15 / 100;
    } lainnya jika role == "siswa" {
        kembalikan total * 10 / 100;
    } lainnya {
        kembalikan 0;
    }
}

cetak("============================================");
cetak("🏪 SISTEM KASIR KANTIN WIDYA JUARA 🏪");
cetak("============================================");

misal harga_makanan = [15000, 12000, 5000]; // Nasi Rames, Soto, Es Jeruk
misal total_tagihan = 0;

untuk item dalam harga_makanan {
    total_tagihan = total_tagihan + item;
}

misal status_pembeli = "siswa";
misal potongan = hitung_diskon_member(total_tagihan, status_pembeli);
misal total_bayar = total_tagihan - potongan;
misal uang_tunai = 50000;
misal sisa_kembalian = uang_tunai - total_bayar;

cetak("Subtotal Belanja : Rp", total_tagihan);
cetak("Diskon Member    : Rp", potongan);
cetak("Total Bayar      : Rp", total_bayar);
cetak("Tunai Diterima   : Rp", uang_tunai);
cetak("Kembalian        : Rp", sisa_kembalian);
cetak("============================================");
cetak("Terima kasih telah berbelanja di Kantin Widya!");
```
