use widya::jalankan;
use widya::value::Value;

#[test]
fn test_widya_stats() {
    let code = r#"
        misal data = [10, 20, 20, 40, 50, 60, 70, 80, 90];
        misal mean = statistik_rata_rata(data);
        misal med = statistik_median(data);
        misal modus = statistik_modus(data);
        misal q1 = statistik_kuartil(data, 1);
        misal z = statistik_zscore([10, 20, 30]);

        kembalikan [mean, med, modus, q1, z];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(48.8889));
        assert_eq!(items[1], Value::Number(50.0));
        assert_eq!(items[2], Value::Number(20.0));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_markov() {
    let code = r#"
        misal m = RantaiMarkov(1);
        markov_latih_teks(m, "halo dunia halo kawan halo semuanya");
        misal next_halo = markov_status_berikutnya(m, "halo");
        kembalikan next_halo;
    "#;
    let res = jalankan(code).unwrap();
    if let Value::String(s) = res {
        assert!(s == "dunia" || s == "kawan" || s == "semuanya");
    } else {
        panic!("Expected String, got {:?}", res);
    }
}

#[test]
fn test_widya_bencode() {
    let code = r#"
        misal payload = {"kunci": "nilai", "angka": 42};
        misal enc = bencode_enkode(payload);
        misal is_valid = bencode_validasi(enc);
        misal dec = bencode_dekode(enc);

        kembalikan [enc, is_valid, dec["kunci"], dec["angka"]];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::String("nilai".to_string()));
        assert_eq!(items[3], Value::Number(42.0));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_simhash() {
    let code = r#"
        misal t1 = "kucing lucu makan ikan segar";
        misal t2 = "kucing lucu makan ikan lezat";
        misal t3 = "pesawat terbang tinggi di angkasa luas";

        misal mirip12 = simhash_apakah_mirip(t1, t2, 10);
        misal mirip13 = simhash_apakah_mirip(t1, t3, 5);

        kembalikan [mirip12, mirip13];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(false));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}
