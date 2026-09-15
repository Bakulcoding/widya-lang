# Tutorial 13 — Cargo Test Suite Rust (60+ Test Files)

**Target**: Menjalankan & menambahkan test suite Rust dengan pattern `r##" inline Widya script inside #[test]` — production grade CI validation.
**Level**: Menengah ⭐⭐
**Waktu**: ~20 menit
**File contoh**: [tests/test_all9layer_core.rs](../tests/test_all9layer_core.rs) | [tests/test_all9layer_full_stack.rs](../tests/test_all9layer_full_stack.rs) | [tests/test_widyadb_full_stack.rs](../tests/test_widyadb_full_stack.rs)

---

## 1. Gambaran Struktur Test Suite Widya

Folder `tests/` berisi **60+ files** `test_*.rs`. Setiap file = 1 test suite domain, berisi 4-24 `#[test]` functions.

Total coverage per domain:

| Kategori | Jumlah Test File | Contoh File |
|----------|-----------------|-------------|
| **9 Layer Universal Stack** | 2 files | test_all9layer_core.rs (8 tests), test_all9layer_full_stack.rs (14 tests) |
| **WidyaOS** | 1 file | test_widyaos_full_stack.rs (24+ tests) |
| **WidyaDB Multi-Modal** | 2 files | test_widyadb_core.rs, test_widyadb_full_stack.rs |
| **Dasar bahasa & stdlib** | 1 file | widya_tests.rs (lexer/parser/interpreter dasar) |
| **Industri vertikal** | 50+ files | test_fhir, test_canbus, test_hl7, test_scada, test_ros2, test_ais, test_etcs, test_5g, test_space TOTAL 50+ |

---

## 2. Cara Menjalankan Cargo Test (Perintah Praktis)

```bash
# ============== DASAR ==============
# Jalankan SEMUA test suite (60+ files, 300+ #[test])
cargo test

# Lihat output per test (tidak capture stdout)
cargo test -- --nocapture

# ============== FILTER BY FILE ==============
# Hanya test 9 Layer Universal Stack (22 passed target)
cargo test --test test_all9layer_core --test test_all9layer_full_stack

# Hanya test WidyaOS
cargo test --test test_widyaos_full_stack

# Hanya test WidyaDB
cargo test --test test_widyadb_core --test test_widyadb_full_stack

# ============== FILTER BY NAMA TEST FUNCTION ==============
cargo test idt_isr                # Test dengan nama mengandung "idt_isr"
cargo test cuda -- --nocapture    # Semua test mengandung CUDA, plus lihat stdout
cargo test widydb_spatial         # Test spatial GIS WidyaDB

# ============== PARALEL / CONCURRENCY ==============
cargo test -- --test-threads=8    # 8 thread parallel (default = CPU core count)
cargo test -- --test-threads=1    # Serial (untuk test yang shared state / race)

# ============== CI MODE EXIT CODE KETAT ==============
cargo test -- --test-threads=4 --color=always 2>&1 | tee hasil_test.log
# Exit code != 0 → gagal di GitHub Actions CI (lihat .github/workflows/ci_cross_platform.yml)
```

---

## 3. Pattern Test Widya Inside Rust (WAJIB — 100% Sesuai Production)

⚠️ **4 aturan WAJIB di setiap test baru** — jika salah satu dilanggar, test bisa compile error atau false negative.

### Aturan 1 — Import WAJIB di TOP setiap file .rs
```rust
use widya::jalankan;
use widya::value::Value;
```

### Aturan 2 — DELIMITER SELALU `r##" ..... "##` (DOUBLE HASH)
Jangan pernah pakai single hash `r#"`. Karena single hash sering ketemu pattern `"#` di inline string Widya (marker byte array, column format code, dll) → delimiter **premature close** → compile error. Double hash hanya tertutup dengan string `"##` yang sangat jarang ada.

### Aturan 3 — SELALU `kembalikan benar;` di BARIS TERAKHIR inline Widya
Fungsi `jalankan()` mengembalikan `Result<Value, Error>`. Kita wajib kembalikan Bool true untuk membandingkan nanti dengan `Value::Bool(true)`.

### Aturan 4 — Pattern unwrap dengan debug eprintln agar mudah cari baris error
```rust
let res = jalankan(script_widya)
    .map_err(|e| { eprintln!("DBG NAMA_TEST: {:?}", e); e })
    .unwrap();
assert_eq!(res, Value::Bool(true));
```

---

## 4. Contoh Lengkap Test Baru (Step-by-Step)

Misal kita mau buat test untuk modul **`sgx_enclave_simulasi.wya`** — test **seal/unseal roundtrip**:

