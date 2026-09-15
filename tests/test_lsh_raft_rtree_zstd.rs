use widya::jalankan;
use widya::value::Value;

#[test]
fn test_lsh_and_raft() {
    let code = r#"
        var lsh = MinHashLSH(16, 0.5)
        var t1 = ["a", "b", "c", "d", "e"]
        var t2 = ["a", "b", "c", "d", "f"]
        var t3 = ["x", "y", "z"]

        lsh_sisip(lsh, "d1", t1)
        lsh_sisip(lsh, "d2", t2)
        lsh_sisip(lsh, "d3", t3)

        var q = ["a", "b", "c", "d"]
        var matches = lsh_kueri(lsh, q)
        var jaccard = lsh_estimasi_jaccard(lsh, t1, t2)

        var klaster = KlasterRaft(["n1", "n2", "n3"])
        var leader_awal = raft_dapatkan_leader(klaster)
        var res = raft_usulkan_perintah(klaster, "CMD_1")
        var next_leader = raft_detak_pemilihan(klaster)
        var st = raft_klaster_status(klaster)

        kembalikan [panjang(matches) >= 2, jaccard > 0.6, res.sukses, leader_awal != "", next_leader != "", st.term == 2.0];
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
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_rtree_and_zstd() {
    let code = r#"
        var rtree = PohonR(4)
        rtree_sisip(rtree, "box1", 0.0, 0.0, 10.0, 10.0, "Area 1")
        rtree_sisip(rtree, "box2", 20.0, 20.0, 30.0, 30.0, "Area 2")

        var hitung = rtree_hitung(rtree)
        var temukan = rtree_kueri_kotak(rtree, 5.0, 5.0, 15.0, 15.0)

        var data = "ZSTD_COMPRESSION_TEST_ZSTD_COMPRESSION_TEST_AAAAAAAAAAAAAA"
        var pkg = zstd_kompres(data)
        var dec = zstd_dekompres(pkg)
        var rasio = zstd_rasio_hemat(data)

        kembalikan [hitung, panjang(temukan), dec == data, rasio >= 0.0];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(2.0));
        assert_eq!(items[1], Value::Number(1.0)); // only box1 overlaps
        assert_eq!(items[2], Value::Bool(true));
        assert_eq!(items[3], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}
