// ==============================================================================
// Widya Web Engine - HTTP Client & Server Level Industri
// ==============================================================================
// Arsitektur: lapisan jaringan Widya yang NYATA (HTTP/HTTPS client, HTTP server,
// WebSocket). Modul terpisah dari megafile stdlib agar type-check cepat & bersih.
//
// HTTP Client: ureq 2 (rustls TLS) - waktu tunggu, header, JSON, body string.
// Semua fungsi mengembalikan Value::Map, tidak pernah panic.
// ==============================================================================

use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::{json_to_value, value_to_json};
use crate::value::{BuiltinFn, BuiltinFunction, Value, WidyaFunction};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

pub fn daftarkan_http(env: &mut Environment) {
    daftar(env, "http_get", Some(1), http_get);
    daftar(env, "http_post", Some(2), http_post);
    daftar(env, "http_get_detil", Some(1), http_get_detil);
    daftar(env, "http_post_detil", Some(2), http_post_detil);
    // ServerHttp: nama tetap terdaftar (stub pasif) sampai T3 menggantinya
    // dengan implementasi server nyata di atas engine src/web.
    daftar(env, "ServerHttp", Some(1), server_http_stub);
}

fn server_http_stub(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let port = match &args[0] {
        Value::Number(n) => *n as u16,
        _ => return Err(Galat::runtime("Port ServerHttp harus berupa angka", span)),
    };
    let mut server = HashMap::new();
    server.insert("_tipe".to_string(), Value::String("ServerHttp".to_string()));
    server.insert("port".to_string(), Value::Number(port as f64));
    server.insert("rute_get".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    server.insert("rute_post".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(server))))
}

fn daftar(env: &mut Environment, nama: &str, arity: Option<usize>, f: BuiltinFn) {
    let builtin = BuiltinFunction {
        name: nama.to_string(),
        arity,
        func: f,
    };
    env.define(nama.to_string(), Value::Builtin(Rc::new(builtin)), true);
}

fn http_get(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(Galat::runtime("URL http_get harus berupa string teks", span)),
    };
    let (timeout, headers, tipe_konten) = http_parse_opts(args.get(1));
    let mut req = ureq::get(&url).timeout(Duration::from_secs(timeout));
    if let Some(t) = &tipe_konten {
        req = req.set("Accept", t);
    }
    if let Some(hs) = &headers {
        for (k, v) in hs {
            req = req.set(k, v);
        }
    }
    match req.call() {
        Ok(resp) => {
            let status = resp.status();
            let (text, _json) = http_body_to_value(resp);
            let mut map = HashMap::new();
            map.insert("sukses".to_string(), Value::Bool(true));
            map.insert("status".to_string(), Value::Number(status as f64));
            map.insert("url".to_string(), Value::String(url));
            map.insert("badan".to_string(), Value::String(text));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Ok(http_map_error(e, url)),
    }
}

fn http_post(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(Galat::runtime("URL http_post harus berupa string teks", span)),
    };
    let payload = args[1].clone();
    let (timeout, headers, tipe_konten) = http_parse_opts(args.get(2));
    let mut req = ureq::post(&url).timeout(Duration::from_secs(timeout));
    if let Some(t) = &tipe_konten {
        req = req.set("Content-Type", t);
    }
    if let Some(hs) = &headers {
        for (k, v) in hs {
            req = req.set(k, v);
        }
    }
    let call_result = match &payload {
        Value::String(s) => req.send_string(s),
        _ => req.send_json(value_to_json(&payload)),
    };
    match call_result {
        Ok(resp) => {
            let status = resp.status();
            let (text, _json) = http_body_to_value(resp);
            let mut map = HashMap::new();
            map.insert("sukses".to_string(), Value::Bool(true));
            map.insert("status".to_string(), Value::Number(status as f64));
            map.insert("url".to_string(), Value::String(url));
            map.insert("badan".to_string(), Value::String(text));
            map.insert("data_terkirim".to_string(), payload);
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Ok(http_map_error(e, url)),
    }
}

