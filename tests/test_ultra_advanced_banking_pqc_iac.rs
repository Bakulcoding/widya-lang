use std::process::Command;

#[test]
fn test_ultra_advanced_iso8583_pqc_and_cloud_iac() {
    let output = Command::new("cargo")
        .args(&["run", "--", "contoh/160_solusi_iso8583_pqc_dan_cloud_iac.wya"])
        .output()
        .expect("Gagal menjalankan contoh 160");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    assert!(output.status.success(), "Eksekusi contoh 160 harus berhasil");
    assert!(stdout.contains("Pemrosesan Transaksi Perbankan ISO 8583:"));
    assert!(stdout.contains("NIST Post-Quantum Cryptography (PQC)"));
    assert!(stdout.contains("Cloud Infrastructure-as-Code (IaC) Spec"));
    assert!(stdout.contains("SELURUH DOMAIN FRONTIER & ENTERPRISE TERATASI 100%"));
}
