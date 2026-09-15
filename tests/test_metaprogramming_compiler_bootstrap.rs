use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave66_grammar_and_macro() {
    let code = r#"
        var dsl = MesinTataBahasaDSL("TestDSL");
        grammar_tambah_aturan_produksi(dsl, "E", "T '+' T");
        var st = grammar_bangun_parser(dsl);
        var ast = grammar_urai_kode_ke_ast(dsl, "10 + 20");

        var makro = MesinMakroHigienis();
        var splice = makro_evaluasi_quasiquote_splice(makro, "val #(name)", {"name": "x"});
        var exp = makro_ekspansi_sintaksis(makro, ast);

        var cek = st["status"] == "PARSER_BERHASIL_DIBANGUN" dan splice["kode_terekspansi"] == "val x" dan exp["level_ekspansi"] == 1.0;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Grammar and Macro test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave66_bootstrap_and_type_inference() {
    let code = r#"
        var boot = PengendaliBootstrapKompilator("TestBoot");
        bootstrap_inisialisasi_tahap(boot, "S0", "Rust", "C99");
        bootstrap_inisialisasi_tahap(boot, "S1", "Widya", "C99");
        var e0 = bootstrap_kompilasi_sumber_ke_target(boot, "S0", "src_code", "C99");
        var e1 = bootstrap_kompilasi_sumber_ke_target(boot, "S1", "src_code", "C99");
        var ver = bootstrap_verifikasi_kesetaraan_biner(boot, "S0", "S1");

        var chk = PemeriksaTipeInferensi();
        var uni = tipe_unifikasi_persamaan(chk, "Int", "Int");
        var inf = tipe_inferensi_ekspresi_ast(chk, "10 + 20");

        var cek = ver["kesetaraan_biner"] == benar dan uni["unifikasi_sukses"] == benar dan inf["status"] == "TIPE_STATIS_VALID";
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Bootstrap and Type Inference test failed");
    assert_eq!(res, Value::Bool(true));
}
