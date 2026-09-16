# T3: ServerHttp Nyata — Rencana Implementasi Komprehensif

**Status**: Desain & persiapan awal (struktur `ServerHandle` + builtin stubs selesai di web/mod.rs)  
**Target**: Server HTTP Widya yang mengeksekusi handler Widya function per request  
**Blocker**: Integrasi `WebServer` dengan fresh `Interpreter` per request (pola `ValueSerial`)

---

## 1. Ringkasan Fungsionalitas T3

### Tujuan
Menggantikan mock `ServerHttp` (simulasi) dengan server HTTP NYATA yang:
- Bind ke port lokal
- Terima request HTTP (GET, POST, OPTIONS)
- Routing ke handler Widya function
- Return response JSON/HTML
- Deterministic (mode "tunggal") atau thread-per-request (mode "utas")

### API Widya yang Diimplementasi

```widya
// Buat server pada port
srv = ServerHttp(3000)                   -> {_tipe:"ServerHttp", port:3000, status:"SIAP"}

// Daftarkan handler
tambah_rute(srv, "GET", "/", fungsi_handler)
tambah_rute(srv, "POST", "/api/data", handler_post)

// Jalankan server (blocking)
jalankan(srv)                            // listen & accept loop sampai tutup()

// Tutup server dari handler lain atau signal
tutup(srv)                               // set flag stop, loop keluar

// Akses request dalam handler
fungsi_handler = fung(req) {
    pesan("Metode: " + req.method)       // "GET", "POST", dll
    pesan("Path: " + req.path)           // "/path"
    pesan("Query: " + req.query)         // map atau nil
    pesan("Header: " + req.header)       // map header
    pesan("Badan: " + req.badan)         // string atau nil
    
    // Return response
    kembalikan {
        status: 200,
        badan: "Halo Dunia",
        tipe_konten: "text/plain",
        header_kustom: {X-Custom: "value"}
    }
}
```

---

## 2. Arsitektur Implementasi

### 2.1 Komponen Utama

#### A. **ServerHandle** (di `src/web/mod.rs`)
```rust
struct ServerHandle {
    port: u16,
    mode: String,                    // "tunggal" | "utas"
    status: String,                  // "SIAP", "BERJALAN", "BERHENTI"
    rute_get: HashMap<String, Value>,    // path -> Value::Function (AST)
    rute_post: HashMap<String, Value>,
    static_map: HashMap<String, String>, // prefix -> folder
    running: bool,                   // flag stop untuk accept loop
}
```

**Penyimpanan Global:**
```rust
static WIDYA_SERVER_HANDLES: OnceLock<Mutex<HashMap<u64, ServerHandle>>> = ...
```

Setiap server diberi ID unik (kombinasi port + timestamp mikro) untuk support multiple servers dalam proses yang sama.

#### B. **RequestInfo Map** (sudah ada di web/mod.rs)
Request HTTP dikonversi ke Value::Map dengan field:
```widya
{
    method: "GET" | "POST",
    path: "/path",
    query: nil atau {key1: val1, ...},
    header: {header1: val1, ...},
    badan: string atau nil (untuk POST)
}
```

#### C. **ResponseInfo Map** (handler return)
Handler Widya mengembalikan map:
```widya
{
    status: 200,
    badan: "string" atau {json},
    tipe_konten: "text/html" (opsional, default "application/json"),
    header_kustom: {X-Custom: "val"} (opsional)
}
```

#### D. **WebServer Integration** (sudah ada)
Engine di `src/web/mod.rs` (`WebServer`, `Respon` enum) SUDAH siap, cuma perlu dipopulasikan dengan handler Widya.

---

### 2.2 Alur Eksekusi

