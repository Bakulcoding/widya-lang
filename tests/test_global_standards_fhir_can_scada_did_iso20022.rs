use std::process::Command;

#[test]
fn test_global_standards_fhir_can_scada_did_iso20022() {
    let output = Command::new("cargo")
        .args(&["run", "--", "contoh/161_solusi_fhir_canbus_scada_did_iso20022.wya"])
        .output()
        .expect("Gagal menjalankan contoh 161");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    assert!(output.status.success(), "Eksekusi contoh 161 harus berhasil");
    assert!(stdout.contains("Interoperabilitas Rekam Medis Elektronik HL7 FHIR:"));
    assert!(stdout.contains("Otomotif ECU CAN Bus & UDS Diagnostic Service"));
    assert!(stdout.contains("Otomasi SCADA Industri (Modbus TCP & OPC-UA):"));
    assert!(stdout.contains("W3C DID & Verifiable Credentials (VC):"));
    assert!(stdout.contains("Standar Finansial Global ISO 20022 pacs.008:"));
    assert!(stdout.contains("SELURUH STANDAR GLOBAL DUNIA TERATASI SECARA PARIPURNA"));
}
