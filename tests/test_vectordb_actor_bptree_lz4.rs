use widya::jalankan;
use widya::value::Value;

#[test]
fn test_vectordb_and_actorsystem() {
    let code = r#"
        var db = BasisDataVektor(3)
        vektor_sisip(db, "vec1", [1.0, 0.0, 0.0], {"kategori": "X"})
        vektor_sisip(db, "vec2", [0.0, 1.0, 0.0], {"kategori": "Y"})
        vektor_sisip(db, "vec3", [0.9, 0.1, 0.0], {"kategori": "X"})

        var count = vektor_hitung(db)
        var knn = vektor_kueri_knn(db, [1.0, 0.0, 0.0], 2)

        var sistem = SistemAktor("SistemUji")
        var aktor = aktor_spawn(sistem, "Aktor1", fungsi(a, p) {
            kembalikan p;
        })
        aktor_kirim(aktor, {"data": 123})
        aktor_kirim(aktor, {"data": 456})

        var mb_len = aktor_mailbox_panjang(aktor)
        var status = aktor_status(aktor)

        kembalikan [count, panjang(knn), mb_len, status];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(3.0));
        assert_eq!(items[1], Value::Number(2.0));
        assert_eq!(items[2], Value::Number(2.0));
        assert_eq!(items[3], Value::String("AKTIF".to_string()));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_bplustree_and_lz4() {
    let code = r#"
        var tree = PohonBPlus(4)
        bplus_sisip(tree, "k1", 100)
        bplus_sisip(tree, "k2", 200)
        bplus_sisip(tree, "k3", 300)

        var v2 = bplus_cari(tree, "k2")
        var rentang = bplus_rentang(tree, "k1", "k2")

        var data = "AAAAABBBBBCCCCCDDDDD"
        var pkg = lz4_kompres(data)
        var dec = lz4_dekompres(pkg)
        var rasio = lz4_rasio_hemat(data)

        kembalikan [v2, panjang(rentang), dec == data, rasio >= 0.0];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(200.0));
        assert_eq!(items[1], Value::Number(2.0));
        assert_eq!(items[2], Value::Bool(true));
        assert_eq!(items[3], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}
