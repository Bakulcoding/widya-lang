# 📘 Silabus & Modul Detail Semester 2 (Kelas 10 Genap)
## Tema: Sistem Tipe Hindley-Milner, Generics, & Struktur Data Koleksi (16 Pertemuan)

---

### 📅 Pertemuan 17: Teori Sistem Tipe Statis & Type Safety
- **Tujuan:** Memahami perbedaan tipe dinamis vs statis, serta jaminan keamanan kompilasi (*compile-time safety*).

---

### 📅 Pertemuan 18: Algoritma Inferensi Tipe Hindley-Milner (HM)
- **Tujuan:** Memahami bagaimana Widya-Lang merekonstruksi tipe data secara deterministik tanpa kewajiban deklarasi manual.
- **Contoh Program (`p18_hindley_milner.wya`):**
```widya
misal bilangan = 42;             // Diinferensi sebagai Int
misal nama_modul = "HM Type Checker"; // Diinferensi sebagai String

fungsi kuadrat_hm(x) {
    kembalikan x * x; // Compiler otomatis mengikat tipe x: Int -> Int
}

cetak("Hasil:", kuadrat_hm(bilangan));
```

---

### 📅 Pertemuan 19: Pemrograman Generic Sederhana (`<T>`)
- **Tujuan:** Membuat fungsi generic yang menerima tipe data fleksibel.
- **Contoh Program (`p19_generics_dasar.wya`):**
```widya
fungsi identitas<T>(nilai: T) -> T {
    kembalikan nilai;
}

misal angka = identitas(100);
misal teks = identitas("Belajar Widya");
cetak("Angka:", angka, ", Teks:", teks);
```

---

### 📅 Pertemuan 20: Generic Multi-Tipe & Pasangan Tuple `(T1, T2)`
- **Tujuan:** Menggunakan tipe gabungan heterogen tuple dan generic multi-parameter.
- **Contoh Program (`p20_tuple.wya`):**
```widya
fungsi buat_pasangan<A, B>(pertama: A, kedua: B) -> (A, B) {
    kembalikan (pertama, kedua);
}

misal posisi_gps = buat_pasangan(-6.2088, 106.8456);
cetak("Koordinat Latitude & Longitude:", posisi_gps);
```

---

### 📅 Pertemuan 21: Struktur Data Larik / Array Dinamis (`[T]`)
- **Tujuan:** Memahami alokasi larik di memori, indeks berbasis nol, dan operasi penambahan elemen.
- **Contoh Program (`p21_array.wya`):**
```widya
misal daftar_skor = [85, 90, 78, 92, 88];
cetak("Banyak data skor:", 5);
cetak("Skor urutan pertama:", daftar_skor[0]);
```

---

### 📅 Pertemuan 22: Iterasi Koleksi Data Menggunakan `untuk ... dalam ...`
- **Tujuan:** Mengolah seluruh data elemen array untuk agregasi statistik (Jumlah, Rata-rata).
- **Contoh Program (`p22_agregasi.wya`):**
```widya
misal data_berat = [45, 52, 60, 48, 55];
misal total_berat = 0;

untuk berat dalam data_berat {
    total_berat = total_berat + berat;
}

misal rata_rata = total_berat / 5;
cetak("Rata-rata berat badan:", rata_rata, "kg");
```

---

### 📅 Pertemuan 23: Algoritma Pencarian Linier (*Linear Search*)
- **Tujuan:** Mencari keberadaan elemen dalam array dengan kompleksitas $O(N)$.
- **Contoh Program (`p23_linear_search.wya`):**
```widya
fungsi cari_data(larik: [Int], kunci: Int) -> Bool {
    untuk item dalam larik {
        jika item == kunci {
            kembalikan benar;
        }
    }
    kembalikan salah;
}

misal nim_terdaftar = [202601, 202602, 202605, 202610];
cetak("Apakah NIM 202605 ada?:", cari_data(nim_terdaftar, 202605));
```

---

### 📅 Pertemuan 24: Algoritma Pencarian Biner (*Binary Search*)
- **Tujuan:** Mengoptimalkan pencarian data terurut dengan kompleksitas $O(\log N)$.
- **Contoh Program (`p24_binary_search.wya`):**
```widya
fungsi binary_search(data: [Int], target: Int, n: Int) -> Int {
    misal low = 0;
    misal high = n - 1;
    
    selama low <= high {
        misal mid = (low + high) / 2;
        jika data[mid] == target {
            kembalikan mid;
        } lainnya jika data[mid] < target {
            low = mid + 1;
        } lainnya {
            high = mid - 1;
        }
    }
    kembalikan -1; // Tidak ditemukan
}
```

---

### 📅 Pertemuan 25: 🎯 Ujian Tengah Semester (UTS) — Struktur Data Antrean (Queue)
- **Tugas Praktik:** Membuat simulasi nomor antrean loket puskesmas menggunakan array generic dengan fungsi `tambah_antrean` dan `panggil_antrean`.

---

### 📅 Pertemuan 26: Algoritma Pengurutan Gelembung (*Bubble Sort*)
- **Tujuan:** Mengurutkan array dengan mekanisme penukaran berulang elemen bersebelahan.

---

### 📅 Pertemuan 27: Algoritma Pengurutan Penyisipan (*Insertion Sort*)
- **Tujuan:** Memahami logika sorting berbasis simulasi kartu tangan.

---

### 📅 Pertemuan 28: Menemukan Nilai Minimum, Maksimum, & Median
- **Tujuan:** Menganalisis sebaran data numerik pada kumpulan record nilai siswa.

---

### 📅 Pertemuan 29: Matriks & Larik Dua Dimensi (`[[Int]]`)
- **Tujuan:** Merepresentasikan tabel grid, papan catur, atau gambar piksel dengan array 2D.
- **Contoh Program (`p29_matriks.wya`):**
```widya
misal baris1 = [1, 0, 0];
misal baris2 = [0, 1, 0];
misal baris3 = [0, 0, 1];

cetak("Matriks Identitas 3x3:");
cetak(baris1);
cetak(baris2);
cetak(baris3);
```

---

### 📅 Pertemuan 30: Deteksi Kebocoran Memori & Profiler Dasar
- **Tujuan:** Memahami cara kerja Safe Reference Counting dan detektor siklus memori (Tarjan Algorithm) di Widya-Lang.

---

### 📅 Pertemuan 31: Optimasi Kompleksitas Algoritma (Big-O Notation)
- **Tujuan:** Mengukur waktu eksekusi program dan efisiensi memori kode siswa.

---

### 📅 Pertemuan 32: 🏆 Ujian Akhir Semester (UAS) — Sistem Analitik Pemeringkatan Siswa
- **Tugas Akhir:** Membangun aplikasi pemeringkatan nilai paralel sekolah: menerima daftar nilai siswa, mengurutkan dari tertinggi ke terendah secara otomatis, menghitung median & modus kelas, dan menampilkan tabel peringkat.
