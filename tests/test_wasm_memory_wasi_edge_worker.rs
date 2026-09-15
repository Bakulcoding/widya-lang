use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave45_wasm_binary_and_validation() {
    let code = r#"
        var modul = ModulWasm("test_mod")
        wasm_tambah_fungsi(modul, "add", ["i32", "i32"], "i32", ["local.get 0", "local.get 1", "i32.add"])

        var biner = wasm_bangun_biner(modul)
        var cek_biner = panjang(biner) > 8

        var valid = wasm_validasi_biner(biner)
        var cek_valid = valid == benar

        var invalid = wasm_validasi_biner([0x00, 0x00, 0x00, 0x00])
        var cek_invalid = invalid == salah

        kembalikan cek_biner dan cek_valid dan cek_invalid
    "#;
    let res = jalankan(code).expect("Wasm binary test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave45_wasm_linear_memory() {
    let code = r#"
        var mem = MemoriWasmLinear(1, 4)
        var cek_cap = mem.kapasitas_byte == 65536

        var wr = wasm_tulis_memori(mem, 100, "WASM_MEMORY_DATA")
        var cek_wr = wr == 16

        var rd = wasm_baca_memori(mem, 100, 16)
        var cek_rd = rd == "WASM_MEMORY_DATA"

        kembalikan cek_cap dan cek_wr dan cek_rd
    "#;
    let res = jalankan(code).expect("Wasm linear memory test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave45_wasi_and_js_bridge() {
    let code = r#"
        var wasi = WASIContext(["app.wasm"], { "ENV": "test" })
        var cek_wasi = wasi.versi_wasi == "wasi_snapshot_preview1"

        var jembatan = JembatanWasmJS()
        wasm_js_daftarkan_fungsi(jembatan, "kali_dua", fungsi(n) {
            kembalikan n * 2
        })

        var res = wasm_js_panggil(jembatan, "kali_dua", [21])
        var cek_js = res == 42

        kembalikan cek_wasi dan cek_js
    "#;
    let res = jalankan(code).expect("WASI and JS Bridge test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave45_edge_worker() {
    let code = r#"
        var worker = PekerjaEdge("/api/test")
        var req = { "url": "https://edge.widya.dev/api/test", "metode": "GET" }
        var res = edge_tangani_permintaan(worker, req)

        var cek_status = res.status == 200
        var cek_latensi = res.latensi_eksekusi_ms < 10.0
        var cek_cnt = worker.total_request_ditangani == 1

        kembalikan cek_status dan cek_latensi dan cek_cnt
    "#;
    let res = jalankan(code).expect("Edge worker test failed");
    assert_eq!(res, Value::Bool(true));
}
