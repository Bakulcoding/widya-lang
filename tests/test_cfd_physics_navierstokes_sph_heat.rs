use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave51_navier_stokes_and_heat() {
    let code = r#"
        var flu = GridFluida2D(8, 0.001, 0.001, 0.1)
        var cek_flu = flu.resolusi == 8.0

        fluida_tambah_densitas(flu, 4, 4, 100.0)
        fluida_tambah_kecepatan(flu, 4, 4, 1.0, 1.0)
        fluida_langkah_simulasi(flu)

        var term = KisiTermal2D(8, 8, 0.1, 0.1)
        termal_set_suhu(term, 4, 4, 100.0)
        termal_langkah_difusi(term)
        var cek_term = term.lebar == 8.0

        kembalikan cek_flu dan cek_term
    "#;
    let res = jalankan(code).expect("CFD Navier-Stokes test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave51_sph_and_wave_equation() {
    let code = r#"
        var sph = SistemPartikelSPH(100, 0.05, 1000.0, 0.001)
        sph_tambah_partikel(sph, 0.5, 1.0, 0.0, 0.0)
        sph_langkah_waktu(sph, 0.01, -9.81)
        var cek_sph = panjang(sph.partikel) == 1 dan sph.partikel[0].y < 1.0

        var wave = KisiGelombang2D(10, 10, 1.0, 0.99)
        gelombang_beri_gangguan(wave, 5, 5, 20.0)
        gelombang_langkah_waktu(wave, 0.1)
        var cek_wave = wave.lebar == 10.0

        kembalikan cek_sph dan cek_wave
    "#;
    let res = jalankan(code).expect("SPH and Wave test failed");
    assert_eq!(res, Value::Bool(true));
}
