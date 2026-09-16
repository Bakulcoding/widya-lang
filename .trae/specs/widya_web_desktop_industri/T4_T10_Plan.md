# T4-T10: Implementasi Lapisan Web & Desktop Industri
## Rencana Lengkap dengan Commit Strategy

---

## T4: WebSocket Server & Client (RFC 6455)

### Deskripsi
Implementasi WebSocket server & client full RFC 6455 tanpa dependency eksternal (hanya std + sha1 + base64).

### Deps
```toml
sha1 = "0.10"      # Handshake SHA1
base64 = "0.22"    # Handshake base64 encoding
```

### API Widya
```widya
ServerWebSocket(port, [opts])  -> handle {_tipe:"ServerWebSocket", port, status:"SIAP"}
ws_terima(ws)                  -> {tipe:"handshake", client_id, header} atau error
ws_baca(ws, client_id)         -> {tipe:"teks"|biner|ping|pong|tutup, data, client_id}
ws_kirim(ws, client_id, payload)   // teks atau biner
ws_tutup(ws, client_id, code: 1000) // default close code 1000
WebSocket(url, [opts])         -> client handle
```

### Pembabakan

#### Fase 4.1: Frame Encoding/Decode (T4.1)
- Implementasi frame parser: read opcode, mask, payload length
- Frame encode: mask client, opcode, fin bit
- Handling: text(1), binary(2), close(8), ping(9), pong(10)
- Max payload 8 MB configurable
- Test: valid/invalid frame handling

#### Fase 4.2: Handshake Server (T4.2)
- Extract `Sec-WebSocket-Key` from headers
- Compute `Sec-WebSocket-Accept = base64(sha1(key + GUID))`
- Return 101 Switching Protocols
- Upgrade connection ke WebSocket mode
- Test: RFC 6455 test vectors

#### Fase 4.3: Handshake Client (T4.3)
- Connect ke server WebSocket
- Send handshake request
- Validate server response
- Upgrade ke WebSocket mode
- Test: loopback widya↔widya

#### Fase 4.4: Server API (T4.4)
- Builtin `ServerWebSocket(port, [opts])`
- `ws_terima(ws)` → blocking handshake + return client_id
- `ws_baca(ws, client_id)` → decode frame, return message
- `ws_kirim(ws, client_id, payload)`
- `ws_tutup(ws, client_id, code)`
- Single-threaded server (mode tunggal)
- Test: chat single client

#### Fase 4.5: Multiple Clients (T4.5)
- Multiple connections per server
- Client management map
- Broadcast function
- Test: chat multi-client (broadcaster + 2 receivers)

#### Fase 4.6: Testing (T4.6)
- Unit tests di `src/web/ws/mod.rs` (frame, handshake)
- Integration test di `tests/widya_tests.rs`
- Interop test: widya ↔ raw TCP (RFC test vector)
- Regression: cargo test --all 0 failures

### Acceptance (TR-4.1 to TR-4.5)
- TR-4.1: Server + client loopback, frame benar
- TR-4.2: Interop widya ↔ raw TCP, handshake + frame OK
- TR-4.3: ping → pong otomatis, close frame → loop exit
- TR-4.4: payload > 1 MB → error terkontrol (bukan OOM)
- TR-4.5: cargo test --all → 0 new failed

---

## T5: FFI C/DLL Nyata

### Deskripsi
FFI calling C libraries (Windows .dll, Unix .so) via libloading.

### Deps
```toml
libloading = "0.8"
```

### API Widya
```widya
muat_pustaka("kernel32.dll")           -> handle {_tipe:"PustakaC", id}
ffi_panggil(handle, "GetTickCount", [])        // auto-infer arg & return
ffi_panggil_tergetik(handle, "GetTickCount", [], "u64")  // strict
tutup_pustaka(handle)
```

### Pembabakan

#### Fase 5.1: Library Loading (T5.1)
- `muat_pustaka(path)` → `libloading::Library`
- Store di `OnceLock<Mutex<HashMap<u64, Library>>>`
- Return handle `{_tipe:"PustakaC", id}`
- Error handling: path tidak ditemukan, format salah
- Test: load kernel32.dll, user32.dll

