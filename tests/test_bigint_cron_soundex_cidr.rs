use widya::jalankan;
use widya::value::Value;

#[test]
fn test_widya_bigint() {
    let code = r#"
        misal a = BigInt("99999999999999999999");
        misal b = BigInt("1");
        misal tambah = bigint_tambah(a, b);
        misal str_tambah = bigint_ke_string(tambah);

        misal p2_8 = bigint_pangkat(BigInt("2"), 8);
        misal str_p2_8 = bigint_ke_string(p2_8);

        misal cmp1 = bigint_bandingkan(a, b);

        kembalikan [str_tambah, str_p2_8, cmp1];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::String("100000000000000000000".to_string()));
        assert_eq!(items[1], Value::String("256".to_string()));
        assert_eq!(items[2], Value::Number(1.0));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_cron_parser() {
    let code = r#"
        misal cron = cron_parse("*/10 * * * *");
        misal desc = cron_deskripsi_manusia(cron);
        misal t0 = 1700000000;
        misal t_next = cron_waktu_berikutnya(cron, t0);
        misal diff = t_next - t0;
        kembalikan [desc, diff > 0];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[1], Value::Bool(true));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_soundex() {
    let code = r#"
        misal s_smith = soundex_kode("Smith");
        misal s_smythe = soundex_kode("Smythe");
        misal mirip = soundex_mirip("Smith", "Smythe");
        misal bukan_mirip = soundex_mirip("Smith", "Johnson");
        kembalikan [s_smith, s_smythe, mirip, bukan_mirip];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::String("S530".to_string()));
        assert_eq!(items[1], Value::String("S530".to_string()));
        assert_eq!(items[2], Value::Bool(true));
        assert_eq!(items[3], Value::Bool(false));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_ipcidr() {
    let code = r#"
        misal net = IPJaringan("192.168.1.0/24");
        misal net_addr = ip_network_address(net);
        misal bcast_addr = ip_broadcast_address(net);
        misal hosts = ip_jumlah_host(net);

        misal in_range1 = ip_apakah_dalam_rentang("192.168.1.100", net);
        misal in_range2 = ip_apakah_dalam_rentang("192.168.2.100", net);

        kembalikan [net_addr, bcast_addr, hosts, in_range1, in_range2];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::String("192.168.1.0".to_string()));
        assert_eq!(items[1], Value::String("192.168.1.255".to_string()));
        assert_eq!(items[2], Value::Number(254.0));
        assert_eq!(items[3], Value::Bool(true));
        assert_eq!(items[4], Value::Bool(false));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}
