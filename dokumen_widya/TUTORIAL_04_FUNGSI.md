# Tutorial 04 — Fungsi & Modularisasi

**Target**: Bisa membuat fungsi sendiri, memahami parameter, return value, scope, lambda/closure, dan fungsi tingkat tinggi.
**Level**: Pemula ⭐
**Waktu**: ~25 menit
**File contoh lengkap**: [04_tingkat_tinggi.wya](../contoh/04_tingkat_tinggi.wya)

---

## 1. Dasar Fungsi: `fungsi` + `kembalikan`

```widya
fungsi sapa(nama) {
    kembalikan "Halo, " + nama + "! 👋";
}

cetak(sapa("Dewi"));   // "Halo, Dewi! 👋"
cetak(sapa("Komang")); // "Halo, Komang! 👋"
```

### Tanpa Return Value (Prosedur)
Jika tidak ada `kembalikan` → fungsi mengembalikan `nihil`:
```widya
fungsi cetak_garis(panjang_garis) {
    misal i = 0;
    selama i < panjang_garis {
        cetak_tanpa_baris("=");
        i = i + 1;
    }
    cetak("");  // newline
}

cetak_garis(40);
cetak("Teks di tengah");
cetak_garis(40);
```

---

## 2. Banyak Parameter & Nilai Default

```widya
// Parameter dengan nilai default (jika tidak diisi → pakai default)
fungsi pangkat(angka, eksponen = 2) {
    misal hasil = 1;
    misal i = 0;
    selama i < eksponen {
        hasil = hasil * angka;
        i = i + 1;
    }
    kembalikan hasil;
}

cetak(pangkat(5));      // 25   (eksponen default 2)
cetak(pangkat(2, 10));  // 1024 (eksponen eksplisit 10)
```

> 💡 Nilai default parameter WAJIB berada DI AKHIR daftar parameter.

---

## 3. Fungsi Rekursif

Fungsi boleh memanggil DIRINYA SENDIRI:
```widya
fungsi faktorial(n) {
    jika n <= 1 { kembalikan 1 };
    kembalikan n * faktorial(n - 1);
}

cetak(faktorial(0));  // 1
cetak(faktorial(5));  // 120 (5*4*3*2*1)
cetak(faktorial(10)); // 3628800
```

> ⚠️ **Depth rekursif**: Untuk loop intensif lebih baik pakai `selama` / `untuk` (lebih hemat stack).

---

## 4. Scope Variabel

- **Variabel luar fungsi** (global): Bisa diakses dari dalam fungsi (jika dideklarasikan SEBELUM fungsi dipanggil).
- **Variabel dalam fungsi** (lokal): Hanya hidup di dalam blok fungsi, luar tidak tahu.
- **Shadowing**: Variabel lokal dengan nama sama menutupi variabel global (tidak disarankan).

```widya
misal GLOBAL_PI = 3.14;  // scope global

fungsi luas_lingkaran(r) {
    // r = scope parameter (lokal)
    misal luas = GLOBAL_PI * r * r;  // akses global oke
    kembalikan luas;
}

cetak(luas_lingkaran(7));  // 153.86
// cetak(luas);  // ❌ GALAT: variabel `luas` tidak dikenal (scope lokal sudah mati)
```

---

## 5. Lambda / Closure (Fungsi Tanpa Nama)

Widya mendukung **closure** — fungsi sebagai nilai (first-class citizen). Bisa disimpan ke variabel, dilempar sebagai parameter:

```widya
// Simpan lambda ke variabel
misal kuadrat = fungsi(x) { kembalikan x * x; };

cetak(kuadrat(9));  // 81

// Lambda yang me-referensi variabel luar = CLOSURE (ingat environment)
misal counter = 0;
misal naikkan = fungsi() {
    counter = counter + 1;
    kembalikan counter;
};

cetak(naikkan());  // 1
cetak(naikkan());  // 2
cetak(naikkan());  // 3
cetak("counter sekarang:", counter);  // 3
```

---

## 6. Fungsi Tingkat Tinggi (Higher-Order Functions)

Fungsi yang **menerima fungsi lain sebagai parameter**, atau **mengembalikan fungsi**:

```widya
// 1. Terima lambda sebagai parameter — `peta` (map)
fungsi peta(larik, transformasi) {
    misal hasil = [];
    untuk item dalam larik {
        hasil = tambah_elemen(hasil, transformasi(item));
    }
    kembalikan hasil;
}

misal angka = [1,2,3,4,5];
misal kuadrat_list = peta(angka, fungsi(x){ kembalikan x*x; });
cetak(kuadrat_list);  // [1, 4, 9, 16, 25]

misal nama_uppercase = peta(["andi", "budi"], fungsi(n){ kembalikan huruf_besar(n); });
cetak(nama_uppercase);  // ["ANDI", "BUDI"]

// 2. Mengembalikan fungsi (Closure Factory)
fungsi pembuat_penjumlah(penambah) {
    kembalikan fungsi(x) {
        kembalikan x + penambah;
    };
}

misal tambah_5 = pembuat_penjumlah(5);
misal tambah_100 = pembuat_penjumlah(100);

cetak(tambah_5(3));     // 8
cetak(tambah_100(23));  // 123
```

Fungsi tingkat tinggi lain yang umum:
```widya
// filter: ambil elemen yang lolos kriteria
fungsi filter(larik, lolos) {
    misal hasil = [];
    untuk x dalam larik {
        jika lolos(x) {
            hasil = tambah_elemen(hasil, x);
        }
    }
    kembalikan hasil;
}

misal genap = filter([1..10], fungsi(n){ kembalikan n%2==0; });  // catatan: range [1..10] bisa pakai helper
```

---

## 🎯 Latihan Mini

1. **Fungsi `cek_prima(n)`** yang mengembalikan `benar` jika n bilangan prima, selainnya `salah`. Test dengan `cek_prima(29)` → benar; `cek_prima(35)` → salah.
2. **Fungsi `rata_rata(larik_angka)`** return rata-ratanya. Contoh `rata_rata([70, 80, 90])` → 80.0.
3. **Fungsi `komposisi(f, g)`** return fungsi baru yang menjalankan `f(g(x))`. Test: `komposisi(fungsi(x){x*2}, fungsi(x){x+3})` dipanggil (5) → hasil `(5+3)*2 = 16`.

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 03 — Kontrol Alur](./TUTORIAL_03_KONTROL_ALUR.md) |
➡️ **[Tutorial 05 — Koleksi Data](./TUTORIAL_05_KOLEKSI_DATA.md)**
