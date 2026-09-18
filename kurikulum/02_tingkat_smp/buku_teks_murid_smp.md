# 📖 Buku Teks Murid: Logika Algoritma & Struktur Data (Tingkat SMP / MTs)

Selamat datang di Buku Teks Siswa **Pemrograman & Algoritma Tingkat SMP** dengan **Widya-Lang**. Pada jenjang ini, kamu akan bertransformasi dari sekadar pengguna menjadi perancang algoritma dan pembuat aplikasi terstruktur.

---

## 📑 Daftar Pembelajaran

1. **Bab 1: Modularitas & Pembuatan Fungsi (`fungsi`)**
2. **Bab 2: Struktur Data Larik / Array (`[T]`) & Iterasi Koleksi**
3. **Bab 3: Algoritma Pencarian & Pengurutan (*Bubble Sort*)**
4. **Bab 4: Logika Game & Pola Pencocokan (*Pattern Matching*)**
5. **Bab 5: Proyek Akhir — Sistem Aplikasi Kasir & Diskon Kantin Sekolah**

---

## 📌 Bab 1: Modularitas & Pembuatan Fungsi (`fungsi`)

Saat program kita bertambah besar, menulis semua kode di satu tempat akan membuat kode berantakan. **Fungsi** adalah blok kode mandiri yang memiliki nama, menerima masukan (*parameter*), melakukan tugas tertentu, dan dapat mengembalikan nilai (*return value*).

### 💡 Sintaks Fungsi di Widya-Lang
```widya
fungsi nama_fungsi(parameter1: Tipe, parameter2: Tipe) -> TipeHasil {
    // Instruksi komputasi
    kembalikan hasil;
}
```

### 🧪 Praktik: Menghitung Luas Bangun Datar
```widya
fungsi hitung_luas_persegi_panjang(panjang: Int, lebar: Int) -> Int {
    kembalikan panjang * lebar;
}

misal p = 12;
misal l = 5;
misal luas = hitung_luas_persegi_panjang(p, l);
cetak("Luas Persegi Panjang (12x5):", luas, "cm²");
```

---

## 📌 Bab 2: Struktur Data Larik (Array / List)

Jika kamu ingin menyimpan nilai rapor seluruh siswa di kelasmu, membuat variabel satu per satu (`nilai1`, `nilai2`, dst.) sangatlah tidak efisien. Di sinilah kita menggunakan **Larik / Array**.

```widya
misal daftar_nilai = [85, 90, 78, 92, 88, 65];

// Mengakses data berdasarkan indeks (dimulai dari indeks 0)
cetak("Nilai siswa pertama:", daftar_nilai[0]); // Output: 85

// Menghitung akumulasi nilai seluruh kelas
misal total = 0;
untuk nilai dalam daftar_nilai {
    total = total + nilai;
}

misal rata_rata = total / 6;
cetak("Rata-rata Nilai Kelas:", rata_rata);
```

---

## 📌 Bab 3: Algoritma Pengurutan (*Sorting Algorithms*)

Bagaimana komputer mengurutkan angka dari terkecil ke terbesar? Salah satu algoritma klasik yang paling mudah dipelajari adalah **Bubble Sort** (Pengurutan Gelembung).

### 🧠 Logika Bubble Sort:
1. Bandingkan dua angka yang bersebelahan.
2. Jika angka kiri lebih besar dari angka kanan, tukar posisinya (*swap*).
3. Ulangi proses ini sampai seluruh elemen berada di posisi yang benar.

```widya
fungsi urutkan_data(data: [Int]) -> [Int] {
    misal n = 5;
    misal hasil = data;
    
    untuk i dalam 0..n {
        untuk j dalam 0..(n - 1) {
            jika hasil[j] > hasil[j + 1] {
                misal temp = hasil[j];
                hasil[j] = hasil[j + 1];
                hasil[j + 1] = temp;
            }
        }
    }
    kembalikan hasil;
}
```

---

## 📌 Bab 4: Interaksi Logika & Game Tebak Angka

Membuat logika permainan komputer memerlukan kombinasi dari:
- Variabel penampung state (target rahasia & jumlah percobaan).
- Pengulangan untuk memberi kesempatan berkali-kali.
- Percabangan untuk memberikan petunjuk (`terlalu besar`, `terlalu kecil`, atau `cocok`).

---

## 🏆 Bab 5: Proyek Rekayasa Akhir — Aplikasi Kasir Kantin

Pada proyek akhir SMP, kamu akan membangun sistem kasir terintegrasi yang menghitung total belanja, menerapkan potongan harga (*diskon*) berdasarkan kategori pembeli (Guru atau Siswa), serta menghitung uang kembalian secara otomatis.

### 📋 Tugas Mandiri Murid:
1. Tambahkan menu baru ke dalam program kasir (misal: "Jus Alpukat", "Mie Ayam").
2. Buat diskon khusus hari Jumat sebesar 20%.
3. Tampilkan struk belanja yang rapi dan menarik.
