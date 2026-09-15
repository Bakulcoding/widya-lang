use widya::jalankan;
use widya::value::Value;

#[test]
fn test_physics_dan_ocr() {
    let code = r#"
        tetap dunia = DuniaFisika(0.0, 9.8);
        fisika_tambah_lingkaran(dunia, "b1", 0.0, 0.0, 2.0, 1.0, 0.5);
        fisika_langkah(dunia, 1.0);
        tetap pos = fisika_posisi_benda(dunia, "b1");
        
        tetap model = buat_model_ocr();
        tetap piksel_x = [
            1.0, 0.0, 1.0,
            0.0, 1.0, 0.0,
            1.0, 0.0, 1.0
        ];
        tetap hasil_ocr = ocr_kenali_karakter(model, piksel_x);
        
        kembalikan [pos["y"] > 0.0, hasil_ocr["karakter"], hasil_ocr["keyakinan"] >= 1.0];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::String("X".to_string()));
        assert_eq!(items[2], Value::Bool(true));
    } else {
        panic!("Expected array return value");
    }
}

#[test]
fn test_regex_dan_markdown() {
    let code = r###"
        tetap dfa = kompilasi_regex_dfa("^[0-9]+$");
        tetap cocok_1 = regex_dfa_cocok(dfa, "12345");
        tetap cocok_2 = regex_dfa_cocok(dfa, "123a5");
        
        tetap md = "# Judul Utama
Paragraf teks
- Item 1
```widya
cetak(1);
```";
        tetap ast = parse_markdown_ast(md);
        tetap html = render_markdown_html(ast);
        tetap ansi = render_markdown_ansi(ast);
        
        kembalikan [cocok_1, cocok_2, panjang(ast["nodes"]) == 4, panjang(html) > 10, panjang(ansi) > 10];
    "###;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(false));
        assert_eq!(items[2], Value::Bool(true));
        assert_eq!(items[3], Value::Bool(true));
        assert_eq!(items[4], Value::Bool(true));
    } else {
        panic!("Expected array return value");
    }
}
