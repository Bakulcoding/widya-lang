use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave61_abm_schelling() {
    let code = r#"
        var abm = ModelBerbasisAgenABM(10, 10, 0.5);
        abm_tambah_agen(abm, "A1", 1, 1, 1);
        abm_tambah_agen(abm, "A2", 1, 1, 2);
        abm_tambah_agen(abm, "B1", 2, 5, 5);

        var dipindah = abm_langkah_simulasi(abm);
        var res = abm_hitung_indeks_segregasi(abm);

        var cek = res["total_agen"] == 3.0 dan res["indeks_kepuasan_segregasi"] > 0.0;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("ABM Schelling test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave61_system_dynamics_rk4() {
    let code = r#"
        var sd = DinamikaSistemStokAlir();
        sd_tambah_stok(sd, "S", 100.0);
        sd_tambah_stok(sd, "D", 0.0);
        sd_tambah_aliran_flow(sd, "F", "S", "D", 0.1);

        sd_langkah_waktu_rk4(sd, 1.0);
        var s = sd_ambil_nilai_stok(sd, "S");
        var d = sd_ambil_nilai_stok(sd, "D");

        var cek = s < 100.0 dan d > 0.0;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("System dynamics test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave61_ecological_network_and_game_theory() {
    let code = r#"
        var eco = JaringEkologiTrofik();
        ekologi_tambah_spesies(eco, "Rumput", 100.0, 0.1);
        ekologi_tambah_spesies(eco, "Kelinci", 20.0, -0.05);
        ekologi_tambah_interaksi_predasi(eco, "Kelinci", "Rumput", 0.001);
        ekologi_simulasi_dinamika_populasi(eco, 0.1, 5);

        var game = TeoriPermainanEvolusi();
        game_set_matriks_imbalan(game, 3.0, 0.0, 5.0, 1.0);
        var g_res = game_simulasi_dinamika_replikator(game, 0.5, 0.01, 10);

        var cek = g_res["fraksi_kooperator_akhir"] >= 0.0 dan g_res["fraksi_pembelot_akhir"] >= 0.0;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Ecological and Game Theory test failed");
    assert_eq!(res, Value::Bool(true));
}
