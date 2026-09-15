use widya::jalankan;
use widya::value::Value;

#[test]
fn test_gas_engine_metering_and_guard() {
    let code = r#"
        var m = MesinGas(100000, 2.0)
        gas_konsumsi(m, 20000, "OP_BASE")
        gas_konsumsi(m, 30000, "OP_STORAGE")

        var sisa = gas_sisa(m)
        var terpakai = gas_total_terpakai(m)

        var oof_caught = salah
        coba {
            gas_konsumsi(m, 60000, "OP_EXCESS")
        } tangkap e {
            oof_caught = benar
        }

        kembalikan [sisa == 50000, terpakai == 50000, oof_caught, gas_apakah_habis(m)]
    "#;

    let res = jalankan(code).expect("Gas engine test failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        for item in b.iter() {
            assert_eq!(*item, Value::Bool(true));
        }
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_evm_abi_encoder_decoder() {
    let code = r#"
        var selector = abi_hitung_selector("transfer(address,uint256)")
        var types = ["address", "uint256"]
        var values = ["0x71C841832049987C62260E21017BA541aA74B1f2", 250000]

        var encoded = abi_enkode(types, values)
        var decoded = abi_dekode(types, encoded)

        kembalikan [
            panjang(selector) == 10,
            panjang(encoded) == 130, // 0x + 64 hex chars * 2 = 130
            decoded[1] == 250000
        ]
    "#;

    let res = jalankan(code).expect("ABI bridge test failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        for item in b.iter() {
            assert_eq!(*item, Value::Bool(true));
        }
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_secp256k1_wallet_and_signatures() {
    let code = r#"
        var dompet = secp256k1_buat_dompet()
        var msg_hash = "0x5a1f893e4b868a25c613098ad8e4b7e12739e5b6ac80f12c823b1901a84f3e69"
        var sig = secp256k1_tanda_tangani(msg_hash, dompet["kunci_privat"])

        var valid = secp256k1_verifikasi(msg_hash, sig, dompet["kunci_publik"])
        var recovered = secp256k1_pulihkan_alamat(msg_hash, sig)

        kembalikan [
            panjang(dompet["alamat"]) == 42,
            valid,
            panjang(recovered) == 42
        ]
    "#;

    let res = jalankan(code).expect("Secp256k1 test failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        for item in b.iter() {
            assert_eq!(*item, Value::Bool(true));
        }
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_defi_amm_constant_product_pool() {
    let code = r#"
        var pool = PoolLikuiditas("TOKEN_A", "TOKEN_B", 1000.0, 4000.0)
        var harga_awal = amm_hitung_harga(pool, "TOKEN_A")

        var swap = amm_tukar_token(pool, "TOKEN_B", 400.0)
        var harga_setelah_swap = amm_hitung_harga(pool, "TOKEN_A")

        var lp = amm_tambah_likuiditas(pool, 100.0, 440.0)

        kembalikan [
            harga_awal == 4.0,
            swap["jumlah_keluar"] > 0.0,
            harga_setelah_swap > 0.0,
            lp["lp_dicetak"] > 0.0
        ]
    "#;

    let res = jalankan(code).expect("DeFi AMM test failed");
    if let Value::Array(arr) = res {
        let b = arr.borrow();
        for item in b.iter() {
            assert_eq!(*item, Value::Bool(true));
        }
    } else {
        panic!("Expected array result");
    }
}
