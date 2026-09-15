# Tutorial 14 — Makro Hygienic & Borrow Checker (Rust-style Ownership)

**Target**: Memakai makro untuk code generation DRY & memahami ownership Widya yang mencegah double-free / use-after-move.
**Level**: Menengah ⭐⭐
**Waktu**: ~20 menit
**File contoh lengkap**: [18_makro_dan_ownership.wya](../contoh/18_makro_dan_ownership.wya) | [src/borrow_checker.rs](../src/borrow_checker.rs)

---

## Bagian 1: Hygienic Macro (Makro)

### Apa itu Hygienic Macro Widya?
Makro Widya bekerja seperti `macro_rules!` Rust: **code template yang di-expand SEMUA di waktu kompilasi (compile-time)**, 0 runtime overhead. Disebut **hygienic** karena variabel di dalam body makro TIDAK AKAN bertabrakan / shadow variabel di call site scope.

### Bentuk Dasar: `makro nama_makro(parameter) { template }`

```widya
// Makro untuk ASSERT_EQ dengan pesan otomatis (mirip assert_eq! Rust)
makro pastikan_sama(aktual, ekspektasi, pesan) {
    jika aktual != ekspektasi {
        cetak("❌ TEST GAGAL:", pesan);
        cetak("   Aktual   :", aktual);
        cetak("   Ekspektasi:", ekspektasi);
        lempar("AssertionEQ gagal");
    } {
        cetak("✅ LULUS:", pesan, "->", aktual);
    }
}

// PAKAI — expand inline di call site
misal x = 5 + 5;
pastikan_sama(x, 10, "5 + 5 harus 10");

misal nama = "Widya";
pastikan_sama(huruf_besar(nama), "WIDYA", "uppercase cocok");
```

### Contoh Makro: Auto Generate Getter/Setter Struktur (DRY)

```widya
// Buat makro @getter_setter: otomatis bikin 2 method per field
makro getter_setter(tipe_struk, nama_field, tipe_field) {
    fungsi tipe_struk + "_ambil_" + nama_field(objek) {
        kembalikan objek[nama_field];
    }
    fungsi tipe_struk + "_setel_" + nama_field(objek, nilai_baru) {
        // runtime type check
        objek[nama_field] = nilai_baru;
    }
}

// Apply ke struktur Pengguna:
struktur Pengguna { id, nama, email, password_hash }

// Expand 3 getter + 3 setter = 6 fungsi tanpa tulis manual:
@getter_setter(Pengguna, id, angka)
@getter_setter(Pengguna, nama, teks)
@getter_setter(Pengguna, email, teks)

misal u = Pengguna(7, "Eka", "eka@widya.id", "sha256$abc");
cetak(Pengguna_ambil_nama(u));     // "Eka"
Pengguna_setel_email(u, "eka_baru@widya.id");
cetak(Pengguna_ambil_email(u));    // "eka_baru@widya.id"
```

### Contoh Makro: `waktukan!` Benchmark Otomatis
```widya
makro waktukan(nama_tes, N_iter, blok_kode) {
    misal start_mikro = 0;
    misal i = 0;
    selama i < N_iter {
        blok_kode;
        i = i + 1;
    }
    cetak("⏱️ [" + nama_tes + "] selesai " + ke_teks(N_iter) + " iterasi.");
}

// Pakai
waktukan("kuadrat 1000", 50000, { misal y = 0; y = y + 1000 * 1000; });
```

> 🔑 **Aturan Makro**:
> 1. Ekspansi SEBELUM interpretasi / type check.
> 2. HANYA ekspresi / statement sederhana.
> 3. Nama variabel di dalam body makro di-prefix dengan ID makro → tidak tabrakan scope call site (hygiene).

---

## Bagian 2: Ownership & Borrow Checker

Widya meniru **Ownership model Rust** untuk 3 tujuan:
1. **0 Garbage Collector** → deterministik memory cleanup.
2. **No double-free**: suatu objek hanya punya PEMILIK TEPAT 1.
3. **No use-after-move**: setelah objek di-*move*, tidak bisa diakses lagi.

