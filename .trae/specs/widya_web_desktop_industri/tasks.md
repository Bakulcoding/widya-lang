# Widya Web & Desktop Level Industri — Implementation Plan (tasks.md)

## Deskripsi Task Umum
10 task implementasi untuk mewujudkan lapisan web & desktop **nyata** (bukan simulasi) sesuai `spec.md`. Pembagian kerja:

- **T1 (HTTP client)** — mandiri, tanpa menunggu task lain (bisa dikerjakan paralel dgn T2).
- **T2 (ekstraksi engine `src/web/`)** — refactor `src/studio.rs`: bersifat menyentuh file studio → **serial sebelum T3**.
- **T3 (ServerHttp nyata)** — di atas engine `src/web/`.
- **T4 (WebSocket)** — menggunakan `src/web/` (handshake/frame).
- **T5 (FFI)** — mandiri (libloading), boleh paralel.
- **T6 (desktop via browser-app)** — di atas engine `src/web/` + CLI main.rs.
- **T7 (WebView2 via wry, fitur `desktop`)** — paling berat; sesudah T6.
- **T8 (contoh + tutorial)**, **T9 (test release + smoke)**, **T10 (verifikasi + commit)** — serial di belakang.

**Backward compat MUTLAK**: seluruh test lama lulus (`cargo test --all` = 283, 0 failed); studio & LSP & 1158 builtin tidak berubah perilaku.

---

## Task 1: HTTP Client Asli (TLS/header/timeout/JSON/binary)
- **Status**: `done` (commit 4916a64 + 513daec)
- **Priority**: high
- **Depends On**: None
- **Description**:
  - Tambah dep `ureq = { version = "2", features = ["json"] }` (+ rustls; tanpa native OpenSSL).
  - Tulis modul `src/stdlib/web.rs` (atau section baru di `mod.rs`) berisi:
    1. `builtin_http_get` — ganti body simulasi: panggil `ureq::get` → kembalikan map `{sukses, status, url, badan}`; `badan` = JSON map bila `Content-Type: application/json`, selain itu string. Timeout default 30 dtk.
    2. `builtin_http_post` — `ureq::post`; payload map/array → JSON, string → raw; kembalikan `{sukses, status, url/dipanggil, badan, data_terkirim}`.
    3. `http_get_detil(url, [opts])` & `http_post_detil(...)` — tambahan: `{timeout_detik, tipe_konten, header:{..}}`; kembalikan plus `header:{..}`.
    4. Error network (koneksi ditolak, timeout, DNS, HTTP 4xx/5xx) → `{sukses:false, status, salah:"..."}` TIDAK panic.
    5. Daftarkan ke `register_stdlib` dengan nama sama (lama) + nama baru.
- **Acceptance Criteria Addressed**: AC-HTTP-1 (client nyata), AC-HTTP-2 (TLS), AC-HTTP-3 (timeout/error tidak panic)
- **Test Requirements**:
  - `rule` TR-1.1: `http_get` terhadap server studio lokal (spun dalam test) port acak → `sukses:true`, `status` terisi, `badan` berisi HTML. Pass: nilai status==200 dan badan memuat string.
  - `rule` TR-1.2: `http_post` JSON ke `/api/run` lokal → `sukses:true`, `badan` memuat `"sukses":true` (parseable).
  - `rule` TR-1.3: `http_get("http://127.0.0.1:1")` (port tertutup, timeout pendek) → `sukses:false`, `salah` berisi string galat; TIDAK panic. Pass: builtin return Ok(map), tidak panic.
  - `rule` TR-1.4: output map memuat semua key lama: `{sukses,status,badan,url}` (GET) & `{sukses,status,badan,data_terkirim}` (POST).
  - `rule` TR-1.5: `cargo test --all` → 0 new failed (regresi).
- **Notes**: Jangan hapus nama lama; tambahkan variasi `..._detil`. Jika `ureq` menimbulkan konflik Cargo feature, pin versi yang build bersih di Windows.

---

