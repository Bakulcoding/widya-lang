use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave44_pcap_parsing_and_filter() {
    let code = r#"
        var pcap = ParserPCAP("network.pcap")
        var cek_total = pcap.total_paket == 3

        var dns_pkts = pcap_saring_protokol(pcap, "DNS")
        var cek_dns = panjang(dns_pkts) == 1

        var tls_pkts = pcap_saring_protokol(pcap, "TLS")
        var cek_tls = panjang(tls_pkts) == 1

        kembalikan cek_total dan cek_dns dan cek_tls
    "#;
    let res = jalankan(code).expect("PCAP test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave44_magic_bytes_and_entropy() {
    let code = r#"
        var elf = forensik_deteksi_magic_byte([0x7F, 0x45, 0x4C, 0x46])
        var cek_elf = elf.tipe == "ELF_EXECUTABLE"

        var png = forensik_deteksi_magic_byte([0x89, 0x50, 0x4E, 0x47])
        var cek_png = png.tipe == "IMAGE_PNG"

        var ent = forensik_hitung_entropi("AAAAAAAAAA")
        var cek_ent = ent == 0.0

        var strings = forensik_ekstrak_string([0x00, 0x57, 0x49, 0x44, 0x59, 0x41, 0x00], 4)
        var cek_str = panjang(strings) == 1 dan strings[0] == "WIDYA"

        kembalikan cek_elf dan cek_png dan cek_ent dan cek_str
    "#;
    let res = jalankan(code).expect("Forensics magic byte and entropy test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave44_yara_matcher() {
    let code = r#"
        var yara = AturanYARA("TrojanRule", ["malicious_payload_abc", "backdoor_cmd"])
        var target_bersih = "aplikasi normal tanpa virus"
        var target_infeksi = "string berisi malicious_payload_abc di dalamnya"

        var res1 = yara_pindai_target(yara, target_bersih)
        var cek_clean = res1.cocok == salah

        var res2 = yara_pindai_target(yara, target_infeksi)
        var cek_infected = res2.cocok == benar dan panjang(res2.pola_ditemukan) == 1

        kembalikan cek_clean dan cek_infected
    "#;
    let res = jalankan(code).expect("YARA test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave44_siem_log_analyzer() {
    let code = r#"
        var siem = PenganalisisLogSIEM()
        var logs = [
            { "ip": "10.0.0.1", "uri": "/index.html" },
            { "ip": "10.0.0.2", "uri": "/login?id=1' OR '1'='1" },
            { "ip": "10.0.0.3", "uri": "/passwd/../../etc/passwd" },
            { "ip": "10.0.0.4", "uri": "/auth" },
            { "ip": "10.0.0.4", "uri": "/auth" }
        ]

        var attacks = siem_deteksi_pola_serangan(siem, logs)
        var cek_attacks = panjang(attacks) == 2

        var anomali = siem_deteksi_anomali_ip(siem, logs, 2)
        var cek_anomali = panjang(anomali) == 1 dan anomali[0].ip == "10.0.0.4"

        kembalikan cek_attacks dan cek_anomali
    "#;
    let res = jalankan(code).expect("SIEM test failed");
    assert_eq!(res, Value::Bool(true));
}
