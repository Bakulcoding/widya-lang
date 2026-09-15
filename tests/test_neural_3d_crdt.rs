use widya::jalankan;
use widya::value::Value;

#[test]
fn test_neural_dan_3d() {
    let kode = r#"
        // 1. WidyaNeural test
        misal lin = LapisanLinier("fc1", [[1.0, 2.0], [3.0, 4.0]], [0.5, 0.5]);
        misal out_lin = forward_linier(lin, [1.0, 1.0]);

        misal attn = LapisanAttention("core_attn", [
            [0.5, 0.2, 0.1, 0.0],
            [0.1, 0.6, 0.3, 0.2],
            [0.0, 0.1, 0.8, 0.4],
            [0.2, 0.3, 0.1, 0.7]
        ]);
        misal out_attn = forward_attention(attn, [[1.0, 0.5, 0.2, 0.1], [0.2, 0.9, 0.4, 0.6]]);
        misal resp_ai = inferensi_llm("Halo Widya", 0.7, 40);

        pastikan(out_lin[0] == 3.5, "Output linear layer [0] harus 3.5");
        pastikan(panjang(out_attn) == 2, "Output attention harus memiliki 2 token output");
        pastikan(panjang(resp_ai) > 10, "Respons LLM harus berisi teks valid");

        // 2. Widya3D test
        misal cam = Kamera3D([0.0, 5.0, 15.0], [0.0, 0.0, 0.0], 60.0);
        misal scene = Adegan3D("Adegan_Tes", "warna_gelap", cam);
        misal obj = ObjekMesh3D("Kubus_Tes", "KUBUS", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], "warna_merah");
        tambah_objek_3d(scene, obj);

        misal html_3d = render_adegan_html(scene, "test_3d.html");
        pastikan(scene.nama == "Adegan_Tes", "Nama adegan 3D harus sesuai");
        pastikan(panjang(html_3d) > 200, "HTML 3D WebGL harus berisi kode Three.js");

        kembalikan [out_lin[0], panjang(out_attn), panjang(scene.daftar_objek)];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(3.5));
        assert_eq!(items[1], Value::Number(2.0));
        assert_eq!(items[2], Value::Number(1.0));
    } else {
        panic!("Hasil harus berupa array");
    }
}

#[test]
fn test_crdt_dan_debugger() {
    let kode = r#"
        misal doc1 = DokumenCRDT("peer_a", "dokumen_1");
        misal doc2 = DokumenCRDT("peer_b", "dokumen_1");

        sisip_karakter_crdt(doc1, 0, "Widya ");
        sisip_karakter_crdt(doc2, 0, "Hebat");

        // Sinkronisasi gabung perubahan
        misal snaps_1 = doc1.daftar_karakter;
        misal snaps_2 = doc2.daftar_karakter;

        gabung_perubahan_crdt(doc1, snaps_2);
        gabung_perubahan_crdt(doc2, snaps_1);

        misal txt1 = baca_teks_crdt(doc1);
        misal txt2 = baca_teks_crdt(doc2);

        pastikan(txt1 == txt2, "CRDT harus mencapai konsistensi data yang sama");

        kembalikan [panjang(txt1), doc1.peer_id, doc2.peer_id];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(11.0));
        assert_eq!(items[1], Value::String("peer_a".to_string()));
        assert_eq!(items[2], Value::String("peer_b".to_string()));
    } else {
        panic!("Hasil harus berupa array");
    }

    // Verify Debugger Struct
    let dbg = widya::tools::debugger::TimeTravelDebugger::new("misal a = 10;\nmisal b = 20;");
    assert_eq!(dbg.source_lines.len(), 2);
}
