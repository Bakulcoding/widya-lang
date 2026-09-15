use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave43_devops_pipeline() {
    let code = r#"
        var pipa = PipaKerjaDevOps("AutoBuild")
        pipa_tambah_tahap(pipa, "Lint", ["widya lint"])
        pipa_tambah_tahap(pipa, "Test", ["cargo test"])

        var cek_tahap = panjang(pipa.tahap) == 2
        var run_ok = pipa_jalankan(pipa)
        var cek_sukses = pipa_apakah_sukses(pipa) == benar dan run_ok == benar

        kembalikan cek_tahap dan cek_sukses
    "#;
    let res = jalankan(code).expect("Devops pipeline test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave43_container_config() {
    let code = r#"
        var k = KonfigurasiKontainer("web-svc", "nginx:alpine", "80:80")
        kontainer_tambah_env(k, "ENV", "prod")

        var cek_valid = kontainer_validasi(k) == benar
        var yaml = kontainer_ke_yaml(k)
        var cek_yaml = panjang(yaml) > 20

        kembalikan cek_valid dan cek_yaml
    "#;
    let res = jalankan(code).expect("Container config test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave43_dotenv_vault() {
    let code = r#"
        var raw = "DB_PORT=5432\nDB_HOST=127.0.0.1\n"
        var v = VaultEnv(raw)

        var port = buka(env_ambil(v, "DB_PORT"))
        var cek_port = port == "5432"

        var res = env_interpolasi_teks(v, "tcp://${DB_HOST}:${DB_PORT}")
        var cek_interp = res == "tcp://127.0.0.1:5432"

        kembalikan cek_port dan cek_interp
    "#;
    let res = jalankan(code).expect("Dotenv vault test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave43_health_check_and_webhook() {
    let code = r#"
        var probe = PemeriksaKesehatan("http://app/health", "HTTP_GET")
        var res = probe_jalankan(probe)
        var cek_probe = res.sehat == benar dan res.kode_http == 200

        var alert = webhook_kirim_alert("http://webhook.url", "Slack", "WARN", "High Mem")
        var cek_alert = alert.status_kirim == "Terkirim" dan alert.platform == "Slack"

        kembalikan cek_probe dan cek_alert
    "#;
    let res = jalankan(code).expect("Health check and webhook test failed");
    assert_eq!(res, Value::Bool(true));
}
