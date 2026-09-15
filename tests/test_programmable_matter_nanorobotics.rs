use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave63_claytronics_and_self_assembly() {
    let code = r#"
        var mat = SistemMateriCatom();
        catom_tambah_unit(mat, "C1", 0.0, 0.0, 0.0);
        catom_atur_adhesi_magnetik(mat, "C1", "Z", 1.0);
        var res_m = catom_eksekusi_rekonfigurasi_bentuk(mat, "KUBUS");

        var sam = PerakitanMandiriMolekul(2.0);
        self_assembly_tambah_ubin_tile(sam, "T1", "g1", "g2", "g0", "g0");
        self_assembly_atur_benih_awal(sam, "Seed", 0, 0);
        var res_s = self_assembly_simulasi_pertumbuhan(sam, 5);

        var cek = res_m["bentuk_tercapai"] == "KUBUS" dan res_s["total_ubin_terakit"] == 6.0;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Claytronics and Self-assembly test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave63_nanorobot_and_molcom() {
    let code = r#"
        var swarm = KawananNanorobot(1000.0, 100.0);
        nanorobot_atur_pembuluh_darah(swarm, 500.0, 50.0);
        nanorobot_tambah_target_tumor(swarm, 200.0, 1.0);
        var res_med = nanorobot_langkah_kemotaksis_dan_injeksi(swarm, 0.1, 10.0);

        var mol = JaringanKomunikasiMolekuler(0.000000001, 10.0);
        var res_tx = molcom_kirim_sinyal_csk(mol, "110", 1000.0);
        var res_rx = molcom_hitung_probabilitas_deteksi(mol, 100.0);

        var cek = res_med["nanorobot_sampai_target"] > 500.0 dan res_rx["bit_error_rate"] <= 0.15;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Nanorobot and MolCom test failed");
    assert_eq!(res, Value::Bool(true));
}