### Step 1 — Buat / Tambah Test di File `tests/test_9layer_extra.rs`
```rust
// file: tests/test_9layer_extra.rs
use widya::jalankan;
use widya::value::Value;

#[test]
fn test_sgx_seal_roundtrip_manual() {
    // Pattern = copy struktur & fungsi dari modul sgx_enclave_simulasi.wya + assertion
    let script = r##"
        // ===== INLINE: SHA256 helper =====
        fungsi sha256_hex(s) {
            misal hash_arr = [0;16];
            misal i = 0;
            selama i < panjang(s) {
                hash_arr[i % 16] = (hash_arr[i % 16] + kode_karakter(s, i)) % 256;
                i = i + 1;
            }
            misal hex_tbl = "0123456789abcdef";
            misal out = "";
            i = 0;
            selama i < 16 {
                misal b = hash_arr[i];
                out = out + potong(hex_tbl, (b >> 4) & 15, (b >> 4) & 15 + 1);
                out = out + potong(hex_tbl, b & 15, b & 15 + 1);
                i = i + 1;
            }
            kembalikan out;
        }
        struktur SGXEnclave { id, state, sealed_data_map,
            fungsi seal_data(kunci, plain) {
                ini.sealed_data_map[kunci] = "SEALED_" + ini.id + "_" + plain;
                kembalikan ini.sealed_data_map[kunci];
            },
            fungsi unseal_data(kunci) {
                misal v = ini.sealed_data_map[kunci];
                misal prefix = "SEALED_" + ini.id + "_";
                kembalikan potong(v, panjang(prefix), panjang(v));
            }
        }
        // ===== ACTUAL TEST =====
        misal e = SGXEnclave("E100", "INITIALIZED_SECURE", {});
        misal rahasia = "RAHASIA_WIDYA_123";
        misal cipher = e.seal_data("key_001", rahasia);
        misal plain_balik = e.unseal_data("key_001");
        pastikan(panjang(sha256_hex("test")) == 64);
        pastikan(plain_balik == rahasia);
        kembalikan benar;
    "##;

    let hasil = jalankan(script)
        .map_err(|e| { eprintln!("DBG test_sgx_seal_roundtrip: {:?}", e); e })
        .unwrap();
    assert_eq!(hasil, Value::Bool(true));
}
```

### Step 2 — Jalankan test spesifik:
```bash
cargo test test_sgx_seal_roundtrip_manual -- --nocapture
```

### Step 3 — Jika gagal?
- Eror `DBG test_sgx_seal_roundtrip: ...` muncul di eprintln — copy body inline script ke file `_debug.wya` lalu jalankan `widya jalankan _debug.wya` → akan diberi LINE NUMBER dan NAMA VAR yang undefined → fix, lalu paste kembali.
- Pola ini 4x lebih cepat ketimbang debug blind dari cargo test output saja.

---

## 5. Ringkasan Perbandingan: Inline Widya di .rs VS `widya jalankan file.wya`

| Aspek | `widya jalankan X.wya` (Scripting test) | `cargo test` (Rust inline test) |
|-------|-----------------------------------------|----------------------------------|
| Kecepatan write / prototipe | ⚡ Super cepat | Lambat (harus pattern r## + import) |
| Line number saat galat | Akurat (baris file persis) | Tidak akurat (line relatif terhadap inline string, harus extract ke debug file) |
| CI integration | Butuh script wrapper bash/PowerShell | Native `cargo test` → GitHub Actions native support, exit code otomatis |
| Coverage measurement | Manual (count assertions) | Tools: `cargo tarpaulin`, `grcov` → line/branch coverage % otomatis |
| Cocok untuk | Self-test modul terpisah (9 modul L2/L3/L6 widya jalankan exit0) | Release gate / aggregate minimum (contoh: ≥22 passed 0 failed AC11) |

---

## 6. Target Acceptance Criteria (AC) Release Gate — Contoh Production

Dari spec widya_semua_layer_100persen AC-11:
```
≥ 22 passed (8 core + 14 full_stack), 0 failed.
```
Command verifikasi resmi:
```bash
cargo test --test test_all9layer_core --test test_all9layer_full_stack 2>&1 | Select-Object -Last 40
```
Output harus mengandung persis:
```
running 8 tests  ... ok. 8 passed; 0 failed.
running 14 tests ... ok. 14 passed; 0 failed.
```

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 12 — Kompilasi 6 Target](./TUTORIAL_12_KOMPILASI_6_TARGET.md) |
➡️ **[Tutorial 14 — Makro & Ownership](./TUTORIAL_14_MAKRO_OWNERSHIP.md)**