## Task 2: Ekstraksi Engine HTTP Studio -> Modul `src/web/`
- **Status**: `done` (commit 4916a64 + 513daec)
- **Priority**: high
- **Depends On**: None (paralel dgn T1), selesai SEBELUM T3
- **Description**:
  - Pindahkan logika HTTP studio (`handle_koneksi`, parse request, routing, CORS, JSON helper `err_*`, `extract_body`, `parse_body`, static-serving, thread-per-koneksi, rate limiter) ke modul baru `src/web/mod.rs` dengan API Publik:
    `WebServer::new(port)`, `registrasi_rute(method, path, handler_fn_or_static)`, `serve_static`, `kirim_response(stream, status, tipe, badan, kustom_header)`, `parse_request(raw) -> RequestInfo {method, path, query, header, badan}`.
  - `studio.rs` di-refactor untuk **memakai** modul ini (duplikasi dihapus). Backward compat: SEMUA endpoint studio (25+) perilaku & status http idéntik.
  - `src/web/mod.rs` di-declare di `src/lib.rs` sebagai `pub mod web;` sehingga `src/stdlib/` dan `src/main.rs` bisa memakai.
- **Acceptance Criteria Addressed**: AC-WEB-ENG (engine tunggal reusable), AC-R1 (backward compat studio)
- **Test Requirements**:
  - `rule` TR-2.1: studio tetap: `GET /` → 200 & berisi string IDE; `/api/run`, `/api/check`, `/api/contoh`, `/api/search`, `/api/run/stream` tetap berfungsi (smoke sama seperti verifikasi T12 studio). Pass: hasil identik sebelumnya.
  - `rule` TR-2.2: rate limit 120→429 tetap.
  - `rule` TR-2.3: `cargo test --all` → 283 passed 0 failed.
  - `rubric` TR-2.4: maintainability — `src/studio.rs` mengecil (engine pindah), tidak ada fungsi `handle_koneksi` tiga kali lipat; fungsi router reuse (Scale 1-5, Threshold 4).
- **Notes**: Jika pemindahan menyentuh `emit_baris`/sink thread_local — utus agar hook SSE tetap (sink thread_local di stdlib tidak berubah).

---

## Task 3: ServerHttp Nyata + Routing Widya
- **Status**: `pending`
- **Priority**: high
- **Depends On**: T2 (wajib `src/web/` ada)
- **Description**:
  - Ganti `builtin_http_server` (simulasi) → benar-benar bind `TcpListener` via `src/web`:
    - `ServerHttp(port, [opts])` → handle `{_tipe, port, mode:"tunggal"|"utas", status:"SIAP", rute:{..}}`. `opts.mode` default `"tunggal"`.
    - `tambah_rute(server, "GET", "/halo", handler)` — handler `Value::Function`; simpan body AST.
    - `sajikan_static(server, "/aset", "folder")` — static files (MIME sederhana).
    - `jalankan(server)` — accept loop **blocking**; per request mengeksekusi handler dengan pola fresh-`Interpreter`+`ValueSerial` (lihat spec 3.2). Request → map `{method, path, query, header, badan}`; kembalian map `{status, badan, tipe_konten?, header_kustom?}`.
    - `tutup(server)` — flag berhenti; loop keluar & listener di-drop.
  - Mode `utas` = thread per koneksi (cara T2 studio). Tambahan: CORS otomatis, OPTIONS 204, JSON default output map/array.
- **Acceptance Criteria Addressed**: AC-WebApp-1 (server benar-benar melayani), AC-WebApp-2 (handler Widya), AC-WebApp-3 (static + JSON + CORS)
- **Test Requirements**:
  - `rule` TR-3.1: Program Widya: `s = ServerHttp(0) ... tambah_rute "GET" "/ping" → fung(req){return {status:200, badan:"pong"}}; jalankan(s)` di dalam thread (`buat_utas`) — test Rust melakukan curl lokal ke port → respon berisi "pong", status 200.
  - `rule` TR-3.2: Handler membaca `req.badan` (POST JSON) → memproses & return JSON → curl verifikasi field benar.
  - `rule` TR-3.3: `sajikan_static` file nyata di `contoh/` → `curl /aset/01_halo_dunia.wya` → 200 + isi file; path traversal (`../`) ditolak 403.
  - `rule` TR-3.4: mode tunggal deterministik: 20 request berurutan → semuanya jalan; tidak ada panik ketika handler error (return `{status:500}`).
  - `rule` TR-3.5: `cargo test --all` → 0 new failed.
- **Notes**: Port 0 → OS pilih gratis (untuk test acak). Jangan jadikan `jalankan` memblokir main thread program setelah program selesai — sediakan jalan keluar via `tutup` / non-blocking poll.

---

