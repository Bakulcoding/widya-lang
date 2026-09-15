use widya::jalankan;
use widya::value::Value;

#[test]
fn test_vector_clock_causality_and_concurrency() {
    let code = r#"
        var node_ids = ["node_A", "node_B", "node_C"]
        var ja = JamVektor("node_A", node_ids)
        var jb = JamVektor("node_B", node_ids)

        // Tick ja
        ja = vclock_detak(ja)
        ja = vclock_detak(ja)

        // Merge to jb and tick jb
        jb = vclock_gabung(jb, ja)
        jb = vclock_detak(jb)

        // Cek causal precedence: ja happened before jb
        var urutan = vclock_bandingkan(ja, jb)
        var konflik = vclock_apakah_konflik(ja, jb)

        // Buat jc independen
        var jc = JamVektor("node_C", node_ids)
        jc = vclock_detak(jc)

        // jb vs jc harus konflik/konkuren
        var konflik_bc = vclock_apakah_konflik(jb, jc)

        kembalikan [urutan, konflik, konflik_bc]
    "#;

    let res = jalankan(code).expect("Vector clock execution failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        assert_eq!(b[0], Value::String("SEBELUM".to_string()));
        assert_eq!(b[1], Value::Bool(false));
        assert_eq!(b[2], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_radix_tree_longest_prefix_match() {
    let code = r#"
        var pohon = PohonRadix()
        radix_sisip(pohon, "192.168.", "LAN_GATEWAY")
        radix_sisip(pohon, "192.168.1.", "SUBNET_OFFICE")
        radix_sisip(pohon, "192.168.1.50", "HOST_PRINTER")

        var m1 = radix_cari_awalan_terpanjang(pohon, "192.168.1.50")
        var m2 = radix_cari_awalan_terpanjang(pohon, "192.168.1.200")
        var m3 = radix_cari_awalan_terpanjang(pohon, "192.168.99.1")

        var exact = radix_cari(pohon, "192.168.1.50")
        var none = radix_cari(pohon, "10.0.0.1")

        kembalikan [m1["nilai"], m2["nilai"], m3["nilai"], exact, none == nil]
    "#;

    let res = jalankan(code).expect("Radix tree execution failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        assert_eq!(b[0], Value::String("HOST_PRINTER".to_string()));
        assert_eq!(b[1], Value::String("SUBNET_OFFICE".to_string()));
        assert_eq!(b[2], Value::String("LAN_GATEWAY".to_string()));
        assert_eq!(b[3], Value::String("HOST_PRINTER".to_string()));
        assert_eq!(b[4], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_blake3_hashing_and_kdf() {
    let code = r#"
        var data = "Halo Dunia Widya-Lang"
        var h1 = blake3_hash(data)
        var h2 = blake3_hash(data)
        var h3 = blake3_hash("Data Lain")

        var kunci = "kunci_rahasia_32_byte_widya_lang"
        var mac1 = blake3_keyed_hash(data, kunci)
        var mac2 = blake3_keyed_hash(data, kunci)

        var kdf1 = blake3_turunkan_kunci("konteks.app.1", "secret_material_123")
        var kdf2 = blake3_turunkan_kunci("konteks.app.2", "secret_material_123")

        kembalikan [h1 == h2, h1 != h3, panjang(h1) == 64, mac1 == mac2, mac1 != h1, kdf1 != kdf2]
    "#;

    let res = jalankan(code).expect("BLAKE3 execution failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        for item in b.iter() {
            assert_eq!(*item, Value::Bool(true));
        }
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_run_length_codec_compression() {
    let code = r#"
        var teks_asli = "AAAAABBBBBBBCCCCCCCCCCDDDDDDDDDDDDDDDDDDDD"
        var comp = rle_kompres(teks_asli)
        var decomp = rle_dekompres(comp)

        var rasio = rle_rasio_hemat(comp)
        var sama = (decomp["teks"] == teks_asli)

        kembalikan [sama, comp["panjang_terkompresi"] < comp["panjang_asli"], rasio > 50.0]
    "#;

    let res = jalankan(code).expect("RLE execution failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        assert_eq!(b[0], Value::Bool(true));
        assert_eq!(b[1], Value::Bool(true));
        assert_eq!(b[2], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}