fn http_get_detil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(Galat::runtime("URL http_get_detil harus berupa string teks", span)),
    };
    let (timeout, headers, tipe_konten) = http_parse_opts(args.get(1));
    let mut req = ureq::get(&url).timeout(Duration::from_secs(timeout));
    if let Some(t) = &tipe_konten {
        req = req.set("Accept", t);
    }
    if let Some(hs) = &headers {
        for (k, v) in hs {
            req = req.set(k, v);
        }
    }
    match req.call() {
        Ok(resp) => {
            let status = resp.status();
            let header_map = http_headers_to_value(&resp);
            let (text, json) = http_body_to_value(resp);
            let mut map = HashMap::new();
            map.insert("sukses".to_string(), Value::Bool(true));
            map.insert("status".to_string(), Value::Number(status as f64));
            map.insert("url".to_string(), Value::String(url));
            map.insert("header".to_string(), header_map);
            map.insert("badan".to_string(), Value::String(text));
            map.insert("badan_json".to_string(), json);
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Ok(http_map_error(e, url)),
    }
}

fn http_post_detil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let url = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(Galat::runtime("URL http_post_detil harus berupa string teks", span)),
    };
    let payload = args[1].clone();
    let (timeout, headers, tipe_konten) = http_parse_opts(args.get(2));
    let mut req = ureq::post(&url).timeout(Duration::from_secs(timeout));
    if let Some(t) = &tipe_konten {
        req = req.set("Content-Type", t);
    }
    if let Some(hs) = &headers {
        for (k, v) in hs {
            req = req.set(k, v);
        }
    }
    let call_result = match &payload {
        Value::String(s) => req.send_string(s),
        _ => req.send_json(value_to_json(&payload)),
    };
    match call_result {
        Ok(resp) => {
            let status = resp.status();
            let header_map = http_headers_to_value(&resp);
            let (text, json) = http_body_to_value(resp);
            let mut map = HashMap::new();
            map.insert("sukses".to_string(), Value::Bool(true));
            map.insert("status".to_string(), Value::Number(status as f64));
            map.insert("url".to_string(), Value::String(url));
            map.insert("header".to_string(), header_map);
            map.insert("badan".to_string(), Value::String(text));
            map.insert("badan_json".to_string(), json);
            map.insert("data_terkirim".to_string(), payload);
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Ok(http_map_error(e, url)),
    }
}

fn http_parse_opts(opts: Option<&Value>) -> (u64, Option<HashMap<String, String>>, Option<String>) {
    let mut timeout = 30u64;
    let mut headers: Option<HashMap<String, String>> = None;
    let mut tipe_konten: Option<String> = None;
    if let Some(Value::Map(m)) = opts {
        let o = m.borrow();
        if let Some(Value::Number(n)) = o.get("timeout_detik") {
            timeout = (*n).max(1.0) as u64;
        }
        if let Some(Value::String(s)) = o.get("tipe_konten") {
            tipe_konten = Some(s.clone());
        }
        if let Some(Value::Map(h)) = o.get("header") {
            let hs = h.borrow();
            let mut hh = HashMap::new();
            for (k, v) in hs.iter() {
                hh.insert(k.clone(), v.to_string_repr());
            }
            headers = Some(hh);
        }
    }
    (timeout, headers, tipe_konten)
}

fn http_headers_to_value(resp: &ureq::Response) -> Value {
    let mut hm = HashMap::new();
    for name in resp.headers_names() {
        if let Some(val) = resp.header(&name) {
            hm.insert(name.to_lowercase(), Value::String(val.to_string()));
        }
    }
    Value::Map(Rc::new(RefCell::new(hm)))
}

fn http_body_to_value(resp: ureq::Response) -> (String, Value) {
    let text = resp.into_string().unwrap_or_default();
    let json = serde_json::from_str::<serde_json::Value>(&text)
        .map(json_to_value)
        .unwrap_or(Value::Nil);
    (text, json)
}

