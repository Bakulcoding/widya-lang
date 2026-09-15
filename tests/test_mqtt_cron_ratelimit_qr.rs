use widya::jalankan;
use widya::value::Value;

#[test]
fn test_mqtt_dan_cron() {
    let code = r#"
        tetap b = BrokerMQTT();
        misal terpanggil = 0;
        
        fungsi h(t, p) {
            kembalikan 1;
        }
        mqtt_langganan(b, "c1", "a/+/c", h);
        tetap n = mqtt_publikasi(b, "a/b/c", "data");
        
        tetap sc = PenjadwalCron();
        fungsi task() { kembalikan 0; }
        cron_jadwalkan(sc, "j1", "*/5 * * * *", task);
        
        tetap match_ok = cron_cocok_waktu("*/5 * * * *", 15, 0);
        tetap match_no = cron_cocok_waktu("*/5 * * * *", 16, 0);
        tetap run_res = cron_jalankan_langkah(sc, 15, 0);
        
        kembalikan [n == 1.0, match_ok, match_no, run_res == 1.0];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::Bool(false));
        assert_eq!(items[3], Value::Bool(true));
    } else {
        panic!("Expected array return value");
    }
}

#[test]
fn test_ratelimit_dan_qr() {
    let code = r#"
        tetap rl = PembatasLaju(2.0, 1.0);
        tetap ok1 = izinkan_permintaan(rl, 1.0);
        tetap ok2 = izinkan_permintaan(rl, 1.0);
        tetap ok3 = izinkan_permintaan(rl, 1.0);
        
        tetap qr = buat_matriks_qr("hello");
        tetap ansi = render_qr_ansi(qr);
        tetap svg = render_qr_svg(qr, 5);
        
        kembalikan [ok1, ok2, ok3, qr["ukuran"] == 21.0, panjang(ansi) > 100, panjang(svg) > 100];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::Bool(false));
        assert_eq!(items[3], Value::Bool(true));
        assert_eq!(items[4], Value::Bool(true));
        assert_eq!(items[5], Value::Bool(true));
    } else {
        panic!("Expected array return value");
    }
}