#### Fase 5.2: Function Lookup (T5.2)
- `get_proc_address(handle, name)` → `libloading::Symbol`
- Cache function pointer
- Type inference: number→i64, bool→bool, string→*const c_char
- Error: symbol tidak ditemukan
- Test: GetTickCount from kernel32

#### Fase 5.3: marshalling Number (T5.3)
- Call with `Value::Number(n)` → convert ke C type:
  - i32: Value::Number dengan range check
  - i64: Value::Number (64-bit safe)
  - f64: Value::Number
- Return: C return → Value::Number
- Test: `GetTickCount()` → angka > 0

#### Fase 5.4: Marshalling Bool & String (T5.4)
- Bool: true/false → `bool`
- String: UTF-8 → `*const c_char` (copy, CStr)
- Return string: `*const c_char` → Value::String
- Memory: C string allocated → copy ke Rust
- Test: `MessageBeep(0)` → void, no crash

#### Fase 5.5: Strict Type (T5.5)
- `ffi_panggil_tergetik(handle, name, args, "return_type")`
- Return types: "void", "u8", "u32", "u64", "i32", "i64", "f32", "f64", "bool"
- Validation: args count & types matching
- Error: tipe tidak match → Galat::runtime (bukan segfault)
- Test: GetTickCount("u64") vs "u32"

#### Fase 5.6: Cleanup (T5.6)
- `tutup_pustaka(handle)` → drop Library
- Prevent double-close
- After close: error terkontrol (bukan crash)
- Test: close → panggil lagi → error, tidak crash

#### Fase 5.7: Testing (T5.7)
- Unit tests di `src/web/ffi/mod.rs`
- Windows-only tests: `[cfg(target_os="windows")]`
- Regression: cargo test --all 0 failures

### Acceptance (TR-5.1 to TR-5.4)
- TR-5.1: `ffi_panggil_tergetik(muat_pustaka("kernel32.dll"), "GetTickCount", [], "u32")` → angka > 0
- TR-5.2: `user32.dll MessageBeep(0)` → Ok, tidak crash
- TR-5.3: `tutup_pustaka` → panggil lagi → error terkontrol
- TR-5.4: cargo test --all → 0 new failed

---

## T6: Desktop via Bundling + Browser App-Mode

### Deskripsi
CLI command `widya app` untuk:
1. `widya app jalan main.wya` → server lokal + browser app-mode
2. `widya app paket <nama> <jalan-masuk>` → distribusi portable

### API CLI
```bash
widya app jalan main.wya [--port N]   # server + browser
widya app paket demo main.wya         # folder distribusi
```

### Pembabakan

#### Fase 6.1: CLI Subcommand (T6.1)
- Tambah subcommand di `src/main.rs`:
  - `AppJalan { berkas, port }`
  - `AppPaket { nama, berkas }`
- Parse args, route ke handler
- Default port acak (0) atau user-specified
- Test: `cargo run -- app jalan`

#### Fase 6.2: App Jalan (T6.2)
- Baca program `main.wya`
- Baca / buat directory `.widya_app_<timestamp>/`
- Copy `main.wya` ke folder
- Run `widya app jalan` dalam folder
- Start server lokal (`ServerHttp` dari T3)
- Detect browser:
  - Windows: `msedge`, `chrome`, `chromium`
  - Mac: `open -a "Safari"` / `open -a "Chrome"`
  - Linux: `xdg-open`
- Launch browser dengan `--app=http://127.0.0.1:port/`
- Wait Ctrl+C / SIGINT → shutdown server
- Cleanup temp folder (opsional)
- Test: browser app-mode terbuka, HTML tersaji

#### Fase 6.3: App Paket (T6.3)
- Buat folder `dist/<nama>/`
- Copy:
  - `main.wya` (renamed sesuai entrypoint)
  - `target/release/widya.exe` (engine)
  - `Widya.toml` (metadata: nama, versi, entrypoint)
  - `README.md` (instruksi penggunaan)
  - `jalankan.bat` (Windows launcher)
  - `jalankan.sh` (Linux/Mac launcher)
- Zip atau not (opsional)
- Test: folder berisi 5+ file, launcher berfungsi

#### Fase 6.4: Static Assets (T6.4)
- Support folder `assets/` dengan:
  - `index.html` (override default)
  - `style.css`, `app.js` (jika app menggunakan HTML + JS)
- Serve static via `WebServer::sajikan_static`
- Test: `widya app jalan` serves assets

