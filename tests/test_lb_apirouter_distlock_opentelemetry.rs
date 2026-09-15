use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave36_load_balancer() {
    let code = r#"
        var lb = PenyeimbangBeban("round_robin")
        lb_tambah_server(lb, "srv1", "http://10.0.0.1", 1)
        lb_tambah_server(lb, "srv2", "http://10.0.0.2", 1)

        var s1 = lb_pilih_server(lb)
        var s2 = lb_pilih_server(lb)
        var s3 = lb_pilih_server(lb)

        var res = s1["id"] == "srv1" dan s2["id"] == "srv2" dan s3["id"] == "srv1"
        kembalikan res
    "#;
    let res = jalankan(code).expect("Execution failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave36_load_balancer_least_conn() {
    let code = r#"
        var lb = PenyeimbangBeban("least_conn")
        lb_tambah_server(lb, "srv1", "http://10.0.0.1", 1)
        lb_tambah_server(lb, "srv2", "http://10.0.0.2", 1)

        lb_laporkan_koneksi(lb, "srv1", 5)
        lb_laporkan_koneksi(lb, "srv2", 1)

        var s = lb_pilih_server(lb)
        kembalikan s["id"] == "srv2"
    "#;
    let res = jalankan(code).expect("Execution failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave36_api_gateway_router() {
    let code = r#"
        var router = RouterAPI()
        router_tambah_rute(router, "GET", "/api/v1/pengguna/:id", "HandlerPengguna")
        router_tambah_rute(router, "POST", "/api/v1/produk", "HandlerProduk")
        router_tambah_rute(router, "*", "/aset/*", "HandlerAset")

        var c1 = router_cocokkan(router, "GET", "/api/v1/pengguna/123")
        var c2 = router_cocokkan(router, "POST", "/api/v1/produk")
        var c3 = router_cocokkan(router, "GET", "/aset/gambar/logo.png")
        var c4 = router_cocokkan(router, "DELETE", "/api/v1/pengguna/123")

        var ok1 = c1["cocok"] == benar dan c1["handler"] == "HandlerPengguna" dan c1["parameter"]["id"] == "123"
        var ok2 = c2["cocok"] == benar dan c2["handler"] == "HandlerProduk"
        var ok3 = c3["cocok"] == benar dan c3["wildcard"] == "gambar/logo.png"
        var ok4 = c4["cocok"] == salah

        kembalikan ok1 dan ok2 dan ok3 dan ok4
    "#;
    let res = jalankan(code).expect("Execution failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave36_distributed_lock() {
    let code = r#"
        var dl = KunciTerdistribusi(1000)
        var k = "kunci_rekening"

        var ak1 = distlock_akuisisi(dl, k, "worker-1", 5000)
        var ak2 = distlock_akuisisi(dl, k, "worker-2", 5000)
        var locked = distlock_apakah_terkunci(dl, k)

        var perp = distlock_perpanjang(dl, k, "worker-1", 2000)
        var lep = distlock_lepas(dl, k, "worker-1")
        var unlocked = distlock_apakah_terkunci(dl, k)

        var ak3 = distlock_akuisisi(dl, k, "worker-2", 5000)

        kembalikan ak1 == benar dan ak2 == salah dan locked == benar dan perp == benar dan lep == benar dan unlocked == salah dan ak3 == benar
    "#;
    let res = jalankan(code).expect("Execution failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave36_opentelemetry_tracing() {
    let code = r#"
        var tr = TracerTelemetry("layanan-auth", "trace-999")
        var s_root = trace_mulai_span(tr, "LoginUser")
        trace_tambah_tag(tr, s_root, "http.method", "POST")

        var s_child = trace_mulai_span(tr, "CheckDB", s_root)
        trace_tambah_tag(tr, s_child, "db.query", "SELECT * FROM users")
        trace_akhiri_span(tr, s_child)

        trace_akhiri_span(tr, s_root)

        var json_str = trace_ekspor_json(tr)
        kembalikan json_str
    "#;
    let res = jalankan(code).expect("Execution failed");
    if let Value::String(s) = res {
        assert!(s.contains("layanan-auth"));
        assert!(s.contains("LoginUser"));
        assert!(s.contains("CheckDB"));
        assert!(s.contains("SELECT * FROM users"));
    } else {
        panic!("Expected string output from trace_ekspor_json");
    }
}
