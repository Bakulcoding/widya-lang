use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave64_tmr_and_ccsds() {
    let code = r#"
        var tmr = MesinRedundansiTMR();
        tmr_injeksi_gangguan_radiasi_seu(tmr, "Core_A", 1.0);
        var kalk = tmr_eksekusi_instruksi_voting(tmr, "ADD", 10.0, 50.0, 50.0);
        var sehat = tmr_ambil_status_kesehatan_core(tmr);

        var ccsds = PaketTelemetriCCSDS(100.0, 1.0);
        ccsds_atur_data_payload(ccsds, "2026-09-14", "ABCD");
        var frame = ccsds_enkapsulasi_frame_transfer(ccsds);
        var crc = ccsds_verifikasi_crc(ccsds);

        var cek = kalk["hasil_voting_mayoritas"] == 50.0 dan kalk["koreksi_dilakukan"] == benar dan crc["crc_valid"] == benar;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("TMR and CCSDS test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave64_dtn_and_fdir() {
    let code = r#"
        var dtn = ProtokolBundleDTN("dtn://mars.sol", "dtn://earth.sol");
        dtn_buat_bundle_antarplanet(dtn, "B1", 1000.0, "Payload");
        dtn_terima_dan_simpan_custody(dtn, "B1");
        var res_dtn = dtn_teruskan_saat_kontak_tersedia(dtn, "B1", 300.0);

        var fdir = PengendaliFDIRWahana("Sat_01");
        fdir_daftarkan_subsistem(fdir, "Gyro", "NOMINAL");
        fdir_picu_anomali_telemetri(fdir, "Gyro", "DRIFT");
        var safe = fdir_eksekusi_pemulihan_safemode(fdir);

        var cek = res_dtn["jendela_kontak_dsn_detik"] == 300.0 dan safe["status_transisi"] == "SAFE_MODE_TERPICU_OTONOM";
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("DTN and FDIR test failed");
    assert_eq!(res, Value::Bool(true));
}
