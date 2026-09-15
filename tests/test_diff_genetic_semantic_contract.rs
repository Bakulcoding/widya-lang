use widya::jalankan;
use widya::value::Value;

#[test]
fn test_diff_dan_genetika() {
    let code = r#"
        tetap teks_lama = "baris 1\nbaris 2\nbaris 3";
        tetap teks_baru = "baris 1\nbaris modifikasi\nbaris 3";
        
        tetap diff_res = hitung_diff(teks_lama, teks_baru);
        tetap patch_str = format_patch_terpadu(diff_res, "test.wya");
        tetap hasil_patch = terapkan_patch(teks_lama, diff_res);
        
        fungsi fit_fn(ind) {
            misal sum = 0;
            untuk i dalam 0..panjang(ind) - 1 {
                sum = sum + ind[i];
            }
            kembalikan sum;
        }
        
        tetap ga_res = optimasi_ga(10, 5, 15, fit_fn, 5.0);
        
        kembalikan [diff_res["total_perubahan"], hasil_patch == teks_baru, ga_res["fitness_terbaik"]];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(2.0)); // 1 deletion + 1 addition
        assert_eq!(items[1], Value::Bool(true));
        if let Value::Number(fit) = items[2] {
            assert!(fit >= 4.0); // should optimize towards max 5 ones
        } else {
            panic!("Expected number for fitness");
        }
    } else {
        panic!("Expected array return value");
    }
}

#[test]
fn test_semantic_dan_smartcontract() {
    let code = r#"
        tetap idx = buat_indeks_vektor(2);
        vektor_tambah(idx, "ai", [1.0, 0.0], "AI Doc");
        vektor_tambah(idx, "web", [0.0, 1.0], "Web Doc");
        
        tetap cari = vektor_kueri_kemiripan(idx, [0.95, 0.05], 1);
        
        tetap ledger = buat_buku_besar({ "0x1": 100.0, "0x2": 50.0 });
        transfer_saldo_ledger(ledger, "0x1", "0x2", 30.0);
        
        fungsi handler(l, k, p) {
            ledger_set_state(l, k, "status", "aktif");
            kembalikan "OK";
        }
        
        tetap receipt = eksekusi_transaksi_ledger(ledger, "0x1", "0xVault", handler, 500.0);
        tetap st = ledger_get_state(ledger, "0xVault", "status");
        
        kembalikan [cari[0]["id"], ledger["saldo"]["0x1"], ledger["saldo"]["0x2"], receipt["sukses"], st];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::String("ai".to_string()));
        if let Value::Number(s1) = items[1] {
            assert!(s1 < 70.0); // 100 - 30 - gas fee
        }
        assert_eq!(items[2], Value::Number(80.0)); // 50 + 30
        assert_eq!(items[3], Value::Bool(true));
        assert_eq!(items[4], Value::String("aktif".to_string()));
    } else {
        panic!("Expected array return value");
    }
}