## Task 4: WebSocket Server & Client (RFC 6455)
- **Status**: `pending`
- **Priority**: high
- **Depends On**: T2 (pakai `src/web` socket handling)
- **Description**:
  - Tambah dep `sha1` + `base64`.
  - Implementasi di `src/web/` (atau `src/web/ws.rs`) untuk dipakai stdlib & studio:
    - `handshake_server(stream, key)` → hitung `Sec-WebSocket-Accept = base64(sha1(key + GUID))`.
    - `baca_bingkai(stream)` — unmask, opcode teks(1)/biner(2)/ping(9)/pong(10)/tutup(8), payload kekuatan terbatas; validasi panjang.
    - `tulis_bingkai(stream, opcode, payload, mask client_bool)`.
    - ping/pong otomatis & `tutup` dengan code 1000.
  - Builtin baru di daftarkan menggantikan simulasi:
    - `ServerWebSocket(port, [opts])` → handle; `ws_terima(ws)` (handshake blocking), `ws_baca(ws)`, `ws_kirim(ws, payload)` (teks default; map→JSON), `ws_tutup(ws)`.
    - `WebSocket(url)` client — handshake ke server Widya/eksternal di loopback; pakai `ws_baca/ws_kirim` sama.
  - Contoh `contoh/201_chat_websocket.wya`: server broadcast ke klien.
- **Acceptance Criteria Addressed**: AC-WS-1 (handshake benar), AC-WS-2 (frame benar, masking client), AC-WS-3 (ping/pong/close), AC-WS-4 (chat nyata loopback)
- **Test Requirements**:
  - `rule` TR-4.1: Server Widya + client Widya loopback: kirim "halo" → balasan mengandung panjang frame benar; verify `payload` decode pas.
  - `rule` TR-4.2: **Interop**: client WebSocket Widya ↔ hand-sanitized raw TCP `Sec-WebSocket-Key` (uji rutin Rust menghitung accept & kirim frame sesuai RFC) — Pass: handshake diterima, frame teks terkirim/terbaca benar. (Uji dengan kurawal — tanpa server eksternal/network.)
  - `rule` TR-4.3: ping → pong otomatis; close frame → loop `ws_baca` berhenti dengan `{tipe:"tutup"}`.
  - `rule` TR-4.4: payload > 1 MB di-batas/error terkontrol (bukan OOM).
  - `rule` TR-4.5: `cargo test --all` → 0 new failed.
- **Notes**: Masking WAJIB untuk client (RFC). Max payload default 8 MB configurable.

---

## Task 5: FFI C/DLL Nyata (libloading)
- **Status**: `pending`
- **Priority**: high
- **Depends On**: None (boleh paralel dgn T1/T6)
- **Description**:
  - Tambah dep `libloading`.
  - Ganti `ffi_call_generic`/`panggil_c_lib` simulasi → nyata:
    - `muat_pustaka(path)` → `libloading::Library` disimpan di map global `pustaka_c` (OnceLock<Mutex<HashMap>>); handle `{_tipe:"PustakaC", id}`.
    - `ffi_panggil(handle, "NamaFunc", [args])` — auto-infer jenis arg (Number→i64, Bool→bool, String→*const u8 UTF-8, Nil→null) & kembalian (i64/i32/f64/bool/void) — jenis kembalian asumsi aman & dapat di-set via `ffi_panggil_tergetik`.
    - `ffi_panggil_tergetik(handle, "NamaFunc", [args], "u8|bool|i64|f64|void")` — marshalling ketat.
    - `tutup_pustaka(handle)`.
  - Validasi: argumen & jenis sebelum transmute; error → `Galat::runtime` (tidak segfault pada tipe keliru).
- **Acceptance Criteria Addressed**: AC-FFI-1 (LoadLibrary) AC-FFI-2 (pemanggilan & return benar), AC-FFI-3 (aman-tipe), AC-FFI-4 (close pustaka)
- **Test Requirements** (Windows):
  - `rule` TR-5.1: `ffi_panggil_tergetik(muat_pustaka("kernel32.dll"), "GetTickCount", [], "u32")` → angka > 1000 (valid, non-panic).
  - `rule` TR-5.2: `user32.dll MessageBeep(0)` dengan void → Ok; tidak crash.
  - `rule` TR-5.3: `tutup_pustaka` → panggil lagi setelah tutup → error terkontrol (bukan crash).
  - `rule` TR-5.4: `cargo test --all` → 0 new failed. (Test FFI ditandai `#[cfg(target_os="windows")]`; di Linux/CI di-skip.)
- **Notes**: simpan daftar pustaka beserta `__0` (handles) agar `Library` (Send+Sync) tidak ditutup ganda. Tipe `*const c_char` hanya valid selama pemanggilan — kembali ke string dengan `CStr` dalam batas call.

