use widya::jalankan;
use widya::value::Value;

#[test]
fn test_geo2d_and_murmur() {
    let code = r#"
        var p1 = Titik2D(0, 0)
        var p2 = Titik2D(3, 4)
        var jarak = geo2d_jarak_titik(p1, p2)

        var g1 = Garis2D(0, 0, 2, 2)
        var g2 = Garis2D(0, 2, 2, 0)
        var intersect = geo2d_apakah_berpotongan(g1, g2)
        var pt = geo2d_titik_potong(g1, g2)

        var poly = [Titik2D(0, 0), Titik2D(4, 0), Titik2D(0, 3)]
        var luas = geo2d_luas_poligon(poly)

        var h32 = murmur3_32("widya", 0)
        var h128 = murmur3_128("widya", 0)
        kembalikan luas;
    "#;

    let res = jalankan(code).unwrap();
    assert_eq!(res, Value::Number(6.0));
}

#[test]
fn test_bigrat_and_iniconfig() {
    let code = r#"
        var r1 = Pecahan(1, 2)
        var r2 = Pecahan(1, 3)
        var rSum = pecahan_tambah(r1, r2)
        var rStr = pecahan_ke_string(rSum)

        var iniStr = "[database]\nhost = localhost\nport = 5432\n"
        var parsed = ini_parse(iniStr)
        var host = ini_ambil_nilai(parsed, "database", "host")
        kembalikan host;
    "#;

    let res = jalankan(code).unwrap();
    assert_eq!(res, Value::String("localhost".to_string()));
}
