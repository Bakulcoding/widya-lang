use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave42_ecs_architecture() {
    let code = r#"
        var dunia = DuniaECS()
        var e1 = ecs_buat_entitas(dunia, "Hero")
        var e2 = ecs_buat_entitas(dunia, "Monster")
        var cek_id = e1 == 1 dan e2 == 2

        ecs_tambah_komponen(dunia, e1, "Posisi", { "x": 10.0, "y": 20.0 })
        var pos = buka(ecs_ambil_komponen(dunia, e1, "Posisi"))
        var cek_pos = pos.x == 10.0 dan pos.y == 20.0

        var non_exist = ecs_ambil_komponen(dunia, e1, "Senjata")
        var cek_none = buka_atau(non_exist, "kosong") == "kosong"

        var entitas_list = ecs_kueri_komponen(dunia, "Posisi")
        var cek_query = panjang(entitas_list) == 1

        kembalikan cek_id dan cek_pos dan cek_none dan cek_query
    "#;
    let res = jalankan(code).expect("ECS test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave42_physics_and_aabb_collision() {
    let code = r#"
        var b1 = BendaFisika2D("Dinamis", 1.0, 0.5, 0.1)
        b1.posisi_x = 0.0
        b1.posisi_y = 0.0
        fisika_terapkan_gaya(b1, 5.0, 0.0)
        var cek_gaya = b1.kecepatan_x == 5.0

        fisika_langkah_simulasi(b1, 0.1, 10.0)
        var cek_sim = b1.posisi_x > 0.0 dan b1.posisi_y > 0.0

        var b2 = BendaFisika2D("Statis", 0.0, 0.0, 0.0)
        b2.posisi_x = 0.0
        b2.posisi_y = 0.0
        var tabrak = fisika_deteksi_tabrakan_aabb(b1, b2)
        var cek_tabrak = tabrak == benar

        kembalikan cek_gaya dan cek_sim dan cek_tabrak
    "#;
    let res = jalankan(code).expect("Physics test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave42_tilemap_and_astar() {
    let code = r#"
        var peta = PetaUbinTilemap(5, 5, 16.0)
        tilemap_set_ubin(peta, 2, 2, "Batu", benar)
        var cek_rintangan = tilemap_apakah_rintangan(peta, 2, 2) == benar
        var cek_bebas = tilemap_apakah_rintangan(peta, 0, 0) == salah

        var rute = astar_cari_jalur(peta, 0, 0, 4, 4)
        var cek_rute = panjang(rute) >= 5

        kembalikan cek_rintangan dan cek_bebas dan cek_rute
    "#;
    let res = jalankan(code).expect("Tilemap test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave42_game_audio_and_loop() {
    let code = r#"
        var suara = SuaraGame("laser.wav", 1.0, salah)
        audio_putar(suara)
        var cek_putar = suara.sedang_memutar == benar

        audio_set_volume(suara, 0.5)
        var cek_vol = suara.volume == 0.5

        var loop_ctrl = PengaturFrameGame(60.0)
        var dt = frame_hitung_delta(loop_ctrl)
        var cek_dt = dt > 0.015 dan dt < 0.018 dan loop_ctrl.total_frame == 1

        kembalikan cek_putar dan cek_vol dan cek_dt
    "#;
    let res = jalankan(code).expect("Audio test failed");
    assert_eq!(res, Value::Bool(true));
}
