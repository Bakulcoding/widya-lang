use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave60_terahertz_channel() {
    let code = r#"
        var kanal = KanalTeraHertz6G(0.3, 10.0);
        thz_set_parameter_lingkungan(kanal, 50.0, 25.0, 1013.25);

        var red = thz_hitung_redaman_molekuler(kanal);
        var cap = thz_hitung_kapasitas_shannon_gbps(kanal, 10.0, 20.0, 7.0);

        var cek = red["total_path_loss_db"] > 0.0 dan cap["kapasitas_gbps"] > 0.0;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Terahertz channel test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave60_ris_metasurface() {
    let code = r#"
        var ris = PermukaanCerdasRIS(8, 8);
        ris_atur_matriks_fasa(ris, 30.0);
        var gain = ris_hitung_penguatan_sinyal_rx(ris, 20.0, 5.0, 10.0);

        var cek = gain["penguatan_array_ris_db"] > 0.0 dan gain["sinyal_terhubung"] == benar;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("RIS metasurface test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave60_isac_and_oran_ric() {
    let code = r#"
        var isac = SistemISAC6G(28.0, 400.0);
        isac_sisip_simbol_radar_komunikasi(isac, "TEST_PAYLOAD");
        var radar = isac_proses_gema_target(isac, 100.0, 10.0);

        var oran = ArsitekturORAN("RIC_Test");
        oran_daftarkan_node_e2(oran, "Node_1", "O-DU", 100.0);
        oran_perbarui_metrik_kpi(oran, "Node_1", 50.0, 200.0, 5.0);
        var xapp = oran_eksekusi_xapp_kebijakan(oran, "xApp_TS", "Node_1", "STEERING");

        var cek = radar["jarak_target_meter"] > 0.0 dan radar["kecepatan_target_m_s"] > 0.0 dan xapp["status"] == "BERHASIL_DILAKSANAKAN";
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("ISAC and O-RAN test failed");
    assert_eq!(res, Value::Bool(true));
}
