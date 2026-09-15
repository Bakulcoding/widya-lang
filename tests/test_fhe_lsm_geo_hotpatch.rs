use widya::jalankan;
use widya::value::Value;

#[test]
fn test_fhe_dan_lsm() {
    let kode = r#"
        // 1. WidyaFHE test
        misal ctx = KonteksFHE("CKKS", 4096);
        misal ct1 = enkripsi_fhe(ctx, 30.0);
        misal ct2 = enkripsi_fhe(ctx, 70.0);

        misal ct_add = fhe_tambah(ct1, ct2);
        misal ct_mul = fhe_kali(ct1, ct2);

        misal res_add = dekripsi_fhe(ctx, ct_add);
        misal res_mul = dekripsi_fhe(ctx, ct_mul);

        pastikan(res_add == 100.0, "Hasil dekripsi FHE tambah harus 100");
        pastikan(res_mul == 2100.0, "Hasil dekripsi FHE kali harus 2100");

        // 2. WidyaLSM test
        misal lsm = BukaLSM("test_db");
        lsm_tulis(lsm, "kunci1", "nilai1");
        misal val_mem = lsm_baca(lsm, "kunci1");

        lsm_flush(lsm);
        misal val_sst = lsm_baca(lsm, "kunci1");

        lsm_hapus(lsm, "kunci1");
        misal val_del = lsm_baca(lsm, "kunci1");

        pastikan(val_mem == "nilai1", "Baca MemTable harus sukses");
        pastikan(val_sst == "nilai1", "Baca SSTable setelah flush harus sukses");
        pastikan(val_del == nihil, "Baca data terhapus harus nihil");

        kembalikan [res_add, res_mul, val_sst];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(100.0));
        assert_eq!(items[1], Value::Number(2100.0));
        assert_eq!(items[2], Value::String("nilai1".to_string()));
    } else {
        panic!("Hasil harus berupa array");
    }
}

#[test]
fn test_geo_dan_hotpatch() {
    let kode = r#"
        // 1. WidyaGeo test
        misal t1 = TitikGeo(0.0, 0.0, 0.0);
        misal t2 = TitikGeo(0.0, 1.0, 0.0);
        misal jarak = hitung_jarak_haversine(t1, t2);

        // Geofence square (0,0) to (2,2)
        misal p1 = TitikGeo(0.0, 0.0, 0.0);
        misal p2 = TitikGeo(2.0, 0.0, 0.0);
        misal p3 = TitikGeo(2.0, 2.0, 0.0);
        misal p4 = TitikGeo(0.0, 2.0, 0.0);
        misal poly = PoligonGeo([p1, p2, p3, p4]);

        misal titik_dalam = TitikGeo(1.0, 1.0, 0.0);
        misal titik_luar = TitikGeo(5.0, 5.0, 0.0);

        misal is_dalam = apakah_dalam_poligon(titik_dalam, poly);
        misal is_luar = apakah_dalam_poligon(titik_luar, poly);
        misal geojson = ekspor_geojson(poly);

        pastikan(jarak > 100000.0, "Jarak 1 derajat bujur harus sekitar 111 km");
        pastikan(is_dalam == benar, "Titik (1,1) harus di dalam poligon (0,0)-(2,2)");
        pastikan(is_luar == salah, "Titik (5,5) harus di luar poligon");
        pastikan(panjang(geojson) > 20, "GeoJSON harus valid");

        // 2. WidyaHotPatch test
        misal f_patch = fungsi(x) { kembalikan x * 10; };
        misal patch_ok = patch_fungsi("target_fn", f_patch);
        misal log = riwayat_patch();

        pastikan(patch_ok == benar, "Hot-patch harus berhasil");
        pastikan(panjang(log) > 0, "Log riwayat patch harus terisi");

        kembalikan [is_dalam, is_luar, patch_ok];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(false));
        assert_eq!(items[2], Value::Bool(true));
    } else {
        panic!("Hasil harus berupa array");
    }
}