fn http_map_error(e: ureq::Error, url: String) -> Value {
    let mut map = HashMap::new();
    match e {
        ureq::Error::Status(code, resp) => {
            let _ = resp.into_string();
            map.insert("sukses".to_string(), Value::Bool(false));
            map.insert("status".to_string(), Value::Number(code as f64));
            map.insert("url".to_string(), Value::String(url));
            map.insert("badan".to_string(), Value::String(String::new()));
            map.insert("salah".to_string(), Value::String(format!("HTTP {}", code)));
        }
        ureq::Error::Transport(t) => {
            map.insert("sukses".to_string(), Value::Bool(false));
            map.insert("status".to_string(), Value::Nil);
            map.insert("url".to_string(), Value::String(url));
            map.insert("badan".to_string(), Value::String(String::new()));
            map.insert("salah".to_string(), Value::String(t.to_string()));
        }
    }
    Value::Map(Rc::new(RefCell::new(map)))
}

// ==============================================================================
// HTTP Server Engine - satu-satunya router/socket untuk Studio & ServerHttp (T3)
// ==============================================================================
// Di-extract dari src/studio.rs (T2) agar: (1) studio memakai ulang tanpa duplikasi,
// (2) ServerHttp nyata (T3) & web menu berbagi engine yang sama. Perilaku transpor
// BYTE-IDENTIK dengan studio lama (status text, urutan header, CORS, rate limit).
// ==============================================================================

use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Instant;

pub(crate) static RATE_LIMITER: OnceLock<Mutex<RateLimiter>> = OnceLock::new();

pub(crate) struct RateLimiter {
    pub(crate) requests: VecDeque<Instant>,
    pub(crate) limit: usize,
    pub(crate) window: Duration,
}

impl RateLimiter {
    fn new(limit: usize, window_secs: u64) -> Self {
        Self {
            requests: VecDeque::new(),
            limit,
            window: Duration::from_secs(window_secs),
        }
    }

    fn check(&mut self) -> bool {
        let now = Instant::now();
        while let Some(&front) = self.requests.front() {
            if now.duration_since(front) > self.window {
                self.requests.pop_front();
            } else {
                break;
            }
        }
        if self.requests.len() < self.limit {
            self.requests.push_back(now);
            true
        } else {
            false
        }
    }
}

// ============================================================================
// T3: ServerHttp Nyata (TODO - implementasi lanjutan)
// ============================================================================
// TODO: Implementasi ServerHttp dengan:
// - ServerHttp, tambah_rute, jalankan, tutup builtin
// - Handler Widya function execution dengan fresh Interpreter per request
// - Arc<Mutex> atau ValueSerial untuk thread-safe handler
// - Static file serving via WebServer::sajikan_static
//
// See: T3_ServerHttp_Plan.md untuk desain lengkap
// ============================================================================

pub(crate) fn get_rate_limiter() -> &'static Mutex<RateLimiter> {
    RATE_LIMITER.get_or_init(|| Mutex::new(RateLimiter::new(120, 60)))
}

pub(crate) fn check_rate() -> bool {
    if let Ok(mut rl) = get_rate_limiter().lock() {
        rl.check()
    } else {
        true
    }
}

pub(crate) fn rate_status_json() -> String {
    if let Ok(rl) = get_rate_limiter().lock() {
        serde_json::json!({
            "sukses": true,
            "limit": rl.limit,
            "window_detik": rl.window.as_secs(),
            "current_count": rl.requests.len(),
            "sisa": rl.limit.saturating_sub(rl.requests.len())
        })
        .to_string()
    } else {
        serde_json::json!({"sukses": false}).to_string()
    }
}

// ---------------------------------------------------------------------------
// Parsing & penulisan respons HTTP (identik dgn studio lama)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct RequestInfo {
    pub method: String,
    pub path: String,
    pub query: Option<String>,
    pub header: String,
    pub badan: String,
}

