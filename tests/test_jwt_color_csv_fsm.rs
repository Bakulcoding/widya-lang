use widya::jalankan;
use widya::value::Value;

#[test]
fn test_jwt_dan_warna() {
    let code = r##"
        misal payload = {
            "user_id": 101,
            "role": "admin"
        };
        tetap token = buat_jwt(payload, "rahasia123", 3600);
        tetap verif_ok = verifikasi_jwt(token, "rahasia123");
        tetap verif_fail = verifikasi_jwt(token, "salah");

        misal rgb = hex_ke_rgb("#3498db");
        tetap hex_back = rgb_ke_hex(rgb.r, rgb.g, rgb.b);
        misal hsl = rgb_ke_hsl(rgb.r, rgb.g, rgb.b);
        misal rgb_from_hsl = hsl_ke_rgb(hsl.h, hsl.s, hsl.l);

        tetap kontras = hitung_kontras_wcag("#ffffff", "#000000");

        kembalikan [
            verif_ok.sah,
            verif_ok.payload.role == "admin",
            verif_fail.sah == salah,
            hex_back == "#3498db",
            kontras == 21.0
        ];
    "##;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::Bool(true));
        assert_eq!(items[3], Value::Bool(true));
        assert_eq!(items[4], Value::Bool(true));
    } else {
        panic!("Harus mengembalikan array");
    }
}

#[test]
fn test_csv_dan_fsm() {
    let code = r##"
        misal csv_text = "id,nama,nilai\n1,Alice,90\n2,\"Bob, Jr.\",85";
        misal parsed = urai_csv(csv_text);
        tetap p_len = panjang(parsed);

        misal fsm = MesinStatus("DIAM");
        fsm_tambah_transisi(fsm, "DIAM", "JALAN", "BERGERAK");
        fsm_tambah_transisi(fsm, "BERGERAK", "BERHENTI", "DIAM");

        tetap awal = fsm_status_saat_ini(fsm);
        tetap dpt_jalan = fsm_dapat_picu(fsm, "JALAN");
        tetap dpt_stop = fsm_dapat_picu(fsm, "BERHENTI");

        tetap ok1 = fsm_picu(fsm, "JALAN");
        tetap st1 = fsm_status_saat_ini(fsm);

        tetap ok_bad = fsm_picu(fsm, "JALAN");
        tetap ok2 = fsm_picu(fsm, "BERHENTI");
        tetap st2 = fsm_status_saat_ini(fsm);

        kembalikan [
            p_len == 2,
            parsed[0].nama == "Alice",
            parsed[1].nama == "Bob, Jr.",
            awal == "DIAM",
            dpt_jalan == benar,
            dpt_stop == salah,
            ok1 == benar,
            st1 == "BERGERAK",
            ok_bad == salah,
            ok2 == benar,
            st2 == "DIAM"
        ];
    "##;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        for item in items.iter() {
            assert_eq!(*item, Value::Bool(true));
        }
    } else {
        panic!("Harus mengembalikan array");
    }
}
