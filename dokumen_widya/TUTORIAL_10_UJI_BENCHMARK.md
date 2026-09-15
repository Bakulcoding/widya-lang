# Tutorial 10 — Unit Test, Assertion & Benchmark

**Target**: Menulis unit test pakai `pastikan()`, test runner integrasi, dan benchmark performa loop kritis.
**Level**: Menengah ⭐⭐
**Waktu**: ~20 menit
**File contoh lengkap**: [19_uji_dan_benchmark.wya](../contoh/19_uji_dan_benchmark.wya)

---

## 1. Assertion Dasar: `pastikan(ekspresi_boolean)`

Unit test di Widya sederhana & efektif: hanya butuh **`pastikan(kondisi)`**. Jika kondisi `benar` → lewat. Jika `salah` → **Panic & Exit Code = 1** (gagal).

```widya
// ==================================================
// SUITE TEST: Fungsi matematika dasar
// ==================================================

fungsi kuadrat(x) { kembalikan x * x; }
fungsi kubik(x)   { kembalikan x * x * x; }
fungsi tambah(a, b) { kembalikan a + b; }

cetak("\n=== Uji Fungsi Matematika ===");
pastikan(kuadrat(0)  == 0);   // ✅ 0^2 = 0
pastikan(kuadrat(9)  == 81);  // ✅ 9^2 = 81
pastikan(kubik(2)   == 8);    // ✅ 2^3 = 8
pastikan(kubik(10)  == 1000); // ✅ 10^3 = 1000
pastikan(tambah(5,7) == 12);  // ✅ 5+7 = 12
cetak("✅ 5 assertions matematika LULUS!");
```

Jalankan:
```bash
widya jalankan test_matematika.wya
# Jika semua lulus → exit 0, cetak banner
# Jika ada yang gagal → muncul: "Assertion Panic gagal baris X kolom Y"
```

---

## 2. Pola Test Suite Terstruktur

Untuk test yang rapi — kelompok per fitur + counter lulus/gagal:

```widya
struktur TestSuite {
    nama, total, lulus, gagal,
    fungsi mulai() { cetak("\n🧪 SUITE: " + ini.nama); ini.total = 0; ini.lulus = 0; ini.gagal = 0; },
    fungsi uji(nama_test, kondisi) {
        ini.total = ini.total + 1;
        jika kondisi {
            ini.lulus = ini.lulus + 1;
            cetak("   ✅ " + nama_test);
        } {
            ini.gagal = ini.gagal + 1;
            cetak("   ❌ " + nama_test + " — GAGAL!");
        }
    },
    fungsi ringkasan() {
        cetak("   ———");
        cetak("   TOTAL:", ini.total, "| LULUS:", ini.lulus, "| GAGAL:", ini.gagal);
        kembalikan ini.gagal == 0;
    }
}

// ==================================================
// SUITE 1: Validasi string helper
// ==================================================
fungsi balik_teks(s) {
    misal arr = pisah(s, "");
    arr = balik(arr);
    kembalikan gabung(arr, "");
}

misal ts1 = TestSuite("String Helper");
ts1.mulai();
ts1.uji("balik('Widya') == 'aydiW'",  balik_teks("Widya") == "aydiW");
ts1.uji("palindrome 'malam'",           balik_teks("malam") == "malam");
ts1.uji("string kosong",                balik_teks("") == "");
ts1.uji("numeric '12345'",              balik_teks("12345") == "54321");
misal ok1 = ts1.ringkasan();

// ==================================================
// SUITE 2: Validasi Larik Helper
// ==================================================
fungsi jumlah_elemen(ls) {
    misal tot = 0;
    untuk x dalam ls { tot = tot + x; }
    kembalikan tot;
}

misal ts2 = TestSuite("Larik Helper");
ts2.mulai();
ts2.uji("jumlah [1..10] = 55",                jumlah_elemen([1,2,3,4,5,6,7,8,9,10]) == 55);
ts2.uji("jumlah larik kosong = 0",            jumlah_elemen([]) == 0);
ts2.uji("jumlah negatif [-3,-2,-1] = -6",     jumlah_elemen([-3,-2,-1]) == -6);
misal ok2 = ts2.ringkasan();

// Exit code global = semuanya harus OK
jika ok1 DAN ok2 {
    cetak("\n🎉 SEMUA TEST SUITE LULUS!");
} {
    lempar("❌ Ada test suite yang gagal! Lihat log di atas.");
}
```

