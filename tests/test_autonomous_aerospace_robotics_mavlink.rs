use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave49_mavlink_protocol() {
    let code = r#"
        var msg = PesanMAVLink(1, 1, 0)
        var hb = mavlink_enkode_heartbeat(msg, "QUADROTOR", "ACTIVE", 128)
        var cek_hb = panjang(hb) == 21

        var dec = mavlink_dekode(hb)
        var cek_dec = dec.system_id == 1 dan dec.crc_valid == benar

        kembalikan cek_hb dan cek_dec
    "#;
    let res = jalankan(code).expect("MAVLink test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave49_ahrs_quaternion() {
    let code = r#"
        var q = KuaternionOrientasi(1.0, 0.0, 0.0, 0.0)
        var euler = ahrs_ke_sudut_euler(q)
        var cek_euler = euler.roll_deg == 0.0 dan euler.pitch_deg == 0.0 dan euler.yaw_deg == 0.0

        var q2 = ahrs_fusi_madgwick_step(q, [0.1, 0.0, 0.0], [0.0, 0.0, 9.81], 0.1, 0.04)
        var cek_q2 = q2.w < 1.0 dan q2.x > 0.0

        kembalikan cek_euler dan cek_q2
    "#;
    let res = jalankan(code).expect("AHRS test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave49_robot_kinematics() {
    let code = r#"
        var arm = LenganRobotik2DOF(10.0, 10.0)
        var fwd = kinematika_maju_2dof(arm, 0.0, 0.0)
        var cek_fwd = fwd.x == 20.0 dan fwd.y == 0.0

        var inv = kinematika_balik_2dof(arm, 10.0, 10.0)
        var cek_inv = inv.solusi_valid == benar

        kembalikan cek_fwd dan cek_inv
    "#;
    let res = jalankan(code).expect("Kinematics test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave49_orbital_mechanics() {
    let code = r#"
        var orbit = ElemenOrbitKepler(7000.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        var pos = orbit_hitung_posisi_kepler(orbit)
        var cek_r = pos.radius_orbital_km == 7000.0
        var cek_v = pos.kecepatan_km_s > 7.0 dan pos.kecepatan_km_s < 8.0

        kembalikan cek_r dan cek_v
    "#;
    let res = jalankan(code).expect("Orbit test failed");
    assert_eq!(res, Value::Bool(true));
}
