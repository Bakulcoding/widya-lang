# 📖 Buku Teks Murid: Petualangan Coding Widya (Tingkat SD / MI)

Selamat datang di dunia petualangan pemrograman bersama **Widya-Lang**! Di buku ini, kamu akan belajar cara memberikan instruksi kepada komputer agar menjadi asisten hebatmu menggunakan bahasa Indonesia yang mudah dipahami.

---

## 🧭 Peta Petualangan Belajar

| Bab | Judul Petualangan | Hal Seru yang Dipelajari |
|:---:|---|---|
| **Bab 1** | Halo Komputer, Kenalkan Namaku! | Mengenal perintah `cetak` dan teks |
| **Bab 2** | Kotak Rahasia Sang Penyihir | Menyimpan nama dan angka ke `misal` (Variabel) |
| **Bab 3** | Mesin Hitung Cerdas | Menjumlahkan uang saku dan jajan (`+`, `-`, `*`, `/`) |
| **Bab 4** | Robot Penjaga Gerbang | Mengambil keputusan dengan `jika` dan `lainnya` |
| **Bab 5** | Mantra Mengulang Tanpa Lelah | Perulangan otomatis dengan `untuk` |
| **Bab 6** | Misi Akhir: Penjelajah Gua Harta Karun | Membuat Game Cerita Pilihan Sendiri |

---

## 🌟 Bab 1: Halo Komputer, Kenalkan Namaku!

Komputer itu seperti robot cerdas, tetapi ia hanya bisa bekerja jika kita memberinya perintah (*instruksi*). Di Widya-Lang, perintah untuk berbicara atau menampilkan tulisan ke layar adalah `cetak()`.

### ✏️ Ayo Mencoba!
Buka file `01_halo_widya.wya`, lalu tuliskan kode berikut:

```widya
cetak("Halo teman-teman!");
cetak("Nama saya Budi, umur 10 tahun.");
cetak("Saya sedang belajar coding bersama Widya-Lang!");
```

### 💡 Catatan Penting:
1. Setiap tulisan atau kalimat harus diapit tanda petik ganda `""`.
2. Jangan lupa sertakan tanda titik koma `;` di akhir perintah sebagai tanda bahwa kalimat instruksimu sudah selesai.

---

## 🌟 Bab 2: Kotak Rahasia Sang Penyihir (Variabel)

Bayangkan kamu memiliki beberapa kotak sepatu kosong di kamarmu. Kamu memberi label di kotak itu:
- Kotak bertuliskan **`nama`** diisi kertas bertuliskan `"Aisyah"`.
- Kotak bertuliskan **`umur`** diisi angka `11`.

Di pemrograman, kotak penyimpanan ini disebut **Variabel**. Di Widya-Lang, kita menggunakan kata **`misal`**.

```widya
misal nama_pahlawan = "Aisyah";
misal kekuatan_serang = 100;

cetak("Pahlawan kita bernama:", nama_pahlawan);
cetak("Memiliki kekuatan:", kekuatan_serang);
```

---

## 🌟 Bab 3: Mesin Hitung Cerdas (Matematika Komputer)

Komputer bisa menghitung angka jutaan dalam waktu kurang dari satu kedipan mata!

Simbol matematika di komputer:
- Tambah: `+`
- Kurang: `-`
- Kali: `*` (Tanda bintang)
- Bagi: `/` (Garis miring)

### 🛒 Contoh Kasus: Membeli Jajan di Kantin
```widya
misal uang_saku = 20000;
misal harga_bakso = 12000;
misal harga_es_teh = 3000;

misal total_jajan = harga_bakso + harga_es_teh;
misal sisa_kembalian = uang_saku - total_jajan;

cetak("Total jajan saya : Rp", total_jajan);
cetak("Sisa uang tabung : Rp", sisa_kembalian);
```

---

## 🌟 Bab 4: Robot Penjaga Gerbang (Percabangan `jika`)

Bagaimana jika kita ingin robot mengambil keputusan?
Contohnya:
- **Jika** hari hujan, kita bawa payung.
- **Lainnya** (jika tidak hujan), kita pakai kacamata hitam.

```widya
misal cuaca = "hujan";

jika cuaca == "hujan" {
    cetak("🌧️ Hari hujan! Jangan lupa bawa payung ya!");
} lainnya {
    cetak("☀️ Hari cerah! Ayo bermain di lapangan!");
}
```

> **Tips:** Tanda `==` (dua tanda sama dengan) digunakan untuk menanyakan *"apakah nilainya sama persis?"*.

---

## 🌟 Bab 5: Mantra Mengulang Tanpa Lelah (`untuk`)

Apakah kamu bosan jika disuruh menulis angka 1 sampai 100 secara manual? Robot tidak akan pernah merasa bosan! Kita bisa menggunakan perintah `untuk`.

```widya
cetak("🚀 Hitung mundur peluncuran roket:");

untuk detik dalam 1..6 {
    cetak("Detik ke-", detik);
}

cetak("🎉 Wussshhhh! Roket terbang ke angkasa!");
```

---

## 🏆 Bab 6: Misi Tantangan Proyek Akhir (Game Cerita Interaktif)

Sekarang saatnya menggabungkan semua ilmumu! Buatlah sebuah game cerita petualangan di mana temanmu bisa memilih jalan (kiri atau kanan) untuk menemukan mahkota raja yang hilang! (Buka file `05_proyek_cerita_petualangan.wya` untuk melihat contoh lengkapnya).

### 📝 Lembar Refleksi Murid:
- [ ] Aku sudah bisa menampilkan tulisan biodataku di layar.
- [ ] Aku mengerti cara menyimpan angka dan nama ke dalam variabel `misal`.
- [ ] Aku bisa membuat simulasi keputusan menggunakan `jika` dan `lainnya`.
- [ ] Aku berhasil membuat proyek game petualanganku sendiri!
