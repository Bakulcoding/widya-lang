use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave57_sat_dpll_solver() {
    let code = r#"
        var sat = PemecahSAT();
        sat_tambah_klausa(sat, [1, 2]);
        sat_tambah_klausa(sat, [-1, 3]);
        sat_tambah_klausa(sat, [-2, -3]);

        var is_sat = sat_selesaikan_dpll(sat);
        var model = sat_ambil_model(sat);

        var cek = is_sat == benar dan (model["x1"] == benar atau model["x1"] == salah);
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("SAT Solver test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave57_smt_linear_solver() {
    let code = r#"
        var smt = PemecahSMT("QF_LRA");
        smt_tambah_variabel(smt, "x", "Real");
        smt_tambah_kendala(smt, "x", ">=", 2.0);
        smt_tambah_kendala(smt, "x", "<=", 4.0);

        var stat = smt_periksa_kepuasan(smt);
        var doc = smt_ekspor_smtlib2(smt);

        var cek = stat == "SAT" dan panjang(doc) > 20;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("SMT Solver test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave57_hoare_and_fol_prover() {
    let code = r#"
        var hoare = PemeriksaHoare();
        var h_res = hoare_verifikasi_kondisi(hoare, "x > 0", "x = x + 1", "x > 1");
        var wp = hoare_hitung_weakest_precondition(hoare, "x", "x + 1", "x > 1");

        var fol = PembuktiTeoremaFOL();
        fol_tambah_aksioma(fol, "Mortal", "Socrates");
        fol_tambah_tujuan(fol, "Mortal", "Socrates");
        var f_res = fol_buktikan_kontradiksi_resolusi(fol);

        var cek = h_res["hoare_triple_valid"] == benar dan wp == "(x + 1) > 1" dan f_res["teorema_terbukti"] == benar;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Hoare Logic & FOL Prover test failed");
    assert_eq!(res, Value::Bool(true));
}