---

## Task 6: Desktop via Bundling + Browser App-mode (`widya app`)
- **Status**: `pending`
- **Priority**: medium
- **Depends On**: T3 (butuh `ServerHttp` nyata untuk menjalankan app)
- **Description**:
  - Subcommand CLI `widya app` di `src/main.rs`:
    - `widya app jalan <main.wya> [--port N]`: (1) baca & jalan program (yang di dalamnya memakai `ServerHttp`+`render_*`), (2) buka browser **app-mode** (`msedge/chrome --app=http://127.0.0.1:port/ --new-window`) otomatis, (3) tunggu Ctrl+C / perintah keluar untuk shutdown bersih.
    - `widya app paket <nama> <jalan-masuk>`: salin program + engine (`target/release/widya.exe`) ke folder `<nama>/`, buat `Widya.toml`, `README.md`, dan `jalankan.bat` (Windows) / `.sh`; hasil siap distribusi.
  - Update `JendelaDesktop` builtin: mode B (`BrowserApp`) memakai jalur ini.
- **Acceptance Criteria Addressed**: AC-APP-1 (satu perintah menjalankan app), AC-APP-2 (hasil paket portable), AC-APP-3 (shutdown bersih)
- **Test Requirements**:
  - `rule` TR-6.1: `widya app jalan contoh/200_web_api_nyata.wya` (port acak) → URL dilaporkan; `curl` port → HTTP 200 berisi HTML app. (Browser tidak harus benar di CI — verifikasi memperlihatkan proses w/ arg `--app` dimunculkan atau `--headless=new` bila perlu; Pass: server naik & HTML tersaji.)
  - `rule` TR-6.2: `widya app paket demo masuk.wya` → folder berisi 4+ file (`widya.exe` engine, `.wya`, `Widya.toml`, launcher); menyalin & menjalankan launcher → HTTP 200.
  - `rule` TR-6.3: webapp memakai `render_dashboard_html`/`render_*` menghasilkan HTML yang divisualisasikan; verifikasi string HTML yang dikembalikan.
  - `rule` TR-7.x (jika T7 selesai): mode A WebView2.
- **Notes**: Deteksi browser app-mode: coba `msedge`, `chrome`, `chromium` via `Command::new` + `windows`? Minimal cross-platform (browse via `start`/`xdg-open` fallback).

---

## Task 7: Jendela Desktop WebView2 (wry) — fitur opsional `desktop`
- **Status**: `pending`
- **Priority**: low (opsional, dependensi berat)
- **Depends On**: T6
- **Description**:
  - Tambah Cargo **feature** `desktop = ["dep:wry"]`; `wry` build hanya ketika fitur aktif (`--features desktop`).
  - Builtin `JendelaDesktop(judul, lebar, tinggi, url)` mode A: buat window WebView2 menuju `http://127.0.0.1:port/` (server lokal dari T3) — arsitektur Tauri-like (UI↔backend via HTTP).
  - Kendala event loop tunggal: jalankan loop window `wry` sebagai penggerak utama; poll accept socket non-blocking di setiap frame; handler server tetap fresh-interpreter.
  - IPC UI→backend via `fetch()` ke `/api/*` buatan user (default menyajikan `render_html`).
- **Acceptance Criteria Addressed**: AC-DSK-1 (window nyata tampil), AC-DSK-2 (HTML tersaji di dalamnya), AC-DSK-3 (backend server tetap merespons saat window terbuka)
- **Test Requirements**:
  - `rule` TR-7.1: Test Rust mem-buat window (headless/tidak) pada Windows → event loop berhenti setelah `tutup` virtual; Server lokal terlihat 200 saat window aktif (verifikasi proses & port).
  - `rule` TR-7.2: `cargo build --release --features desktop` sukses tanpa error (di Windows).
  - `rule` TR-7.3: build **tanpa** fitur tetap sukses & `cargo test --all` 0 failed (fitur terisolasi).
  - `rule` TR-7.4: dokumentasi di file Cargo/BUID (opsional): cara build biner desktop.
- **Notes**: Jika wry/WebView2 bermasalah di lingkungan, task ini boleh **defer tanpa mencoret** T6 (browser-app tetap level industri untuk distribusi). Catat di docs.

---

