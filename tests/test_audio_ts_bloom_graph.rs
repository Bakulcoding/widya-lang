use widya::jalankan;
use widya::value::Value;

#[test]
fn test_audio_dan_timeseries() {
    let kode = r#"
        // 1. WidyaAudio test
        misal nada = buat_nada(440.0, "SINUS", 0.1, 0.5);
        misal nada_env = terapkan_adsr(nada, 0.1, 0.1, 0.8, 0.2);
        misal wav_ok = ekspor_wav(nada_env, "test_audio.wav");

        pastikan(panjang(nada) > 1000, "Jumlah sampel nada harus > 1000");
        pastikan(wav_ok == benar, "Ekspor WAV harus sukses");

        // 2. WidyaTimeSeries test
        misal ts = DeretWaktu("metrik_test");
        tambah_titik_waktu(ts, 1.0, 10.0);
        tambah_titik_waktu(ts, 2.0, 20.0);
        tambah_titik_waktu(ts, 3.0, 30.0);

        misal data = [10.0, 20.0, 30.0, 40.0, 50.0];
        misal sma = rata_rata_bergerak(data, 3);
        misal pred = prediksi_eksponensial(data, 0.3, 2);

        pastikan(panjang(sma) == 5, "Panjang hasil SMA harus sama dengan data");
        pastikan(panjang(pred) == 2, "Panjang hasil prediksi harus 2");

        kembalikan [panjang(nada), panjang(sma), panjang(pred)];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[1], Value::Number(5.0));
        assert_eq!(items[2], Value::Number(2.0));
    } else {
        panic!("Hasil harus berupa array");
    }
}

#[test]
fn test_bloom_dan_graph() {
    let kode = r#"
        // 1. WidyaBloom test
        misal bf = FilterBloom(100);
        bloom_tambah(bf, "item1");
        bloom_tambah(bf, "item2");

        misal ada1 = bloom_mungkin_ada(bf, "item1");
        misal tidak_ada = bloom_mungkin_ada(bf, "item_alien");

        pastikan(ada1 == benar, "Item1 harus ada dalam Bloom Filter");
        pastikan(tidak_ada == salah, "Item_alien harus bernilai salah dalam Bloom Filter");

        // 2. HyperLogLog test
        misal hll = HyperLogLog(6);
        hll_tambah(hll, "userA");
        hll_tambah(hll, "userB");
        hll_tambah(hll, "userA");
        misal count_hll = hll_hitung_unik(hll);
        pastikan(count_hll >= 1.0, "Estimasi kardinalitas HLL harus valid");

        // 3. WidyaGraphEngine test
        misal g = GrafJaringan(salah);
        tambah_simpul(g, "1", "Node1");
        tambah_simpul(g, "2", "Node2");
        tambah_simpul(g, "3", "Node3");
        tambah_sisi(g, "1", "2", 5.0);
        tambah_sisi(g, "2", "3", 7.0);
        tambah_sisi(g, "1", "3", 20.0);

        misal rute = jalur_terpendek_dijkstra(g, "1", "3");
        pastikan(rute.jarak_total == 12.0, "Jarak terpendek Dijkstra 1->2->3 harus 12");
        pastikan(panjang(rute.jalur) == 3, "Jalur terpendek harus melalui 3 simpul");

        kembalikan [ada1, tidak_ada, rute.jarak_total];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(false));
        assert_eq!(items[2], Value::Number(12.0));
    } else {
        panic!("Hasil harus berupa array");
    }
}
