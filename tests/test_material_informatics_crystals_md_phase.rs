use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave59_crystal_and_cif() {
    let code = r#"
        var k = KisiKristal3D("NaCl");
        kristal_set_parameter_sel(k, 5.64, 5.64, 5.64, 90.0, 90.0, 90.0);
        kristal_tambah_atom_fraksional(k, "Na", 0.0, 0.0, 0.0, 22.99);
        kristal_tambah_atom_fraksional(k, "Cl", 0.5, 0.5, 0.5, 35.45);

        var d = kristal_hitung_densitas_teoritis(k);
        var cif = kristal_ekspor_cif(k);

        var cek = d["volume_angstrom3"] > 100.0 dan d["densitas_g_cm3"] > 0.0 dan panjang(cif) > 50;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Crystal and CIF test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave59_molecular_dynamics_verlet() {
    let code = r#"
        var md = SimulatorMolekulerMD(0.01, 3.4, 40.0);
        md_tambah_partikel(md, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0);
        md_tambah_partikel(md, 4.0, 0.0, 0.0, -100.0, 0.0, 0.0);

        md_langkah_integrasi_verlet(md, 1.0);
        var e = md_hitung_energi_sistem(md);

        var cek = e["energi_kinetik_ev"] > 0.0 dan e["temperatur_kelvin"] > 0.0;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Molecular dynamics test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave59_phase_diagram_and_descriptors() {
    let code = r#"
        var paduan = DiagramFasePaduan("A", "B");
        fase_set_parameter_interaksi(paduan, 5000.0);
        var g = fase_hitung_energi_gibbs(paduan, 0.5, 1000.0);
        var t = fase_analisis_kesetimbangan_tuas(paduan, 0.5, 0.2, 0.8);

        var desc = DeskriptorMaterial();
        var tol = material_hitung_faktor_toleransi_perovskit(160.0, 60.0, 140.0);
        var stab = material_prediksi_stabilitas_energi(0.9, 3.0);

        var cek = g["delta_h_mix_j_mol"] > 0.0 dan t["fraksi_fase_alfa"] == 0.5 dan tol["faktor_toleransi_goldschmidt"] > 0.9;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Phase diagram and descriptors test failed");
    assert_eq!(res, Value::Bool(true));
}
