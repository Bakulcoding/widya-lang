use widya::jalankan;
use widya::value::Value;

#[test]
fn test_enterprise_database_and_k8s_observability() {
    let code = r#"
        impor "modul/database_driver.wya";
        impor "modul/observability_k8s.wya";

        var pool = buat_kolam_koneksi("Postgres", "localhost", 5432, "test_db", 10);
        var tx_res = db_eksekusi_transaksi_acid(pool, "TX_TEST_1", [["PUT", "k1", 100.0], ["PUT", "k2", 200.0]]);
        redis_set_dengan_ttl(pool, "k_test", "nilai_aman", 10);
        var cached = redis_ambil(pool, "k_test");

        var reg = buat_registry_metrik("test-app");
        prometheus_tambah_counter(reg, "requests_total", 50.0);
        prometheus_set_gauge(reg, "active_threads", 4.0);
        reg["status_layanan"] = "READY";
        var r_probe = k8s_readiness_probe(reg);
        var prom_txt = prometheus_ekspor_teks(reg);

        var valid = tx_res["sukses"] dan (cached == "nilai_aman") dan r_probe["ready"] dan (panjang(prom_txt) > 0.0);
        kembalikan valid;
    "#;
    let res = jalankan(code).expect("Enterprise database & observability test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_enterprise_rbac_async_and_streaming() {
    let code = r#"
        impor "modul/autentikasi_enterprise.wya";
        impor "modul/async_runtime.wya";
        impor "modul/streaming_io.wya";

        var rbac = buat_manajer_rbac();
        var tok_admin = terbitkan_token_sesi(rbac, "u1", "AdminUser", "Admin", 3600);
        var izin_admin = validasi_dan_periksa_izin(rbac, tok_admin, "tulis");

        var tok_view = terbitkan_token_sesi(rbac, "u2", "ViewUser", "Viewer", 3600);
        var izin_view = validasi_dan_periksa_izin(rbac, tok_view, "hapus");

        var p_async = buat_kolam_tugas_asinkron("TestAsyncPool");
        var handle = jalankan_tugas_asinkron(p_async, 50.0);
        var res_async = tunggu_hasil_tugas(p_async, handle);

        var stream = buat_saluran_streaming_berkas("ChunkStream", 512);
        streaming_alirkan_potongan(stream, "CHUNK_A");
        streaming_alirkan_potongan(stream, "CHUNK_B");
        var chunks = streaming_kumpulkan_seluruh_potongan(stream);

        var valid = izin_admin["diizinkan"] dan (!izin_view["diizinkan"]) dan (res_async == 50.0) dan (panjang(chunks) == 2.0);
        kembalikan valid;
    "#;
    let res = jalankan(code).expect("Enterprise RBAC, async, and streaming test failed");
    assert_eq!(res, Value::Bool(true));
}
