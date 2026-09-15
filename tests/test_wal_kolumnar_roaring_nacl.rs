use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wal_and_kolumnar_execution() {
    let code = r#"
        var wal = LogWAL("akun_bank")
        var l1 = wal_tulis(wal, "PUT", "user:101", 500000)
        var l2 = wal_tulis(wal, "PUT", "user:102", 750000)
        var l3 = wal_tulis(wal, "PUT", "user:101", 620000)
        var l4 = wal_tulis(wal, "DELETE", "user:102", 0)

        var len = wal_panjang(wal)
        var state = wal_pulihkan(wal)

        var cols = ["id", "nama", "harga", "stok"]
        var tabel = TabelKolumnar(cols)
        kolumnar_tambah_baris(tabel, {"id": 1, "nama": "Laptop", "harga": 15000000, "stok": 10})
        kolumnar_tambah_baris(tabel, {"id": 2, "nama": "Mouse", "harga": 500000, "stok": 50})
        kolumnar_tambah_baris(tabel, {"id": 3, "nama": "Monitor", "harga": 4500000, "stok": 20})

        var total_harga = kolumnar_kueri_agregasi(tabel, "harga", "SUM")
        var avg_harga = kolumnar_kueri_agregasi(tabel, "harga", "AVG")
        var max_harga = kolumnar_kueri_agregasi(tabel, "harga", "MAX")
        var min_harga = kolumnar_kueri_agregasi(tabel, "harga", "MIN")

        kembalikan [len, state["user:101"], ada_kunci(state, "user:102"), total_harga, avg_harga, max_harga, min_harga];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(4.0));
        assert_eq!(items[1], Value::Number(620000.0));
        assert_eq!(items[2], Value::Bool(false));
        assert_eq!(items[3], Value::Number(20000000.0));
        assert_eq!(items[4], Value::Number(20000000.0 / 3.0));
        assert_eq!(items[5], Value::Number(15000000.0));
        assert_eq!(items[6], Value::Number(500000.0));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_roaring_and_nacl_execution() {
    let code = r#"
        var bm1 = RoaringBitmap([10, 20, 30, 40])
        var bm2 = RoaringBitmap([20, 30, 50])

        roaring_tambah(bm1, 50)
        var ada20 = roaring_apakah_ada(bm1, 20)
        var irisan = roaring_irisan(bm1, bm2)
        var gabungan = roaring_gabungan(bm1, bm2)

        var hitung_irisan = roaring_hitung(irisan)
        var hitung_gabungan = roaring_hitung(gabungan)

        var kunci = "widya_lang_super_secret_key_32b!"
        var nonce = "widya_nonce!"
        var pesan = "Kriptografi Kelas Dunia Widya-Lang"

        var cipher = chacha20_enkripsi(pesan, kunci, nonce)
        var pulih = chacha20_dekripsi(cipher, kunci, nonce)

        var tag = poly1305_otentikasi(pesan, kunci)
        var valid = poly1305_verifikasi(pesan, tag, kunci)
        var palsu = poly1305_verifikasi("Pesan Palsu", tag, kunci)

        kembalikan [ada20, hitung_irisan, hitung_gabungan, pulih == pesan, valid, !palsu];
    "#;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Number(3.0)); // 20, 30, 50
        assert_eq!(items[2], Value::Number(5.0)); // 10, 20, 30, 40, 50
        assert_eq!(items[3], Value::Bool(true));
        assert_eq!(items[4], Value::Bool(true));
        assert_eq!(items[5], Value::Bool(true));
    } else {
        panic!("Expected array result");
    }
}
