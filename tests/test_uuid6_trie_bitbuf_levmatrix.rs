use widya::jalankan;
use widya::value::Value;

#[test]
fn test_widya_uuid_v6() {
    let code = r#"
        misal id1 = uuid_v6();
        misal ts1 = uuid_v6_ke_timestamp(id1);
        misal comb = uuid_ke_comb(id1, ts1);
        kembalikan [panjang(id1) == 36, ts1 > 0, panjang(comb) == 36];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::Bool(true));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_trie_prefix() {
    let code = r#"
        misal trie = PohonPrefix();
        trie_sisip(trie, "widya", "bahasa");
        trie_sisip(trie, "widuri", "bunga");
        trie_sisip(trie, "wisata", "liburan");

        misal c_widya = trie_cari(trie, "widya");
        misal c_robot = trie_cari(trie, "robot");
        misal saran = trie_cocok_awalan(trie, "wid", 5);
        misal total = trie_jumlah_kata(trie);

        kembalikan [c_widya, c_robot, saran, total];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::String("bahasa".to_string()));
        assert_eq!(items[1], Value::Nil);
        if let Value::Array(s) = &items[2] {
            let matches = s.borrow();
            assert_eq!(matches.len(), 2);
            assert_eq!(matches[0], Value::String("widuri".to_string()));
            assert_eq!(matches[1], Value::String("widya".to_string()));
        } else {
            panic!("Expected array of matches");
        }
        assert_eq!(items[3], Value::Number(3.0));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_bit_buffer() {
    let code = r#"
        misal bb = BitBuffer();
        bitbuf_tulis_bit(bb, 25, 5); // 0b11001
        bitbuf_tulis_bit(bb, 6, 3);   // 0b110
        bitbuf_tulis_bit(bb, 65, 8);  // 0b01000001 = 'A'

        misal p = bitbuf_panjang_bit(bb);
        misal b1 = bitbuf_baca_bit(bb, 5);
        misal b2 = bitbuf_baca_bit(bb, 3);
        misal b3 = bitbuf_baca_bit(bb, 8);

        kembalikan [p, b1, b2, b3];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(16.0));
        assert_eq!(items[1], Value::Number(25.0));
        assert_eq!(items[2], Value::Number(6.0));
        assert_eq!(items[3], Value::Number(65.0));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_levenshtein_matrix() {
    let code = r#"
        misal m = levenshtein_matriks("cat", "cut");
        misal ops = levenshtein_langkah_operasi("cat", "cut");
        misal rasio = levenshtein_rasio_kemiripan("cat", "cut");

        kembalikan [m[3][3], panjang(ops), rasio];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(1.0));
        assert_eq!(items[1], Value::Number(3.0));
        assert_eq!(items[2], Value::Number(0.6667));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}