```
1. Pengguna kode Widya
   srv = ServerHttp(3000)
   tambah_rute(srv, "GET", "/", fung_handler)
   jalankan(srv)  // <- blocking call

2. builtin_jalankan(server_handle)
   a) Extract server_id dari handle
   b) Akses WIDYA_SERVER_HANDLES[server_id]
   c) Buat WebServer::new(port)?
   d) Register semua rute:
      - Untuk setiap path di rute_get/rute_post:
        ws.registrasi_rute("GET", path, |req_info| {
            // Clone function AST dari ServerHandle
            // Buat Interpreter baru
            // Set env dengan function
            // Eksekusi handler(req_info)
            // Return response map -> Respon::Json(...)
        })
   e) ws.jalankan() // blocking accept loop

3. Request masuk dari client
   a) WebServer::layani_koneksi menerima TCP stream
   b) Parse request -> RequestInfo
   c) Cari di route_map -> handler function
   d) Handler: 
      - Buat fresh Interpreter
      - Clone function AST ke env
      - Call interpreter.call_function(handler, [req_info])
      - Get response map
      - Convert ke Respon enum
   e) Kirim response via kirim_response()

4. tutup(srv) dipanggil (dari handler lain / signal)
   a) Set ServerHandle.running = false
   b) Accept loop deteksi flag, keluar
   c) Close TcpListener
```

---

## 3. Pembabakan Implementasi

### **Fase 1: Persiapan (SELESAI)**
- [x] Definisi `ServerHandle` struct
- [x] Global `WIDYA_SERVER_HANDLES` OnceLock
- [x] Builtin stubs: `ServerHttp`, `tambah_rute`, `jalankan`, `tutup`
- [x] Convert request -> RequestInfo map (SUDAH dari T2)

**File**: `src/web/mod.rs` lines 303-450 (approx)

---

### **Fase 2: Handler Execution (NEXT)**

#### 2a. Fresh Interpreter per Request
- Implementasi pola: Clone function AST → buat Interpreter baru → set env → execute
- Helper function: `fn execute_handler_widya(fn_ast, req_info) -> Result<Value, Galat>`
- Gunakan `crate::interpreter::Interpreter::new()` + `interpreter.call_function()`
- **Risk**: Cycle detection (AST borrow); mitigation: ensure AST is Copy-able atau use serialization (ValueSerial)

#### 2b. WebServer Integration dalam `builtin_jalankan`
```rust
fn builtin_jalankan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let server_id = extract_server_id(args)?;
    let handles = widya_server_handles().lock().unwrap();
    let handle = handles.get(&server_id).ok_or(...)?;
    
    let mut ws = WebServer::new(handle.port)?;
    
    // Register rute GET
    for (path, fn_value) in &handle.rute_get {
        let fn_clone = fn_value.clone();
        ws.registrasi_rute("GET", path, move |req_info| {
            match execute_handler_widya(&fn_clone, req_info) {
                Ok(resp_map) => convert_response_map_to_respon(resp_map),
                Err(e) => Respon::Status(500, ...)
            }
        });
    }
    
    // Register rute POST (sama)
    // ...
    
    // Run server (blocking)
    ws.jalankan();
    
    Ok(Value::Nil)
}
```

**Challenges:**
- `fn_clone` adalah `Value::Function` (Rc<RefCell<Function>>) non-Copy, non-Send
  - **Solution**: Extract AST dari Function, simpan di struct Send-able (misal `Arc<Program>`)
  - Atau: Use `ValueSerial` pattern (serialize Value ke JSON, deserialize di thread)
- Closure dalam `registrasi_rute` harus `Send + Sync + 'static`
  - **Solution**: Wrap dalam Arc, atau use thread_local interpreter pool

**File**: `src/web/mod.rs` builtin_jalankan() updated

---

### **Fase 3: Response Conversion**

#### 3a. Map Response Handler ke Respon Enum
```rust
fn convert_response_map_to_respon(resp_map: &Value) -> Respon {
    match resp_map {
        Value::Map(m) => {
            let m = m.borrow();
            let status = m.get("status")
                .and_then(|v| if let Value::Number(n) = v { Some(*n as u16) } else { None })
                .unwrap_or(200);
            let badan = m.get("badan").cloned().unwrap_or(Value::Nil);
            let tipe_konten = m.get("tipe_konten")
                .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                .unwrap_or_else(|| "application/json".to_string());
            
            let badan_str = if let Value::String(s) = &badan { 
                s.clone() 
            } else { 
                value_to_json(&badan).to_string() // JSON encode
            };
            
            match tipe_konten.as_str() {
                "text/html" | "text/plain" => Respon::Html(badan_str),
                _ => Respon::Json(badan_str),
            }
        }
        _ => Respon::Status(500, serde_json::json!({"sukses": false}).to_string()),
    }
}
```

