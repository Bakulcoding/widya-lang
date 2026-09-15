use widya::jalankan;
use widya::value::Value;

#[test]
fn test_dns_dan_circuitbreaker() {
    let code = r#"
        tetap dns = ServerDNS();
        dns_tambah_rekod(dns, "contoh.com", "A", "10.0.0.1", 300.0);
        tetap q = dns_kueri(dns, "contoh.com", "A");
        
        tetap cb = PemutusSirkuit(1, 1000.0);
        fungsi fail_fn() { lempar("err"); }
        fungsi fallback_fn() { kembalikan "SAFE"; }
        
        tetap r1 = eksekusi_dengan_pemutus(cb, fail_fn, fallback_fn);
        tetap st = status_pemutus(cb);
        
        kembalikan [q[0]["nilai"], r1, st["state"]];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::String("10.0.0.1".to_string()));
        assert_eq!(items[1], Value::String("SAFE".to_string()));
        assert_eq!(items[2], Value::String("TERBUKA".to_string()));
    } else {
        panic!("Expected array return value");
    }
}

#[test]
fn test_trie_dan_tar() {
    let code = r#"
        tetap t = PohonTrie();
        trie_sisip(t, "widya", 10.0);
        trie_sisip(t, "web", 5.0);
        tetap ada = trie_apakah_ada(t, "widya");
        tetap saran = trie_saran_awalan(t, "w", 2);
        
        tetap tar = buat_arsip_tar();
        tar_tambah_berkas(tar, "test.txt", "isi teks berkas");
        tetap biner = tar_ekspor_biner(tar);
        tetap entri = tar_baca_entri(biner);
        
        kembalikan [ada, panjang(saran) == 2, panjang(biner) > 500, entri[0]["jalur"], entri[0]["konten"]];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::Bool(true)); // Tar block is at least 512 bytes header
        assert_eq!(items[3], Value::String("test.txt".to_string()));
        assert_eq!(items[4], Value::String("isi teks berkas".to_string()));
    } else {
        panic!("Expected array return value");
    }
}
