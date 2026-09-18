# 📘 Silabus & Modul Detail Semester 1 (Kelas 10 Ganjil)
## Tema: Fondasi Berpikir Komputasional & Pemrograman Modular (16 Pertemuan)

---

### 📅 Pertemuan 01: Pengenalan Bahasa Widya-Lang & Ekosistem Toolchain
- **Tujuan:** Memahami filosofi Widya-Lang, instalasi CLI, ekstensi VS Code, dan menjalankan program pertama.
- **Teori:** Kode sumber, Compiler vs Interpreter, peran biner native.
- **Contoh Program (`p01_halo_widya.wya`):**
```widya
cetak("============================================");
cetak("⚡ SELAMAT DATANG DI WIDYA-LANG ECOSYSTEM ⚡");
cetak("============================================");
cetak("Nama Siswa : Satria Nusantara");
cetak("Kelas      : X Rekayasa Perangkat Lunak 1");
```
- **Latihan Soal:**
  1. Tuliskan kode program untuk mencetak profil sekolahmu secara lengkap!
  2. Jelaskan fungsi tanda titik koma (`;`) pada akhir baris program!

---

### 📅 Pertemuan 02: Lingkungan REPL & Alur Kompilasi Sederhana
- **Tujuan:** Terampil menggunakan `widya repl` untuk *rapid prototyping* dan perintah `widya run file.wya`.
- **Teori:** Perbedaan pengujian interaktif konsol dengan eksekusi file skrip.
- **Latihan Soal:** Jalankan mode REPL, hitung ekspresi matematika langsung, lalu simpan perintah ke file `.wya`.

---

### 📅 Pertemuan 03: Variabel (`misal`), Immutability, & Tipe Data Primitif
- **Tujuan:** Memahami konsep alokasi variabel dan tipe data: Integer (`Int`), Desimal (`Float`), Kalimat (`String`), Logika (`Bool`).
- **Contoh Program (`p03_variabel.wya`):**
```widya
misal nama_lengkap = "Ahmad Fadhil";
misal umur = 16;
misal tinggi_badan = 168.5;
misal aktif = benar;

cetak("Nama   :", nama_lengkap);
cetak("Umur   :", umur, "tahun");
cetak("Tinggi :", tinggi_badan, "cm");
cetak("Aktif  :", aktif);
```

---

### 📅 Pertemuan 04: Operator Aritmatika & Perhitungan Geometri
- **Tujuan:** Menguasai operator `+`, `-`, `*`, `/`, `%` untuk menyelesaikan kasus sains dan matematika.
- **Contoh Program (`p04_geometri.wya`):**
```widya
misal panjang = 20;
misal lebar = 15;
misal luas = panjang * lebar;
misal keliling = 2 * (panjang + lebar);

cetak("Luas Persegi Panjang     :", luas, "m²");
cetak("Keliling Persegi Panjang :", keliling, "m");
```

---

### 📅 Pertemuan 05: Operator Relasional & Logika Boolean
- **Tujuan:** Menggunakan operator perbandingan (`==`, `!=`, `<`, `>`, `<=`, `>=`) dan logika (`&&`, `||`, `!`).
- **Contoh Program (`p05_logika.wya`):**
```widya
misal nilai_teori = 80;
misal nilai_praktik = 85;
misal lulus_kkm = (nilai_teori >= 75) && (nilai_praktik >= 75);

cetak("Status Kelulusan Siswa:", lulus_kkm);
```

---

### 📅 Pertemuan 06: Struktur Percabangan Tunggal & Ganda (`jika` / `lainnya`)
- **Tujuan:** Mengontrol alur program berdasarkan pengujian kondisi logika.
- **Contoh Program (`p06_percabangan.wya`):**
```widya
misal saldo = 50000;
misal harga_tiket = 35000;

jika saldo >= harga_tiket {
    misal sisa = saldo - harga_tiket;
    cetak("✅ Pembelian tiket bioskop berhasil! Sisa saldo:", sisa);
} lainnya {
    cetak("❌ Saldo tidak mencukupi untuk membeli tiket.");
}
```

---

