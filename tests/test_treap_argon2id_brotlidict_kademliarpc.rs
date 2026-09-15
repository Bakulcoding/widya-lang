use widya::jalankan;
use widya::value::Value;

#[test]
fn test_treap_cartesian_bst_heap() {
    let code = r#"
        var t = PohonTreap()
        treap_sisip(t, 50, "Node_50", 100)
        treap_sisip(t, 25, "Node_25", 80)
        treap_sisip(t, 75, "Node_75", 90)

        var c1 = treap_cari(t, 50)
        var c2 = treap_cari(t, 25)
        var c3 = treap_cari(t, 99)

        treap_hapus(t, 25)
        var c4 = treap_cari(t, 25)

        kembalikan [c1, c2, c3 == nil, c4 == nil]
    "#;

    let res = jalankan(code).expect("Treap execution failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        assert_eq!(b[0], Value::String("Node_50".to_string()));
        assert_eq!(b[1], Value::String("Node_25".to_string()));
        assert_eq!(b[2], Value::Bool(true));
        assert_eq!(b[3], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_argon2id_password_hashing() {
    let code = r#"
        var pwd = "WidyaSuperSecret2026!"
        var salt = "unique_salt_123"
        var hash = argon2id_hash(pwd, salt, 2, 128)

        var ok1 = argon2id_verifikasi(pwd, hash)
        var ok2 = argon2id_verifikasi("WrongPassword", hash)

        kembalikan [ok1, ok2 == salah]
    "#;

    let res = jalankan(code).expect("Argon2id execution failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        assert_eq!(b[0], Value::Bool(true));
        assert_eq!(b[1], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_brotli_shared_dictionary() {
    let code = r#"
        var dict = "HEADER_API_KEY_AUTH_TOKEN_V1_METADATA_JSON_SCHEMA"
        var data = "HEADER_API_KEY_AUTH_TOKEN_V1_METADATA_JSON_SCHEMA_PAYLOAD_VALUE_999"

        var comp = brotli_kamus_kompres(data, dict)
        var decomp = brotli_kamus_dekompres(comp, dict)

        kembalikan [decomp["teks"] == data]
    "#;

    let res = jalankan(code).expect("Brotli dict execution failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        assert_eq!(b[0], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_kademlia_rpc_messaging() {
    let code = r#"
        var req = kademlia_rpc_buat_pesan("FIND_NODE", "node_A", "node_B", {"key": "target_dht"})
        var resp = kademlia_rpc_buat_respons(req, {"nodes": ["node_C", "node_D"]})

        var parsed = kademlia_rpc_uraikan_pesan(req)

        kembalikan [
            req["tipe_rpc"] == "FIND_NODE",
            resp["tipe_rpc"] == "FIND_NODE_RESPONSE",
            resp["pengirim_id"] == "node_B",
            resp["target_id"] == "node_A",
            parsed["tipe_rpc"] == "FIND_NODE"
        ]
    "#;

    let res = jalankan(code).expect("Kademlia RPC execution failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        for item in b.iter() {
            assert_eq!(*item, Value::Bool(true));
        }
    } else {
        panic!("Expected array result");
    }
}
