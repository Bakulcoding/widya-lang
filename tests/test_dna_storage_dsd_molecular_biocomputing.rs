use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave56_dna_storage_encoding_and_decoding() {
    let code = r#"
        var enkoder = EnkoderPenyimpananDNA(3);
        var teks = "DNA_STORAGE_2026";
        var dna = dna_enkode_biner(enkoder, teks);
        var dekode = dna_dekode_biner(enkoder, dna);
        var gc = dna_hitung_gc_content(dna);

        var cek = teks == dekode dan panjang(dna) > 10 dan gc >= 0.0;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("DNA Storage Encoding test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave56_reed_solomon_bio_codec() {
    let code = r#"
        var korektor = KorektorGalatBioDNA(4);
        var payload = dna_tambah_primer_dan_paritas(korektor, "ACGTACGT", "ATGC", "GCAT");
        var pulih = dna_koreksi_dan_pulihkan(korektor, payload);

        var cek = panjang(payload) == 20 dan pulih["status"] == "TERKOREKSI_SEMPURNA";
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Reed Solomon Bio Codec test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave56_dsd_kinetics_and_molecular_logic() {
    let code = r#"
        var sirkuit = SirkuitDSD(37.0);
        dsd_tambah_untai(sirkuit, "InA", "t1", 50.0);
        dsd_tambah_untai(sirkuit, "Gate", "t1*", 50.0);
        dsd_tambah_untai(sirkuit, "Out", "t2", 0.0);
        dsd_tambah_untai(sirkuit, "Waste", "none", 0.0);
        dsd_tambah_reaksi(sirkuit, "InA", "Gate", "Out", "Waste", 0.1);
        var kin = dsd_simulasi_kinetika(sirkuit, 1.0, 5.0);

        var gerbang_and = GerbangLogikaMolekuler("AND", 10.0);
        var res_and = gerbang_bio_evaluasi(gerbang_and, 20.0, 15.0);

        var cek = kin["Out"] > 0.0 dan res_and["output_logika"] == benar;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("DSD & Molecular Logic test failed");
    assert_eq!(res, Value::Bool(true));
}