#### Fase 6.5: Testing (T6.5)
- Integration test: `widya app jalan contoh/200_web_api_nyata.wya`
- Smoke: server naik (200), browser proses terdeteksi (arg `--app`)
- Windows-only (browser detection), CI headless optional
- Regression: cargo test --all 0 failures

### Acceptance (TR-6.1 to TR-6.3)
- TR-6.1: `widya app jalan contoh/200_web_api_nyata.wya` → server 200, HTML served, browser app-mode dimunculkan
- TR-6.2: `widya app paket demo main.wya` → folder dengan 5+ file (engine, wya, Widya.toml, launcher)
- TR-6.3: webapp `render_dashboard_html` → HTML visual (string HTML valid)
- TR-6.4: cargo test --all → 0 new failed

---

## T7: Jendela Desktop WebView2 (Wry)

### Deskripsi
Mode desktop native via WebView2 (Windows-only), arsitektur Tauri-like.

### Deps
```toml
[features]
desktop = ["dep:wry"]

[dependencies]
wry = { version = "0.4x", optional = true }
webview2-com = { version = "0.24", optional = true }
```

### API Widya
```widya
JendelaDesktop(judul, lebar, tinggi, url) -> window WebView2
```

### Pembabakan

#### Fase 7.1: Optional Feature (T7.1)
- Tambah feature `desktop = ["wry"]` di Cargo.toml
- `cargo build --release` → tanpa desktop (lightweight)
- `cargo build --release --features desktop` → dengan WebView2
- CI: build tanpa desktop (default)
- Test: build bersih dengan & tanpa feature

#### Fase 7.2: Event Loop (T7.2)
- Event loop penggerak utama: `wry` event loop (bukan `WebServer`)
- Poll socket server non-blocking setiap frame:
  - Windows: `PeekMessage` / `GetMessage`
  - Linux/Mac: `select()` / `epoll`
- Handler server tetap fresh-interpreter per request
- Thread pool untuk handler (non-blocking)
- Test: window tetap responsif saat handler lama

#### Fase 7.3: WebView2 Setup (T7.3)
- `wry::WebViewBuilder`
- URL: `http://127.0.0.1:port/`
- Size: width, height
- Debug mode (optional)
- Error handling: WebView2 tidak terinstall → fallback ke browser mode
- Test: window tampil, HTML tersaji

#### Fase 7.4: IPC UI↔Backend (T7.4)
- Backend: HTTP server (`/api/*`)
- Frontend: `fetch()` ke `/api/*`
- Default handler: `render_html()` untuk UI
- Request map: method, path, query, header, badan
- Response: status, badan (JSON), header_kustom
- Test: UI kirim fetch → backend proses → response

#### Fase 7.5: Window Lifecycle (T7.5)
- `tutup()` signal → window close + server shutdown
- Graceful: server wait pending requests → close
- Cleanup: drop resources
- Error: double-close, panic safety
- Test: close window → process exit bersih

#### Fase 7.6: Testing (T7.6)
- Windows-only: `#[cfg(all(target_os="windows", target_arch="x86_64"))]`
- Headless test (optional): virtual display
- Manual: `cargo run --features desktop -- contoh/203_app_desktop.wya`
- CI: skip jika tidak WebView2
- Regression: cargo test --all 0 failures

### Acceptance (TR-7.1 to TR-7.4)
- TR-7.1: Test Rust buat window (headless/tidak) → event loop berhenti setelah `tutup`
- TR-7.2: `cargo build --release --features desktop` → 0 error
- TR-7.3: Build tanpa feature → 0 error, cargo test --all 0 failed
- TR-7.4: `JendelaDesktop` → window tampil, HTML tersaji, server respons

---

## T8: Contoh Nyata + Tutorial

### Deskripsi
Contoh program dan dokumentasi untuk level industri.

### Pembabakan

#### Fase 8.1: Contoh HTTP API (T8.1)
- `contoh/200_web_api_nyata.wya`
- REST API nyata dengan `ServerHttp`:
  - `GET /api/data` → JSON list
  - `POST /api/data` → simpan
  - `DELETE /api/data/:id` → hapus
  - Static `assets/index.html` untuk admin UI
- Handler: CRUD in-memory map
- Test: `curl http://localhost:port/api/data`

