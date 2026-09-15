use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave48_geojson_and_wkt() {
    let code = r#"
        var gj = ParserGeoJSON("{\"type\": \"Feature\", \"geometry\": {\"type\": \"Point\", \"coordinates\": [106.84, -6.20]}}")
        var cek_gj = gj.tipe_geometri == "Point"

        var wkt = geojson_ke_wkt(gj)
        var cek_wkt = wkt == "POINT(106.8456 -6.2088)"

        var gj2 = wkt_ke_geojson("POLYGON((0 0, 10 0, 10 10, 0 10, 0 0))")
        var cek_gj2 = gj2.tipe_geometri == "Polygon"

        kembalikan cek_gj dan cek_wkt dan cek_gj2
    "#;
    let res = jalankan(code).expect("GeoJSON/WKT test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave48_spatial_topology() {
    let code = r#"
        var poly = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0], [0.0, 0.0]]
        var in_pt = spasial_titik_dalam_poligon([5.0, 5.0], poly)
        var out_pt = spasial_titik_dalam_poligon([15.0, 15.0], poly)

        var luas = spasial_hitung_luas_poligon(poly)
        var cek_luas = luas > 1000000.0 // 100 * 111 * 111 ~ 1.23e6

        var garis = [[0.0, 0.0], [1.0, 0.0]]
        var pnjg = spasial_hitung_panjang_garis(garis)
        var cek_pnjg = pnjg == 111.0

        kembalikan in_pt == benar dan out_pt == salah dan cek_luas dan cek_pnjg
    "#;
    let res = jalankan(code).expect("Topology test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave48_rtree_and_geohash() {
    let code = r#"
        var rtree = IndeksRTree(8)
        rtree_sisip(rtree, "p1", 0.0, 0.0, 2.0, 2.0, {})
        rtree_sisip(rtree, "p2", 10.0, 10.0, 12.0, 12.0, {})

        var res = rtree_kueri_kotak(rtree, 1.0, 1.0, 3.0, 3.0)
        var cek_rtree = panjang(res) == 1 dan res[0].id == "p1"

        var code_geo = geohash_enkode(-6.2088, 106.8456, 6)
        var cek_geo_len = panjang(code_geo) == 6

        var dec = geohash_dekode(code_geo)
        var cek_dec = dec.lintang < 0.0 dan dec.bujur > 100.0

        kembalikan cek_rtree dan cek_geo_len dan cek_dec
    "#;
    let res = jalankan(code).expect("R-Tree and Geohash test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave48_crs_projection() {
    let code = r#"
        var proj = ProyeksiCRS("EPSG:4326", "EPSG:3857")
        var res = crs_transformasi_koordinat(proj, 0.0, 0.0)
        var cek_origin = res.x >= -0.001 dan res.x <= 0.001 dan res.y >= -0.001 dan res.y <= 0.001

        var res_jkt = crs_transformasi_koordinat(proj, 106.8456, -6.2088)
        var cek_jkt = res_jkt.x > 10000000.0 dan res_jkt.y < 0.0

        kembalikan cek_origin dan cek_jkt
    "#;
    let res = jalankan(code).expect("CRS test failed");
    assert_eq!(res, Value::Bool(true));
}
