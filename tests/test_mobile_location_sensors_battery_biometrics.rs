use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave40_gps_and_geofencing() {
    let code = r#"
        var gps = LokasiGPS(5.0, 10000)
        var pos = gps_dapatkan_posisi(gps)
        var cek_pos = pos.latitude == -6.2088 dan pos.longitude == 106.8456

        var jarak = gps_hitung_jarak(-6.2088, 106.8456, -6.9175, 107.6191)
        var cek_jarak = jarak > 100000 dan jarak < 200000

        var masuk = gps_geofence_apakah_masuk(-6.2088, 106.8456, -6.2080, 106.8450, 500.0)
        var cek_geofence = masuk == benar

        kembalikan cek_pos dan cek_jarak dan cek_geofence
    "#;
    let res = jalankan(code).expect("GPS test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave40_motion_sensors_and_haptics() {
    let code = r#"
        var accel = SensorGerak("akselerometer")
        var v_accel = sensor_baca_vektor(accel)
        var cek_accel = v_accel.z == 9.81

        var gyro = SensorGerak("giroskop")
        var v_gyro = sensor_baca_vektor(gyro)
        var cek_gyro = v_gyro.x == 0.01

        var getar = haptic_getar(200)
        var cek_getar = panjang(getar) > 0

        kembalikan cek_accel dan cek_gyro dan cek_getar
    "#;
    let res = jalankan(code).expect("Sensor test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave40_battery_power_management() {
    let code = r#"
        var bat = StatusBaterai()
        var cek_pct = bat.persentase == 88.0
        var cek_charge = baterai_apakah_mengisi(bat) == benar
        var cek_lpm = baterai_hemat_daya(bat) == salah

        kembalikan cek_pct dan cek_charge dan cek_lpm
    "#;
    let res = jalankan(code).expect("Battery test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave40_biometrics_and_keystore() {
    let code = r#"
        var auth = biometrik_autentikasi("Login", "Fingerprint")
        var cek_auth = auth.sukses == benar dan auth.metode == "Fingerprint"

        kunci_aman_simpan("secret_api_key", "WIDYA_SECRET_999")
        var val = kunci_aman_ambil("secret_api_key")
        var cek_val = buka(val) == "WIDYA_SECRET_999"

        var del = kunci_aman_hapus("secret_api_key")
        var val_after = kunci_aman_ambil("secret_api_key")
        var cek_del = del == benar dan buka_atau(val_after, "tidak_ada") == "tidak_ada"

        kembalikan cek_auth dan cek_val dan cek_del
    "#;
    let res = jalankan(code).expect("Biometrics test failed");
    assert_eq!(res, Value::Bool(true));
}