**File**: `src/web/mod.rs` helper function

---

### **Fase 4: Mode Tunggal vs Utas**

#### 4a. Mode "tunggal"
- Single-threaded event loop
- `WebServer::jalankan()` runs in main interpreter thread
- Handler runs inline (no thread spawn)
- **Deterministic**, cocok untuk development & simple apps

**Implementation**: Current `WebServer::jalankan()` already single-threaded per design.

#### 4b. Mode "utas" (thread-per-request)
- Each request spawned in thread
- Fresh Interpreter cloned per thread
- Handler runs independently
- **Risk**: Rc<RefCell> can't cross thread boundary → need ValueSerial

**Implementation**: Defer to post-T3 enhancement; T3 defaults to "tunggal".

**File**: `src/web/mod.rs` builtin_jalankan() mode flag check

---

### **Fase 5: Static File Serving (OPTIONAL T3)**

```rust
sajikan_static(srv, "/aset", "folder_path")
```

- Map URL prefix to disk folder
- In `layani_koneksi`: check if path matches prefix, serve file with MIME type
- **Defer**: Could be added after core handler works

---

### **Fase 6: Testing & Verification**

#### 6a. Unit Tests (in `web/mod.rs` #[cfg(test)])
- Test `ServerHandle` creation & rute registration
- Mock handler execution (simplified interpreter call)
- Response map conversion

#### 6b. Integration Tests (`tests/widya_tests.rs`)
- Start server on random port
- Make HTTP requests
- Verify response status, body, headers
- Test GET, POST, OPTIONS
- Test error handling (404, 500)

#### 6c. Regression (cargo test --all)
- Must remain 289+ passed / 0 failed
- Ensure studio still works (no regression)

**Acceptance Criteria (TR-3.1 to TR-3.5)**:
- TR-3.1: Simple Widya program with ServerHttp + tambah_rute + jalankan → local curl gets response
- TR-3.2: Handler reads POST JSON → processes → returns JSON result
- TR-3.3: sajikan_static serves files (if implemented)
- TR-3.4: Multiple requests handled (no panic on errors)
- TR-3.5: cargo test --all → 0 new failures

**File**: `tests/widya_tests.rs` new test functions

---

## 4. Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│ Widya Program Execution (Main Interpreter Thread)           │
└─────────────────────────────────────────────────────────────┘
                           ↓
         srv = ServerHttp(3000)
         tambah_rute(srv, "GET", "/", handler_fn)
                           ↓
         jalankan(srv)  // builtin_jalankan called
                           ↓
    ┌────────────────────────────────────────────────────┐
    │ builtin_jalankan(server_handle)                    │
    │  - Extract ServerHandle from WIDYA_SERVER_HANDLES  │
    │  - Create WebServer::new(port)                     │
    │  - For each rute in handle.rute_get/post:         │
    │    - Clone function AST                           │
    │    - Register closure with WebServer              │
    │  - Call ws.jalankan() [BLOCKING]                  │
    └────────────────────────────────────────────────────┘
                           ↓
    ┌────────────────────────────────────────────────────┐
    │ WebServer Accept Loop (in separate thread spawned) │
    │ for stream in listener.incoming():                 │
    │   spawn thread: layani_koneksi(stream)             │
    └────────────────────────────────────────────────────┘
                           ↓
    ┌────────────────────────────────────────────────────┐
    │ layani_koneksi(stream, route_handlers)             │
    │  1. Read HTTP request from stream                  │
    │  2. Parse → RequestInfo map                        │
    │  3. Find matching rute                             │
    │  4. Call handler closure:                          │
    │     - Create fresh Interpreter                    │
    │     - Set env with function AST                    │
    │     - Execute: interpreter.call_function(handler, [req_info])
    │  5. Get response map from handler                  │
    │  6. Convert to Respon enum                         │
    │  7. Send HTTP response via kirim_response()        │
    │  8. Close connection                               │
    └────────────────────────────────────────────────────┘
