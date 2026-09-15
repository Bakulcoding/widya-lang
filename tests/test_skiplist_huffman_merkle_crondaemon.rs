use widya::jalankan;
use widya::value::Value;

#[test]
fn test_skiplist_and_huffman() {
    let code = r#"
        var sl = DaftarLompat(16)
        skiplist_sisip(sl, "kunci_1", "Nilai 1")
        skiplist_sisip(sl, "kunci_2", "Nilai 2")
        var cari = skiplist_cari(sl, "kunci_1")

        var teks = "ABRACADABRA"
        var pkg = huffman_kompres(teks)
        var dec = huffman_dekompres(pkg)
        var hemat = huffman_rasio_hemat(teks)

        kembalikan [cari, dec == teks, hemat > 0.0];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::String("Nilai 1".to_string()));
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_merkle_and_crondaemon() {
    let code = r#"
        var data = ["tx1", "tx2", "tx3", "tx4"]
        var tree = PohonMerkle(data)
        var root = merkle_root(tree)
        var proof = merkle_buat_bukti(tree, 1)
        var valid = merkle_verifikasi_bukti(root, "tx2", proof)

        var daemon = DaemonCron()
        daemon_cron_daftar(daemon, "task1", "* * * * *", fungsi(n, w) {
            kembalikan 1;
        })
        var run = daemon_cron_detak(daemon, 60)

        kembalikan [valid, panjang(run) == 1];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}
