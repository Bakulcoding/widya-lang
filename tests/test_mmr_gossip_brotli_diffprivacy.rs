use widya::jalankan;
use widya::value::Value;

#[test]
fn test_mmr_and_gossip() {
    let code = r#"
        var mmr = MerkleMountainRange()
        mmr_tambah(mmr, "tx1")
        mmr_tambah(mmr, "tx2")
        mmr_tambah(mmr, "tx3")

        var len = mmr_panjang(mmr)
        var root = mmr_bag_peaks(mmr)
        var proof = mmr_buat_bukti(mmr, 1)

        var cluster = KlasterGossip(["nodeA", "nodeB", "nodeC"])
        gossip_siarkan(cluster, "nodeA", {"msg": "hello"})
        var delivered = gossip_detak(cluster)
        var stB = gossip_keadaan_node(cluster, "nodeB")

        kembalikan [len, panjang(root) == 64, proof.valid, delivered > 0.0, stB.status == "HIDUP"];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(3.0));
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::Bool(true));
        assert_eq!(items[3], Value::Bool(true));
        assert_eq!(items[4], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_brotli_and_diffprivacy() {
    let code = r#"
        var teks = "ABCDEF_ABCDEF_ABCDEF_GHIJKL_GHIJKL"
        var pkg = brotli_kompres(teks)
        var dec = brotli_dekompres(pkg)
        var rasio = brotli_rasio_hemat(teks)

        var noise = dp_laplace_noise(1.0)
        var noisy_count = dp_kueri_hitung(100.0, 0.5)
        var noisy_avg = dp_kueri_rata_rata([10.0, 20.0, 30.0, 40.0], 1.0, 0.0, 50.0)

        kembalikan [dec == teks, rasio >= 0.0, noisy_count >= 0.0, noisy_avg >= 0.0];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::Bool(true));
        assert_eq!(items[3], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}
