# Widya Web & Desktop Level Industri — Specification & Architecture

## 1. Tujuan

Menjadikan Widya layak dipakai membangun **aplikasi web-based dan desktop produksi** dengan mengganti seluruh lapisan jaringan/UI yang saat ini **masih simulasi** menjadi implementasi **nyata**:

1. **HTTP client** asli (real HTTP/HTTPS, TLS, timeout, header, streaming).
2. **HTTP server** asli (bind port, routing GET/POST, JSON, static file, SSE) dengan handler berupa fungsi Widya.
3. **WebSocket** asli (RFC 6455: handshake + framing, ping/pong, close; server & client).
4. **Desktop app** asli (window via WebView2/wry dengan backend server lokal, plus opsi `widya app` bundling browser-app-mode).
5. **FFI C/DLL** asli (muat DLL dan panggil fungsi native Windows/C).

Semua harus **backward compatible**: tidak mengubah perilaku/status 1158 builtin lain, interpreter, LSP, studio, dan seluruh test (`cargo test --all` = 283 lulus).

## 2. Hasil Audit Kondisi Saat Ini (2026-09-16)

| Builtin / Area | Status Saat Ini | Lokasi Kode | Kesimpulan |
|---|---|---|---|
| `ServerHttp(port)` | Membuat `HashMap` palsu, tidak bind socket | `src/stdlib/mod.rs:3333` `builtin_http_server` | **Simulasi** |
| `http_get`/`http_post` | Respons JSON hardcoded, bukan HTTP asli | `src/stdlib/mod.rs:3347/3362` | **Simulasi** |
| `ServerWebSocket` | Map palsu berisi klien fiktif | `src/stdlib/mod.rs:3706` | **Simulasi** |
| `ServerGraphQL` / `ServerGRPC` / `ServerDNS` | Map palsu | stdlib | **Simulasi** |
| `JendelaDesktop(...)` | Map palsu, tidak membuka window | `src/stdlib/mod.rs:23776` | **Simulasi** |
| `ffi_call_generic` / `panggil_c_lib` | `println` lalu return `0.0`/string | `src/stdlib/mod.rs:3690/3696` | **Simulasi** |
| `render_html`, `render_dashboard_html`, `render_game_html`, `render_adegan_html`, `render_pemutar_audio_html`, `render_markdown_html` | Menghasilkan string HTML dari widget/Value Widya | stdlib (berfungsi) | **Nyata** (template engine HTML saja) |
| Engine HTTP studio (`studio.rs`) | HTTP server nyata: thread-per-connection, routing, CORS, JSON, SSE, static, rate limit | `src/studio.rs` | **Nyata** — aset utama untuk dijadikan engine bersama |
| Concurrency `buat_utas`/`saluran` | Menjalankan body fungsi di thread baru dengan `Interpreter` baru + pertukaran nilai via `ValueSerial` | `src/stdlib/mod.rs:2250` | **Nyata** — pola baku untuk callback lintas thread |
| FFI node, wasm, ebpf, gpu, llvm, compiler | Di luar cakupan dokumen ini | — | Lihat roadmap lanjutan |

**Kesimpulan audit:** komponen web/desktop yang ada **belum level industri** — semuanya *demo/simulasi*. Yang nyata dan bisa didaur ulang: engine HTTP studio, pola concurrency `ValueSerial`, interpreter & LSP.

## 3. Kendala Arsitektur & Keputusan Design

### 3.1 Kendala utama: `Value` berbasis `Rc<RefCell>` (bukan `Send`)
- `Value::Map(Rc<RefCell<HashMap>>)`, `Environment` berisi `Rc` → **tidak bisa dipindah lintas thread**.
- Callback Widya (fungsi yang diregistrasikan sebagai handler server) tidak dapat dipanggil langsung dari thread lain jika membawa environment.

### 3.2 Pola yang dipakai (sudah terbukti di `buat_utas`, `src/stdlib/mod.rs:2250`)
- Untuk menjalankan handler di thread server:
  1. Salin **body AST** fungsi handler (`f.body.clone()`).
  2. Buat `Interpreter::new()` **baru** di thread tujuan.
  3. Sebelum eksekusi, bind parameter fungsi ke nilai request yang masuk.
  4. Masukkan/masukkan kembalian nilai lewat `ValueSerial` (type `Send` yang sudah ada).
- **Konsekuensi (dokumentasikan):** handler server berjalan di environment baru, hanya menerima argumen request — state silang utas memakai `saluran`/DB global, konsisten dengan semantik `buat_utas`.

