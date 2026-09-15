# Tutorial 09 — File I/O & Serialization JSON

**Target**: Baca/tulis file teks, serialisasi/de-serialisasi JSON, pattern dump/load state aplikasi.
**Level**: Menengah ⭐⭐
**Waktu**: ~15 menit
**File contoh lengkap**: [15_ekosistem_canggih.wya](../contoh/15_ekosistem_canggih.wya), [09_aplikasi_flutter.wya](../contoh/09_aplikasi_flutter.wya)

---

## 1. Tulis File Teks: `tulis_berkas(path, isi)`

```widya
// Tulis file di relative path (dari CWD terminal widya dijalankan)
misal path = "hasil_laporan.txt";
misal isi_laporan = 
    "=== LAPORAN PENJUALAN HARI INI ===\n" +
    "Tanggal : 2026-09-15\n" +
    "Item    : 42 buah\n" +
    "Total   : Rp 3.750.000\n";

tulis_berkas(path, isi_laporan);
cetak("✅ File tersimpan di:", path);
```

> 💡 **Permission**: Folder tujuan harus sudah ada dan user punya write access. Jika folder tidak ada → galat. Buat folder terlebih dahulu via OS / script installer.

---

## 2. Baca File Teks: `baca_berkas(path)`

```widya
misal isi = baca_berkas("hasil_laporan.txt");
cetak("=== Isi file yang dibaca ===");
cetak(isi);
cetak("Panjang file:", panjang(isi), "karakter");
```

Jika file tidak ada → return `nihil` (tidak crash tapi nilai nihil — bisa cek):
```widya
misal config = baca_berkas("config.tidak_ada.wya");
jika config == nihil {
    cetak("⚠️ Config tidak ditemukan — pakai default!");
    config = "port=8080\nhost=0.0.0.0\n";
}
```

---

## 3. Serialisasi: Object Widya → Teks JSON: `ke_json(nilai)`

`ke_json` bekerja untuk: teks, angka, boolean, nihil, larik, kamus, DAN struktur OOP — otomatis konversi ke field dict:

```widya
// Kasus 1: Array sederhana
misal angka = [1, 1, 2, 3, 5, 8, 13];
misal json1 = ke_json(angka);
cetak("JSON array:", json1);  // "[1,1,2,3,5,8,13]"

// Kasus 2: Dict / Kamus bersarang
misal data_aplikasi = {
    "versi": "1.0.0",
    "nama_app": "Widya POS",
    "pelanggan_aktif": 128,
    "pengaturan": {
        "bahasa": "id-ID",
        "mode_malam": benar,
        "max_history": 500
    },
    "produk_terlaris": [
        {"id":1, "nama":"Kopi Hitam", "harga":5000},
        {"id":2, "nama":"Roti Bakar",  "harga":15000}
    ]
};
misal json2 = ke_json(data_aplikasi);
cetak("Panjang JSON data aplikasi:", panjang(json2), "karakter");
tulis_berkas("data_aplikasi_state.json", json2);
```

---

## 4. De-serialisasi: Teks JSON → Object: `dari_json(teks)`

```widya
// Baca JSON string tadi
misal json_dari_file = baca_berkas("data_aplikasi_state.json");
misal state_loaded = dari_json(json_dari_file);

// Akses field kembali seperti semula!
cetak("App version setelah load:", state_loaded["versi"]);
cetak("Mode malam?", state_loaded["pengaturan"]["mode_malam"]);
cetak("Produk terlaris #2:", state_loaded["produk_terlaris"][1]["nama"]);  // "Roti Bakar"
```

### Validasi Setelah Load
Selalu validasi data hasil `dari_json` (JSON dari luar tidak bisa dipercaya):
```widya
fungsi state_valid(obj) {
    jika obj == nihil { kembalikan salah };
    jika tipe(obj) != "kamus" { kembalikan salah };
    jika obj["versi"] == nihil { kembalikan salah };
    jika obj["pengaturan"] == nihil { kembalikan salah };
    jika obj["produk_terlaris"] == nihil { kembalikan salah };
    kembalikan benar;
}

jika state_valid(state_loaded) {
    cetak("✅ State valid!");
} {
    cetak("❌ State korup! Pakai state default kosong.");
}
```

---

## 5. Pola Lengkap: Save / Load Session Aplikasi

```widya
struktur SessionApp {
    username, last_login, role, cart_items, theme,
    fungsi dump_json() {
        kembalikan ke_json({
            "u": ini.username,
            "ll": ini.last_login,
            "r": ini.role,
            "c": ini.cart_items,
            "t": ini.theme
        });
    }
}

fungsi session_load(path) {
    misal raw = baca_berkas(path);
    jika raw == nihil { kembalikan SessionApp("tamu", "2026-01-01", "guest", [], "cerah"); }
    misal d = dari_json(raw);
    kembalikan SessionApp(d["u"], d["ll"], d["r"], d["c"], d["t"]);
}

fungsi session_simpan(s, path) {
    tulis_berkas(path, s.dump_json());
    cetak("💾 Session tersimpan:", path);
}

// Pakai
misal sesi = SessionApp("dinda", "2026-09-15T14:30:00", "admin", [{"id":9}], "gelap");
session_simpan(sesi, "sesi_dinda.json");

// Simulasi restart app: load dari file
misal sesi_reload = session_load("sesi_dinda.json");
cetak("👋 Selamat kembali,", sesi_reload.username, "(" + sesi_reload.role + ") — Theme:", sesi_reload.theme);
```

---

## ⚠️ Keamanan File I/O

1. **Jangan pernah `dari_json()` dari input user yang tidak terverifikasi**. Selalu validasi skema.
2. **Jangan tulis credential (password, API key, private key) ke JSON plaintext**. Jika perlu, enkripsi terlebih dahulu (lihat modul `privasi_keamanan.wya`).
3. **Path traversal attack**: Jika path berasal dari input user, pastikan tidak mengandung `../` yang bisa keluar folder kerja.

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 08 — Impor Modul](./TUTORIAL_08_IMPOR_MODUL.md) |
➡️ **[Tutorial 10 — Uji & Benchmark](./TUTORIAL_10_UJI_BENCHMARK.md)**