#### Fase 8.2: Contoh WebSocket Chat (T8.2)
- `contoh/201_chat_websocket.wya`
- Server: `ServerWebSocket`, broadcast message
- Client: loopback test (2 client saling kirim)
- Output: stdout show pesan terima
- Test: client 1 → kirim "halo" → client 2 terima

#### Fase 8.3: Contoh FFI Windows (T8.3)
- `contoh/202_ffi_windows.wya`
- Windows-only: `muat_pustaka("kernel32.dll")`
- `ffi_panggil_tergetik("GetTickCount")` → angka
- Optional: `MessageBeep(0)` → sound
- cfg-gated: `#[cfg(target_os="windows")]`
- Test: `cargo run -- contoh/202_ffi_windows.wya`

#### Fase 8.4: Contoh Desktop App (T8.4)
- `contoh/203_app_desktop.wya`
- Dashboard HTML via `render_dashboard_html`
- ServerHttp + Web server mode
- CLI: `widya app jalan contoh/203_app_desktop.wya`
- Test: server 200, HTML rendered, browser/app window tampil

#### Fase 8.5: Tutorial (T8.5)
- Update: `dokumen_widya/TUTORIAL_11_CLI_LENGKAP.md`
- Section baru: "🌐 Web + 🖥️ Desktop Level Industri"
- Content:
  - Penjelasan arsitektur (HTTP client/server, WebSocket, FFI, desktop)
  - Sintaks API (ServerHttp, tambah_rute, ServerWebSocket, muat_pustaka, dll)
  - Contoh lengkap (copy-paste)
  - Tabel endpoint/API baru
  - Konsep fresh-interpreter per request
  - Limitasi (handler thread, static vs dynamic)
- Table of contents di `TUTORIAL_INDEX.md`

#### Fase 8.6: Testing (T8.6)
- Integration test:
  - `cargo run -- contoh/200_web_api_nyata.wya` → server 200, curl CRUD
  - `contoh/201_chat_websocket.wya` → stdout show pesan
  - `contoh/202_ffi_windows.wya` → GetTickCount > 0
  - `contoh/203_app_desktop.wya` via `widya app jalan` → server 200 HTML
- Regression: cargo test --all 0 failures

### Acceptance (TR-8.1 to TR-8.4)
- TR-8.1: `cargo run -- contoh/200_web_api_nyata.wya` → server 200, curl GET/POST/DELETE → JSON
- TR-8.2: `contoh/201_chat_websocket.wya` → client & server Widya saling kirim, stdout show pesan
- TR-8.3: `contoh/202_ffi_windows.wya` → GetTickCount > 0
- TR-8.4: `contoh/203_app_desktop.wya` via `widya app jalan` → server 200 HTML
- TR-8.5: Tutorial section > 60 baris, TUTORIAL_INDEX.md terupdate
- TR-8.6: cargo test --all → 0 new failed

---

## T9: Test Penuh, Build Release, Smoke End-to-End

### Deskripsi
Verifikasi final: test suite, release build, smoke testing.

### Pembabakan

#### Fase 9.1: Test Suite (T9.1)
- `cargo test --all` sampai **0 failed**
- Capture output: jumlah test, failures, suites
- Catat: `283 + 6 (T1 web tests) + X (T4) + Y (T5) + Z (T6) + W (T7) + V (T8) = Total`
- Setiap suite perlu:
  - Unit test di source
  - Integration test di `tests/`
  - Regression check per task

#### Fase 9.2: Build Release (T9.2)
- `cargo build --release` → 0 warning
- `cargo build --release --features desktop` (Windows) → 0 warning
- Size target: binary < 20 MB (tanpa desktop), < 50 MB (dengan wry)
- Strip debug info (optional)
- Test: binary bisa dijalankan, CLI works

#### Fase 9.3: Smoke Studio (T9.3)
- Start: `widya studio --port <port>`
- Smoke endpoints (25+):
  - `GET /` → 200, HTML IDE
  - `GET /api/contoh` → 200, JSON
  - `POST /api/run` → 200, `{"sukses":true,...}`
  - `POST /api/check` → 200, diagnostics
  - SSE `/api/run/stream` → text/event-stream
  - Rate limit: request 121 → 429
  - OPTIONS → 204
  - 404 unknown endpoint
- Smoke: semua endpoint 200 (kecuali rate limit 429)

