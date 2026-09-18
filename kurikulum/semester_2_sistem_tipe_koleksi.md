# 📘 Silabus & Modul Detail Semester 2 (Kelas 10 Genap)
## Tema: Sistem Tipe Hindley-Milner, Generics, & Struktur Data Koleksi (Pertemuan 17 - 32)

---

### 📅 Pertemuan 17: Teori Sistem Tipe Statis & Jaminan Keamanan Kompilasi
- **Tujuan Pembelajaran:** Siswa memahami perbedaan fundamental antara bahasa bertipe dinamis (interpretasi) dan bertipe statis (*compile-time checking*), serta mengapa kesalahan tipe di tahap kompilasi lebih murah diperbaiki daripada bug di lingkungan produksi.
- **Teori & Konsep:**
  - *Type Soundness*: Jaminan bahwa program yang terkompilasi tidak akan mengalami *Type Confusion*.
  - Perbandingan efisiensi alokasi memori biner bertipe statis vs *dynamically boxed objects*.
- **Contoh Program (`p17_type_safety.wya`):**
```widya
// Kompiler Widya menjamin keamanan tipe secara langsung
misal skor_angka = 100;
misal nama_user = "Budi";

cetak("Tipe terverifikasi secara statis sejak awal!");
```
- **Latihan Soal & Tugas:**
  1. Apa keuntungan utama deteksi kesalahan tipe pada saat kompilasi dibandingkan runtime?
  2. Jelaskan mengapa sistem tipe statis menghasilkan performa eksekusi yang lebih cepat!

---

### 📅 Pertemuan 18: Algoritma Inferensi Tipe Hindley-Milner (HM)
- **Tujuan Pembelajaran:** Siswa memahami bagaimana algoritma Hindley-Milner secara otomatis menentukan tipe data (*type reconstruction*) tanpa memaksa programmer menulis tipe manual secara berulang.
- **Teori & Konsep:**
  - *Unification Algorithm*: Mekanisme penyatuan variabel tipe bebas $\alpha, \beta$ menjadi tipe konkret (`Int`, `String`, dll.).
  - *Zero Boilerplate*: Menulis kode sebersih bahasa skrip dengan keamanan sekuat bahasa sistem tingkat tinggi.
- **Contoh Program (`p18_hindley_milner.wya`):**
```widya
fungsi kuadrat_hm(x) {
    kembalikan x * x; // Compiler otomatis menyimpulkan x: Int -> Int
}

misal angka = 12;
misal hasil = kuadrat_hm(angka);
cetak("Kuadrat dari", angka, "adalah:", hasil);
```
- **Latihan Soal & Tugas:**
  1. Analisis bagaimana fungsi `kuadrat_hm` di atas menyimpulkan tipe data parameternya!
  2. Buat fungsi `kali_tiga(n)` tanpa deklarasi tipe manual dan verifikasi inferensinya.

---

