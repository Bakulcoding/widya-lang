use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave52_openxr_session_and_hand_tracking() {
    let code = r#"
        var sesi = SesiXR("IMMERSIVE_VR", "LOCAL_FLOOR")
        xr_mulai_sesi(sesi)
        var cek_aktif = sesi.status_aktif == benar

        var mata = xr_dapatkan_tampilan_mata(sesi)
        var cek_mata = mata.mata_kiri.ipd_offset < 0.0 dan mata.mata_kanan.ipd_offset > 0.0

        var tangan = PelacakTanganXR("KANAN")
        xr_set_sendi_tangan(tangan, "THUMB_TIP", 0.1, 1.0, -0.3)
        xr_set_sendi_tangan(tangan, "INDEX_TIP", 0.11, 1.01, -0.3)
        var gest = xr_hitung_pose_gestur(tangan)
        var cek_pinch = gest.cubit == benar dan gest.gestur_terdeteksi == "PINCH"

        kembalikan cek_aktif dan cek_mata dan cek_pinch
    "#;
    let res = jalankan(code).expect("OpenXR test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave52_ar_plane_and_spatial_audio() {
    let code = r#"
        var pelacak = PelacakBidangAR()
        ar_tambah_bidang(pelacak, "p1", "HORIZONTAL", 2.0, 2.0)
        var cek_plane = panjang(pelacak.bidang_terdeteksi) == 1

        var hit = ar_lakukan_hit_test(pelacak, 0.0, 1.0, -2.0)
        var cek_hit = hit.hit_ditemukan == benar

        ar_buat_jangkar_spasial(pelacak, "anc_1", 0.0, 0.0, -2.0)
        var cek_anc = panjang(pelacak.jangkar_spasial) == 1

        var audio = xr_audio_spasial_3d(16, [2.0, 0.0, 0.0], [0.0, 0.0, 0.0])
        var cek_audio = audio.jarak_meter == 2.0 dan audio.gain_atenuasi == 0.5

        var html = xr_render_aplikasi_webxr_html(SesiXR("IMMERSIVE_AR", "LOCAL_FLOOR"), "Test")
        var cek_html = panjang(html) > 50

        kembalikan cek_plane dan cek_hit dan cek_anc dan cek_audio dan cek_html
    "#;
    let res = jalankan(code).expect("AR and Spatial Audio test failed");
    assert_eq!(res, Value::Bool(true));
}
