use widya::jalankan;
use widya::value::Value;

#[test]
fn test_hlc_causality() {
    let code = r#"
        var hlc1 = JamHLC("node-A")
        var hlc2 = JamHLC("node-B")

        var ts1 = hlc_detak(hlc1)
        var ts2 = hlc_perbarui(hlc2, ts1)
        var ts3 = hlc_detak(hlc2)

        var c1 = hlc_apakah_sebelum(ts1, ts2)
        var c2 = hlc_apakah_sebelum(ts2, ts3)
        var fmt = hlc_format(ts3)

        kembalikan c1 && c2
    "#;
    let res = jalankan(code).expect("HLC execution failed");
    match res {
        Value::Bool(b) => assert!(b, "Causality tracking in HLC should maintain partial order"),
        _ => panic!("Expected boolean result"),
    }
}

#[test]
fn test_quadtree_spatial_partitioning() {
    let code = r#"
        // 100x100 space with capacity 2 per node
        var qt = PohonQuad(0, 0, 100, 100, 2)
        quad_sisip(qt, 10, 10, "Unit-1")
        quad_sisip(qt, 12, 15, "Unit-2")
        quad_sisip(qt, 50, 50, "Unit-3")
        quad_sisip(qt, 90, 90, "Unit-4")

        var total = quad_hitung(qt)
        var radius_units = quad_kueri_lingkaran(qt, 10, 10, 10.0)
        var point_match = quad_kueri_titik(qt, 50, 50)

        kembalikan [total, panjang(radius_units), panjang(point_match)]
    "#;
    let res = jalankan(code).expect("QuadTree execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::Number(4.0));
            assert_eq!(items[1], Value::Number(2.0));
            assert_eq!(items[2], Value::Number(1.0));
        }
        _ => panic!("Expected array result"),
    }
}

#[test]
fn test_shamir_secret_sharing_threshold() {
    let code = r#"
        var rahasia = "KUNCI-SUPER-RAHASIA-2026"
        // 3-of-5 threshold scheme
        var shares = shamir_bagi_rahasia(rahasia, 3, 5)

        // Ambil 3 share saja (indeks 0, 2, 4)
        var subset_3 = [shares[0], shares[2], shares[4]]
        var pulih_3 = shamir_gabung_rahasia(subset_3)

        // Ambil 4 share (indeks 1, 2, 3, 4)
        var subset_4 = [shares[1], shares[2], shares[3], shares[4]]
        var pulih_4 = shamir_gabung_rahasia(subset_4)

        kembalikan [pulih_3, pulih_4]
    "#;
    let res = jalankan(code).expect("Shamir execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::String("KUNCI-SUPER-RAHASIA-2026".to_string()));
            assert_eq!(items[1], Value::String("KUNCI-SUPER-RAHASIA-2026".to_string()));
        }
        _ => panic!("Expected array result"),
    }
}

#[test]
fn test_snappy_compression_and_decompression() {
    let code = r#"
        var teks = "WIDYA_SNAPPY_STREAMING_COMPRESSION_FRAME_LEVEL_SPEED_TEST_2026_WIDYA_SNAPPY_STREAMING_COMPRESSION_FRAME_LEVEL_SPEED_TEST_2026"
        var comp = snappy_kompres(teks)
        var decomp = snappy_dekompres(comp)
        var rasio = snappy_rasio_hemat(teks)

        kembalikan [comp, decomp, rasio > 0.2]
    "#;
    let res = jalankan(code).expect("Snappy execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            let original = "WIDYA_SNAPPY_STREAMING_COMPRESSION_FRAME_LEVEL_SPEED_TEST_2026_WIDYA_SNAPPY_STREAMING_COMPRESSION_FRAME_LEVEL_SPEED_TEST_2026";
            assert_eq!(items[1], Value::String(original.to_string()));
            assert_eq!(items[2], Value::Bool(true));
        }
        _ => panic!("Expected array result"),
    }
}