#### Fase 9.4: Smoke T1-T3 (T9.4)
- T1 HTTP client: `http_get("http://127.0.0.1:port")` → sukses:true, status 200
- T2 Engine: studio tetap 25+ endpoint identik
- T3 ServerHttp: `ServerHttp(3000)`, tambah_rute, jalankan → curl localhost:3000 → response

#### Fase 9.5: Smoke T4-T7 (T9.5)
- T4 WebSocket: server + client Widya loopback, handshake OK
- T5 FFI: `GetTickCount` > 0, `MessageBeep` no crash
- T6 Desktop app: `widya app jalan` → server 200, browser/app mode
- T7 WebView2 (Windows): window tampil, HTML served

#### Fase 9.6: Smoke T8 (T9.6)
- `contoh/200_web_api_nyata.wya` → CRUD via curl
- `contoh/201_chat_websocket.wya` → stdout show pesan
- `contoh/202_ffi_windows.wya` → GetTickCount
- `contoh/203_app_desktop.wya` → `widya app jalan` → 200 HTML

#### Fase 9.7: Dokumentasi (T9.7)
- Update `docs/`:
  - API reference: ServerHttp, ServerWebSocket, FFI, Desktop
  - Architecture: HTTP engine, WebSocket frame, WebView2 flow
  - Examples: copy-paste untuk semua contoh
- Update `Cargo.toml` description & keywords
- Changelog: T1-T8 summary

#### Fase 9.8: Final Check (T9.8)
- Checklist:
  - [ ] `cargo test --all` → 0 failed
  - [ ] `cargo build --release` → 0 warning
  - [ ] `cargo build --release --features desktop` → 0 warning (Windows)
  - [ ] Studio smoke (25+ endpoint) → semua OK
  - [ ] T1-T8 smoke → semua OK
  - [ ] Tutorial + examples → tersedia
  - [ ] Documentation → update
  - [ ] Tasks.md → semua status `done`
  - [ ] Spec.md → revisi jika ada perubahan design
- Catat: "Catatan Verifikasi Final" di tasks.md

### Acceptance (TR-9.1 to TR-9.3)
- TR-9.1: `cargo test --all` → 0 failed, pass count tercatat
- TR-9.2: Smoke semua daftar (studio, contoh, web, desktop) → pass
- TR-9.3: 0 warning release inti & fitur desktop

---

## T10: Verifikasi Akhir + Commit

### Deskripsi
Verifikasi semua AC, update dokumentasi, commit.

### Pembabakan

#### Fase 10.1: AC Checklist (T10.1)
- Review spec.md bagian 9 (definisi level industri):
  - [ ] HTTP client & server bekerja nyata (bukan simulasi)
  - [ ] WebSocket RFC 6455 interoperable
  - [ ] FFI memanggil DLL/so asli
  - [ ] Desktop: window benar-benar tampil & HTML berjalan
  - [ ] Seluruh test lulus 0 failed
  - [ ] Dokumentasi tutorial terbarui
  - [ ] Contoh nyata tersedia
  - [ ] Tidak ada regresi fitur lama (studio, LSP, 1158 builtin, CLI)

#### Fase 10.2: Update Documentation (T10.2)
- Tasks.md: semua T1-T10 status `done`
- Spec.md: revisi jika ada perubahan design
- TUTORIAL_INDEX.md: link ke tutorial baru
- docs/: API reference updated

#### Fase 10.3: Commit (T10.3)
- Review git status, diff
- Staged files:
  - Code: `src/web/mod.rs`, `src/lib.rs`, `src/stdlib/mod.rs`, `src/studio.rs`, `tests/widya_tests.rs`
  - Config: `Cargo.toml`, `Cargo.lock`
  - Docs: `.trae/specs/`, `dokumen_widya/`
- Unstaged (gitignore):
  - `artifacts/`, `temp_verify.html`, `target/`, `debug/`, `.widya_app_*`
