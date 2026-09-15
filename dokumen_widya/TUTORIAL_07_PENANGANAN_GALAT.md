# Tutorial 07 — Penanganan Galat (Exception + Result Pattern)

**Target**: Menggunakan `coba/tangkap`, `lempar`, dan tipe `Hasil` / `Pilihan` (Rust-style Result/Option) untuk error handling idiomatik.
**Level**: Menengah ⭐⭐
**Waktu**: ~20 menit
**File contoh lengkap**: [07_penanganan_galat.wya](../contoh/07_penanganan_galat.wya)

---

## 1. Lempar Galat Pakai `lempar`

Gunakan `lempar` kapanpun kondisi tidak terpenuhi dan tidak bisa lanjut (fail-fast):

```widya
fungsi hitung_akar_kuadrat(n) {
    jika n < 0 {
        lempar("akarKuadratError: tidak bisa akar dari bilangan negatif: " + ke_teks(n));
    }
    // Metode Newton-Raphson sederhana
    misal tebakan = n / 2.0;
    misal iterasi = 0;
    selama iterasi < 20 {
        tebakan = 0.5 * (tebakan + n / tebakan);
        iterasi = iterasi + 1;
    }
    kembalikan tebakan;
}
```

---

## 2. Tangkap Galat Pakai `coba ... tangkap`

```widya
misal daftar_nilai = [16, -4, 0, 25, 100];

untuk n dalam daftar_nilai {
    coba {
        misal akar = hitung_akar_kuadrat(n);
        cetak("✅ akar(" + ke_teks(n) + ") =", akar);
    } tangkap pesan_galat {
        cetak("❌ Gagal hitung akar(" + ke_teks(n) + "):", pesan_galat);
    }
}
```

Output:
```
✅ akar(16) = 4
❌ Gagal hitung akar(-4): akarKuadratError: tidak bisa akar dari bilangan negatif: -4
✅ akar(0) = 0
✅ akar(25) = 5
✅ akar(100) = 10
```

---

## 3. Blok `akhirnya` (Finally) — Selalu Dijalankan

Gunakan `akhirnya` untuk cleanup (tutup file, bebaskan resource) yang harus berjalan SUKSES maupun GAGAL:

```widya
fungsi proses_dengan_file(path) {
    misal file_handle = {"status":"terbuka"};  // simulasi file dibuka
    coba {
        misal data = baca_berkas(path);
        jika panjang(data) == 0 { lempar("File kosong!"); }
        cetak("Proses data:", panjang(data), "byte");
    } tangkap e {
        cetak("⚠️ Galat:", e);
    } akhirnya {
        // PASTI dijalankan
        file_handle["status"] = "ditutup";
        cetak("🔒 File ditutup. Status:", file_handle["status"]);
    }
}
```

> 💡 `akhirnya` dijalankan BAHKAN jika blok `coba` ada `kembalikan` atau `berhenti` — cocok untuk resource cleanup.

---

## 4. Pola `Hasil` (Rust-style Result) — Tanpa Exception, Lebih Eksplisit

Untuk fungsi library yang pemanggilnya HARUS menangani, lebih baik **return enum-style dict** (OK/ERR) daripada lempar exception:

```widya
// Konstruksi pola Hasil:
//   OK  → {"ok": benar, "nilai": <value>}
//   ERR → {"ok": salah, "galat": <pesan error>}

fungsi Hasil_Ok(v)    { kembalikan {"ok": benar, "nilai": v}; }
fungsi Hasil_Err(msg) { kembalikan {"ok": salah, "galat": msg}; }

// Fungsi yang return Hasil (TIDAK PERNAH lempar!)
fungsi bagi_selamat(a, b) {
    jika b == 0 { kembalikan Hasil_Err("Pembagian dengan nol!"); };
    kembalikan Hasil_Ok(a / b);
}
```

### Cara Konsumsi Hasil
```widya
misal hasil = bagi_selamat(10, 3);

jika hasil["ok"] {
    cetak("✅ Hasil bagi:", hasil["nilai"]);
} {
    cetak("❌ Galat:", hasil["galat"]);
}

// Chaining multiple Hasil dengan pola cepat
misal h1 = bagi_selamat(100, 0);
misal h2 = jika h1["ok"] { bagi_selamat(h1["nilai"], 2) } { h1 };
cetak("Chaining:", jika h2["ok"] { "SUKSES" } { h2["galat"] });
```

---

## 5. Pola `Pilihan` (Rust-style Option) — Untuk Nilai Yang Mungkin Kosong

Biasa untuk query database, pencarian elemen — 2 kemungkinan: **Ada** atau **Tidak Ada**:

```widya
// Pola Pilihan:
//   ADA    → {"ada": benar, "nilai": v}
//   TIDAK  → {"ada": salah}

fungsi Pilihan_Ada(v)   { kembalikan {"ada": benar, "nilai": v}; }
fungsi Pilihan_Kosong() { kembalikan {"ada": salah}; }

fungsi cari_user_by_id(daftar_user, id_cari) {
    untuk u dalam daftar_user {
        jika u["id"] == id_cari { kembalikan Pilihan_Ada(u); }
    }
    kembalikan Pilihan_Kosong();
}

// Pakai
misal users = [{"id":1,"nama":"A"}, {"id":2,"nama":"B"}];
misal user1 = cari_user_by_id(users, 1);
misal user9 = cari_user_by_id(users, 999);

cetak(jika user1["ada"] { "Ditemukan: " + user1["nilai"]["nama"] } { "User tidak ditemukan" });  // A
cetak(jika user9["ada"] { "Ditemukan" } { "User 999 tidak ada" });  // Tidak ada
```

---

## 🎯 Kapan Pakai Yang Mana?

| Pola | Kapan Digunakan | Contoh |
|------|-----------------|--------|
| `lempar` + `coba/tangkap` | Error FATAL yang tidak bisa recover di dekat titik galat | Validasi input user salah; koneksi DB putus di tengah transaksi |
| `Hasil` (OK/ERR dict) | Fungsi library yang pemanggilnya wajib handle eksplisit | `bagi_selamat()`, `parse_json_aman()`, `http_get()` |
| `Pilihan` (ADA/KOSONG) | Kasus **nilai mungkin kosong itu normal** bukan error | `cari_by_id()`, `dapatkan_cache()`, `first_element_of_empty_array` |
| `akhirnya` | Cleanup resource apapun hasilnya | Tutup file/network handle; unlock mutex; rollback jika gagal |

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 06 — Struktur & OOP](./TUTORIAL_06_OOP_STRUKTUR.md) |
➡️ **[Tutorial 08 — Impor Modul](./TUTORIAL_08_IMPOR_MODUL.md)**
