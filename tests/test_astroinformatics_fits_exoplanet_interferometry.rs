use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave58_fits_and_wcs() {
    let code = r#"
        var fits = CitraFITS(32, 32, -32);
        fits_tambah_kartu_header(fits, "TELESCOP", "JWST", "Space Telescope");
        fits_tambah_kartu_header(fits, "CRVAL1", 100.0, "Ref RA");
        fits_tambah_kartu_header(fits, "CRPIX1", 16.0, "Ref Pix X");
        fits_tambah_kartu_header(fits, "CDELT1", -0.01, "Scale RA");
        fits_set_piksel(fits, 16, 16, 999.0);

        var wcs = fits_konversi_piksel_ke_langit_wcs(fits, 16.0, 16.0);

        var cek = wcs["ra_derajat"] == 100.0 dan wcs["ekuinoks"] == "J2000";
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("FITS and WCS test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave58_exoplanet_transit() {
    let code = r#"
        var kurva = KurvaCahayaBintang("Kepler-186", 0.5);
        transit_tambah_titik_fluks(kurva, 0.0, 1.0);
        transit_tambah_titik_fluks(kurva, 0.5, 0.99);
        var hasil = transit_analisis_kedalaman_eksoplanet(kurva);

        var cek = hasil["kedalaman_transit"] > 0.0 dan hasil["radius_planet_bumi"] > 0.0;
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Exoplanet Transit test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave58_radio_interferometry_and_astrometry() {
    let code = r#"
        var array = InterferometerRadio(230.0);
        interferometer_tambah_antena(array, "A1", 0.0, 0.0, 0.0);
        interferometer_tambah_antena(array, "A2", 100.0, 0.0, 0.0);
        var baselines = interferometer_hitung_baseline_uv(array, 0.0, 0.0);

        var astrometri = AstrometriBintang();
        var d = astrometri_hitung_jarak_paralaks(100.0); // 100 mas -> 10 pc
        var sp = astrometri_klasifikasi_spektral(5778.0, 1.0);

        var cek = baselines == 1.0 dan d["jarak_parsec"] == 10.0 dan sp["kelas_spektral"] == "G (Kuning seperti Matahari)";
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Radio Interferometry and Astrometry test failed");
    assert_eq!(res, Value::Bool(true));
}
