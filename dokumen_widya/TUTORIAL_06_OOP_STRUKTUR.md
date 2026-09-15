# Tutorial 06 — Struktur & Pemrograman Berorientasi Objek (OOP)

**Target**: Menggunakan `struktur` untuk tipe data custom, method, constructor, dan pola OOP idiomatik Widya.
**Level**: Menengah ⭐⭐
**Waktu**: ~25 menit
**File contoh lengkap**: [06_oop_dan_struktur.wya](../contoh/06_oop_dan_struktur.wya)

---

## 1. Apa itu `struktur`?

`struktur` Widya = gabungan `struct` + `class` di bahasa lain. Bisa punya **field data** + **method (fungsi anggota)**.

### Bentuk Minimal
```widya
struktur Titik {
    x,  // field 1
    y   // field 2
}

// Constructor otomatis — nama fungsi = nama struktur, urut parameter sesuai field deklarasi
misal p = Titik(3, 4);
cetak("x:", p.x, "y:", p.y);   // x: 3 y: 4
```

---

## 2. Tambahkan Method ke Struktur

Method = `fungsi` di dalam badan `struktur`. Akses field sendiri via keyword **`ini`** (setara `this` / `self`).

```widya
struktur RekeningBank {
    pemilik,
    saldo,

    fungsi setor(jumlah) {
        ini.saldo = ini.saldo + jumlah;
        cetak("✅ Setor:", jumlah, "→ Saldo baru:", ini.saldo);
    }

    fungsi tarik(jumlah) {
        jika jumlah > ini.saldo {
            cetak("❌ Gagal tarik", jumlah, "- saldo kurang!");
            kembalikan salah;
        }
        ini.saldo = ini.saldo - jumlah;
        cetak("✅ Tarik:", jumlah, "→ Sisa:", ini.saldo);
        kembalikan benar;
    }

    fungsi info() {
        kembalikan "Rekening " + ini.pemilik + " — Rp " + ke_teks(ini.saldo);
    }
}

// Pakai
misal rek_budi = RekeningBank("Budi", 500000);
cetak(rek_budi.info());       // "Rekening Budi — Rp 500000"
rek_budi.setor(300000);       // Saldo jadi 800000
rek_budi.tarik(1000000);      // ❌ Gagal (1jt > 800rb)
rek_budi.tarik(250000);       // ✅ Sisa 550000
```

---

## 3. Encapsulation + Constructor Kustom

Widya tidak punya keyword `private` — tapi kita bisa pakai **factory function** (fungsi pembuat object) untuk validasi SEBELUM object terbentuk:

```widya
struktur Produk {
    id, nama, harga, stok,
    fungsi ringkasan() {
        kembalikan "#" + ke_teks(ini.id) + " " + ini.nama + " — Rp " + ke_teks(ini.harga);
    }
}

// Factory function: validasi dulu sebelum return object
fungsi produk_baru(id, nama, harga, stok) {
    jika harga < 0 { lempar "Harga produk tidak boleh negatif!" };
    jika stok < 0 { lempar "Stok tidak boleh negatif!" };
    jika panjang(nama) < 3 { lempar "Nama minimal 3 karakter!" };
    kembalikan Produk(id, nama, harga, stok);
}

misal p1 = produk_baru(1, "Laptop Gaming", 15000000, 12);
cetak(p1.ringkasan());  // ✅ OK

// Produk berikut akan GAGAL (lempar galat) karena validasi
// misal p2 = produk_baru(2, "Hp", -5000, -2);  // ❌ 3 validasi gagal
```

---

## 4. Composition > Inheritance

Widya mengutamakan **Komposisi** (object di-dalam object) — ini lebih fleksibel ketimbang inheritance class.

```widya
// Komponen 1
struktur Alamat {
    jalan, kota, provinsi,
    fungsi lengkap() { kembalikan ini.jalan + ", " + ini.kota + " - " + ini.provinsi; }
}

// Komponen 2
struktur Kontak { email, telepon }

// Komposisi: Pelanggan = Alamat + Kontak + data sendiri
struktur Pelanggan {
    kode, nama, alamat, kontak,
    fungsi label_kirim() {
        kembalikan "Kepada Yth. " + ini.nama + "\n" + ini.alamat.lengkap();
    }
}

// Bangun object via komposisi
misal pel = Pelanggan(
    "P-001",
    "Siti Rahmawati",
    Alamat("Jl. Mawar No. 7", "Surabaya", "Jawa Timur"),
    Kontak("siti@widya.ai", "0812-3456-7890")
);

cetak(pel.label_kirim());
cetak("Telp:", pel.kontak.telepon);
```

---

## 5. Pola "Trait" / Duck Typing di Widya

Jika dua struktur punya **nama method SAMA** — keduanya bisa dipakai secara polimorfik (tidak perlu keyword `interface`).

```widya
struktur Lingkaran { r, fungsi luas(){ kembalikan 3.14 * ini.r * ini.r; } }
struktur PersegiPanjang { p, l, fungsi luas(){ kembalikan ini.p * ini.l; } }
struktur Segitiga { a, t, fungsi luas(){ kembalikan 0.5 * ini.a * ini.t; } }

// Fungsi ini TERIMA APAPUN yang punya method `.luas()`
fungsi cetak_luas(bangun) {
    cetak("Luas bangunan =", bangun.luas());
}

cetak_luas(Lingkaran(7));             // 153.86
cetak_luas(PersegiPanjang(5, 8));    // 40
cetak_luas(Segitiga(6, 4));          // 12
```

> 🔑 **Kesimpulan OOP Widya**:
> - Pakai `struktur` untuk custom type
> - Akses field sendiri: `ini.nama_field`
> - Constructor default: `NamaStruktur(field1, field2, ...)`
> - Validasi sebelum object dibuat: pakai factory function
> - Sukai **Komposisi** bukan inheritance; Pakai **Duck Typing** untuk polimorfisme

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 05 — Koleksi Data](./TUTORIAL_05_KOLEKSI_DATA.md) |
➡️ **[Tutorial 07 — Penanganan Galat](./TUTORIAL_07_PENANGANAN_GALAT.md)**