## Task 8: Contoh Nyata + Tutorial CLI
- **Status**: `pending`
- **Priority**: medium
- **Depends On**: T1–T7 (contoh memakai builtin final)
- **Description**:
  - `contoh/200_web_api_nyata.wya` — REST API nyata (ServerHttp + CRUD in-memory, JSON, static).
  - `contoh/201_chat_websocket.wya` — chat loopback (server WS + client WS).
  - `contoh/202_ffi_windows.wya` — FFI kernel32/user32 ~~demo~~ nyata (cfg-gated).
  - `contoh/203_app_desktop.wya` — dashboard HTML via `render_dashboard_html` + ServerHttp + `widya app jalan`.
  - Tutorial: tambah section "🌐 Web + 🖥️ Desktop Level Industri" di `dokumen_widya/TUTORIAL_11_CLI_LENGKAP.md` (baris >= 60): penjelasan, sintaks, contoh, table endpoint/API baru, konsep single vs thread, dan limitasi (handler fresh-interpreter).
  - Link di `TUTORIAL_INDEX.md`.
- **Acceptance Criteria Addressed**: AC-DOC-1 (dokumentasi & contoh tersedia), AC-DOC-2 (semua contoh berjalan)
- **Test Requirements**:
  - `rule` TR-8.1: `cargo run -- contoh/200_web_api_nyata.wya` (atau jalur yang sesuai) → server naik; `curl` GET `/api/data` → JSON.
  - `rule` TR-8.2: `contoh/201_chat_websocket.wya` → client & server Widya saling kirim; output pada stdout berisi pesan.
  - `rule` TR-8.3: `contoh/202_ffi_windows.wya` → cetak `GetTickCount` (>0).
  - `rule` TR-8.4: `contoh/203_app_desktop.wya` via `widya app jalan` → server 200 HTML.
- **Notes**: contoh dibuat deterministik & cepat (bukan infinite loop) agar bisa di-smoke oleh test.

---

## Task 9: Test Penuh, Build Release, Smoke End-to-End
- **Status**: `pending`
- **Priority**: high
- **Depends On**: T1–T8
- **Description**:
  - Jalan `cargo test --all` sampai 0 failed di semua suite.
  - `cargo build --release` dan `cargo build --release --features desktop` (Windows) → 0 warning.
  - Smoke end-to-end (artifacts):
    - studio (`widya studio --port`) → 25+ endpoint 200, SSE stream, rate 429 pada 121.
    - `contoh/200_web_api_nyata.wya` → curl CRUD.
    - `contoh/201_chat_websocket.wya` → loopback handshake+teks.
    - `contoh/202_ffi_windows.wya` → GetTickCount.
    - `contoh/203_app_desktop.wya` / `widya app` → HTTP 200 + (bila T7 ada) window lifecycle.
- **Acceptance Criteria Addressed**: AC-FINAL (seluruh suite + smoke)
- **Test Requirements**:
  - `rule` TR-9.1: `cargo test --all` → 0 failed, pass count tercatat di tasks.md.
  - `rule` TR-9.2: smoke daftar di atas → semuanya pass (bukti output disimpan di `artifacts/`).
  - `rule` TR-9.3: 0 warning release inti & fitur desktop.
- **Notes**: rekam bukti di bagian "Catatan Verifikasi Final".

---

## Task 10: Verifikasi Akhir + Update Dokumen + Commit
- **Status**: `pending`
- **Priority**: high
- **Depends On**: T9
- **Description**:
  - Review checklist seluruh AC di `spec.md` bagian 9 (definisi level industri).
  - Update `tasks.md` status → `done` untuk semua; tambah "Catatan Verifikasi Final".
  - Hapus batasan "simulasi" yang tercatat di dokumen lama (jika ada); update TUTORIAL + INDEX.
  - Commit (1 atau lebih) dengan pesan deskriptif gaya repo.
- **Acceptance Criteria Addressed**: AC-META
- **Test Requirements**:
  - `rule` TR-10.1: seluruh T1–T9 status `done` + AC checklist terpenuhi di spec.
  - `rule` TR-10.2: `git status` bersih (kecuali artifacts/scratch yang disengaja).
- **Notes**: jangan commit `artifacts/` & `/tmp` scratch.

---

## Pendaftaran Dep & Fitur Baru (summary)
```
[dependencies]
ureq = { version = "2", features = ["json"] }
sha1 = "0.10"
base64 = "0.22"
libloading = "0.8"

[features]
desktop = ["wry"]
wry = { version = "0.4x", optional = true }
```
> Verifikasi build-clean setiap penambahan dep di Windows (`cargo build` setelah `cargo update`).`