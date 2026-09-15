use widya::jalankan;
use widya::value::Value;

#[test]
fn test_kademlia_dht() {
    let code = r#"
        var kad = TabelKademlia("node-pusat-01", 20)
        kademlia_tambah_node(kad, "node-alfa-02", "192.168.1.2")
        kademlia_tambah_node(kad, "node-beta-03", "192.168.1.3")
        kademlia_tambah_node(kad, "node-gamma-04", "192.168.1.4")

        kademlia_simpan(kad, "ipfs_hash_1", "DATA_BLOB_A")
        var val1 = kademlia_ambil(kad, "ipfs_hash_1")

        var nearest = kademlia_node_terdekat(kad, "ipfs_hash_1", 2)

        kembalikan [val1, panjang(nearest)]
    "#;
    let res = jalankan(code).expect("Kademlia execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::String("DATA_BLOB_A".to_string()));
            assert_eq!(items[1], Value::Number(2.0));
        }
        _ => panic!("Expected array result"),
    }
}

#[test]
fn test_merkle_patricia_trie() {
    let code = r#"
        var mpt = MerklePatriciaTrie()
        mpt_pasang(mpt, "saldo:0xAlice", 1000)
        mpt_pasang(mpt, "saldo:0xBob", 500)
        mpt_pasang(mpt, "saldo:0xCharlie", 250)

        var root1 = mpt_root_hash(mpt)
        var alice_val = mpt_ambil(mpt, "saldo:0xAlice")

        mpt_hapus(mpt, "saldo:0xBob")
        var root2 = mpt_root_hash(mpt)
        var bob_val = mpt_ambil(mpt, "saldo:0xBob")

        kembalikan [alice_val, bob_val == Kosong().nilai, panjang(root1) == 64, root1 != root2]
    "#;
    let res = jalankan(code).expect("MPT execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::Number(1000.0));
            assert_eq!(items[1], Value::Bool(true));
            assert_eq!(items[2], Value::Bool(true));
            assert_eq!(items[3], Value::Bool(true));
        }
        _ => panic!("Expected array result"),
    }
}

#[test]
fn test_zfp_scientific_compression() {
    let code = r#"
        var sinyal = [3.14159, 2.71828, 1.41421, 1.61803, 0.57721, 3.14159, 2.71828]
        var comp = zfp_kompres_array(sinyal, 4)
        var decomp = zfp_dekompres_array(comp)
        var rasio = zfp_rasio_hemat(sinyal, 4)

        var dekat = mutlak(decomp[0] - sinyal[0]) < 0.001

        kembalikan [panjang(decomp), dekat, rasio > 0.3]
    "#;
    let res = jalankan(code).expect("ZFP execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::Number(7.0));
            assert_eq!(items[1], Value::Bool(true));
            assert_eq!(items[2], Value::Bool(true));
        }
        _ => panic!("Expected array result"),
    }
}

#[test]
fn test_verkle_tree_polynomial_commitments() {
    let code = r#"
        var vt = PohonVerkle(16)
        verkle_sisip(vt, "akun:0x1", "100_ETH")
        verkle_sisip(vt, "akun:0x2", "250_ETH")
        verkle_sisip(vt, "akun:0x3", "500_ETH")

        var val2 = verkle_ambil(vt, "akun:0x2")
        var root_com = verkle_komitmen_root(vt)
        var proof = verkle_buat_bukti(vt, "akun:0x2")

        kembalikan [val2, panjang(root_com) == 64, proof.valid, proof.key == "akun:0x2"]
    "#;
    let res = jalankan(code).expect("Verkle tree execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::String("250_ETH".to_string()));
            assert_eq!(items[1], Value::Bool(true));
            assert_eq!(items[2], Value::Bool(true));
            assert_eq!(items[3], Value::Bool(true));
        }
        _ => panic!("Expected array result"),
    }
}
