# Tutorial 03 — Kontrol Alur (Percabangan & Perulangan)

**Target**: Menguasai `jika/kalau`, `selama`, `untuk X dalam Y`, dan pola kontrol alur tingkat lanjut.
**Level**: Pemula ⭐
**Waktu**: ~20 menit
**File contoh lengkap**: [03_koleksi_data.wya](../contoh/03_koleksi_data.wya), [05_pola_bintang.wya](../contoh/05_pola_bintang.wya)

---

## 1. Percabangan `jika / kalau`

### Bentuk Dasar
```widya
misal umur = 18;

jika umur >= 17 {
    cetak("Boleh membuat KTP");
}
```

### Dengan Blok Kalau (Else If)
```widya
misal nilai = 82;

jika nilai >= 90 {
    cetak("Predikat: A");
} kalau nilai >= 80 {
    cetak("Predikat: B");
} kalau nilai >= 70 {
    cetak("Predikat: C");
} kalau nilai >= 60 {
    cetak("Predikat: D");
} {
    cetak("Predikat: E — Tidak Lulus");
}
```

> ⚠️ **Catatan Penting tentang `lainya`**:
> Keyword `lainya` (setara else) **TERSEDIA** tapi untuk pola inline satu baris terkadang parser tidak mengenalinya jika tidak ada spasi yang tepat.
>
> ✅ **Pattern yang SELALU bekerja** (disarankan):
> ```widya
> jika x > 5 { kembalikan "besar" };
> kembalikan "kecil";
> ```
> (Ini semantik `jika ... else` — jika kondisi salah, lanjut ke pernyataan berikutnya.)

### Ternary / Ekspresi Bersyarat
`jika` BISA dipakai sebagai EKSPRESI (mengembalikan nilai):
```widya
misal status = jika umur >= 18 { "DEWASA" } { "ANAK" };
cetak(status);   // "DEWASA" jika umur>=18
```

---

## 2. Perulangan `selama` (While)

Loop selama kondisi masih `benar`:
```widya
// Hitung 1 sampai 5
misal i = 1;
selama i <= 5 {
    cetak("i =", i);
    i = i + 1;
}
```

### `berhenti` (break) & `lanjut` (continue)
```widya
// Cari angka prima pertama > 20
misal n = 21;
selama benar {
    misal prima = benar;
    misal d = 2;
    selama d * d <= n {
        jika n % d == 0 { prima = salah; berhenti; }
        d = d + 1;
    }
    jika prima { cetak("Prima pertama >20:", n); berhenti; }
    n = n + 1;
}
```

---

## 3. Perulangan `untuk X dalam Y` (For-In)

Widya memakai `untuk ... dalam ...` (bukan range numerik langsung).

### Iterasi Larik (Array)
```widya
misal buah = ["apel", "pisang", "mangga"];

untuk b dalam buah {
    cetak("Saya suka:", b);
}
```

### Iterasi Indeks + Elemen
Pola umum jika butuh indeks:
```widya
misal warna = ["merah", "hijau", "biru"];

misal idx = 0;
untuk w dalam warna {
    cetak("Warna ke-" + ke_teks(idx) + ":", w);
    idx = idx + 1;
}
```

> 💡 Jika butuh range 0..N, bikin larik indeks manual:
> ```widya
> misal angka_n = [0,1,2,3,4,5,6,7,8,9];
> untuk i dalam angka_n {
>     cetak("Kuadrat dari", i, "=", i*i);
> }
> ```

### Iterasi Kamus (Dict)
```widya
misal profil = {
    "nama": "Siti",
    "umur": 25,
    "kota": "Bandung",
    "aktif": benar
};

// Dapatkan daftar key dulu pakai `kunci_kamus()`
untuk k dalam kunci_kamus(profil) {
    cetak(k, "→", profil[k]);
}
```

---

## 4. Contoh Praktik: Pola Bintang Segitiga

File contoh: [05_pola_bintang.wya](../contoh/05_pola_bintang.wya)
```widya
misal tinggi = 6;
misal baris = 1;
selama baris <= tinggi {
    // Cetak spasi dulu (rata kanan)
    misal spasi = tinggi - baris;
    misal line = "";

    misal s = 0;
    selama s < spasi { line = line + " "; s = s+1; }

    // Cetak bintang (2*baris - 1 buah)
    misal jml_b = 2*baris - 1;
    misal b = 0;
    selama b < jml_b { line = line + "*"; b = b+1; }

    cetak(line);
    baris = baris + 1;
}
```

Output:
```
     *
    ***
   *****
  *******
 *********
***********
```

---

## 5. Match Bersyarat Pola (Switch-Case Style)

Widya tidak punya keyword `switch` — pakai rantai `jika / kalau`:
```widya
fungsi nama_hari(angka_hari) {
    jika angka_hari == 1 { kembalikan "Senin" };
    kalau angka_hari == 2 { kembalikan "Senin SALAH — Selasa" };
    kalau angka_hari == 3 { kembalikan "Rabu" };
    kalau angka_hari == 4 { kembalikan "Kamis" };
    kalau angka_hari == 5 { kembalikan "Jumat" };
    kalau angka_hari == 6 { kembalikan "Sabtu" };
    kalau angka_hari == 7 { kembalikan "Minggu" };
    kembalikan "Tidak valid";
}

cetak(nama_hari(5));  // "Jumat"
```

---

## 🎯 Latihan Mini

1. **Kalkulator BMI**: Buat variabel `berat = 65` (kg) dan `tinggi_m = 1.70`. Hitung `bmi = berat / (tinggi_m ** 2)`. Cetak kategori:
   - `< 18.5` → "Kurus"
   - `18.5 - 24.9` → "Normal"
   - `25 - 29.9` → "Gemuk"
   - `>= 30` → "Obesitas"

2. **FizzBuzz 1 sampai 30**:
   - Jika angka kelipatan 3 DAN 5 → cetak `FizzBuzz`
   - Jika hanya kelipatan 3 → `Fizz`
   - Jika hanya kelipatan 5 → `Buzz`
   - Selainnya → cetak angkanya

3. **Jumlah total deret ganjil 1..99** (1+3+5+…+99). Jawaban benar = **2500**.

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 02 — Halo Dunia](./TUTORIAL_02_HALO_DUNIA.md) |
➡️ **[Tutorial 04 — Fungsi](./TUTORIAL_04_FUNGSI.md)**
