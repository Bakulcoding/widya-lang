use widya::jalankan;
use widya::value::Value;

#[test]
fn test_widya_ring_buffer() {
    let code = r#"
        misal rb = RingBuffer(3);
        ring_dorong(rb, 10);
        ring_dorong(rb, 20);
        ring_dorong(rb, 30);
        misal p1 = ring_panjang(rb);
        misal f1 = ring_apakah_penuh(rb);

        ring_dorong(rb, 40); // Overwrite oldest (10)
        misal arr1 = ring_ke_array(rb);
        misal pop1 = ring_tarik(rb); // Pop 20
        misal peek1 = ring_lihat(rb); // Peek 30

        kembalikan [p1, f1, arr1, pop1, peek1];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(3.0));
        assert_eq!(items[1], Value::Bool(true));
        if let Value::Array(sub) = &items[2] {
            let s = sub.borrow();
            assert_eq!(s[0], Value::Number(20.0));
            assert_eq!(s[1], Value::Number(30.0));
            assert_eq!(s[2], Value::Number(40.0));
        } else {
            panic!("Expected array");
        }
        assert_eq!(items[3], Value::Number(20.0));
        assert_eq!(items[4], Value::Number(30.0));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_bitwise_ops() {
    let code = r#"
        misal rot_l = bit_rotasi_kiri(1, 4); // 16
        misal rot_r = bit_rotasi_kanan(16, 2); // 4
        misal pop = bit_hitung_satu(42); // 42 = 0b101010 -> 3 bits
        misal ext = bit_ekstrak(212, 2, 4); // 212 = 0b11010100 -> bits 2..5 = 0101 = 5
        kembalikan [rot_l, rot_r, pop, ext];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(16.0));
        assert_eq!(items[1], Value::Number(4.0));
        assert_eq!(items[2], Value::Number(3.0));
        assert_eq!(items[3], Value::Number(5.0));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_hyperloglog() {
    let code = r#"
        misal hll = HyperLogLog(8);
        untuk i dalam 0..100 {
            hll_tambah(hll, "user_" + teks(i % 10)); // 10 unique
        }
        misal est = hll_estimasi(hll);
        kembalikan est;
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Number(n) = res {
        // Estimation of 10 should be close to 10 (e.g. within [7, 13])
        assert!(n >= 7.0 && n <= 13.0, "HLL estimate {} should be near 10", n);
    } else {
        panic!("Expected Number, got {:?}", res);
    }
}

#[test]
fn test_widya_version_range() {
    let code = r#"
        misal c1 = semver_cocok_rentang("1.2.5", "^1.2.0");
        misal c2 = semver_cocok_rentang("2.0.0", "^1.2.0");
        misal c3 = semver_cocok_rentang("1.2.5", "~1.2.0");
        misal c4 = semver_cocok_rentang("1.3.0", "~1.2.0");

        misal pkts = ["1.0.0", "1.2.0", "1.2.8", "1.3.0", "2.0.0"];
        misal best = semver_pilih_terbaik(pkts, "^1.2.0");

        kembalikan [c1, c2, c3, c4, best];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(false));
        assert_eq!(items[2], Value::Bool(true));
        assert_eq!(items[3], Value::Bool(false));
        assert_eq!(items[4], Value::String("1.3.0".to_string()));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}
