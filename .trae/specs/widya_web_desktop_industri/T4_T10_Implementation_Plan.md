# T4-T10: Implementasi Komprehensif WebSocket, FFI, Desktop & Dokumentasi

## Ringkasan Eksekutif

Dokumen ini merupakan rencana implementasi detail untuk task T4 hingga T10 dalam membangun lapisan web & desktop level industri untuk Widya. Setiap task dirancang dengan:

- **Deskripsi lengkap**: tujuan, scope, dan deliverable
- **Dependencies**: task-task yang harus selesai terlebih dahulu
- **API & Specification**: signature fungsi, parameter, return value
- **Implementation Plan**: breakdown ke fase-fase kecil dengan deliverable per fase
- **Acceptance Criteria (AC)**: definisi keberhasilan terukur
- **Test Requirements**: unit test, integration test, smoke test
- **Verification Checklist**: cara memverifikasi AC terpenuhi

---

## T4: WebSocket Server & Client (RFC 6455)

### Deskripsi
Implementasi WebSocket server & client fully compliant RFC 6455 menggunakan `std::net` dengan dependencies minimal (`sha1 0.10` + `base64 0.22`). Menggantikan simulasi `ServerWebSocket` dengan implementasi nyata yang support handshake, frame encoding/decoding, masking, ping/pong otomatis, dan close frame handling.

### Dependencies
```toml
sha1 = "0.10"      # SHA1 hashing untuk handshake
base64 = "0.22"    # Base64 encoding untuk Sec-WebSocket-Accept
```

### API Specification

#### Server WebSocket
```widya
ServerWebSocket(port, [opts]) 
  -> handle {_tipe:"ServerWebSocket", port, status:"SIAP", clients_count}

ws_terima(ws_handle) 
  -> {tipe:"handshake", client_id, header_map} | error

ws_baca(ws_handle, client_id) 
  -> {tipe:"teks"|"biner"|"ping"|"pong"|"tutup", data, client_id}

ws_kirim(ws_handle, client_id, payload)
  -> void | error  # payload: string (teks default) atau biner

ws_tutup(ws_handle, client_id, [code])
  -> void | error  # code default 1000 (normal closure)
```

#### Client WebSocket
```widya
WebSocket(url, [opts])
  -> client_handle {_tipe:"WebSocketClient", url, status:"TERKONEKSI"}

ws_baca(client_handle)
  -> {tipe:"teks"|"biner"|"ping"|"pong"|"tutup", data}

ws_kirim(client_handle, payload)
  -> void | error

ws_tutup(client_handle, [code])
  -> void | error
```

### Implementation Plan

#### Fase 4.1: Frame Parser & Encoder (T4.1)
**Deliverable**: `src/web/ws/frame.rs` dengan fungsi parsing & encoding frame WebSocket

**Deskripsi**:
- Baca frame header: fin bit, opcode (1=text, 2=binary, 8=close, 9=ping, 10=pong), mask bit, payload length (1-3 byte)
- Parse extended payload length (7, 16, 63 bit)
- Unmask payload jika mask bit set (XOR dengan 4-byte mask key)
- Validate frame per RFC: client frame HARUS termasuk mask, server frame JANGAN mask
- Encode frame: set opcode, fin bit, mask bit (1 untuk client), payload length, mask key (4 byte random), XOR payload

**Test**:
- Unit test: valid frame (text "hello") → parse OK, data="hello"
- Unit test: masked frame (client) → unmask benar
- Unit test: invalid frame (payload > 8 MB) → error terkontrol
- Regression: `cargo test --all` → 0 new failed

**Acceptance**: Frame parser/encoder bekerja benar sesuai RFC 6455 section 5.2

---

#### Fase 4.2: Handshake Server (T4.2)
**Deliverable**: `src/web/ws/handshake.rs` dengan server-side handshake

**Deskripsi**:
- Extract `Sec-WebSocket-Key` dari HTTP header upgrade request
- Compute `Sec-WebSocket-Accept = base64(sha1(key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"))`
- Return HTTP 101 Switching Protocols dengan header:
  - `Upgrade: websocket`
  - `Connection: Upgrade`
  - `Sec-WebSocket-Accept: <computed>`
- Upgrade koneksi ke WebSocket mode (frame-based, bukan HTTP lagi)

**Test**:
- Unit test: RFC 6455 test vector (Sec-WebSocket-Key="dGhlIHNhbXBsZSBub25jZQ==") → Sec-WebSocket-Accept="s3pPLMBiTxaQ9kYGzzhZRbK+xOo="
- Integration test: HTTP upgrade request → server return 101 + correct header

**Acceptance**: Server handshake RFC-compliant, test vector pass

---

#### Fase 4.3: Handshake Client (T4.3)
**Deliverable**: Client WebSocket handshake & upgrade

**Deskripsi**:
- Generate random `Sec-WebSocket-Key` (16 byte random, base64 encode)
- Send HTTP upgrade request:
  - `GET /path HTTP/1.1`
  - `Host: ...`
  - `Upgrade: websocket`
  - `Connection: Upgrade`
  - `Sec-WebSocket-Key: <random>`
  - `Sec-WebSocket-Version: 13`
- Validate server response: 101 status, valid Sec-WebSocket-Accept
- Upgrade ke frame mode

**Test**:
- Integration test: Widya client → Widya server loopback → handshake OK
- Test invalid response (404 atau salah Accept) → error

**Acceptance**: Client handshake bekerja loopback dengan server T4.2

---

#### Fase 4.4: Server API & Client Management (T4.4)
**Deliverable**: Modul `src/web/ws/mod.rs` dengan server handle, client map, API

