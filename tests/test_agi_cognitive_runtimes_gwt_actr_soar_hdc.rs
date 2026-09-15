use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave65_gwt_and_actr() {
    let code = r#"
        var gwt = PapanGlobalWorkspace(0.5);
        gwt_daftarkan_modul_kognitif(gwt, "Vision", 0.9, "LOOK");
        gwt_daftarkan_modul_kognitif(gwt, "Audio", 0.3, "LISTEN");
        var win = gwt_kompetisi_fokus_atensi(gwt);
        var broad = gwt_siarkan_kesadaran_global(gwt);

        var actr = MesinKognitifACTR();
        actr_tambah_chunk_deklaratif(actr, "Item", {"name": "Apple"});
        var ret = actr_kueri_ingatan_retrieval(actr, "Apple");
        actr_daftarkan_aturan_produksi(actr, "Eat", "HUNGRY", "EAT_APPLE", 3.0);
        var siklus = actr_siklus_eksekusi(actr);

        var cek = win["modul_fokus"] == "Vision" dan ret["skor_aktivasi_total"] > 1.0 dan siklus["aturan_terpicu"] == "Eat";
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("GWT and ACT-R test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave65_soar_and_hdc() {
    let code = r#"
        var soar = ArsitekturKognitifSOAR("Goal");
        soar_usulkan_operator(soar, "Op1", 2.0, "Desc1");
        soar_usulkan_operator(soar, "Op2", 5.0, "Desc2");
        var dec = soar_evaluasi_dan_pilih_operator(soar);
        var imp = soar_tangani_kebuntuan_impasse(soar, "TIE");

        var hdc = RuangHiperdimensiHDC(1000);
        var v1 = hdc_buat_hipervektor(hdc, "A");
        var v2 = hdc_buat_hipervektor(hdc, "B");
        var vb = hdc_ikat_konsep_bind(hdc, v1, v2);
        var sim_self = hdc_ukur_kesamaan_simbolik(hdc, v1, v1);
        var sim_diff = hdc_ukur_kesamaan_simbolik(hdc, v1, v2);

        var cek = dec["operator_terpilih"] == "Op2" dan sim_self == 1.0 dan sim_diff < 0.2;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("SOAR and HDC test failed");
    assert_eq!(res, Value::Bool(true));
}