### 3.3 Pilihan bentuk server
- `ServerHttp` **default single-thread event loop blocking** pada thread pemanggil — deterministik, tanpa masalah `Send`.
- Untuk concurrent: `ServerHttp` menyediakan mode `utas=true` yang menjalankan tiap request di `Interpreter` baru per koneksi (pola 3.2), jadi request bisa paralel tanpa berbagi `Rc`.
- Engine HTTP dipindah dari `studio.rs` ke modul bersama `src/web/` dan **studio memakai ulang modul tersebut** (tanpa duplikasi) — rawat backward compat studio.

### 3.4 Desktop
- Arsitektur Tauri-like: **HTML/JS (dari `render_*` atau proyek user) + backend server lokal (engine `src/web/`) + shell window**.
- Dua mode:
  - `widya app jalan main.wya` → jalankan server lokal + buka browser **app-mode** (Chromium `--app`) — no dependency berat, lintas platform.
  - `JendelaDesktop` → **WebView2** nyata (crate `wry`/`webview2-com`) menunjuk `http://127.0.0.1:port/` — window asli Windows. Komunikasi UI↔backend via HTTP ke server lokal (IPC model Tauri), menghindari pemindahan `Rc` lintas thread.

## 4. API Surface yang Direncanakan

### 4.1 HTTP Client (mengganti implementasi lama, mempertahankan nama & shape respons lama)
```
http_get(url)                      -> {sukses, status, badan, url}          (lama tetap kompatibel)
http_post(url, payload)            -> {sukses, status, badan, data_terkirim} (lama tetap kompatibel)
http_get_detil(url, [opts])        -> {sukses, status, header, badan, badan_biner?, salah}
http_post_detil(url, badan, [opts])-> {sukses, status, header, badan, salah}
opts map: {timeout_detik, tipe_konten, header: {..}, json: bool}
```
- Deployment TLS: `ureq` + `rustls` (tanpa native OpenSSL) — industri, ringan, sync & mudah dipakai dari loop interpreter.
- Timeout default 30 dtk; error network → `sukses:false` + `salah` (bukan panic).

### 4.2 HTTP Server
```
ServerHttp(port, [opts])         -> handle {_tipe:"ServerHttp", port, mode: "tunggal"|"utas"}
tambah_rute(server, "GET|POST|PUT|DELETE|OPTIONS|*", "/path", fungsi_handler)
sajikan_static(server, "/url-prefix", "folder-disk")      // static file + MIME
jalankan(server)                 // blocking: accept loop sampai tutup()
tutup(server)                    // set flag berhenti (gracias => loop keluar)
sse_editor(server, "/sse", fungsi_handler)  // HELP: output streaming per peristiwa (underscore)
```
- Request masuk dikonversi ke map:
  `{method, path, query:{..}, header:{..}, badan: <string|map-json|nil>}`
- Handler mengembalikan map: `{status: 200, badan: <string|map> , tipe_konten?, header_kustom?}`; default JSON output bila badan adalah map/number/bool.
- CORS + OPTIONS otomatis (seperti engine studio), rate limit opsional.
- Server tunggal = deterministik; mode `utas` = thread per koneksi (pola 3.2).

### 4.3 WebSocket (RFC 6455)
```
ServerWebSocket(port, [opts])    -> handle {_tipe:"ServerWebSocket", port,...}
ws_terima(ws)                    // blocking: handshake (SHA1+base64) + loop bingkai
ws_baca(ws)                      -> {tipe:"teks|biner|ping|pong|tutup", data}
ws_kirim(ws, teks|biner)         // frame encode (masking utk client)
ws_tutup(ws)                     // bingkai close 1000
WebSocket(url, [opts])           -> handle client; ws_baca/ws_kirim yang sama
```
- Implementasi murni `std::net` + kripto `sha1` + `base64` (dua dep kecil).
- Baris terpisah: pendukung chat real-time.

### 4.4 Desktop
```
JendelaDesktop(judul, lebar, tinggi, url) -> window WebView2 (mode A) atau BrowserApp (mode B)
widya app jalan <main.wya> [--port N]     // subcommand: server lokal + shell window/browser
widya app paket <nama> <jalan-masuk>      // hasil: dir <nama>/ berisi app + launcher (desktop)
```
- CLI baru didaftarkan di `src/main.rs` (pola subcommand `studio`, `lsp`, dst).
- Karena WebView2 butuh event loop window, mode A memakai event loop `wry` sebagai penggerak utama dan mem-poll socket server lokal secara non-blocking (GetMessage/UntukWindowProc) — implementasi rinci di task T6-T7.

### 4.5 FFI
```
muat_pustaka("user32.dll")           -> handle {_tipe:"PustakaC"}
ffi_panggil(handle, "MessageBeep", [0])                        // auto-infer jenis arg
ffi_panggil_tergetik(handle, "GetTickCount", [], "u64")        // jenis kembalian eksplisit
tutup_pustaka(handle)
```
- Backend: crate `libloading` (LoadLibraryW/dlsym) — Windows & Unix.
- Marshal: number→i32/i64/f64, bool→bool, string→*const c_char (UTF-8), map/array→di-serialisasi JSON pointer (ffi_panggil_tergetik memakai layout) — versi ketat dibuat eksplisit agar aman.
- Test di Windows: `GetTickCount`, `MessageBeep(0)`, `uvwGetUtcOffset` dari C runtime.