**Deskripsi**:
- `ServerWebSocket(port)`: bind TCP listener, return handle dengan map client (client_id → stream + state)
- `ws_terima(ws)`: blocking accept koneksi baru, lakukan handshake, return client_id + headers
- `ws_baca(ws, client_id)`: baca frame dari client, decode, return message dengan tipe
- `ws_kirim(ws, client_id, payload)`: encode frame (mask=1), tulis ke stream
- `ws_tutup(ws, client_id, code=1000)`: kirim close frame dengan code, hapus client dari map

**Mode**: Single-threaded → deterministic. Per request di handler, loop `ws_baca` blocking (pola query database serial di T3 engine).

**Test**:
- Integration test: server terima client, baca "halo", kirim balik "dunia" → client terima
- Test multiple reads: kirim 3 pesan, baca 3 kali
- Test close: ws_tutup → client disconnect, ws_baca return {tipe:"tutup"}

**Acceptance**: Server API fungsional, single client flow OK

---

#### Fase 4.5: Multiple Clients & Broadcast (T4.5)
**Deliverable**: Support multi-client, broadcast function

**Deskripsi**:
- Client map: HashMap<client_id, Stream + buffer>
- `ws_broadcast(ws, payload)`: kirim ke semua client (except self atau all)
- Ping/pong otomatis: server kirim ping setiap N detik (default 30s), auto-respond pong
- Close handling: client disconnect → remove dari map, notify others (opsional)

**Test**:
- Chat test: 2 client, sender mengirim message, broadcaster kirim ke recipient, recipient terima
- Ping test: client receive ping → auto-respond pong

**Acceptance**: Multi-client broadcast bekerja, ping/pong otomatis

---

#### Fase 4.6: Testing & Regression (T4.6)
**Deliverable**: Test suite penuh, no new failures

**Deskripsi**:
- Unit test `src/web/ws/frame.rs`: frame parsing, encoding, masking
- Unit test `src/web/ws/handshake.rs`: handshake compute, RFC vectors
- Integration test `tests/widya_tests.rs`: server↔client loopback, chat multi-client
- Smoke test: example `contoh/201_chat_websocket.wya` jalankan & verify output
- Regression: `cargo test --all` → 0 new failed

**Acceptance**: Semua test pass, 0 regresi

---

### Acceptance Criteria (AC-WS)

| AC | Kriteria | Verifikasi |
|----|----------|-----------|
| AC-WS-1 | Handshake server & client RFC 6455 compliant | Test vector pass, 101 response correct |
| AC-WS-2 | Frame encoding/decoding benar (masking client, unmask server) | Unit test frame parser/encoder |
| AC-WS-3 | Ping/pong otomatis, close frame handling | Auto-respond pong, close → disconnect |
| AC-WS-4 | Chat loopback multi-client (sender→broadcast→receiver) | Integration test chat app |
| AC-WS-5 | Payload size limit (8 MB default, error > limit) | Test payload > 8MB → error |
| AC-WS-6 | Backward compat (0 new failed test, studio still 25+ endpoint) | `cargo test --all` 0 failed |

---

## T5: FFI C/DLL (libloading)

### Deskripsi
Implementasi FFI (Foreign Function Interface) untuk memanggil C library / DLL Windows menggunakan crate `libloading`. Menggantikan simulasi `ffi_call_generic` dengan implementasi nyata yang support auto-type-inference dan strict type specification.

### Dependencies
```toml
libloading = "0.8"
```

### API Specification

#### Library Loading
```widya
muat_pustaka(path_string)
  -> handle {_tipe:"PustakaC", id, path}

tutup_pustaka(handle)
  -> void | error
```

#### Function Calling
```widya
ffi_panggil(handle, function_name_string, [args_array])
  -> result (type auto-inferred from args)

ffi_panggil_tergetik(handle, function_name_string, [args_array], return_type_string)
  -> result (type strictly typed)
  # return_type: "void", "u8", "u32", "u64", "i32", "i64", "f32", "f64", "bool", "string"
```

### Implementation Plan

#### Fase 5.1: Library Loading (T5.1)
**Deliverable**: `src/web/ffi/mod.rs` library loader

**Deskripsi**:
- Global map (OnceLock<Mutex<HashMap<u64, libloading::Library>>>)
- `muat_pustaka(path)`: 
  - Windows: `LoadLibraryW(utf16(path))`
  - Unix: `dlopen(path, RTLD_LAZY)`
  - Store di map, generate unique id
  - Return handle {_tipe:"PustakaC", id, path}
- Error handling: path not found, format invalid → return error (tidak panic)

**Test**:
- Windows: Load "kernel32.dll" → success
- Test invalid path → error, tidak crash
- Test double-load same library → different id

**Acceptance**: Library loading bekerja, error handling OK

---

#### Fase 5.2: Function Lookup (T5.2)
**Deliverable**: Symbol resolution & caching

**Deskripsi**:
- `get_proc_address(handle, func_name)`:
  - Retrieve library dari map
  - `libloading::Symbol::get()` by name
  - Cache dalam HashMap<(lib_id, name), Symbol>
  - Error: symbol not found → return error
- Support Windows mangled names (opsional)

**Test**:
- Lookup "GetTickCount" dari kernel32 → success
- Test invalid function name → error

**Acceptance**: Symbol lookup bekerja

---

#### Fase 5.3: Type Marshalling - Numbers (T5.3)
**Deliverable**: Auto-infer & convert Value::Number → C types

**Deskripsi**:
- Auto-infer: Value::Number → i64 (default)
- Strict types: 
  - i32: range check (-2^31 to 2^31-1), error jika out of range
  - u32: range check (0 to 2^32-1)
  - i64: Value::Number (64-bit safe)
  - f32/f64: Value::Number → f64, then cast
