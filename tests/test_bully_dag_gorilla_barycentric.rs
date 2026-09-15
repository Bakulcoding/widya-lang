use widya::jalankan;
use widya::value::Value;

#[test]
fn test_bully_election_protocol() {
    let code = r#"
        var klaster = KlasterBully(["node-1", "node-2", "node-3", "node-4"])
        var leader_awal = bully_dapatkan_leader(klaster)

        // Matikan node pemimpin tertinggi (node-4)
        bully_matikan_node(klaster, "node-4")

        // Picu pemilihan ulang (node-2 memulai pemilihan)
        var leader_baru = bully_mulai_pemilihan(klaster, "node-2")

        // Detak klaster otomatis
        var detak_leader = bully_detak(klaster)

        kembalikan [leader_awal, leader_baru, detak_leader]
    "#;
    let res = jalankan(code).expect("Bully election failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::String("node-4".to_string()));
            assert_eq!(items[1], Value::String("node-3".to_string()));
            assert_eq!(items[2], Value::String("node-3".to_string()));
        }
        _ => panic!("Expected array result"),
    }
}

#[test]
fn test_merkle_dag_storage() {
    let code = r#"
        var dag = MerkleDAG()

        // Buat simpul daun (leaf blobs)
        var cid_leaf1 = dag_buat_node(dag, "DATA_FRAGMENT_PART_1", [])
        var cid_leaf2 = dag_buat_node(dag, "DATA_FRAGMENT_PART_2", [])

        // Buat simpul induk direktori yang menautkan daun
        var cid_root = dag_buat_node(dag, "ROOT_MANIFEST", [cid_leaf1, cid_leaf2])

        var root_node = dag_ambil_node(dag, cid_root)
        var traversal = dag_traversal_dfs(dag, cid_root)

        kembalikan [panjang(cid_root) > 10, root_node.data, panjang(traversal)]
    "#;
    let res = jalankan(code).expect("MerkleDAG execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::Bool(true));
            assert_eq!(items[1], Value::String("ROOT_MANIFEST".to_string()));
            assert_eq!(items[2], Value::Number(3.0));
        }
        _ => panic!("Expected array result"),
    }
}

#[test]
fn test_gorilla_time_series_compression() {
    let code = r#"
        var ts_floats = [100.5, 100.5, 100.5, 100.8, 100.8, 101.2, 101.2, 101.2, 102.0]
        var comp = gorilla_kompres_seri(ts_floats)
        var decomp = gorilla_dekompres_seri(comp)
        var rasio = gorilla_rasio_hemat(ts_floats)

        var sama = decomp[0] == ts_floats[0] && decomp[3] == ts_floats[3]

        kembalikan [panjang(decomp), sama, rasio > 0.4]
    "#;
    let res = jalankan(code).expect("Gorilla compression failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::Number(9.0));
            assert_eq!(items[1], Value::Bool(true));
            assert_eq!(items[2], Value::Bool(true));
        }
        _ => panic!("Expected array result"),
    }
}

#[test]
fn test_barycentric_polynomial_interpolation() {
    let code = r#"
        // Titik sampel f(x) = x^2 pada x = [1, 2, 3, 4] -> y = [1, 4, 9, 16]
        var interp = BarycentricInterpolator([1.0, 2.0, 3.0, 4.0], [1.0, 4.0, 9.0, 16.0])

        // Evaluasi pada x = 2.5 (harus mendekati 2.5^2 = 6.25)
        var y_2_5 = barycentric_evaluasi(interp, 2.5)

        // Evaluasi titik tepat pada x = 3.0 (harus tepat 9.0)
        var y_3 = barycentric_evaluasi(interp, 3.0)

        // Tambah titik x = 5.0, y = 25.0
        barycentric_tambah_titik(interp, 5.0, 25.0)
        var y_4_5 = barycentric_evaluasi(interp, 4.5)

        var dekat_2_5 = mutlak(y_2_5 - 6.25) < 0.0001
        var dekat_4_5 = mutlak(y_4_5 - 20.25) < 0.0001

        kembalikan [y_3, dekat_2_5, dekat_4_5]
    "#;
    let res = jalankan(code).expect("Barycentric interpolation failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::Number(9.0));
            assert_eq!(items[1], Value::Bool(true));
            assert_eq!(items[2], Value::Bool(true));
        }
        _ => panic!("Expected array result"),
    }
}