```

---

## 5. Known Challenges & Solutions

### Challenge 1: Rc<RefCell> Non-Send Across Threads
**Problem**: `Value::Function` contains Rc (not Send), can't move into thread closure.

**Solutions** (ranked by feasibility):
1. **Use Mode "tunggal"**: Single-threaded; runs handler in main thread. ✅ **Simplest for T3**
2. **Serialize AST to Arc<Program>**: Extract program AST from Function, wrap in Arc (Send-able), deserialize in thread. ⚠️ **Needs AST serialization**
3. **ValueSerial Pattern**: Serialize Value to JSON in main thread, deserialize in thread as fresh Value. ⚠️ **Loses type info**

**Decision**: T3 defaults to "tunggal" (single-threaded). Thread-per-request ("utas") deferred to T3b or future.

### Challenge 2: Fresh Interpreter Setup
**Problem**: Need to set up environment with handler function, execute it, get result.

**Solution**: 
```rust
let mut interp = Interpreter::new();
// Clone function AST into env
let fn_ast = extract_function_ast(&handler_fn_value)?;
interp.define_function("__handler".to_string(), fn_ast)?;
// Call function with request
let result = interp.call_function("__handler", &[req_info_value])?;
```

Requires: `Interpreter` API to support programmatic function definition (may need refactor).

### Challenge 3: Response Conversion
**Problem**: Handler returns arbitrary map; need to convert to HTTP response.

**Solution**: Strict schema for response map:
```widya
{
    status: number (default 200),
    badan: string | map (auto-JSON if map),
    tipe_konten: string (default "application/json"),
    header_kustom: map (optional, merged into response headers)
}
```

Missing fields → defaults. Invalid fields → ignored.

---

## 6. Implementation Checklist

### Phase 1: Prep (DONE)
- [x] ServerHandle struct definition
- [x] WIDYA_SERVER_HANDLES global
- [x] Builtin stubs

### Phase 2: Handler Execution (TODO)
- [ ] Fresh Interpreter creation per request
- [ ] Function AST cloning / extraction
- [ ] execute_handler_widya() helper
- [ ] Request map building inside handler

### Phase 3: WebServer Integration (TODO)
- [ ] Update builtin_jalankan to create WebServer
- [ ] Register rute closures
- [ ] Call ws.jalankan() (blocking)

### Phase 4: Response Conversion (TODO)
- [ ] convert_response_map_to_respon() function
- [ ] Handle map/string badan
- [ ] Status code extraction
- [ ] Content-Type negotiation

### Phase 5: Mode Flag (TODO)
- [ ] Parse mode from ServerHttp opts
- [ ] Single-threaded execution (default)
- [ ] Thread-per-request stub (for future "utas")

### Phase 6: Testing (TODO)
- [ ] Unit tests for ServerHandle
- [ ] Integration test: start server + curl
- [ ] Handler execution test
- [ ] Error handling test
- [ ] Regression: cargo test --all

### Phase 7: Documentation (TODO)
- [ ] Update TUTORIAL with ServerHttp examples
- [ ] Example contoh/201_simple_api.wya

---

## 7. Success Criteria (Acceptance)

**T3 adalah DONE ketika:**
1. ✅ ServerHttp(port) builtin works
2. ✅ tambah_rute registers handler functions
3. ✅ jalankan(server) starts server & accepts requests
4. ✅ Request → handler → response flow works end-to-end
5. ✅ Handler receives req map with method, path, badan
6. ✅ Handler returns response map {status, badan, ...}
7. ✅ curl / browser can hit localhost:port and get responses
8. ✅ All TR-3.1 to TR-3.5 tests pass
9. ✅ cargo test --all = 0 new failures (regresi maintained)
10. ✅ Example program contoh/201_simple_api.wya works

---

## 8. Timeline Estimate

| Phase | Tasks | Time |
|-------|-------|------|
| 1 | Prep (DONE) | ✅ Done |
| 2 | Handler exec + AST clone | 45 min |
| 3 | WebServer integration | 30 min |
| 4 | Response conversion | 20 min |
| 5 | Mode flag (defer for now) | - |
| 6 | Testing & verification | 60 min |
| 7 | Documentation | 30 min |
| **Total** | | **~3 hours** |

---

## 9. Next Steps (After T3)

- **T4**: WebSocket RFC 6455 (sha1 + base64 handshake, frame encode/decode)
- **T5**: FFI libloading (C DLL calls)
- **T6-T7**: Desktop (widya app, WebView2 wry)
- **T8-T10**: Examples, docs, commit