- Call function with correct type via unsafe function pointer
- Return: C return value → Value::Number

**Test**:
- Call kernel32.GetTickCount() → u32 → Value::Number > 1000
- Test range overflow: pass i32 < -2^31 → error

**Acceptance**: Number marshalling bekerja, type safety

---

#### Fase 5.4: Type Marshalling - Bool & String (T5.4)
**Deliverable**: Bool & String type support

**Deskripsi**:
- Bool: Value::Bool → C bool (true=1, false=0)
- String: Value::String → *const c_char (UTF-8 null-terminated)
  - Allocate CString dari Rust string
  - Pass pointer to C
  - After return, decode *const c_char → Value::String (dengan length limit)
- Error: type mismatch → return error (bukan crash)

**Test**:
- Call MessageBeep(0: i32) → void (no return)
- Test string arg (jika ada API yang take string)

**Acceptance**: Bool & String marshalling, no crash on type error

---

#### Fase 5.5: Strict Type Specification (T5.5)
**Deliverable**: `ffi_panggil_tergetik` API

**Deskripsi**:
- `ffi_panggil_tergetik(handle, name, args, return_type_str)`:
  - Parse return_type: "void"|"u8"|"u32"|"u64"|"i32"|"i64"|"f32"|"f64"|"bool"|"string"
  - Validate arg count & types match function signature (best effort)
  - Marshal args & call
  - Convert return sesuai type
- Error: tipe tidak cocok → return error Rust (tidak segfault)

**Test**:
- GetTickCount([], "u32") → u32 result
- GetTickCount([], "i64") → cast to i64 (verify value still > 0)
- MessageBeep([0], "void") → no return

**Acceptance**: Strict type API bekerja aman

---

#### Fase 5.6: Cleanup & Lifecycle (T5.6)
**Deliverable**: Library close & double-close safety

**Deskripsi**:
- `tutup_pustaka(handle)`: remove dari map, drop Library (auto-unload)
- After close: function call → error (bukan segfault)
- Prevent double-close: check id exists before drop

**Test**:
- Load → close → call again → error
- No crash after close

**Acceptance**: Cleanup safe, error on use-after-close

---

#### Fase 5.7: Testing & Regression (T5.7)
**Deliverable**: Full test suite, Windows cfg-gated

**Deskripsi**:
- Unit test: `#[cfg(target_os="windows")]` in `src/web/ffi/mod.rs`
- Integration test: Windows-only test dalam `tests/widya_tests.rs`
- Regression: `cargo test --all` 0 new failed
- Linux/Mac: tests skipped (no-op)

**Acceptance**: Tests pass on Windows, skipped on others, 0 regresi

---

### Acceptance Criteria (AC-FFI)

| AC | Kriteria | Verifikasi |
|----|----------|-----------|
| AC-FFI-1 | Load "kernel32.dll" berhasil | Test muat_pustaka |
| AC-FFI-2 | Call GetTickCount() → u32 > 1000 | Test ffi_panggil_tergetik |
| AC-FFI-3 | MessageBeep(0) void → no crash | Test void return |
| AC-FFI-4 | After tutup_pustaka → error on call | Test cleanup safety |
| AC-FFI-5 | Type mismatch → error (tidak segfault) | Test i32 overflow, wrong type |
| AC-FFI-6 | Backward compat (0 new failed test, studio still work) | `cargo test --all` 0 failed |

---

## T6: Desktop App CLI (widya app jalan/paket)

### Deskripsi
CLI command `widya app` untuk menjalankan Widya app dengan server lokal + browser app-mode, atau membuat distribusi portable. Tidak memerlukan dependensi berat (tanpa wry/WebView2 di sini).

### Dependencies
```
(none - builtin CLI)
```

### API Specification

#### CLI Commands
```bash
widya app jalan <program.wya> [--port N] [--no-browser]
  # Jalankan program (expected: ServerHttp di dalamnya)
  # Port: N atau 0 (OS pilih)
  # Buka browser app-mode otomatis (unless --no-browser)

widya app paket <nama> <program.wya> [--output dir]
  # Buat distribusi: dir <nama>/ berisi program + engine + launcher
  # Output: default "dist/" atau custom
```

### Implementation Plan

#### Fase 6.1: CLI Subcommand Parsing (T6.1)
**Deliverable**: `src/main.rs` subcommand structure

**Deskripsi**:
- Extend clap enum: `Commands { App(AppCmd) }`
- `AppCmd` enum: `Jalan { program, port, no_browser }` | `Paket { nama, program, output }`
- Parse args → dispatch ke handler
- Default port 0 (OS pick), default browser enabled

**Test**:
- CLI parse: `widya app jalan test.wya` → OK
- CLI parse: `widya app paket demo test.wya` → OK

**Acceptance**: CLI parsing bekerja

---

#### Fase 6.2: App Jalan (Run App) (T6.2)
**Deliverable**: Server spawn + browser launch

**Deskripsi**:
- Read program file (check exists)
- Create temp directory (`.widya_app_<timestamp>/`)
- Copy program to temp dir
- Spawn child process: `widya run program.wya` in temp dir
  - Capture stdout/stderr
  - Wait for server startup signal (or ping localhost:port)
- Detect free port (if port=0)
- Launch browser:
  - Windows: try `msedge`, `chrome`, `chromium` in PATH
  - Mac: `open -a "Safari"` or `open -a "Google Chrome"`
  - Linux: `xdg-open`
  - URL: `http://127.0.0.1:port/`
  - Args: `--app=<URL> --new-window`
- Wait for Ctrl+C (SIGINT) or server exit
- Kill browser process (graceful shutdown)
- Cleanup temp dir (opsional, warn user)