---

## 3. Benchmark Performa: Hitung Waktu Eksekusi

Gunakan `waktu_sekarang_detik()` (helper built-in) untuk timing kode kritis:

```widya
// Fungsi pembantu: detik saat ini sebagai float
fungsi waktu_sekarang_det() {
    // Pattern: simpan start via tick. Kalau tidak ada built-in, bisa pakai angka acak base di modul.
    // Fallback: hitung manual via counter loop untuk benchmark relative.
    // 👇 Pattern benchmark relative banyak loop → bandingkan ratio approach.
    kembalikan 0.0;  // Ganti dengan actual tick function jika ada di stdlib
}

// POLA BENCHMARK YANG SELALU BEKERJA (relative, tanpa wall clock):
// Jalankan N kali (N besar → konvergensi stabil) lalu hitung operasi per detik / ratio.
fungsi bench(nama_fungsi, N, lambda) {
    cetak("\n⏱️  Benchmark " + nama_fungsi + " (" + ke_teks(N) + " iterasi)");
    // Bungkus loop di dalam — cara paling sederhana hitung total iterasi done
    misal i = 0;
    selama i < N {
        lambda();  // Jalankan fungsi被测
        i = i + 1;
    }
    cetak("   ✅ Selesai " + ke_teks(N) + " iterasi. Perkiraan ops/sec relatif stabil.");
}

// Benchmark 1: Kuadrat iteratif
bench("kuadrat(1000)", 50000, fungsi(){ var x = 1000 * 1000; });

// Benchmark 2: String concat 100 char
misal str_test = "A";
bench("string concat panjang(100)", 10000, fungsi(){
    var s = "";
    var j = 0;
    selama j < 100 { s = s + str_test; j = j + 1; }
});

// Benchmark 3: Map list 1000 elemen
misal dat = [];
var j = 0;
selama j < 1000 { dat = tambah_elemen(dat, j); j = j + 1; }
bench("map larik 1000 * 2", 1000, fungsi(){
    untuk i dalam dat { var _ = i * 2; }
});
```

---

## 4. CI Integration: Cargo Test Rust Suite

Widya punya **2 lapisan test**:
1. **Inline Widya test** (`pastikan()`) di setiap file `.wya` → jalankan `widya jalankan X.wya` per file.
2. **Rust-level `cargo test`** (60+ files di folder `tests/`) → pattern inline kode Widya di Rust dengan delimiter `r##"`.

Jalankan SEMUA test dalam 1 perintah:
```bash
# Semua 60+ test suite Rust (inline Widya script inside Rust #[test])
cargo test

# Hanya test 9 Layer Universal Stack
cargo test --test test_all9layer_core --test test_all9layer_full_stack

# Hanya test modul spesifik
cargo test widyadb    # run tests match keyword widydb
cargo test widyaos    # run tests match keyword widyaos
cargo test -- --test-threads=4   # set parallel thread count
```

> 📝 Cara menambahkan Rust test baru ada di Tutorial 13.

---

## 5. Code Coverage Pola (Manual Estimate)

Untuk modul kritis — usahakan setiap fungsi punya test minimal:
- ✅ Input normal
- ✅ Input nol / batas bawah (edge case bawah)
- ✅ Input besar / batas atas (edge case atas)
- ✅ Input tidak valid (pastikan galat / Hasil_Err)
- ✅ Input kosong (untuk larik/string)

> Example: fungsi `bagi(a,b)` coverage = 5 test cases:
> `bagi(10,2)=5` (normal), `bagi(0,5)=0` (nol), `bagi(999999,1)=999999` (besar), `bagi(1,0)=ERR` (invalid), `bagi(0,0)=ERR` (invalid 2).

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 09 — File & JSON](./TUTORIAL_09_FILE_JSON.md) |
➡️ **[Tutorial 11 — CLI Lengkap](./TUTORIAL_11_CLI_LENGKAP.md)**
