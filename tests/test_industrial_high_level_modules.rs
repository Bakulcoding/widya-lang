use widya::jalankan;
use widya::value::Value;

#[test]
fn test_industrial_high_level_privacy_and_fintech() {
    let code = r#"
        impor "modul/privasi_keamanan.wya";

        var brankas = buat_brankas_rahasia("BankTest");
        brankas_simpan_rahasia(brankas, "KUNCI_1", "DATA_SUPER_RAHASIA");
        var val_opt = brankas_ambil_rahasia(brankas, "KUNCI_1");

        var ctx = inisialisasi_komputasi_rahasia_fhe();
        var data_nums = [10.0, 20.0, 30.0];
        var enc_total = komputasi_total_terenkripsi(ctx, data_nums);
        var dec_total = dekripsi_fhe(ctx, enc_total);

        var zk_res = buat_bukti_audit_zk("TRX_1", 100.0, 50.0);
        var dp_res = audit_privasi_differensial(data_nums, 1.0);

        var valid = (dec_total == 60.0) dan zk_res["apakah_sah"] dan (dp_res["total_sampel"] == 3.0);
        kembalikan valid;
    "#;
    let res = jalankan(code).expect("Industrial high-level privacy module test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_industrial_high_level_microservices_and_iot() {
    let code = r#"
        impor "modul/microservice_stack.wya";
        impor "modul/edge_iot.wya";

        var mesh = buat_mesh_layanan("TestMesh");
        daftarkan_node_backend(mesh, "node-1", "http://127.0.0.1:8080", 1);
        var hub = buat_hub_perangkat_edge("EDGE_TEST");

        var sensor_data = hub_baca_telemetri_sensor(hub, 2, "0x50");
        var rpc_res = eksekusi_panggilan_resilient(mesh, "/sensor/data", sensor_data);

        var valid = (rpc_res["status"] == 200.0) dan (panjang(mesh["layanan_terdaftar"]) == 1.0);
        kembalikan valid;
    "#;
    let res = jalankan(code).expect("Industrial high-level microservice & IoT test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_industrial_high_level_data_and_fintech() {
    let code = r#"
        impor "modul/data_engineering.wya";
        impor "modul/fintech_settlement.wya";

        var data_points = [[10.0, "A"], [20.0, "B"], [30.0, "C"]];
        var etl_res = jalankan_etl_sederhana(data_points, ["nilai", "kategori"], 2.0);

        var bursa = buat_mesin_bursa_pasar("WIDYA_IDR");
        var match_res = eksekusi_pesanan_limit(bursa, "BELI", 10000.0, 5.0);
        var risiko = kalkulasi_risiko_opsi(100.0, 100.0, 1.0, 0.2, 0.05);

        var valid = (etl_res["total_baris"] == 3.0) dan (bursa["total_volume"] == 5.0) dan (risiko["harga_call"] > 0.0);
        kembalikan valid;
    "#;
    let res = jalankan(code).expect("Industrial data and fintech test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_industrial_high_level_ai_rag() {
    let code = r#"
        impor "modul/ai_rag_stack.wya";

        var rag = buat_mesin_rag_cerdas("KnowledgeBaseTest", 16);
        rag_indeks_dokumen(rag, "DOK_1", "Widya adalah bahasa pemrograman performa tinggi dengan fitur privasi ZK dan FHE.");
        rag_indeks_dokumen(rag, "DOK_2", "Sistem mikroservis Widya dilengkapi Load Balancer dan OpenTelemetry Tracing.");

        var konteks = rag_kueri_konteks_terkait(rag, "privasi dan FHE", 1);
        var prompt = jalankan_sintesis_rag(rag, "Jelaskan privasi di Widya", 1);

        var valid = (rag["total_dokumen"] == 2.0) dan (panjang(konteks) > 0.0) dan (panjang(prompt) > 0.0);
        kembalikan valid;
    "#;
    let res = jalankan(code).expect("Industrial AI RAG module test failed");
    assert_eq!(res, Value::Bool(true));
}