pub fn parse_request(raw: &str) -> RequestInfo {
    let mut info = RequestInfo::default();
    let (head, badan) = match raw.find("\r\n\r\n") {
        Some(pos) => (&raw[..pos], raw[(pos + 4)..].to_string()),
        None => (raw, String::new()),
    };
    info.header = head.to_string();
    info.badan = badan;
    let mut parts = head.lines().next().unwrap_or("").split_whitespace();
    info.method = parts.next().unwrap_or("").to_string();
    let path_full = parts.next().unwrap_or("/");
    let mut split = path_full.splitn(2, '?');
    info.path = split.next().unwrap_or("/").to_string();
    info.query = split.next().map(str::to_string);
    info
}

pub fn json_response(status: u16, body: &str) -> String {
    let status_text = match status {
        200 => "200 OK",
        201 => "201 CREATED",
        400 => "400 BAD REQUEST",
        404 => "404 NOT FOUND",
        429 => "429 TOO MANY REQUESTS",
        500 => "500 INTERNAL SERVER ERROR",
        _ => "200 OK",
    };
    format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
        status_text,
        body.len(),
        body
    )
}

pub fn kirim_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    badan: &str,
    kustom_header: Option<(&str, &str)>,
) {
    let status_text = match status {
        200 => "200 OK",
        201 => "201 CREATED",
        400 => "400 BAD REQUEST",
        404 => "404 NOT FOUND",
        429 => "429 TOO MANY REQUESTS",
        500 => "500 INTERNAL SERVER ERROR",
        _ => "200 OK",
    };
    let hdr = match kustom_header {
        Some((k, v)) => format!("\r\n{}: {}", k, v),
        None => String::new(),
    };
    let _ = stream.write_all(format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *{}\r\n\r\n{}",
        status_text, content_type, badan.len(), hdr, badan
    ).as_bytes());
}

pub(crate) fn err_429_string() -> String {
    let body = serde_json::json!({
        "sukses": false,
        "galat": "Rate limit terlampaui. Maksimal 120 permintaan per menit.",
        "rate_limit": 120,
        "window_detik": 60
    }).to_string();
    json_response(429, &body)
}

// ---------------------------------------------------------------------------
// WebServer - accept loop thread-per-koneksi + router (API umum untuk T3)
// ---------------------------------------------------------------------------

/// Hasil dari handler rute. `Alir` untuk respons streaming (SSE dsb.) yang
/// menulis langsung ke socket.
pub enum Respon {
    Tulis(String),
    Json(String),
    Html(String),
    Status(u16, String),
    Alir(Box<dyn FnOnce(&mut TcpStream) + Send>),
}

struct RuteReg {
    method: String,
    path: String,
    handler: Box<dyn Fn(&RequestInfo) -> Respon + Send + Sync>,
}

pub struct WebServer {
    listener: TcpListener,
    rute: Vec<RuteReg>,
    tanpa_rate: Vec<(String, String)>,
}

impl WebServer {
    pub fn new(port: u16) -> std::io::Result<Self> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        Ok(Self {
            listener,
            rute: Vec::new(),
            tanpa_rate: Vec::new(),
        })
    }

    pub fn registrasi_rute<F>(&mut self, method: &str, path: &str, handler: F)
    where
        F: Fn(&RequestInfo) -> Respon + Send + Sync + 'static,
    {
        self.rute.push(RuteReg {
            method: method.to_string(),
            path: path.to_string(),
            handler: Box::new(handler),
        });
    }

    pub fn tanpa_rate_limit(&mut self, method: &str, path: &str) {
        self.tanpa_rate.push((method.to_string(), path.to_string()));
    }

    pub fn jalankan(self) {
        let rute = Arc::new(self.rute);
        let tanpa_rate = Arc::new(self.tanpa_rate);
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                let rute = Arc::clone(&rute);
                let tanpa_rate = Arc::clone(&tanpa_rate);
                std::thread::spawn(move || layani_koneksi(stream, rute, tanpa_rate));
            }
        }
    }
}