### 📅 Pertemuan 07: Percabangan Majemuk Bertingkat (`lainnya jika`)
- **Tujuan:** Menyelesaikan evaluasi skenario multi-kondisi (skala nilai huruf A, B, C, D).
- **Contoh Program (`p07_grade_nilai.wya`):**
```widya
misal nilai = 88;

jika nilai >= 90 {
    cetak("Grade: A (Sangat Memuaskan)");
} lainnya jika nilai >= 80 {
    cetak("Grade: B (Baik)");
} lainnya jika nilai >= 70 {
    cetak("Grade: C (Cukup)");
} lainnya {
    cetak("Grade: D (Remedial)");
}
```

---

### 📅 Pertemuan 08: 🎯 Ujian Tengah Semester (UTS) — Simulator Tiket Interaktif
- **Tugas Praktik:** Siswa secara mandiri membuat aplikasi konsol `uts_tiket_kereta.wya` dengan logika tarif berdasarkan kelas (Eksekutif/Bisnis/Ekonomi), diskon usia (Lansia/Pelajar), dan perhitungan uang kembalian.

---

### 📅 Pertemuan 09: Perulangan Terhitung (`untuk ... dalam rentang`)
- **Tujuan:** Mengotomatisasi instruksi berulang dengan rentang numerik `a..b`.
- **Contoh Program (`p09_loop_hitung.wya`):**
```widya
cetak("Daftar Kuadrat Angka 1 sampai 5:");
untuk i dalam 1..6 {
    cetak("Angka", i, "kuadrat =", i * i);
}
```

---

### 📅 Pertemuan 10: Perulangan Kondisional (`selama`) & Kontrol Loop (`berhenti`, `lanjut`)
- **Tujuan:** Menjalankan perulangan berbasis kondisi dinamis dengan kontrol interupsi.
- **Contoh Program (`p10_selama.wya`):**
```widya
misal counter = 1;
selama counter <= 5 {
    jika counter == 3 {
        cetak("Melewati angka 3");
        counter = counter + 1;
        lanjut;
    }
    cetak("Hitungan:", counter);
    counter = counter + 1;
}
```

---

### 📅 Pertemuan 11: Modularitas Kode: Pembuatan Fungsi Dasar (`fungsi`)
- **Tujuan:** Membagi kode kompleks menjadi fungsi-fungsi kecil yang reusable.
- **Contoh Program (`p11_fungsi_dasar.wya`):**
```widya
fungsi cetak_garis_pembatas() {
    cetak("============================================");
}

fungsi sapa_pengguna(nama: String) {
    cetak("Halo", nama, "! Selamat datang di laboratorium komputer.");
}

cetak_garis_pembatas();
sapa_pengguna("Nadia");
cetak_garis_pembatas();
```

---

### 📅 Pertemuan 12: Fungsi dengan Nilai Kembalian (`kembalikan`)
- **Tujuan:** Merancang fungsi komputasi yang mengirimkan hasil olah data ke pemanggil.
- **Contoh Program (`p12_fungsi_kembalian.wya`):**
```widya
fungsi hitung_diskon(total: Int, persentase: Int) -> Int {
    kembalikan total * persentase / 100;
}

misal total_belanja = 200000;
misal potongan = hitung_diskon(total_belanja, 15);
cetak("Total Bayar Akhir: Rp", total_belanja - potongan);
```

---

### 📅 Pertemuan 13: Ruang Lingkup Variabel (Local vs Global Scope) & Shadowing
- **Tujuan:** Memahami siklus hidup variabel (*variable lifetime*) dan isolasi memori fungsi.

---

### 📅 Pertemuan 14: Fungsi Rekursif & Konsep Call Stack
- **Tujuan:** Memahami algoritma rekursi, kondisi basis (*base case*), dan eksekusi stack memori.
- **Contoh Program (`p14_faktorial.wya`):**
```widya
fungsi faktorial(n: Int) -> Int {
    jika n <= 1 {
        kembalikan 1;
    }
    kembalikan n * faktorial(n - 1);
}

cetak("Faktorial 5! =", faktorial(5)); // Output: 120
```

---

### 📅 Pertemuan 15: Perancangan Arsitektur Mini-Game Terminal
- **Tujuan:** Menyusun bagan alir (*Flowchart*) dan struktur modular sebelum membuat proyek akhir semester.

---

### 📅 Pertemuan 16: 🏆 Ujian Akhir Semester (UAS) — Proyek Game "Petualangan Ksatria"
- **Tugas Akhir:** Membangun aplikasi game petualangan terminal berbasis teks lengkap dengan pemilihan senjata, pertempuran turn-based sederhana, inventaris koin, dan skor akhir.