**Test**:
- Integration: `widya app jalan contoh/200_web_api_nyata.wya` → server HTTP 200, curl response OK
- Test port detection (port=0 → OS assign)
- Test SIGINT handling

**Acceptance**: App runs, browser launches, Ctrl+C cleanup

---

#### Fase 6.3: App Paket (Bundle App) (T6.3)
**Deliverable**: Portable distribution folder

**Deskripsi**:
- Create dir `<output>/<nama>/`
- Copy files:
  1. Program (rename to `main.wya` or keep original name)
  2. `Widya.toml` metadata:
     ```toml
     [app]
     name = "<nama>"
     version = "0.1.0"
     entry = "main.wya"
     ```
  3. `README.md` (template with usage instructions)
  4. Windows launcher: `jalankan.bat` → `widya app jalan main.wya --no-browser`
  5. Unix launcher: `jalankan.sh` → same
  6. Optionally: `target/release/widya` binary (if --include-binary)
- Output summary: "Package created at <path>, ready to distribute"

**Test**:
- Integration: `widya app paket demo contoh/200_web_api_nyata.wya` → folder contains 5+ files
- Manual: run `jalankan.bat` → server OK

**Acceptance**: Package structure OK, launcher works

---

#### Fase 6.4: Static Assets Support (T6.4)
**Deliverable**: Serve `assets/` folder if exists

**Deskripsi**:
- Detect `assets/` folder in program directory
- If exists, register static route `/assets` → serve files from `assets/`
- Support: HTML, CSS, JS, images (basic MIME types)
- Allow override default index.html with `assets/index.html`

**Test**:
- Create `assets/style.css`, run app → `curl http://localhost:port/assets/style.css` → content OK

**Acceptance**: Static assets served

---

#### Fase 6.5: Testing & Regression (T6.5)
**Deliverable**: Full test suite

**Deskripsi**:
- Smoke test: `widya app jalan contoh/200_web_api_nyata.wya` → 200, curl works
- Test SIGINT handling (kill process, verify cleanup)
- Regression: `cargo test --all` 0 new failed
- Note: browser detection may need platform-specific test (skip in headless CI)

**Acceptance**: Smoke tests pass, 0 regresi

---

### Acceptance Criteria (AC-APP)