fn layani_koneksi(
    mut stream: TcpStream,
    rute: Arc<Vec<RuteReg>>,
    tanpa_rate: Arc<Vec<(String, String)>>,
) {
    let mut buffer = [0; 262144];
    if let Ok(bytes_read) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let info = parse_request(&request);

        let exempt = tanpa_rate.iter().any(|(m, p)| *m == info.method && *p == info.path);
        if !exempt && !check_rate() {
            let _ = stream.write_all(err_429_string().as_bytes());
            return;
        }

        for reg in rute.iter() {
            if reg.method == info.method && reg.path == info.path {
                match (reg.handler)(&info) {
                    Respon::Tulis(raw) => {
                        let _ = stream.write_all(raw.as_bytes());
                    }
                    Respon::Json(body) => {
                        let _ = stream.write_all(json_response(200, &body).as_bytes());
                    }
                    Respon::Html(body) => {
                        kirim_response(&mut stream, 200, "text/html; charset=utf-8", &body, None);
                    }
                    Respon::Status(st, body) => {
                        let _ = stream.write_all(json_response(st, &body).as_bytes());
                    }
                    Respon::Alir(f) => {
                        f(&mut stream);
                    }
                }
                return;
            }
        }

        if info.method == "OPTIONS" {
            let cors = "HTTP/1.1 204 NO CONTENT\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 0\r\n\r\n";
            let _ = stream.write_all(cors.as_bytes());
            return;
        }

        let not_found = serde_json::json!({
            "sukses": false,
            "galat": format!("Endpoint tidak ditemukan: {} {}", info.method, info.path)
        }).to_string();
        let _ = stream.write_all(json_response(404, &not_found).as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;
    use std::collections::HashMap;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    fn buat_server(badan: &str, header: &[(&str, &str)]) -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let body = badan.to_string();
        let hdrs: Vec<(String, String)> = header.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        thread::spawn(move || {
            if let Some(mut stream) = listener.incoming().flatten().next() {
                // Baca seluruh header sampai "\r\n\r\n" lalu body sesuai Content-Length
                // agar ureq tidak kena EOF/RST saat parallelism tinggi.
                let mut buf = Vec::new();
                let mut chunk = [0u8; 1024];
                let content_length = loop {
                    let n = match stream.read(&mut chunk) {
                        Ok(0) => break 0,
                        Ok(n) => n,
                        Err(_) => break 0,
                    };
                    buf.extend_from_slice(&chunk[..n]);
                    if let Some(pos) = find_headers_end(&buf) {
                        let heads = String::from_utf8_lossy(&buf[..pos]).to_string();
                        let cl = heads
                            .lines()
                            .find(|l| l.to_ascii_lowercase().starts_with("content-length:"))
                            .and_then(|l| l.split(':').nth(1))
                            .and_then(|v| v.trim().parse::<usize>().ok())
                            .unwrap_or(0);
                        break cl;
                    }
                };
                while buf.len() - find_headers_end(&buf).unwrap_or(0) < content_length {
                    let n = match stream.read(&mut chunk) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    };
                    buf.extend_from_slice(&chunk[..n]);
                }
                let mut resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close",
                    body.len()
                );
                for (k, v) in &hdrs {
                    resp.push_str(&format!("\r\n{}: {}", k, v));
                }
                resp.push_str("\r\n\r\n");
                resp.push_str(&body);
                let _ = stream.write_all(resp.as_bytes());
            }
        });
        port
    }

    fn find_headers_end(buf: &[u8]) -> Option<usize> {
        buf.windows(4).position(|w| w == b"\r\n\r\n").map(|i| i + 4)
    }

    fn val_field(m: &HashMap<String, Value>, key: &str) -> String {
        m.get(key).map(|v| v.to_string_repr()).unwrap_or_default()
    }

    fn map_from_result(val: &Value) -> HashMap<String, Value> {
        match val {
            Value::Map(m) => m.borrow().clone(),
            other => panic!("expected Value::Map, got {:?}", other),
        }
    }

    fn assert_salah(val: &Value) {
        let m = map_from_result(val);
        assert_eq!(val_field(&m, "sukses"), "salah", "expected sukses=false");
        assert!(m.get("salah").is_some(), "expected error message in 'salah'");
    }

    #[test]
    fn tr_1_1_http_get_json() {
        let port = buat_server(r#"{"hello":"widya"}"#, &[("Content-Type", "application/json")]);
        let result = http_get(
            &[Value::String(format!("http://127.0.0.1:{}", port))],
            &crate::error::Span::new(0, 0),
        )
        .unwrap();
        let m = map_from_result(&result);
        assert_eq!(val_field(&m, "sukses"), "benar");
        assert_eq!(val_field(&m, "status"), "200");
        assert_eq!(val_field(&m, "badan"), r#"{"hello":"widya"}"#);
        assert!(val_field(&m, "url").contains("127.0.0.1"));
    }

    #[test]
    fn tr_1_2_http_post() {
        let port = buat_server(r#"{"received":true}"#, &[("Content-Type", "application/json")]);
        let payload = Value::String("test_payload".to_string());
        let result = http_post(
            &[
                Value::String(format!("http://127.0.0.1:{}", port)),
                payload,
            ],
            &crate::error::Span::new(0, 0),
        )
        .unwrap();
        let m = map_from_result(&result);
        assert_eq!(val_field(&m, "sukses"), "benar");
        assert_eq!(val_field(&m, "badan"), r#"{"received":true}"#);
        assert_eq!(val_field(&m, "data_terkirim"), "test_payload");
    }

    #[test]
    fn tr_1_3_tidak_panik_salah_port() {
        // Port 1 harus ditolak (tidak panic)
        let result = http_get(
            &[Value::String("http://127.0.0.1:1".to_string())],
            &crate::error::Span::new(0, 0),
        );
        assert!(result.is_ok(), "http_get tidak boleh panic/return Err");
        assert_salah(&result.unwrap());
    }

    #[test]
    fn tr_1_4_detil_header() {
        let port = buat_server("ok", &[("X-Test-Hdr", "magic")]);
        let result = http_get_detil(
            &[Value::String(format!("http://127.0.0.1:{}", port))],
            &crate::error::Span::new(0, 0),
        )
        .unwrap();
        let m = map_from_result(&result);
        assert_eq!(val_field(&m, "sukses"), "benar");
        assert_eq!(val_field(&m, "badan"), "ok");
        let headers = match m.get("header") {
            Some(Value::Map(h)) => h.borrow().clone(),
            other => panic!("expected header map, got {:?}", other),
        };
        assert_eq!(val_field(&headers, "x-test-hdr"), "magic");
    }

    #[test]
    fn tr_1_4_detil_badan_json() {
        let port = buat_server(r#"{"key":"val"}"#, &[("Content-Type", "application/json")]);
        let result = http_get_detil(
            &[Value::String(format!("http://127.0.0.1:{}", port))],
            &crate::error::Span::new(0, 0),
        )
        .unwrap();
        let m = map_from_result(&result);
        assert_eq!(val_field(&m, "sukses"), "benar");
        let json_map = match m.get("badan_json") {
            Some(Value::Map(h)) => h.borrow().clone(),
            other => panic!("expected badan_json map, got {:?}", other),
        };
        assert_eq!(val_field(&json_map, "key"), "val");
    }

    #[test]
    fn tr_1_5_post_detil_payload() {
        let port = buat_server("ok", &[]);
        let payload = Value::Number(42.0);
        let result = http_post_detil(
            &[
                Value::String(format!("http://127.0.0.1:{}", port)),
                payload,
            ],
            &crate::error::Span::new(0, 0),
        )
        .unwrap();
        let m = map_from_result(&result);
        assert_eq!(val_field(&m, "sukses"), "benar");
        assert_eq!(val_field(&m, "badan"), "ok");
    }
}