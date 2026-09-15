use std::process::Command;

#[test]
fn test_deep_industrial_maritime_avionics_ev_5g_dicom_ros2() {
    let output = Command::new("cargo")
        .args(&["run", "--", "contoh/162_solusi_deep_industry_maritim_avionik_ev_dan_ros2.wya"])
        .output()
        .expect("Gagal menjalankan contoh 162");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    assert!(output.status.success(), "Eksekusi contoh 162 harus berhasil");
    assert!(stdout.contains("Pelacakan Navigasi Maritim & GPS Kapal:"));
    assert!(stdout.contains("Avionika ARINC 429 Kokpit Altimeter:"));
    assert!(stdout.contains("SPKLU Pengisian Daya Kendaraan Listrik (OCPP 2.0.1):"));
    assert!(stdout.contains("Smart Grid Gardu Induk IEC 61850 GOOSE:"));
    assert!(stdout.contains("Telekomunikasi 5G Core Diameter OCS Real-Time Charging:"));
    assert!(stdout.contains("Citra Radiologi Medis DICOM PACS:"));
    assert!(stdout.contains("Geofisika & Eksplorasi Seismik SEG-Y:"));
    assert!(stdout.contains("Robotika Industri ROS2 DDS Wire Serialization:"));
    assert!(stdout.contains("SELURUH DOMAIN INDUSTRI DUNIA TELAH TERCATAT LENGKAP & SEMPURNA"));
}
