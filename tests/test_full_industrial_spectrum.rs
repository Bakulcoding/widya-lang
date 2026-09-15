use widya::jalankan;
use widya::value::Value;

#[test]
fn test_spectrum_distributed_and_cybersecurity() {
    let code = r#"
        impor "modul/keamanan_siber_forensik.wya";
        impor "modul/sistem_terdistribusi.wya";

        var soc = buat_pusat_operasi_keamanan();
        var logs = [
            { "ip": "192.168.1.1", "uri": "/home", "status": 200 },
            { "ip": "10.0.0.99", "uri": "/exec?cmd=id", "status": 403 }
        ];
        var soc_res = analisa_keamanan_log(soc, logs, 3);
        var file_scan = periksa_file_biner(soc, "hello world plaintext safe code");

        var cluster = buat_klaster_konsensus("Node_A", ["Node_A", "Node_B"]);
        var tx_res = usulkan_transaksi_terdistribusi(cluster, "APPEND_EVENT_LOG");

        var valid = (soc_res["skor_risiko"] >= 0.0) dan (!file_scan["terdeteksi_malware"]) dan tx_res["sukses"];
        kembalikan valid;
    "#;
    let res = jalankan(code).expect("Cybersecurity and distributed test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_spectrum_devops_and_archival() {
    let code = r#"
        impor "modul/cloud_devops_arsip.wya";

        var devops = buat_orkestrator_devops("ProductionPipeline");
        devops_tambah_tahap_otomatis(devops, "Build", ["widya bangun"]);
        var run_res = devops_jalankan_dan_verifikasi(devops);

        var bag_res = buat_kemasan_arsip_bagit("ArsipPreservasi2026", {});

        var valid = run_res["sukses"] dan bag_res["apakah_sah"];
        kembalikan valid;
    "#;
    let res = jalankan(code).expect("DevOps and archival test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_spectrum_spatial_robotics_dsp_and_formal() {
    let code = r#"
        impor "modul/spasial_dan_robotika.wya";
        impor "modul/dsp_dan_multimedia.wya";
        impor "modul/formal_dan_sains_komputasi.wya";

        var robot = buat_stasiun_kontrol_robotika(10.0, 10.0);
        var move_res = robotika_gerak_ke_target(robot, 10.0, 10.0);

        var dsp_res = proses_spektrum_sinyal([1.0, 0.0, -1.0, 0.0]);

        var sat_res = selesaikan_formula_sat([[1, 2], [-1, 2]]);
        var smt_res = verifikasi_kendala_linear_smt([["x", ">=", 1.0], ["x", "<=", 10.0]]);

        var valid = (panjang(dsp_res["spektrum"]) > 0.0) dan sat_res["apakah_sat"] dan smt_res["apakah_puas"];
        kembalikan valid;
    "#;
    let res = jalankan(code).expect("Spatial, robotics, DSP, and formal test failed");
    assert_eq!(res, Value::Bool(true));
}
