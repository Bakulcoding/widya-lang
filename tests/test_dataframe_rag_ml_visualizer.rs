use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave37_dataframe_operations() {
    let code = r#"
        var cols = ["id", "nama", "nilai"]
        var rows = [
            ["u1", "Andi", 85],
            ["u2", "Budi", 92],
            ["u3", "Citra", 78],
            ["u4", "Dewi", 95]
        ]
        var df = DataFrame(cols, rows)
        var count_total = df_jumlah_baris(df)

        // Saring nilai > 80
        var df_lulus = df_saring(df, "nilai", ">", 80)
        var count_lulus = df_jumlah_baris(df_lulus)

        // Pilih kolom nama & nilai
        var df_sub = df_pilih_kolom(df_lulus, ["nama", "nilai"])
        var arr = df_ke_array(df_sub)

        kembalikan count_total == 4 dan count_lulus == 3 dan arr[0]["nama"] == "Andi"
    "#;
    let res = jalankan(code).expect("DataFrame test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave37_dataframe_join_and_groupby() {
    let code = r#"
        var df_a = DataFrame(["dept", "gaji"], [
            ["IT", 1000],
            ["HR", 800],
            ["IT", 1200]
        ])
        var df_agg = df_kelompokkan_dan_agregasi(df_a, "dept", "gaji", "sum")
        var arr_agg = df_ke_array(df_agg)

        var df_b = DataFrame(["dept", "lokasi"], [
            ["IT", "Lantai 3"],
            ["HR", "Lantai 1"]
        ])
        var df_merged = df_gabung(df_a, df_b, "dept")
        var count_merged = df_jumlah_baris(df_merged)

        kembalikan arr_agg[0]["sum_gaji"] == 2200 dan count_merged == 3
    "#;
    let res = jalankan(code).expect("DataFrame join test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave37_rag_pipeline() {
    let code = r#"
        var rag = RAGPipeline("kb_test", 16)
        rag_tambah_dokumen(rag, "doc1", "Widya adalah bahasa pemrograman cepat dan aman.")
        rag_tambah_dokumen(rag, "doc2", "Kompilasi LLVM dan WebAssembly didukung penuh.")

        var konteks = rag_kueri_konteks(rag, "Bagaimana kecepatan bahasa Widya?", 1)
        var prompt = rag_sintesis_prompt(rag, "Apa fitur utama Widya?", 1)

        kembalikan panjang(konteks) == 1 dan konteks[0]["doc_id"] == "doc1"
    "#;
    let res = jalankan(code).expect("RAG Pipeline test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave37_ml_decision_tree_and_kmeans() {
    let code = r#"
        var x = [
            [1.0, 2.0],
            [1.5, 1.8],
            [8.0, 9.0],
            [8.5, 9.5]
        ]
        var y = ["rendah", "rendah", "tinggi", "tinggi"]

        var tree = PohonKeputusan(3)
        ml_latih_pohon(tree, x, y)

        var pred1 = ml_prediksi_pohon(tree, [1.2, 2.1])
        var pred2 = ml_prediksi_pohon(tree, [8.2, 9.1])

        var km = ml_kmeans_klaster(x, 2, 20)

        kembalikan pred1 == "rendah" dan pred2 == "tinggi" dan panjang(km["centroid"]) == 2
    "#;
    let res = jalankan(code).expect("ML Classifier test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave37_data_visualizer_svg() {
    let code = r#"
        var vis = VisualisasiData("Test Chart")
        var svg_bar = chart_batang_svg(vis, ["A", "B"], [10, 20], 400, 200)
        var svg_line = chart_garis_svg(vis, ["A", "B"], [10, 20], 400, 200)
        var svg_scatter = chart_sebar_svg(vis, [[1, 2], [3, 4]], 400, 200)

        var is_svg = svg_bar != nil dan svg_line != nil dan svg_scatter != nil
        kembalikan is_svg
    "#;
    let res = jalankan(code).expect("Data visualizer test failed");
    assert_eq!(res, Value::Bool(true));
}
