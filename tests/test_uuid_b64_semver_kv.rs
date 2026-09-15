use widya::jalankan;
use widya::value::Value;

#[test]
fn test_uuid_dan_base64() {
    let code = r#"
        misal v4 = buat_uuid_v4();
        misal v7 = buat_uuid_v7();
        
        tetap cek_v4 = validasi_uuid(v4);
        tetap cek_v7 = validasi_uuid(v7);
        tetap cek_bad = validasi_uuid("sembarangan");

        misal detail_v7 = urai_uuid(v7);

        misal teks_asli = "Widya-Lang 2026";
        misal b64 = enkode_base64(teks_asli);
        misal b64_url = enkode_base64_url(teks_asli);
        tetap dekod = dekode_base64(b64);

        misal h = enkode_hex("halo");
        tetap dekod_h = dekode_hex(h);

        kembalikan [
            cek_v4,
            cek_v7,
            cek_bad,
            detail_v7.valid,
            detail_v7.versi == 7,
            dekod == teks_asli,
            dekod_h == "halo"
        ];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::Bool(false));
        assert_eq!(items[3], Value::Bool(true));
        assert_eq!(items[4], Value::Bool(true));
        assert_eq!(items[5], Value::Bool(true));
        assert_eq!(items[6], Value::Bool(true));
    } else {
        panic!("Harus mengembalikan array");
    }
}

#[test]
fn test_semver_dan_kvstore() {
    let code = r#"
        misal u = urai_semver("1.2.3-rc.1");
        tetap cmp_lt = bandingkan_semver("1.0.0", "1.0.1");
        tetap cmp_gt = bandingkan_semver("2.0.0", "1.9.9");
        tetap cmp_eq = bandingkan_semver("1.2.3", "1.2.3");

        tetap m_caret = cocokkan_semver("1.2.5", "^1.2.0");
        tetap m_tilde = cocokkan_semver("1.3.0", "~1.2.0");
        tetap m_gte = cocokkan_semver("2.0.0", ">=1.0.0");

        misal kv = PenyimpananKV();
        kv_pasang(kv, "app:nama", "WidyaApp");
        kv_pasang(kv, "app:versi", "1.0.0");
        kv_pasang(kv, "user:1", "John");

        tetap v_nama = kv_ambil(kv, "app:nama");
        tetap ink = kv_inkremen(kv, "counter", 10);
        tetap v_counter = kv_ambil(kv, "counter");

        misal pindai = kv_pindai_awalan(kv, "app:");
        tetap len_pindai = panjang(pindai);

        tetap hapus_sukses = kv_hapus(kv, "user:1");
        tetap v_user_after = kv_ambil(kv, "user:1");

        kembalikan [
            u.valid,
            u.mayor == 1,
            u.minor == 2,
            u.tambalan == 3,
            cmp_lt == -1,
            cmp_gt == 1,
            cmp_eq == 0,
            m_caret,
            m_tilde,
            m_gte,
            v_nama == "WidyaApp",
            ink == 10,
            v_counter == 10,
            len_pindai == 2,
            hapus_sukses,
            v_user_after == nihil
        ];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::Bool(true));
        assert_eq!(items[3], Value::Bool(true));
        assert_eq!(items[4], Value::Bool(true));
        assert_eq!(items[5], Value::Bool(true));
        assert_eq!(items[6], Value::Bool(true));
        assert_eq!(items[7], Value::Bool(true));
        assert_eq!(items[8], Value::Bool(false));
        assert_eq!(items[9], Value::Bool(true));
        assert_eq!(items[10], Value::Bool(true));
        assert_eq!(items[11], Value::Bool(true));
        assert_eq!(items[12], Value::Bool(true));
        assert_eq!(items[13], Value::Bool(true));
        assert_eq!(items[14], Value::Bool(true));
        assert_eq!(items[15], Value::Bool(true));
    } else {
        panic!("Harus mengembalikan array");
    }
}
