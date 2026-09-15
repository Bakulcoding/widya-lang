use widya::jalankan;
use widya::value::Value;

#[test]
fn test_avl_and_geohash() {
    let code = r#"
        var t = PohonAVL()
        avl_sisip(t, "10", "A")
        avl_sisip(t, "20", "B")
        avl_sisip(t, "30", "C")
        var cari_b = avl_cari(t, "20")
        var h = avl_tinggi(t)

        var g = geohash_enkode(-6.175392, 106.827153, 6)
        var dec = geohash_dekode(g)
        var nbr = geohash_tetangga(g)
        kembalikan [cari_b, h <= 2, g, dec.lintang < 0.0];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::String("B".to_string()));
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[3], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_cincinhash_and_toml() {
    let code = r#"
        var c = CincinHash(50)
        cincin_tambah_node(c, "node1")
        cincin_tambah_node(c, "node2")
        var target = cincin_dapatkan_node(c, "my_cache_key")

        var tomlStr = "[server]\nhost = \"localhost\"\nport = 8080\n"
        var parsed = toml_parse(tomlStr)
        var port = toml_ambil_nilai(parsed, "server.port")
        kembalikan [target != nihil, port];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Number(8080.0));
    } else {
        panic!("Expected array result");
    }
}
