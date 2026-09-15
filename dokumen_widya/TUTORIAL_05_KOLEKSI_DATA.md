# Tutorial 05 — Koleksi Data (Larik & Kamus)

**Target**: Mahir mengoperasikan array (larik) dan dict (kamus), termasuk pencarian, penyortiran, filter/map/reduce.
**Level**: Pemula ⭐
**Waktu**: ~25 menit
**File contoh lengkap**: [03_koleksi_data.wya](../contoh/03_koleksi_data.wya)

---

## 1. Larik (Array) — Operasi Dasar

### Inisialisasi
```widya
misal kosong = [];
misal angka = [3, 1, 4, 1, 5, 9, 2, 6];
misal campuran = ["teks", 42, benar, 3.14, [1,2,3]];  // heterogen diperbolehkan
```

### Akses Elemen (0-Indexed)
```widya
cetak(angka[0]);   // 3 (elemen PERTAMA)
cetak(angka[5]);   // 9
cetak(angka[100]); // nihil (jika index out of range → tidak crash, return nihil)

// Ubah elemen via assignment
angka[0] = 999;
cetak(angka);  // [999, 1, 4, 1, 5, 9, 2, 6]
```

### Panjang & Keberadaan Elemen
```widya
cetak(panjang(angka));             // 8
cetak(apakah_ada(angka, 9));       // benar (cari nilai 9)
cetak(apakah_ada(angka, 9999));    // salah
cetak(index_dari(angka, 5));       // 4 (posisi 5 ada di index 4)
```

### Tambah / Hapus Elemen
```widya
// ➕ Tambah di AKHIR
misal daftar = ["a", "b"];
daftar = tambah_elemen(daftar, "c");
cetak(daftar);   // ["a","b","c"]

// ➕ Sisipkan di POSISI TERTENTU
daftar = sisip_elemen(daftar, 1, "X");
cetak(daftar);   // ["a","X","b","c"]

// ➖ Hapus berdasarkan INDEX
daftar = hapus_index(daftar, 0);  // hapus elemen index 0 ("a")
cetak(daftar);   // ["X","b","c"]

// ➖ Hapus berdasarkan NILAI (pertama yang ketemu)
daftar = hapus_nilai(daftar, "b");
cetak(daftar);   // ["X","c"]
```

---

## 2. Penyortiran & Pembalikan Larik

```widya
misal nilai = [73, 92, 85, 68, 95, 88, 71];

// Urut menaik (ASC)
cetak(urutkan(nilai));       // [68, 71, 73, 85, 88, 92, 95]

// Urut menurun (DESC) — pakai urutkan + balik
cetak(balik(urutkan(nilai))); // [95, 92, 88, 85, 73, 71, 68]

// Pembalikan saja (tanpa sort)
misal kata = ["saya", "suka", "Widya"];
cetak(balik(kata));  // ["Widya", "suka", "saya"]
```

---

## 3. Irisan (Slice) Larik

```widya
misal huruf = ["a","b","c","d","e","f","g"];

// potong(larik, start_inclusive, end_exclusive)
cetak(potong(huruf, 0, 3));   // ["a","b","c"]
cetak(potong(huruf, 2, 5));   // ["c","d","e"]
cetak(potong(huruf, 4, 100)); // ["e","f","g"] (out of range → sampai akhir)
```

---

## 4. Kamus (Dictionary / Hash Map / Associative Array)

### Inisialisasi & Akses
```widya
misal user = {
    "id": 101,
    "nama_lengkap": "Ratna Dewi",
    "email": "ratna@widya.ai",
    "umur": 28,
    "aktif": benar,
    "roles": ["admin", "editor"]
};

// Akses dengan key
cetak(user["nama_lengkap"]);   // "Ratna Dewi"
cetak(user["umur"]);           // 28
cetak(user["roles"]);          // ["admin","editor"]
cetak(user["tidak_ada"]);      // nihil (key tidak ada → aman, tidak crash)

// Ubah / tambah field
user["umur"] = 29;
user["alamat"] = "Jakarta Selatan";
```

### Daftar Key & Value
```widya
cetak(kunci_kamus(user));   // ["id", "nama_lengkap", "email", "umur", "aktif", "roles", "alamat"]
cetak(nilai_kamus(user));   // [101, "Ratna Dewi", "ratna@widya.ai", 29, benar, ["admin","editor"], "Jakarta Selatan"]
```

### Hapus Key
```widya
user = hapus_kunci(user, "alamat");
cetak(kunci_kamus(user));  // alamat hilang
```

---

## 5. Pola Umum: Map, Filter, Reduce (Manual & Idiomatik)

### 🔹 Map — Transformasi Setiap Elemen
```widya
fungsi map_larik(ls, f) {
    misal out = [];
    untuk x dalam ls { out = tambah_elemen(out, f(x)); }
    kembalikan out;
}

misal nilai_ujian = [60, 70, 80, 90];
misal nilai_naik_10persen = map_larik(nilai_ujian, fungsi(n){ kembalikan n * 1.1; });
cetak(nilai_naik_10persen);  // [66, 77, 88, 99]
```

### 🔹 Filter — Hanya Ambil Yang Lolos Kriteria
```widya
fungsi filter_larik(ls, lolos) {
    misal out = [];
    untuk x dalam ls { jika lolos(x) { out = tambah_elemen(out, x); } }
    kembalikan out;
}

misal produk = [
    {"nama": "Laptop", "harga": 12000000},
    {"nama": "Mouse",  "harga": 250000},
    {"nama": "HP",     "harga": 8000000},
    {"nama": "Kabel",  "harga": 50000}
];

misal produk_dibawah_1jt = filter_larik(produk, fungsi(p){ kembalikan p["harga"] < 1000000; });
// → [Mouse 250rb, Kabel 50rb]
```

### 🔹 Reduce — Agregasi Jadi Satu Nilai
```widya
fungsi reduce_larik(ls, awal, operasi) {
    misal akumulator = awal;
    untuk x dalam ls { akumulator = operasi(akumulator, x); }
    kembalikan akumulator;
}

misal total_harga = reduce_larik(produk, 0, fungsi(acc, p){ kembalikan acc + p["harga"]; });
cetak("Total semua produk:", total_harga);  // 20300000
```

---

## 🎯 Latihan Mini

1. **Kamus Frekuensi Kata**: Diberikan `teks = "widya lang widya rust widya os widya db"`, buat kamus yang menghitung berapa kali setiap kata muncul. Jawaban: `{"widya":4, "lang":1, "rust":1, "os":1, "db":1}`.

2. **Sorting Struct-style array**: Dari `produk` (section 5 filter), **URUTKAN berdasarkan harga DESC** (termahal di atas) — HP (8jt), Laptop (12jt → WRONG, perbaiki: **Laptop 12jt > HP 8jt > Mouse > Kabel**).

3. **Grouping umur**:
   ```widya
   misal orang = [
       {"nama":"A","umur":15},
       {"nama":"B","umur":24},
       {"nama":"C","umur":17},
       {"nama":"D","umur":30},
       {"nama":"E","umur":28}
   ];
   ```
   Buat kamus: `{"anak": [A,C], "dewasa": [B,D,E]}` (anak = umur <18, dewasa = >=18).

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 04 — Fungsi](./TUTORIAL_04_FUNGSI.md) |
➡️ **[Tutorial 06 — OOP & Struktur](./TUTORIAL_06_OOP_STRUKTUR.md)**
