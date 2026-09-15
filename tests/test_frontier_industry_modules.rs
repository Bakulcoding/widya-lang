use std::process::Command;

#[test]
fn test_frontier_space_bms_bim_genome_sdr_etcs_gs1() {
    let output = Command::new("cargo")
        .args(&["run", "--", "contoh/163_solusi_space_bms_bim_genome_sdr_etcs_gs1.wya"])
        .output()
        .expect("Gagal menjalankan contoh 163");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    assert!(output.status.success(), "Eksekusi contoh 163 harus berhasil");
    assert!(stdout.contains("Telemetri & Perintah Satelit ESA ECSS PUS:"));
    assert!(stdout.contains("Manajemen Baterai Kendaraan Listrik (BMS):"));
    assert!(stdout.contains("Model Bangunan Konstruksi Digital BIM IFC:"));
    assert!(stdout.contains("Rekayasa Bioinformatika Mutasi Genomika VCF:"));
    assert!(stdout.contains("Radio Astronomi & SDR VITA 49 VRT Packet:"));
    assert!(stdout.contains("Transportasi Persinyalan Kereta Cepat ETCS:"));
    assert!(stdout.contains("Logistik Global & Lacak Balak GS1 EPCIS:"));
    assert!(stdout.contains("SELURUH 44 MODUL INDUSTRI TERATASI SEMPURNA TANPA TANDING"));
}
