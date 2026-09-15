use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave50_fft_and_ifft() {
    let code = r#"
        var sig = [1.0, 0.0, -1.0, 0.0]
        var fft = dsp_fft_maju(sig)
        var cek_fft = panjang(fft) == 4 dan fft[0].magnitudo == 0.0

        var ifft = dsp_fft_balik(fft)
        var cek_ifft = panjang(ifft) == 4 dan ifft[0] > 0.99 dan ifft[0] < 1.01

        kembalikan cek_fft dan cek_ifft
    "#;
    let res = jalankan(code).expect("FFT/IFFT test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave50_filter_biquad_and_convolution() {
    let code = r#"
        var lpf = FilterBiquadIIR("LOWPASS", 500.0, 44100.0, 0.7071)
        var filtered = filter_proses_sinyal(lpf, [1.0, 1.0, 1.0, 1.0])
        var cek_flt = panjang(filtered) == 4

        var conv = dsp_konvolusi_sinyal([1.0, 2.0], [0.5, 0.5])
        var cek_conv = panjang(conv) == 3 dan conv[0] == 0.5 dan conv[1] == 1.5

        kembalikan cek_flt dan cek_conv
    "#;
    let res = jalankan(code).expect("Filter and Convolution test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave50_spectrogram_and_compressor() {
    let code = r#"
        var audio = [1.0, 0.5, -0.5, -1.0, 1.0, 0.5, -0.5, -1.0, 1.0, 0.5, -0.5, -1.0, 1.0, 0.5, -0.5, -1.0]
        var spec = dsp_spektrogram(audio, 8, 4)
        var cek_spec = spec.total_frame == 3

        var comp = dsp_kompresor_dinamis(audio, -6.0, 2.0)
        var cek_comp = panjang(comp) == 16 dan comp[0] < 1.0

        kembalikan cek_spec dan cek_comp
    "#;
    let res = jalankan(code).expect("Spectrogram and Compressor test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave50_video_frame_processing() {
    let code = r#"
        var frame = BingkaiVideo(640, 480, "YUV420P")
        var cek_init = frame.lebar == 640.0 dan frame.tinggi == 480.0

        var rgb = video_konversi_yuv_ke_rgb(frame)
        var cek_rgb = rgb.format_piksel == "RGB24"

        var scaled = video_ubah_skala_bilinear(rgb, 320, 240)
        var cek_scaled = scaled.lebar == 320.0 dan scaled.tinggi == 240.0

        kembalikan cek_init dan cek_rgb dan cek_scaled
    "#;
    let res = jalankan(code).expect("Video frame test failed");
    assert_eq!(res, Value::Bool(true));
}