### 3 Aturan Emas Ownership Widya

#### Aturan 1: Setiap Nilai Punya Tepat Satu Owner
```widya
misal s1 = "String besar 10 MB";  // s1 = owner
misal s2 = s1;                     // ⚠️ MOVE (bukan copy!) — s2 JADI owner
cetak(s1);  // ❌ GALAT: use after move! s1 sudah TIDAK VALID lagi (di-fungsikan)
cetak(s2);  // ✅ OK, s2 adalah owner baru yang sah
```

#### Aturan 2: Ketika Owner Keluar Scope → Nilai Di-drop Otomatis (free memory)
```widya
{
    misal buffer = [0.0; 1000000];  // 1 juta float (~8 MB)
    // pakai buffer di sini...
}   // ✅ KURUNG TUTUP: buffer otomatis di-drop. 8 MB langsung dikembalikan ke OS.
cetak(buffer);  // ❌ GALAT: buffer sudah di-drop (scope habis)
```

#### Aturan 3: Borrow = Pinjam Sementara (Tidak Transfer Ownership)

Ada 2 jenis pinjaman:
- **Pinjaman Baca (immutable borrow `&`)**: Bisa banyak pinjaman sekaligus. Tidak boleh ubah nilai.
- **Pinjaman Tulis (mutable borrow `&mut`)**: MAKS 1 pinjaman saja (exclusive write). Tidak boleh ada pinjaman baca bersamaan.

```widya
struktur DataSensor { id, nilai }

fungsi baca_data(d &DataSensor) {     // PINJAM BACA: tidak punya ownership
    kembalikan d.nilai;               // ✅ OK, baca diizinkan
    // d.nilai = d.nilai + 1;         // ❌ GALAT: tidak bisa tulis di borrow immutable
}

fungsi ubah_data(d &mut DataSensor) { // PINJAM TULIS: exclusive
    d.nilai = d.nilai + 100;          // ✅ OK
}

// Penggunaan
misal sensor = DataSensor("S1", 27.5);
cetak(baca_data(&sensor));            // ✅ Pinjam baca → "27.5"
ubah_data(&mut sensor);               // ✅ Pinjam tulis → nilai jadi 127.5
cetak(baca_data(&sensor));            // ✅ Pinjam baca lagi → 127.5
// ✅ ownership tetap di SENSOR asli, fungsi tidak pernah ambil alih kepemilikan
```

### Pola Aman: Copy Trait untuk Tipe Kecil
Tipe primitif (angka, boolean, char kecil) otomatis **Copy** — tidak pernah move, malah di-copy byte-by-byte:

```widya
misal a = 42;
misal b = a;   // ✅ COPY (bukan move), karena `angka` trait = Copy
cetak(a);      // ✅ OK (a masih valid, tidak di-move)
cetak(b);      // ✅ OK (b adalah salinan independen)
```

| Tipe | Default Behavior |
|------|-----------------|
| angka, boolean, char ≤ 4 byte | **Copy** |
| string panjang, larik besar, struktur object besar | **Move** (ownership transfer) |
| string kecil (< 24 byte) | SSO = Small String Optimization → Copy juga |

---

## 🎯 Latihan Mini Gabungan

1. **Buat makro `@sql_table(NAMA_TABEL, kolom1, kolom2, kolom3)`** yang otomatis generate:
   - `fungsi NAMA_TABEL_baris(kol1, kol2, kol3)`
   - `fungsi NAMA_TABEL_insert(db, baris)`
   - `fungsi NAMA_TABEL_cari_by_id(db, id_cari)`

2. **Contoh ownership**: Tulis kode Widya yang melanggar Aturan Emas No 3 (pinjam tulis bersamaan dengan pinjam baca) — lalu perbaiki dengan cara scope kurung tambahan untuk mengakhiri pinjam tulis lebih dulu.

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 13 — Cargo Test Suite](./TUTORIAL_13_CARGO_TEST.md) |
➡️ **[Tutorial 15 — FFI C & Bare-Metal GPIO](./TUTORIAL_15_FFI_BAREMETAL.md)**
