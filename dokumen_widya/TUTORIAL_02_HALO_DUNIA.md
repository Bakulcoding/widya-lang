# Tutorial 02 — Halo Dunia, Variabel & Tipe Data

**Target**: Memahami sintaks dasar Widya, variabel mutable, konstanta, tipe data built-in, dan operator.
**Level**: Pemula ⭐
**Waktu**: ~20 menit
**File contoh lengkap**: [01_halo_dunia.wya](../contoh/01_halo_dunia.wya)

---

## 1. Program Pertama: Halo Dunia

Buat file `halo.wya` lalu isi:
```widya
cetak("Halo Dunia dari Widya-Lang! 🇮🇩");
```

Jalankan:
```bash
widya jalankan halo.wya
```

Output:
```
Halo Dunia dari Widya-Lang! 🇮🇩
```

> 💡 Fungsi `cetak()` bisa menerima banyak argumen, tipe apapun, dan otomatis di-join dengan spasi:
> ```widya
> cetak("Nilai a =", 42, "| b =", 3.14, "| ok?", benar);
> ```

---

## 2. Komentar

```widya
// Komentar satu baris

/*
  Komentar
  banyak
  baris
*/
```

---

## 3. Variabel Mutable (`misal`) & Konstanta (`tetap`)

```widya
// ➡️ Variabel BISA diubah (mutable)
misal nama = "Budi";
cetak("Nama awal:", nama);
nama = "Budi Santoso";
cetak("Nama setelah diubah:", nama);

// ➡️ Konstana TIDAK BISA diubah (immutable)
tetap PHI = 3.14159;
tetap NAMA_APLIKASI = "Sistem Widya";
tetap VERSI_MAYOR = 1;

// PHI = 4.0;  // ❌ GALAT: konstanta tidak bisa di-assign ulang
```

**Aturan praktis**: Pakai `tetap` SEBANYAK MUNGKIN. Pakai `misal` hanya jika memang perlu di-reassign.

---

## 4. Tipe Data Built-In Widya (6 Tipe Dasar)

| Tipe | Kata Kunci | Contoh Nilai | Keterangan |
|------|-----------|--------------|-----------|
| **Teks (String)** | `teks` (UTF-8) | `"Halo"`, `"🇮🇩"`, `'a'` | Unicode penuh, bisa emoji |
| **Angka Bulat (Integer)** | `angka` (64-bit) | `42`, `-100`, `0xDEADBEEF` | Otomatis arbitrary precision jika overflow |
| **Angka Desimal (Float)** | `angka` (64-bit) | `3.14`, `1.5e10` | IEEE 754 double |
| **Boolean (Logika)** | `logika` | `benar`, `salah` | Setara `true`/`false` |
| **Nihil (Null/None)** | `nihil` | `nihil` | Tidak ada nilai |
| **Larik (Array)** | `larik` | `[1,2,3]`, `["a","b"]` | Koleksi terurut, heterogen diperbolehkan |
| **Kamus (Dict/Map)** | `kamus` | `{"nama": "Andi", "umur": 20}` | Key-value, key biasanya teks |

Fungsi `tipe(nilai)` mengembalikan nama tipe sebagai teks:
```widya
cetak(tipe("Widya"));     // "teks"
cetak(tipe(42));          // "angka"
cetak(tipe(benar));       // "logika"
cetak(tipe([1,2,3]));     // "larik"
cetak(tipe({"a":1}));     // "kamus"
cetak(tipe(nihil));       // "nihil"
```

> 🔮 **Struktur Objek kustom** (`struktur`) dibahas di Tutorial 06. Tipe `Hasil` (Result) & `Pilihan` (Option) di Tutorial 07.

---

## 5. Operator Aritmatika

```widya
misal a = 20;
misal b = 6;

cetak(a + b);   // 26  (tambah)
cetak(a - b);   // 14  (kurang)
cetak(a * b);   // 120 (kali)
cetak(a / b);   // 3.3333333333 (bagi, selalu float jika tidak habis)
cetak(a % b);   // 2   (modulo / sisa bagi)
cetak(a ^ b);   // 64000000? Tidak — tunggu, operator PEMANGKATAN = **
cetak(2 ** 8);  // 256 (pangkat: 2^8)
cetak(a // b);  // 3 (pembagian bulat floor)
```

**Precedensi operator** sama dengan matematika biasa: `**` tertinggi → `* / % //` → `+ -`. Pakai tanda kurung untuk eksplisit:
```widya
cetak((2 + 3) * 4);   // 20 (bukan 14)
```

---

## 6. Operator Perbandingan & Logika

```widya
misal x = 10;
misal y = 20;

// Perbandingan
cetak(x == y);   // salah  (sama)
cetak(x != y);   // benar  (tidak sama)
cetak(x < y);    // benar  (kurang dari)
cetak(x >= 10);  // benar  (lebih dari sama dengan)

// Logika Boolean
cetak(benar DAN salah);   // salah
cetak(benar ATAU salah);  // benar
cetak(BUKAN benar);       // salah
```

> 💡 **Short-circuit**: Ekspresi `A DAN B` tidak mengevaluasi B jika A sudah salah. `A ATAU B` tidak mengevaluasi B jika A sudah benar.

---

## 7. Manipulasi Teks (String)

```widya
misal s1 = "Widya";
misal s2 = "Lang";

cetak(s1 + " " + s2);      // "Widya Lang" (konkatenasi)
cetak(panjang(s1));        // 5 (panjang string)
cetak(huruf_kecil("WIDYA")); // "widya"
cetak(huruf_besar("lang"));  // "LANG"
cetak(potong("WidyaLang", 0, 5));  // "Widya" (substring: start inklusif, end eksklusif)
cetak(ke_teks(42));        // "42" (konversi angka ke teks)
cetak(ke_angka("3.14"));   // 3.14 (konversi teks ke angka)
cetak(pisah("a,b,c", ","));  // ["a","b","c"] (split string)
cetak(gabung(["X","Y"], "-")); // "X-Y" (join array dengan delimiter)
```

---

## 🎯 Latihan Mini (Self-Check)

Buat file `latihan02.wya`, lalu tulis kode Widya untuk:
1. Buat konstanta `JUMLAH_SISWA = 32`
2. Buat variabel `nilai_ujian = 78.5`
3. Cetak teks: `"Di kelas X ada 32 siswa. Rata-rata ujian = 78.5"` (pakai parameter berganda ke `cetak()`)
4. Hitung `(nilai_ujian * 1.1) - 5` lalu cetak hasilnya
5. Periksa apakah hasil langkah 4 ≥ 80 — cetak boolean-nya

Jalankan dan pastikan tidak ada galat.

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 01 — Instalasi](./TUTORIAL_01_INSTALASI.md) |
➡️ **[Tutorial 03 — Kontrol Alur](./TUTORIAL_03_KONTROL_ALUR.md)**
