use std::process::Command;

#[test]
fn test_apex_industrial_nuclear_chemical_protein_wind_geotiff_v2x() {
    let output = Command::new("cargo")
        .args(&["run", "--", "contoh/164_solusi_nuklir_isa88_protein_turbin_geotiff_v2x.wya"])
        .output()
        .expect("Gagal menjalankan contoh 164");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    assert!(output.status.success(), "Eksekusi contoh 164 harus berhasil");
    assert!(stdout.contains("Fisika Nuklir & Perisai Radiasi"));
    assert!(stdout.contains("Otomasi Pabrik Kimia & Farmasi ISA-88 Batch Control:"));
    assert!(stdout.contains("Bioinformatika Makromolekul 3D Protein PDB:"));
    assert!(stdout.contains("Smart Energy Turbin Angin IEC 61400-25:"));
    assert!(stdout.contains("Penginderaan Jauh Satelit GeoTIFF (NDVI Vegetasi):"));
    assert!(stdout.contains("Otomotif Nirkabel V2X & Keselamatan C-ITS CAM (ETSI):"));
    assert!(stdout.contains("SELURUH 50 MODUL STANDAR INDUSTRI TERPENUHI LENGKAP & SEMPURNA"));
}
