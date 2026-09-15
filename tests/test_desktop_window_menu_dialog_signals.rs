use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave39_desktop_window_lifecycle() {
    let code = r#"
        var win = JendelaDesktop("Aplikasi Utama", 1024, 768)
        var cek_judul = win.judul == "Aplikasi Utama"
        var cek_lebar = win.lebar == 1024
        var cek_tinggi = win.tinggi == 768
        var cek_buka1 = jendela_apakah_terbuka(win) == benar

        jendela_set_judul(win, "Widya Enterprise")
        var cek_judul2 = win.judul == "Widya Enterprise"

        jendela_set_ukuran(win, 1920, 1080)
        var cek_lebar2 = win.lebar == 1920

        jendela_pusatkan(win)
        var cek_pos = win.posisi_x >= 0

        jendela_set_fullscreen(win, benar)
        var cek_fs = win.fullscreen == benar

        jendela_tutup(win)
        var cek_buka2 = jendela_apakah_terbuka(win) == salah

        kembalikan cek_judul dan cek_lebar dan cek_tinggi dan cek_buka1 dan cek_judul2 dan cek_lebar2 dan cek_pos dan cek_fs dan cek_buka2
    "#;
    let res = jalankan(code).expect("Desktop window test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave39_desktop_menu_and_tray() {
    let code = r#"
        var menu = MenuBilah("Edit")
        menu_tambah_item(menu, "Salin", "Ctrl+C")
        menu_tambah_item(menu, "Tempel", "Ctrl+V")
        menu_tambah_pemisah(menu)
        menu_tambah_item(menu, "Pilih Semua", "Ctrl+A")

        var cek_menu = panjang(menu.item) == 4

        var tray = BakiSistem("logo.ico", "Layanan Aktif")
        tray_set_tooltip(tray, "Layanan Berjalan")
        tray_kirim_notifikasi(tray, "Pesan", "Koneksi Terhubung")
        var cek_tray = panjang(tray.notifikasi_terakhir) > 0

        kembalikan cek_menu dan cek_tray
    "#;
    let res = jalankan(code).expect("Desktop menu test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave39_native_dialogs() {
    let code = r#"
        var buka = dialog_buka_berkas("*.txt")
        var cek_buka = panjang(buka) > 0

        var simpan = dialog_simpan_berkas("data.csv")
        var cek_simpan = panjang(simpan) > 0

        var folder = dialog_pilih_folder()
        var cek_folder = panjang(folder) > 0

        var msg = dialog_pesan("Konfirmasi", "Peringatan", "Data akan disimpan")
        var cek_msg = msg.ditekan == "OK"

        kembalikan cek_buka dan cek_simpan dan cek_folder dan cek_msg
    "#;
    let res = jalankan(code).expect("Native dialog test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave39_reactive_signals_and_effects() {
    let code = r#"
        var counter = SinyalReaktif(100)
        var cek_awal = sinyal_ambil(counter) == 100

        var reaktif_log = []
        efek_reaktif(counter, fungsi(val) {
            reaktif_log = tambah(reaktif_log, val)
        })

        // Efek reaktif dipicu langsung saat registrasi (100)
        var cek_log1 = panjang(reaktif_log) == 1

        sinyal_perbarui(counter, 200)
        var cek_val2 = sinyal_ambil(counter) == 200
        var cek_log2 = panjang(reaktif_log) == 2
        var cek_versi = counter.versi == 2

        kembalikan cek_awal dan cek_log1 dan cek_val2 dan cek_log2 dan cek_versi
    "#;
    let res = jalankan(code).expect("Reactive signal test failed");
    assert_eq!(res, Value::Bool(true));
}