## 5. Dependensi Baru (disetujui user)

| Crate | Versi | Keperluan | Catatan |
|---|---|---|---|
| `ureq` | ^2 | HTTP client + TLS (rustls) | sync, json feature |
| `sha1` | ^0.10 | WS handshake | |
| `base64` | ^0.22 | WS handshake | |
| `libloading` | ^0.8 | FFI DLL | Windows/Unix |
| `wry` *(task T7)* | ^0.4x | WebView2 window | berat, hanya untuk mode desktop; disatukan di belakang fitur `–features desktop` |
| `serde` (implisit via serde_json) | — | ValueSerial → struct | sudah ada |

> `wry` dijadikan *optional feature* (fitur kargo `"desktop"`) agar `cargo build --release` inti tetap ringan & cepat; biner desktop di-build dengan `--features desktop`.

## 6. Verifikasi & Test Strategy

- Unit test Rust `#[test]` memanggil builtin langsung melalui `register_stdlib` (pola test stdlib yang sudah ada).
- **Integration test offline (tanpa internet):** Server Widya nyata diwariskan `ServerHttp` pada port acak → Widya client (`http_get`) memanggil dirinya sendiri; WebSocket klien↔server Widya di loopback; FFI memanggil API Windows; desktop hanya verifikasi proses launcher naik + HTTP 200.
- **Regresi:** `cargo test --all` tetap 283 lulus (0 failed); semua endpoint studio 200 (kecuali 429>120); CLI `widya studio --port x` tetap jalan.
- Contoh nyata: `contoh/200_web_api_nyata.wya`, `201_chat_websocket.wya`, `202_ffi_windows.wya`, `203_app_desktop.wya` → dijalankan/smoke.

## 7. Roadmap (peta task detail di `tasks.md`)

1. T1: HTTP client asli (TLS/header/timeout/JSON/binary) + uji.
2. T2: Ekstraksi engine HTTP studio → `src/web/` + regresi studio.
3. T3: `ServerHttp` nyata (tambah_rute/sajikan_static/jalankan/tutup, mode tunggal & utas) + uji.
4. T4: WebSocket server & client RFC 6455 + chat demo + uji.
5. T5: FFI `libloading` (muat/panggil/tutup, tergetik) + uji Windows.
6. T6: `widya app jalan/paket` (server lokal + browser app-mode) + demo.
7. T7: `JendelaDesktop` WebView2 via wry (fitur `desktop`, opsional) + demo.
8. T8: Contoh + tutorial CLI (bagian "Web & Desktop Industri").
9. T9: Test penuh, build release, smoke end-to-end lintas komponen.
10. T10: Verifikasi akhir + pembaruan `tasks.md` + commit.

## 8. Risiko & Mitigasi

| Risiko | Mitigasi |
|---|---|
| Callback Widya tidak `Send` | Handler dijalankan di `Interpreter` baru per req/thread (pola `buat_utas`+`ValueSerial`) |
| `wry` dependensi berat / build lambat | Fitur kargo `"desktop"` opsional; inti tetap tanpa wry |
| TLS handshake lambat di debug | Gunakan ureq+rustls default; timeout eksplisit |
| FFI salah tipe → crash | API tergetik wajib untuk tipe non-scalar; argumen divalidasi sebelum `transmute` call |
| Backward compat shape respons `http_get`/`http_post` | Pertahankan field lama (`sukses/status/badan/url/data_terkirim`); field baru hanya DITAMBAH |
| Perilaku lama demo (`15_ekosistem_canggih.wya`) berubah karena HTTP kini asli | Update contoh ke respons nyata; shape key tetap ada |

## 9. Definisi "Level Industri" (kriteria kelulusan keseluruhan)
1. HTTP client & server bekerja pada jaringan nyata (bukan simulasi): bind, koneksi, header, JSON, TLS, timeout, error handling.
2. WebSocket RFC 6455 interoperable (handshake benar, frame masking benar, ping/pong).
3. FFI memanggil DLL/so asli dan mengembalikan nilai benar.
4. Desktop: window benar-benar tampil & aplikasi HTML berjalan di dalamnya (WebView2/browser-app), komunikasi UI↔backend lewat HTTP.
5. Seluruh test lulus 0 failed; dokumentasi tutorial terbarui; contoh nyata tersedia.
6. Tidak ada regresi fitur lama (studio, LSP, 1158 builtin, CLI).