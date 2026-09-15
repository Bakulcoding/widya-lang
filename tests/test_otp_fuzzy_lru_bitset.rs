use widya::jalankan;
use widya::value::Value;

#[test]
fn test_otp_dan_fuzzy() {
    let code = r##"
        tetap secret = "JBSWY3DPEHPK3PXP";
        tetap code_totp = buat_totp(secret);
        tetap verif_ok = verifikasi_totp(code_totp, secret);
        tetap verif_bad = verifikasi_totp("999999", secret);

        tetap dist = jarak_levenshtein("kucing", "kucing");
        tetap dist2 = jarak_levenshtein("kitten", "sitting");

        tetap sim = kemiripan_teks("widya", "widya");
        tetap sim_part = kemiripan_teks("widya", "widy");

        tetap cands = ["widya", "python", "rust", "aljabar"];
        tetap matches = cari_kata_mirip("widy", cands, 0.5);

        kembalikan [
            verif_ok,
            verif_bad == salah,
            dist == 0,
            dist2 == 3,
            sim == 1.0,
            sim_part == 0.8,
            panjang(matches) >= 1,
            matches[0].kata == "widya"
        ];
    "##;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        for item in items.iter() {
            assert_eq!(*item, Value::Bool(true));
        }
    } else {
        panic!("Harus mengembalikan array");
    }
}

#[test]
fn test_lru_dan_bitset() {
    let code = r##"
        misal c = CacheLRU(2);
        lru_pasang(c, "x", 10);
        lru_pasang(c, "y", 20);
        
        tetap v_x = lru_ambil(c, "x");
        lru_pasang(c, "z", 30); // y digusur karena x baru saja diakses

        tetap v_y_after = lru_ambil(c, "y");
        tetap v_x_after = lru_ambil(c, "x");
        tetap v_z_after = lru_ambil(c, "z");

        misal b1 = HimpunanBit(16);
        misal b2 = HimpunanBit(16);

        bit_pasang(b1, 0, benar);
        bit_pasang(b1, 1, benar);
        bit_pasang(b2, 1, benar);
        bit_pasang(b2, 2, benar);

        tetap c_b1 = bit_hitung_aktif(b1);
        misal band = bit_operasi_dan(b1, b2);
        misal bor = bit_operasi_atau(b1, b2);
        misal bxor = bit_operasi_xor(b1, b2);

        kembalikan [
            v_x == 10,
            v_y_after == nihil,
            v_x_after == 10,
            v_z_after == 30,
            c_b1 == 2,
            bit_hitung_aktif(band) == 1,
            bit_hitung_aktif(bor) == 3,
            bit_hitung_aktif(bxor) == 2,
            bit_ambil(band, 1) == benar,
            bit_ambil(band, 0) == salah
        ];
    "##;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        for item in items.iter() {
            assert_eq!(*item, Value::Bool(true));
        }
    } else {
        panic!("Harus mengembalikan array");
    }
}