### 📅 Pertemuan 19: Pemrograman Generic Sederhana (`<T>`)
- **Tujuan Pembelajaran:** Siswa mampu membuat fungsi generic parameter tipe yang dapat menerima berbagai variasi tipe data tanpa duplikasi kode (*DRY - Don't Repeat Yourself*).
- **Teori & Konsep:**
  - Monomorphization: Kompiler Widya membangkitkan kode mesin khusus untuk setiap tipe konkret yang digunakan tanpa overhead runtime.
- **Contoh Program (`p19_generics_dasar.wya`):**
```widya
fungsi identitas<T>(nilai: T) -> T {
    kembalikan nilai;
}

misal a = identitas(500);
misal b = identitas("Widya-Lang");
misal c = identitas(benar);

cetak("Generic Int    :", a);
cetak("Generic String :", b);
cetak("Generic Bool   :", c);
```
- **Latihan Soal & Tugas:**
  1. Buat fungsi generic `duplikasi<T>(item: T) -> (T, T)` yang mengembalikan sepasang tuple berisi item yang sama!

---

### 📅 Pertemuan 20: Generic Multi-Tipe & Pasangan Heterogen Tuple `(A, B)`
- **Tujuan Pembelajaran:** Mengelompokkan dua atau lebih nilai dengan tipe data yang berbeda ke dalam satu unit data tuple heterogen.
- **Teori & Konsep:**
  - Tipe Produk Aljabar (*Cartesian Product Type*).
  - Destructuring tuple langsung ke dalam variabel individual.
- **Contoh Program (`p20_tuple.wya`):**
```widya
fungsi buat_pasangan<A, B>(pertama: A, kedua: B) -> (A, B) {
    kembalikan (pertama, kedua);
}

misal (latitude, longitude) = buat_pasangan(-6.2088, 106.8456);
cetak("Latitude  :", latitude);
cetak("Longitude :", longitude);
```
- **Latihan Soal & Tugas:**
  1. Buat fungsi yang menerima data nama siswa (`String`) dan nilai rapor (`Int`), lalu mengembalikan tuple `(String, Int)`!

---

### 📅 Pertemuan 21: Struktur Data Larik / Array Dinamis (`[T]`)
- **Tujuan Pembelajaran:** Memahami representasi koleksi sekuensial di memori komputer, pengindeksan array (0-indexed), dan pengecekan batas (*bounds checking*).
- **Teori & Konsep:**
  - Lokasi memori contiguous (berurutan) dan akses waktu konstan $O(1)$.
- **Contoh Program (`p21_array_dasar.wya`):**
```widya
misal daftar_nilai = [88, 92, 75, 84, 90];

cetak("Jumlah elemen larik :", 5);
cetak("Nilai pertama (idx 0):", daftar_nilai[0]);
cetak("Nilai ketiga  (idx 2):", daftar_nilai[2]);
```
- **Latihan Soal & Tugas:**
  1. Buat array yang menyimpan 6 nama mata pelajaran pilihanmu dan cetak mata pelajaran ke-4!

---

### 📅 Pertemuan 22: Iterasi Koleksi Data & Agregasi Statistik
- **Tujuan Pembelajaran:** Mengolah seluruh elemen array menggunakan perulangan untuk menghitung metrik statistik (Total Jumlah, Nilai Rata-rata).
- **Contoh Program (`p22_agregasi.wya`):**
```widya
misal data_berat = [45, 52, 60, 48, 55];
misal total_berat = 0;

untuk berat dalam data_berat {
    total_berat = total_berat + berat;
}

misal rata_rata = total_berat / 5;
cetak("Total Bobot Keseluruhan :", total_berat, "kg");
cetak("Rata-rata Bobot Siswa   :", rata_rata, "kg");
```
- **Latihan Soal & Tugas:**
  1. Buat program yang menghitung total uang kas kelas dari array pembayaran 8 siswa!

---

### 📅 Pertemuan 23: Algoritma Pencarian Linier (*Linear Search*)
- **Tujuan Pembelajaran:** Mengimplementasikan algoritma penelusuran sekuensial dari indeks awal hingga akhir dengan kompleksitas waktu $O(N)$.
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

misal data_nis = [202601, 202602, 202605, 202610];
misal dicari = 202605;

jika cari_data(data_nis, dicari) {
    cetak("✅ Data NIS", dicari, "ditemukan dalam sistem sekolah!");
} lainnya {
    cetak("❌ Data NIS", dicari, "tidak ditemukan.");
}
```
- **Latihan Soal & Tugas:**
  1. Modifikasi fungsi di atas agar mengembalikan posisi indeks di mana data ditemukan (atau -1 jika tidak ada)!

---

### 📅 Pertemuan 24: Algoritma Pencarian Biner (*Binary Search*)
- **Tujuan Pembelajaran:** Mengimplementasikan algoritma pencarian bagi-dua (*divide and conquer*) pada data terurut dengan kompleksitas efisien $O(\log N)$.
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
    kembalikan -1;
}

misal data_terurut = [10, 20, 30, 40, 50, 60, 70];
misal posisi = binary_search(data_terurut, 40, 7);
cetak("Angka 40 ditemukan pada indeks:", posisi);
```
- **Latihan Soal & Tugas:**
  1. Jelaskan mengapa data wajib terurut terlebih dahulu sebelum menjalankan Binary Search!

---

### 📅 Pertemuan 25: 🎯 Ujian Tengah Semester (UTS) — Struktur Data Antrean (Queue)
- **Tugas Praktik Mandiri:** Siswa mengimplementasikan simulasi nomor antrean loket pelayanan sekolah menggunakan array generic dengan fungsi `tambah_antrean`, `layani_antrean`, dan `lihat_antrean_terdepan`.

---

### 📅 Pertemuan 26: Algoritma Pengurutan Gelembung (*Bubble Sort*)
- **Tujuan Pembelajaran:** Mengurutkan kumpulan angka acak dengan mekanisme penukaran berpasangan (*swap*) secara berulang.
- **Contoh Program (`p26_bubble_sort.wya`):**
```widya
fungsi bubble_sort(arr: [Int]) -> [Int] {
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

misal nilai_acak = [64, 34, 25, 12, 22];
cetak("Sebelum Diurutkan :", nilai_acak);
cetak("Setelah Diurutkan :", bubble_sort(nilai_acak));
```

---

### 📅 Pertemuan 27: Algoritma Pengurutan Penyisipan (*Insertion Sort*)
- **Tujuan Pembelajaran:** Memahami algoritma pengurutan stabil yang efisien untuk kumpulan data kecil atau data yang hampir terurut.

---

### 📅 Pertemuan 28: Menemukan Nilai Ekstrem (Minimum & Maksimum)
- **Tujuan Pembelajaran:** Menganalisis sebaran nilai dengan menentukan skor tertinggi dan terendah dalam satu lintasan iterasi $O(N)$.
- **Contoh Program (`p28_min_max.wya`):**
```widya
misal skor_ujian = [72, 95, 60, 88, 91, 55];
misal nilai_max = skor_ujian[0];
misal nilai_min = skor_ujian[0];

untuk skor dalam skor_ujian {
    jika skor > nilai_max {
        nilai_max = skor;
    }
    jika skor < nilai_min {
        nilai_min = skor;
    }
}

cetak("Skor Tertinggi Kelas :", nilai_max);
cetak("Skor Terendah Kelas  :", nilai_min);
```

---

### 📅 Pertemuan 29: Matriks & Larik Dua Dimensi (`[[Int]]`)
- **Tujuan Pembelajaran:** Menggunakan representasi baris dan kolom untuk tabel data, papan permainan grid, atau pemrosesan citra dasar.
- **Contoh Program (`p29_matriks.wya`):**
```widya
misal baris1 = [1, 2, 3];
misal baris2 = [4, 5, 6];
misal baris3 = [7, 8, 9];

cetak("Tampilan Grid 3x3:");
cetak(baris1);
cetak(baris2);
cetak(baris3);
```

---

### 📅 Pertemuan 30: Pengenalan Manajemen Memori & Tarjan Cycle Detector
- **Tujuan Pembelajaran:** Memahami bagaimana Widya-Lang mengelola alokasi memori secara aman tanpa jeda *Garbage Collector* melalui *Safe Reference Counting* dan algoritma deteksi siklus Tarjan.

---

### 📅 Pertemuan 31: Analisis Kompleksitas Algoritma (Big-O Notation)
- **Tujuan Pembelajaran:** Membandingkan efisiensi algoritma berdasarkan konsumsi waktu ($O(1)$, $O(\log N)$, $O(N)$, $O(N^2)$) dan penggunaan ruang RAM.

---

### 📅 Pertemuan 32: 🏆 Ujian Akhir Semester (UAS) — Sistem Analitik Pemeringkatan Siswa
- **Tugas Proyek Akhir:** Membangun aplikasi pemeringkatan nilai paralel:
  1. Menginput data nilai 10 siswa.
  2. Mengurutkan ranking secara otomatis dari tertinggi ke terendah menggunakan Bubble/Insertion Sort.
  3. Menghitung rata-rata kelas, nilai tertinggi, dan nilai terendah.
  4. Menampilkan tabel rekapitulasi nilai yang rapi di konsol terminal.
