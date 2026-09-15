use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave53_lif_neuron_and_stdp() {
    let code = r#"
        var neuron = NeuronLIF(-70.0, -55.0, 10.0, 1.0, 2.0)
        var cek_init = neuron.v_rest_mv == -70.0

        var res = neuron_stimulasi_arus(neuron, 3.0, 1.0)
        var cek_dv = res.potensial_v > -70.0

        var syn = SinapsisSTDP(0.5, 1.0, 0.01, 0.012, 20.0)
        var ltp = stdp_perbarui_bobot(syn, 0.0, 10.0)
        var cek_ltp = ltp.bobot_baru > 0.5

        var ltd = stdp_perbarui_bobot(syn, 20.0, 10.0)
        var cek_ltd = ltd.bobot_baru < ltp.bobot_baru

        kembalikan cek_init dan cek_dv dan cek_ltp dan cek_ltd
    "#;
    let res = jalankan(code).expect("LIF and STDP test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave53_eeg_and_bci_decoder() {
    let code = r#"
        var proc = ProsesorSinyalEEG(256.0)
        var spec = eeg_ekstrak_gelombang(proc, [10.0, 20.0, -10.0, -20.0])
        var cek_pwr = spec.daya_alpha > 0.0 dan spec.daya_beta > 0.0

        var focus = eeg_hitung_indeks_fokus(proc, spec)
        var cek_foc = focus.indeks_fokus > 0.0

        var bci = DekoderBCI("MOTOR_IMAGERY")
        var classif = bci_klasifikasi_niat(bci, spec)
        var cek_bci = classif.probabilitas_keyakinan > 0.8

        kembalikan cek_pwr dan cek_foc dan cek_bci
    "#;
    let res = jalankan(code).expect("EEG and BCI test failed");
    assert_eq!(res, Value::Bool(true));
}
