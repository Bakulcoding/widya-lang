use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave62_oais_lifecycle() {
    let code = r#"
        var oais = PaketArsipOAIS("AIP_TEST", "Koleksi");
        oais_tambah_konten_sip(oais, "file.txt", "text/plain", 100.0, "abc");
        oais_tambah_metadata_premis(oais, "E1", "INGEST", "Agent1");
        oais_transformasi_ke_aip(oais, 50.0);
        var dip = oais_hasilkan_paket_dip(oais, "PUBLIK");

        var cek = oais["status_kunci_arsip"] == "TERKUNCI_IMUTABEL" dan dip["izin_akses"] == "PUBLIK";
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("OAIS test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave62_bagit_packaging() {
    let code = r#"
        var bag = PaketBagIt("test_bag");
        bagit_tambah_berkas_payload(bag, "test.txt", "konten tes");
        bagit_set_metadata_info(bag, "Admin", "Desc", "Org");
        var man = bagit_hasilkan_manifest(bag);
        var verif = bagit_verifikasi_integritas(bag);

        var cek = verif["valid_and_complete"] == benar dan verif["jumlah_payload"] == 1.0;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("BagIt test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave62_warc_and_bitrot_scrubbing() {
    let code = r#"
        var warc = PerekamWARC("WebArchive");
        warc_tambah_rekaman_http(warc, "https://example.com", "2026-09-14", 200, "Header", "BodyText");
        var dig = warc_hitung_digest_fixity(warc, "https://example.com");

        var scrub = PembersihBitRot(4096);
        scrub_daftarkan_blok_data(scrub, "B1", "DATA_ORIGINAL");
        scrub_buat_paritas_pemulihan(scrub, ["B1"]);
        scrub_simulasi_korupsi_bit(scrub, "B1", 0, "DATA_CORRUPT");
        var res = scrub_pindai_dan_perbaiki_bitrot(scrub);

        var cek = res["blok_rusak_terdeteksi"] == 1.0 dan res["blok_berhasil_dipulihkan"] == 1.0;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("WARC and BitRot test failed");
    assert_eq!(res, Value::Bool(true));
}
