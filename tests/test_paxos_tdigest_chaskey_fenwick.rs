use widya::jalankan;
use widya::value::Value;

#[test]
fn test_paxos_consensus_protocol() {
    let code = r#"
        var klaster = KlasterPaxos(["acc-1", "acc-2", "acc-3", "acc-4", "acc-5"])

        // Fasa 1: Prepare (Proposal n = 10, Proposer = "prop-A")
        var prep = paxos_siapkan(klaster, 10, "prop-A")

        // Fasa 2: Propose / Accept (Proposal n = 10, Value = "TX_BLOCK_HASH_888")
        var prop = paxos_usulkan(klaster, 10, "TX_BLOCK_HASH_888", "prop-A")

        // Putuskan nilai
        paxos_putuskan(klaster, prop.value)

        var final_val = paxos_nilai_terpilih(klaster)

        kembalikan [prep.ok, prop.ok, final_val]
    "#;
    let res = jalankan(code).expect("Paxos execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::Bool(true));
            assert_eq!(items[1], Value::Bool(true));
            assert_eq!(items[2], Value::String("TX_BLOCK_HASH_888".to_string()));
        }
        _ => panic!("Expected array result"),
    }
}

#[test]
fn test_tdigest_quantile_estimation() {
    let code = r#"
        var td = TDigest(100)

        // Tambah 1000 nilai terurut
        var i = 1
        selama (i <= 100) {
            tdigest_tambah(td, i * 1.0)
            i = i + 1
        }

        var p50 = tdigest_persentil(td, 0.5)
        var p90 = tdigest_persentil(td, 0.9)
        var cdf50 = tdigest_cdf(td, 50.0)

        var dekat_50 = mutlak(p50 - 50.0) <= 2.0
        var dekat_90 = mutlak(p90 - 90.0) <= 2.0
        var cdf_dekat = mutlak(cdf50 - 0.5) <= 0.05

        kembalikan [dekat_50, dekat_90, cdf_dekat]
    "#;
    let res = jalankan(code).expect("TDigest execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::Bool(true));
            assert_eq!(items[1], Value::Bool(true));
            assert_eq!(items[2], Value::Bool(true));
        }
        _ => panic!("Expected array result"),
    }
}

#[test]
fn test_chaskey_mac_authentication() {
    let code = r#"
        var kunci = "KUNCI_128_BIT_IoT_SECURE"
        var pesan = "SUHU_SENSOR:28.5_CELSIUS"

        var mac_hex = chaskey_hitung_mac(kunci, pesan)
        var verif_ok = chaskey_verifikasi_mac(kunci, pesan, mac_hex)
        var verif_palsu = chaskey_verifikasi_mac(kunci, "PESAN_PALSU", mac_hex)

        kembalikan [panjang(mac_hex) == 32, verif_ok, verif_palsu == salah]
    "#;
    let res = jalankan(code).expect("Chaskey MAC execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::Bool(true));
            assert_eq!(items[1], Value::Bool(true));
            assert_eq!(items[2], Value::Bool(true));
        }
        _ => panic!("Expected array result"),
    }
}

#[test]
fn test_fenwick_tree_prefix_and_range() {
    let code = r#"
        var array_awal = [3.0, 2.0, -1.0, 6.0, 5.0, 4.0, -3.0, 3.0, 7.0, 2.0]
        var fenwick = PohonFenwick(array_awal)

        // Kueri prefix sum 1..5: 3 + 2 - 1 + 6 + 5 = 15
        var pref_5 = fenwick_kueri_awalan(fenwick, 5)

        // Kueri range sum 3..7: -1 + 6 + 5 + 4 - 3 = 11
        var range_3_7 = fenwick_kueri_rentang(fenwick, 3, 7)

        // Perbarui indeks 3 (+6): nilai array[2] dari -1 menjadi 5
        fenwick_perbarui(fenwick, 3, 6.0)
        var range_3_7_baru = fenwick_kueri_rentang(fenwick, 3, 7) // 11 + 6 = 17

        kembalikan [pref_5, range_3_7, range_3_7_baru]
    "#;
    let res = jalankan(code).expect("Fenwick tree execution failed");
    match res {
        Value::Array(arr) => {
            let items = arr.borrow();
            assert_eq!(items[0], Value::Number(15.0));
            assert_eq!(items[1], Value::Number(11.0));
            assert_eq!(items[2], Value::Number(17.0));
        }
        _ => panic!("Expected array result"),
    }
}