| AC | Kriteria | Verifikasi |
|----|----------|-----------|
| AC-APP-1 | `widya app jalan` → server HTTP 200 | Smoke test + curl |
| AC-APP-2 | Browser app-mode launches (or detectable in CI) | Process detection |
| AC-APP-3 | `widya app paket` → distributable folder | Folder structure check |
| AC-APP-4 | Launcher script works standalone | Run paket launcher |
| AC-APP-5 | Static assets served from `assets/` | Curl /assets/* |
| AC-APP-6 | Backward compat (0 new failed test) | `cargo test --all` 0 failed |

---

## T7: WebView2 Desktop Window (wry - Optional Feature)

### Deskripsi
Desktop window implementasi via WebView2 (Windows-only) menggunakan crate `wry`. Fitur opsional behind `--features desktop` untuk menjaga inti tetap ringan. Pola Tauri-like: HTML frontend + local HTTP backend.

### Dependencies
```toml
[features]
desktop = ["dep:wry"]

[dependencies.wry]
version = "0.40"
optional = true
```

### API Specification

#### Builtin
```widya
JendelaDesktop(judul_string, lebar_int, tinggi_int, url_string)
  -> window_handle {_tipe:"JendelaDesktop", judul, width, height, status:"TERBUKA"}
  # Mode A: actual WebView2 window (if feature desktop enabled)
  # Mode B: fallback to browser app-mode (if feature not enabled)
```

### Implementation Plan

#### Fase 7.1: Optional Feature Isolation (T7.1)
**Deliverable**: Feature flag, build clean with/without

**Deskripsi**:
- Add to Cargo.toml: `desktop = ["dep:wry"]`, `wry = { ... optional = true }`
- Conditional compilation: `#[cfg(feature = "desktop")]`
- `cargo build --release` (default) → no wry, small binary
- `cargo build --release --features desktop` → with wry, larger binary
- Fallback logic: if feature not enabled, use T6 browser-app instead

**Test**:
- Build clean: `cargo build --release` → no error
- Build with feature: `cargo build --release --features desktop` → no error (Windows)
- Test on Linux: feature disabled automatically (build OK)

**Acceptance**: Feature isolation complete, no build errors

---

#### Fase 7.2: Event Loop Integration (T7.2)
**Deliverable**: wry event loop as main driver, non-blocking socket poll

**Deskripsi**:
- `wry` runs event loop: Window message loop (GetMessage/DispatchMessage on Windows)
- Server socket (from T3) run non-blocking:
  - Wrap TcpListener with non-blocking mode
  - In wry frame callback, poll listener (try_accept)
  - If connection pending, spawn handler thread (or queue)
  - Handler = fresh Interpreter per request (pola T3)
- Keep UI responsive while handler processes

**Test**:
- Integration: window open, navigate to `http://localhost:port/`, can interact while handler processes

**Acceptance**: Event loop responsive, UI not frozen during processing

---

#### Fase 7.3: WebView2 Window Setup (T7.3)
**Deliverable**: wry::WebViewBuilder window creation

**Deskripsi**:
- `wry::WebViewBuilder::new()`
- Set URL: `http://127.0.0.1:port/` (local server from T3)
- Set window title, width, height
- Init developer tools (optional, debug mode)
- Error handling: WebView2 runtime not installed → fallback to T6 browser-app or error message

**Test**:
- Integration: window opens, loads local URL, HTML rendered

**Acceptance**: Window appears, HTML loads

---

#### Fase 7.4: IPC (UI ↔ Backend) via HTTP (T7.4)
**Deliverable**: Frontend fetch → Backend HTTP server

**Deskripsi**:
- Frontend (HTML in WebView): use `fetch()` API to call `http://127.0.0.1:port/api/*`
- Backend: HTTP server (T3 engine) handles requests
- Request JSON ↔ Response JSON pattern
- No special IPC protocol needed (standard HTTP)
- Example: UI button click → `fetch("/api/action") → backend process → response`

**Test**:
- Integration: Frontend fetch call → backend handler responds

**Acceptance**: HTTP-based IPC works

---

#### Fase 7.5: Window Lifecycle & Shutdown (T7.5)
**Deliverable**: Clean window close, server shutdown

**Deskripsi**:
- Window close signal → trigger shutdown
- Server: finish pending requests, close listener
- Cleanup: drop window, exit event loop
- Graceful: no hanging processes
- Error handling: panic safety, cleanup even on error

**Test**:
- Integration: close window → process exits cleanly
- Test: kill window mid-request → server cleanup OK

**Acceptance**: Clean shutdown, no orphan processes

---

#### Fase 7.6: Testing & Regression (T7.6)
**Deliverable**: Feature test, platform gating, no regresi

**Deskripsi**:
- Windows-only tests: `#[cfg(all(target_os="windows", feature="desktop"))]`
- Headless test (optional): xvfb or virtual display
- Manual: `cargo run --features desktop -- contoh/203_app_desktop.wya`
- CI: skip if WebView2 not available
- Regression: `cargo test --all` 0 new failed

**Acceptance**: Tests pass on Windows, skipped elsewhere, 0 regresi

---

### Acceptance Criteria (AC-DSK)

| AC | Kriteria | Verifikasi |
|----|----------|-----------|
| AC-DSK-1 | Window opens (WebView2) | Visual confirmation or process detection |
| AC-DSK-2 | HTML/CSS rendered inside window | Window shows content |
| AC-DSK-3 | Backend server responds to fetch calls | HTTP 200 + JSON |
| AC-DSK-4 | Feature flag works (build with/without) | Build clean on both modes |
| AC-DSK-5 | Clean shutdown on window close | No orphan processes |
| AC-DSK-6 | Backward compat (0 new failed test, fallback if no wry) | `cargo test --all` 0 failed, T6 fallback works |

---

## T8: Contoh Nyata + Tutorial CLI Lengkap

### Deskripsi
Empat contoh program Widya demonstrasi fitur web/desktop (T1-T7), dan tutorial tertulis menjelaskan arsitektur serta API.

### Deliverables

#### Contoh Program

**contoh/200_web_api_nyata.wya** (HTTP Server + REST API + Static Files)
```widya
# REST API demonstrasi T3 ServerHttp
# GET /api/data → JSON list
# POST /api/data → tambah item
# DELETE /api/data/:id → hapus item
# Static: /assets/* → serve dari folder assets/

data = []

server = ServerHttp(3000)
tambah_rute(server, "GET", "/api/data", func(req) {
  return {status: 200, badan: data}
})
tambah_rute(server, "POST", "/api/data", func(req) {
  data.tambah(req.badan)
  return {status: 201, badan: {sukses: true}}
})
sajikan_static(server, "/assets", "assets")
jalankan(server)
```

**contoh/201_chat_websocket.wya** (WebSocket Chat Server + Client Loopback)
```widya
# Chat demonstrasi T4 WebSocket
# Server broadcast, client loopback

server_ws = ServerWebSocket(3001)
clients = []

# Server loop (dalam thread)
buat_utas(func() {
  ulang {
    client_id = ws_terima(server_ws)
    clients.tambah(client_id)
    
    ulang {
      msg = ws_baca(server_ws, client_id)
      jika msg.tipe == "tutup" {
        lepas
      }
      cetak("Chat: " + msg.data)
      # Broadcast
      untuk client di clients {
        ws_kirim(server_ws, client, msg.data)
      }
    }
  }
})

# Client test
client = WebSocket("ws://127.0.0.1:3001")
ws_kirim(client, "halo dari client")
msg = ws_baca(client)
cetak("Received: " + msg.data)
ws_tutup(client)
```

**contoh/202_ffi_windows.wya** (FFI Windows API Call)
```widya
# FFI demonstrasi T5 (Windows-only)

jika platform == "windows" {
  lib = muat_pustaka("kernel32.dll")
  tick = ffi_panggil_tergetik(lib, "GetTickCount", [], "u32")
  cetak("Current tick: " + tick)
  tutup_pustaka(lib)
} sebaliknya {
  cetak("FFI demo hanya untuk Windows")
}
```

**contoh/203_app_desktop.wya** (Desktop App + Dashboard HTML)
```widya
# Desktop app demonstrasi T6 + T7
# Jalankan dengan: widya app jalan contoh/203_app_desktop.wya

server = ServerHttp(0)
tambah_rute(server, "GET", "/", func(req) {
  html = render_dashboard_html({
    title: "Widya Desktop App",
    content: "Selamat datang di aplikasi desktop Widya!"
  })
  return {status: 200, badan: html, tipe_konten: "text/html"}
})
jalankan(server)
```

#### Tutorial Section

**dokumen_widya/TUTORIAL_11_CLI_LENGKAP.md** - Section Baru

Add section "🌐 Web & 🖥️ Desktop Level Industri" (60+ lines):

- Penjelasan arsitektur: HTTP engine, WebSocket frame, FFI call, desktop modes
- Syntax & API: `ServerHttp`, `tambah_rute`, `ServerWebSocket`, `WebSocket`, `muat_pustaka`, `ffi_panggil_tergetik`, `JendelaDesktop`, `widya app jalan`
- Copy-paste example untuk setiap API
- Tabel endpoint HTTP standar
- Konsep fresh-interpreter per request
- Limitasi & best practice
- Troubleshoot: port already in use, browser not found, etc.

Update **TUTORIAL_INDEX.md**: link ke section baru

### Acceptance Criteria (AC-DOC)

| AC | Kriteria | Verifikasi |
|----|----------|-----------|
| AC-DOC-1 | `contoh/200_*` jalan → HTTP 200 | `cargo run -- contoh/200_web_api_nyata.wya` + curl |
| AC-DOC-2 | `contoh/201_*` jalan → chat output | Output on stdout |
| AC-DOC-3 | `contoh/202_*` jalan (Windows) → tick > 0 | Windows-only test |
| AC-DOC-4 | `contoh/203_*` jalan via `widya app jalan` | Server 200 + HTML |
| AC-DOC-5 | Tutorial section 60+ lines, clear API explanation | Word count, link in INDEX |
| AC-DOC-6 | All examples copy-paste runnable | No syntax errors, tested |

---

## T9: Test Penuh, Build Release, Smoke End-to-End

### Deskripsi
Verifikasi final: run full test suite, build release clean, smoke test semua komponen T1-T8, dokumentasi updated.

### Implementation Plan

#### Fase 9.1: Full Test Suite (T9.1)
**Deliverable**: All tests pass, 0 failed

**Deskripsi**:
- Run: `cargo test --all`
- Expected: 283 (T1-T3) + X (T4) + Y (T5) + Z (T6) + W (T7) + V (T8) test cases
- Result: all pass, 0 failed
- Record: test count in tasks.md "Catatan Verifikasi Final"

**Test**:
- `cargo test --all 2>&1 | tee artifacts/test_output.txt`
- Parse output: count passed/failed

**Acceptance**: All tests pass, count recorded

---

#### Fase 9.2: Release Build Clean (T9.2)
**Deliverable**: Release binary, 0 warning

**Deskripsi**:
- Build: `cargo build --release`
- Expected: 0 warning
- Size: binary < 20 MB (inti), < 50 MB (dengan desktop feature)
- Build: `cargo build --release --features desktop` (Windows)
- Expected: 0 warning

**Test**:
- `cargo build --release 2>&1 | tee artifacts/build_output.txt`
- Check for warnings

**Acceptance**: Clean build, size acceptable

---

#### Fase 9.3: Smoke Studio (T9.3)
**Deliverable**: 25+ endpoints working

**Deskripsi**:
- Start: `widya studio --port <N>` in background
- Smoke test endpoints:
  - `GET /` → 200, HTML IDE
  - `GET /api/contoh` → 200, JSON contoh list
  - `POST /api/run` → 200, run result
  - `POST /api/check` → 200, diagnostic
  - `GET /api/run/stream` → 200, SSE stream
  - `GET /unknown` → 404
  - Rate limit: 121 requests → 429 (on 121st)
  - OPTIONS → 204
- Expected: all pass

**Test**:
- Script `tests/smoke_studio.sh` (or .bat for Windows)
- Curl each endpoint, check status

**Acceptance**: Studio smoke pass

---

#### Fase 9.4: Smoke T1-T3 (T9.4)
**Deliverable**: HTTP client/server/engine working

**Deskripsi**:
- T1: `http_get("http://127.0.0.1:studio_port/api/contoh")` → 200, JSON parsed
- T2: Studio engine still 25+ endpoint (no regression)
- T3: `ServerHttp` API works (from smoke test script)

**Acceptance**: T1-T3 smoke pass

---

#### Fase 9.5: Smoke T4-T7 (T9.5)
**Deliverable**: WebSocket, FFI, Desktop working

**Deskripsi**:
- T4: `contoh/201_chat_websocket.wya` output contains message
- T5: `contoh/202_ffi_windows.wya` (Windows) output contains tick > 0
- T6: `widya app jalan contoh/200_web_api_nyata.wya` → server 200, browser detected
- T7: (Windows) Build `--features desktop` → 0 warning

**Acceptance**: T4-T7 smoke pass (platform-gated)

---

#### Fase 9.6: Smoke T8 (T9.6)
**Deliverable**: Examples & tutorial complete

**Deskripsi**:
- T8.1: `contoh/200_web_api_nyata.wya` → curl CRUD works
- T8.2: `contoh/201_chat_websocket.wya` → stdout message OK
- T8.3: `contoh/202_ffi_windows.wya` → tick value OK
- T8.4: `contoh/203_app_desktop.wya` via `widya app jalan` → 200 HTML
- Tutorial: TUTORIAL_11 section exists, 60+ lines

**Acceptance**: T8 smoke pass, tutorial exists

---

#### Fase 9.7: Documentation Update (T9.7)
**Deliverable**: Docs updated, API reference complete

**Deskripsi**:
- Update `docs/API_REFERENCE.md`: add HTTP, WebSocket, FFI, Desktop sections
- Update `README.md` if needed: mention new features
- Update `Cargo.toml` description: "Widya: Bahasa pemrograman dengan web/desktop level industri"
- Changelog: T1-T8 summary

**Acceptance**: Docs updated and clear

---

#### Fase 9.8: Final Verification Checklist (T9.8)
**Deliverable**: Checklist completion, recorded in tasks.md

**Checklist**:
- [ ] `cargo test --all` → 0 failed
- [ ] `cargo build --release` → 0 warning
- [ ] `cargo build --release --features desktop` (Windows) → 0 warning
- [ ] Studio smoke (25+ endpoint) → all OK
- [ ] T1-T8 smoke → all OK
- [ ] Examples runnable → all OK
- [ ] Tutorial updated → link in INDEX
- [ ] Documentation complete → API reference
- [ ] tasks.md status → all `done`
- [ ] spec.md reviewed & updated

**Record**: results in tasks.md "Catatan Verifikasi Final" section

**Acceptance**: All items checked, recorded

---

### Acceptance Criteria (AC-TEST)

| AC | Kriteria | Verifikasi |
|----|----------|-----------|
| AC-TEST-1 | `cargo test --all` → 0 failed, count recorded | Test output logged |
| AC-TEST-2 | Release build clean (0 warning, size OK) | Build output logged |
| AC-TEST-3 | Studio smoke 25+ endpoint pass | Endpoint test script |
| AC-TEST-4 | T1-T8 smoke all pass | Smoke test output |
| AC-TEST-5 | Examples runnable & produce expected output | Example output logged |
| AC-TEST-6 | Documentation complete & referenced | Docs review |

---

## T10: Verifikasi Akhir + Dokumentasi + Commit

### Deskripsi
Final verification of all acceptance criteria, documentation updates, and commit to repository.

### Implementation Plan

#### Fase 10.1: AC Checklist Review (T10.1)
**Deliverable**: All AC from spec.md 9 verified

**Deskripsi**:
- Review spec.md section 9 (Industry-level definition)
- Check each criterion:
  1. HTTP client & server bekerja nyata (bukan simulasi) → T1 + T3 smoke ✓
  2. WebSocket RFC 6455 interoperable → T4 handshake test ✓
  3. FFI memanggil DLL asli → T5 GetTickCount ✓
  4. Desktop window tampil → T6 + T7 smoke ✓
  5. Semua test lulus 0 failed → T9 test suite ✓
  6. Dokumentasi tutorial updated → T8 tutorial ✓
  7. Contoh nyata tersedia → T8 examples ✓
  8. Tidak ada regresi → T9 studio 25+ endpoint, `cargo test --all` 0 failed ✓

**Acceptance**: All 8 criteria verified

---

#### Fase 10.2: Update Documentation (T10.2)
**Deliverable**: tasks.md, spec.md, docs updated with final status

**Deskripsi**:
- Update `tasks.md`: all T1-T10 status → `done`
- Add "Catatan Verifikasi Final" section:
  ```
  Catatan Verifikasi Final (2026-09-16):
  
  ✓ T1: HTTP client (ureq) implemented, TR-1.1 to TR-1.5 pass
  ✓ T2: Engine extracted to src/web/mod.rs, studio regression 0 failed
  ✓ T3: ServerHttp nyata working, TR-3.1 to TR-3.5 pass
  ✓ T4: WebSocket RFC 6455, TR-4.1 to TR-4.5 pass
  ✓ T5: FFI libloading, TR-5.1 to TR-5.4 pass
  ✓ T6: Desktop app CLI, TR-6.1 to TR-6.3 pass
  ✓ T7: WebView2 (optional), TR-7.1 to TR-7.4 pass
  ✓ T8: Examples + tutorial, TR-8.1 to TR-8.6 pass
  ✓ T9: Full test suite + smoke, TR-9.1 to TR-9.3 pass
  ✓ T10: Final verification + commit
  
  Total test count: [X] passed / 0 failed
  Binary size: inti [Y] MB, desktop [Z] MB
  Warnings: 0
  
  Status: READY FOR PRODUCTION
  ```
- Update TUTORIAL_INDEX.md: link to TUTORIAL_11 section
- Update `docs/API_REFERENCE.md` with T1-T7 APIs
- Update README.md description if needed

**Acceptance**: All docs updated, status recorded

---

#### Fase 10.3: Commit to Git (T10.3)
**Deliverable**: Signed commit to main/develop branch

**Deskripsi**:
- Review `git status` (check no unintended files)
- Stage files:
  - Code: `src/web/**/*.rs`, `src/stdlib/mod.rs`, `src/main.rs`, `src/lib.rs`
  - Config: `Cargo.toml`, `Cargo.lock`
  - Docs: `.trae/specs/`, `dokumen_widya/`, `docs/`
  - Examples: `contoh/200-203.wya`
  - Tests: `tests/*.rs`
- Unstaged (gitignore):
  - `target/`, `debug/`, `artifacts/`, `.widya_app_*`
- Commit message (style per repo):
  ```
  feat(web): Widya Web & Desktop Level Industri (T1-T10)
  
  Complete implementation of web & desktop layer:
  
  - T1: HTTP client real (ureq 2.12 + rustls, TLS/timeout/JSON/error)
  - T2: Engine extraction → src/web/mod.rs, studio reuse
  - T3: ServerHttp real (routing/static/JSON/CORS/fresh-interpreter handler)
  - T4: WebSocket RFC 6455 (sha1+base64 handshake, frame encode/decode, multi-client)
  - T5: FFI C/DLL (libloading, auto-infer + strict type, marshal number/bool/string)
  - T6: Desktop app CLI (widya app jalan/paket, browser app-mode, portable bundle)
  - T7: WebView2 wry (optional feature "desktop", Tauri-like IPC)
  - T8: Examples + tutorial (200-203.wya, TUTORIAL_11 section)
  - T9: Full test suite (0 failed), release build (0 warning), smoke all components
  - T10: Final verification + AC checklist + documentation
  
  Regression testing:
  - cargo test --all: [X] passed / 0 failed (no new failures)
  - Studio endpoints: 25+ all working (rate limit, SSE, CORS, etc.)
  - All examples runnable and produce expected output
  
  Backward compatibility:
  - No breaking changes to existing API or stdlib
  - HTTP client/server shapes match old interface (sukses/status/badan/url)
  - WebSocket/FFI/Desktop use new builtins (no conflict)
  - Feature flag "desktop" optional (inti remains lightweight)
  
  Ready for production use.
  ```
- Push to branch: `git push -u origin feat/web-desktop-industri`

**Acceptance**: Commit recorded, branch pushed

---

#### Fase 10.4: Create Pull Request (T10.4)
**Deliverable**: PR/MR open for review

**Deskripsi**:
- Use `gh pr create` (GitHub) or `glab mr create` (GitLab)
- Title: `feat: Widya Web & Desktop Level Industri (T1-T10)`
- Description: (use commit message, add extra context if needed)
- Add checklist:
  ```markdown
  ## Acceptance Criteria Checklist
  - [x] AC-HTTP (T1): Client real (TLS, timeout, JSON)
  - [x] AC-WEB-ENG (T2): Engine reusable (studio no change)
  - [x] AC-WebApp (T3): ServerHttp real (handler, static, CORS)
  - [x] AC-WS (T4): WebSocket RFC 6455 (handshake, frame, multi-client)
  - [x] AC-FFI (T5): FFI real (LoadLibrary, call, close, marshal)
  - [x] AC-APP (T6): Desktop CLI (server, browser, paket)
  - [x] AC-DSK (T7): WebView2 (window, IPC, lifecycle)
  - [x] AC-DOC (T8): Examples + tutorial (60+ lines, 4 examples)
  - [x] AC-TEST (T9): Full suite (0 failed, build clean, smoke)
  - [x] AC-META (T10): Verification + docs + commit
  - [x] Regression: cargo test --all = 0 failed
  - [x] Build: cargo build --release = 0 warning
  - [x] Studio: 25+ endpoint = all working
  ```
- Request review if needed

**Acceptance**: PR created and visible

---

#### Fase 10.5: Post-Commit (T10.5)
**Deliverable**: Tag, release notes, cleanup

**Deskripsi**:
- Tag version (if releasing): `git tag v0.5.0 feat/web-desktop-industri`
- Draft release notes (GitHub/GitLab Releases):
  - Title: "Widya v0.5.0: Web & Desktop Level Industri"
  - Content: T1-T10 summary, new features, examples, upgrade guide
- Update main README.md if needed:
  - Add section: "New in v0.5.0: Real HTTP, WebSocket, FFI, Desktop"
  - Link to tutorial, examples, API reference
- Optional: announce in Discord/community forum

**Acceptance**: Version tagged, release notes drafted, community notified

---

### Acceptance Criteria (AC-META)

| AC | Kriteria | Verifikasi |
|----|----------|-----------|
| AC-META-1 | Semua T1-T10 status `done` di tasks.md | tasks.md review |
| AC-META-2 | All 8 industry criteria verified (spec.md 9) | Checklist complete |
| AC-META-3 | Documentation updated (tutorial, API ref, README) | Docs review |
| AC-META-4 | Commit created with proper message & files staged | `git log --oneline` check |
| AC-META-5 | PR/MR open (if applicable) or commit merged | PR/branch visible |
| AC-META-6 | `git status` clean (no untracked code files) | Status check |

---

## Verification Checklist (Overall)

### Before Starting Implementation
- [ ] Review spec.md & tasks.md thoroughly
- [ ] Verify all dependencies in Cargo.toml
- [ ] Set up test infrastructure (test directory, frameworks)
- [ ] Identify regression test baseline (current 283 tests)

### During Implementation (per task T4-T10)
- [ ] Implement each phase
- [ ] Write unit tests
- [ ] Write integration tests
- [ ] Run smoke test
- [ ] Verify no regressions (`cargo test --all`)
- [ ] Update documentation as you go
- [ ] Commit work (or accumulate for final commit)

### Before Final Verification (T9)
- [ ] Run full test suite: `cargo test --all`
- [ ] Build release: `cargo build --release`
- [ ] Build with features (if applicable): `cargo build --release --features desktop`
- [ ] Run smoke tests for all components
- [ ] Review all documentation

### Final Steps (T10)
- [ ] Review all AC from spec.md section 9
- [ ] Update tasks.md with final status & notes
- [ ] Update spec.md if design changed
- [ ] Stage all files in git
- [ ] Create commit with proper message
- [ ] Push to branch & create PR if applicable
- [ ] Tag version if releasing

---

## Success Criteria Summary

**Project Success = All of the following:**

1. **Code Quality**:
   - `cargo test --all` → 0 failed (all test pass)
   - `cargo build --release` → 0 warning
   - No code duplication (reuse engine from T2)
   - Follow existing code style & conventions

2. **Feature Completeness**:
   - T1: HTTP client real with TLS, timeout, JSON
   - T2: Engine reusable (studio uses it)
   - T3: ServerHttp works with routing & handler
   - T4: WebSocket RFC 6455 compliant
   - T5: FFI calls C/DLL real (Windows)
   - T6: Desktop app CLI bundling
   - T7: WebView2 optional feature (not blocking if wry problems)
   - T8: 4 examples + tutorial 60+ lines
   - T9: All smoke tests pass
   - T10: Documentation, commit, release ready

3. **Backward Compatibility**:
   - No breaking changes to existing builtins
   - Studio 25+ endpoints still working
   - HTTP client/server response shapes match old interface
   - All new features behind feature flags or separate API (WebSocket, FFI, Desktop)

4. **Documentation**:
   - Tutorial updated with examples
   - API reference complete
   - Examples copy-paste runnable
   - README reflects new capabilities

5. **Testing**:
   - Unit tests for all modules
   - Integration tests for cross-module interaction
   - Smoke tests for end-to-end workflows
   - No regression in existing functionality

---

**End of T4-T10 Comprehensive Implementation Plan**

Generated: 2026-09-16
Status: Ready for Implementation