- Commit message style:
  ```
  feat(web): implementasi HTTP client real (ureq) + engine studio terpisah

  - T1: http_get/post dengan TLS/timeout/JSON/error handling
  - T2: ekstraksi engine studio → src/web/mod.rs, studio reuse
  - T3: ServerHttp nyata (tambah_rute/sajikan_static/jalankan/tutup)
  - T4: WebSocket RFC 6455 (sha1+base64 handshake, frame encode)
  - T5: FFI libloading (muat_panggil_tutup, tergetik)
  - T6: Desktop app CLI (widya app jalan/paket)
  - T7: WebView2 wry (opsional feature desktop)
  - T8: Contoh + tutorial CLI (200..203 .wya)
  - T9: Test penuh (289+), release build, smoke end-to-end
  - T10: Verifikasi AC + commit

  Regresi: cargo test --all = 289 passed / 0 failed
  ```

#### Fase 10.4: Branch & PR (T10.4)
- Push ke new branch: `git push -u origin feat/web-desktop-industri`
- Create PR/MR:
  - Title: `feat: Widya Web & Desktop Level Industri`
  - Description: summary T1-T10, testing, breaking changes (none)
  - Checklist: AC checklist, test result, smoke result

#### Fase 10.5: Post-Commit (T10.5)
- Tag version: `git tag v0.5.0` (jika rilis)
- Draft release notes (GitHub/GitLab)
- Update README.md jika perlu
- Announce: Widya Discord/forum (opsional)

### Acceptance (TR-10.1 to TR-10.2)
- TR-10.1: Seluruh T1-T10 status `done`, AC checklist terpenuhi
- TR-10.2: `git status` bersih (kecuali artifacts/scratch yang disengaja)

---

## Commit Strategy Summary

| Task | Commit | Files |
|------|--------|-------|
| T1 | `feat(web): http client real (ureq) + unit test` | `src/web/mod.rs`, `tests/widya_tests.rs`, `Cargo.toml` |
| T2 | `refactor: ekstraksi engine studio → src/web/mod.rs` | `src/web/mod.rs`, `src/studio.rs` |
| T3 | `feat(web): ServerHttp nyata (handler Widya per request)` | `src/web/mod.rs` (tambah builtin) |
| T4 | `feat(web): WebSocket RFC 6455 (handshake + frame)` | `src/web/ws/mod.rs`, `src/lib.rs` |
| T5 | `feat(ffi): libloading C/DLL (muat_panggil_tutup)` | `src/web/ffi/mod.rs`, `src/lib.rs` |
| T6 | `feat(cli): widya app jalan/paket (browser app + distribusi)` | `src/main.rs` (subcommand), `contoh/200..203.wya` |
| T7 | `feat(desktop): WebView2 wry (optional feature)` | `src/web/desktop/mod.rs`, `Cargo.toml` |
| T8 | `docs: contoh + tutorial CLI (200..203 .wya + tutorial)` | `contoh/`, `dokumen_widya/TUTORIAL_*.md` |
| T9 | `test: smoke penuh, release build, verifikasi final` | `tests/`, documentation updates |
| T10 | `refactor: update tasks.md + commit` | `.trae/specs/widya_web_desktop_industri/` |

**Total Commits**: 10 (1 per task), atau 1 mega-commit jika team prefer.

---

## Timeline Estimate (Total)

| Task | Estimasi | Notes |
|------|----------|-------|
| T1 | 2 jam | HTTP client (ureq) |
| T2 | 1.5 jam | Engine extract |
| T3 | 3 jam | ServerHttp (complicated) |
| T4 | 4 jam | WebSocket (frame + handshake) |
| T5 | 2 jam | FFI (libloading) |
| T6 | 2.5 jam | Desktop app CLI |
| T7 | 4 jam | WebView2 (complex, Windows only) |
| T8 | 2 jam | Contoh + tutorial |
| T9 | 1 jam | Test release smoke |
| T10 | 0.5 jam | Documentation + commit |
| **Total** | **22 jam** | ~3 hari kerja (8 jam/hari) |

---

## Catatan Penting

1. **Backward compat MUTLAK**: Setiap task harus maintain `cargo test --all = 289+ passed / 0 failed`.
2. **Build clean**: `cargo build --release` tanpa warning.
3. **Commit per task**: Setiap task selesai → commit (sesuai rencana T10).
4. **Feature flag**: Desktop (wry) optional via `--features desktop`.
5. **Platform**: FFI T5 & WebView2 T7 Windows-only (cfg-gated).
6. **Test coverage**: Unit + integration + smoke.
7. **Documentation**: Update tutorial + contoh setiap task.
8. **Smoke testing**: Setiap task → smoke before commit.

---

**End of T4-T10 Plan**

