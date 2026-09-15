use widya::jalankan;
use widya::value::Value;

#[test]
fn test_aritmatika_dasar() {
    let hasil = jalankan("misal a = 10 + 5 * 2; kembalikan a;").unwrap();
    assert_eq!(hasil, Value::Number(20.0));

    let hasil_pangkat = jalankan("kembalikan 2 ^ 3;").unwrap();
    assert_eq!(hasil_pangkat, Value::Number(8.0));

    let hasil_modulo = jalankan("kembalikan 10 % 3;").unwrap();
    assert_eq!(hasil_modulo, Value::Number(1.0));
}

#[test]
fn test_string_dan_konkat() {
    let hasil = jalankan(r#"
        misal nama = "Widya";
        misal salam = "Halo, " + nama + "!";
        kembalikan salam;
    "#).unwrap();
    assert_eq!(hasil, Value::String("Halo, Widya!".to_string()));
}

#[test]
fn test_percabangan_jika_kalau_lainnya() {
    let kode = r#"
        fungsi cek_nilai(skor) {
            jika skor >= 85 {
                kembalikan "A";
            } kalau skor >= 70 {
                kembalikan "B";
            } lainnya {
                kembalikan "C";
            }
        }
        kembalikan [cek_nilai(90), cek_nilai(75), cek_nilai(50)];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::String("A".to_string()));
        assert_eq!(items[1], Value::String("B".to_string()));
        assert_eq!(items[2], Value::String("C".to_string()));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_perulangan_selama_dan_untuk() {
    let kode = r#"
        misal total = 0;
        untuk i dalam 1..5 {
            total += i;
        }
        kembalikan total;
    "#;
    let hasil = jalankan(kode).unwrap();
    assert_eq!(hasil, Value::Number(15.0));

    let kode_while = r#"
        misal hitung = 0;
        misal x = 0;
        selama x < 10 {
            x += 1;
            jika x == 5 {
                lanjut;
            }
            jika x > 7 {
                berhenti;
            }
            hitung += 1;
        }
        kembalikan hitung;
    "#;
    let hasil_while = jalankan(kode_while).unwrap();
    assert_eq!(hasil_while, Value::Number(6.0));
}

#[test]
fn test_fungsi_rekursif_fibonacci() {
    let kode = r#"
        fungsi fib(n) {
            jika n <= 1 {
                kembalikan n;
            }
            kembalikan fib(n - 1) + fib(n - 2);
        }
        kembalikan fib(10);
    "#;
    let hasil = jalankan(kode).unwrap();
    assert_eq!(hasil, Value::Number(55.0));
}

#[test]
fn test_closure_dan_lingkup_leksikal() {
    let kode = r#"
        fungsi buat_penambah(x) {
            kembalikan fungsi(y) {
                kembalikan x + y;
            };
        }
        misal tambah5 = buat_penambah(5);
        kembalikan tambah5(10);
    "#;
    let hasil = jalankan(kode).unwrap();
    assert_eq!(hasil, Value::Number(15.0));
}

#[test]
fn test_daftar_dan_kamus() {
    let kode = r#"
        misal angka = [1, 2, 3];
        tambah(angka, 4);
        angka[0] = 99;

        misal data = {
            "nama": "Widya",
            "poin": 100
        };
        data["poin"] += 50;

        kembalikan [angka[0], angka[-1], data["nama"], data["poin"]];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(99.0));
        assert_eq!(items[1], Value::Number(4.0));
        assert_eq!(items[2], Value::String("Widya".to_string()));
        assert_eq!(items[3], Value::Number(150.0));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_stdlib_matematika_dan_string() {
    let kode = r#"
        misal p = akar(16);
        misal t = huruf_besar("widya");
        misal gab = gabung(["A", "B", "C"], "-");
        misal pis = pisah("halo dunia", " ");
        kembalikan [p, t, gab, panjang(pis)];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(4.0));
        assert_eq!(items[1], Value::String("WIDYA".to_string()));
        assert_eq!(items[2], Value::String("A-B-C".to_string()));
        assert_eq!(items[3], Value::Number(2.0));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_konstanta_tetap_menolak_mutasi() {
    let kode = r#"
        tetap KODE = 123;
        KODE = 456;
    "#;
    let hasil = jalankan(kode);
    assert!(hasil.is_err(), "Konstanta tidak boleh dapat diubah");
}

#[test]
fn test_struktur_dan_oop() {
    let kode = r#"
        struktur Persegi {
            panjang,
            lebar,

            fungsi luas() {
                kembalikan ini.panjang * ini.lebar;
            }

            fungsi keliling() {
                kembalikan 2 * (ini.panjang + ini.lebar);
            }
        }

        misal p = Persegi(10, 5);
        kembalikan [p.panjang, p.lebar, p.luas(), p.keliling()];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(10.0));
        assert_eq!(items[1], Value::Number(5.0));
        assert_eq!(items[2], Value::Number(50.0));
        assert_eq!(items[3], Value::Number(30.0));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_coba_tangkap_dan_lempar() {
    let kode = r#"
        fungsi bagi_aman(a, b) {
            coba {
                jika b == 0 {
                    lempar "Tidak bisa membagi nol";
                }
                kembalikan a / b;
            } tangkap pesan_galat {
                kembalikan "Tertangkap: " + pesan_galat;
            }
        }

        kembalikan [bagi_aman(10, 2), bagi_aman(10, 0)];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(5.0));
        assert_eq!(items[1], Value::String("Tertangkap: Tidak bisa membagi nol".to_string()));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_json_dan_sorting() {
    let kode = r#"
        misal data_teks = "{\"nama\": \"Widya\", \"skor\": 95}";
        misal objek = dari_json(data_teks);
        misal json_kembali = ke_json(objek);

        misal angka = [5, 1, 9, 3];
        misal terurut = urutkan(angka);
        misal terbalik = balik(terurut);

        kembalikan [objek["nama"], terurut[0], terbalik[0]];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::String("Widya".to_string()));
        assert_eq!(items[1], Value::Number(1.0));
        assert_eq!(items[2], Value::Number(9.0));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_fitur_mirip_rust() {
    let kode = r#"
        // 1. Result Pattern (Ok & Err)
        fungsi bagi_aman(a, b) {
            jika b == 0 {
                kembalikan Err("Pembagian dengan nol dilarang!");
            }
            kembalikan Ok(a / b);
        }

        misal res_sukses = bagi_aman(10, 2);
        misal nilai_sukses = buka(res_sukses);

        misal res_gagal = bagi_aman(10, 0);
        misal nilai_fallback = buka_atau(res_gagal, 999);

        // 2. Option Pattern (Ada & Kosong)
        fungsi cari_data(ada_kah) {
            jika ada_kah {
                kembalikan Ada("Ditemukan");
            }
            kembalikan Kosong();
        }

        misal opt = cari_data(benar);
        misal teks_opt = buka(opt);

        // 3. Native Enum & Pattern Matching (cocokkan)
        enum StatusKoneksi {
            Terhubung,
            Memuat(persen),
            Terputus(alasan)
        }

        fungsi periksa_status(st) {
            kembalikan cocokkan st {
                StatusKoneksi::Terhubung => "Koneksi Aktif",
                StatusKoneksi::Memuat(p) => "Sedang Memuat: " + ke_teks(p) + "%",
                StatusKoneksi::Terputus(alasan) => "Galat: " + alasan,
                _ => "Status Tidak Diketahui"
            };
        }

        misal s1 = StatusKoneksi::Terhubung;
        misal s2 = StatusKoneksi::Memuat(75);
        misal s3 = StatusKoneksi::Terputus("Timeout Server");

        misal teks_s1 = periksa_status(s1);
        misal teks_s2 = periksa_status(s2);
        misal teks_s3 = periksa_status(s3);

        // 4. If Let Pattern (jika misal)
        misal info_paket = "Belum Dikirim";
        jika misal StatusKoneksi::Memuat(persen) = s2 {
            info_paket = "Proses: " + ke_teks(persen) + "%";
        }

        // 5. Rust-like Try Operator (?)
        fungsi operasi_berantai(a, b) {
            misal r1 = bagi_aman(a, b)?;
            kembalikan Ok(r1 * 10);
        }

        misal hasil_coba_ok = operasi_berantai(20, 4);
        misal hasil_coba_err = operasi_berantai(20, 0);

        // 6. Traits / Sifat & Impl / Terapkan
        sifat Dihitung {
            fungsi kali_dua();
        }

        struktur TitikData {
            nilai,
            fungsi inisialisasi(n) {
                ini.nilai = n;
            }
        }

        terapkan Dihitung untuk TitikData {
            fungsi kali_dua() {
                kembalikan ini.nilai * 2;
            }
        }

        misal titik = TitikData(25);
        misal hasil_trait = titik.kali_dua();

        // 7. Functional Collection Iterators (petakan, saring, lipat)
        misal deret = [1, 2, 3, 4, 5];
        misal genap = saring(deret, fungsi(x) { kembalikan x % 2 == 0; });
        misal kuadrat = petakan(genap, fungsi(x) { kembalikan x * x; });
        misal total = lipat(kuadrat, 0, fungsi(acc, x) { kembalikan acc + x; }); // (2*2) + (4*4) = 4 + 16 = 20

        // 8. Assertion (Pastikan)
        pastikan(nilai_sukses == 5, "Harus bernilai 5");
        pastikan(nilai_fallback == 999, "Harus bernilai fallback 999");
        pastikan(teks_s1 == "Koneksi Aktif", "Status 1 harus aktif");
        pastikan(buka(hasil_coba_ok) == 50, "Hasil try operator harus 50");
        pastikan(apakah_err(hasil_coba_err), "Hasil try operator harus err");
        pastikan(hasil_trait == 50, "Hasil trait method harus 50");
        pastikan(total == 20, "Hasil lipat harus 20");

        kembalikan [nilai_sukses, info_paket, buka(hasil_coba_ok), hasil_trait, total];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(5.0));
        assert_eq!(items[1], Value::String("Proses: 75%".to_string()));
        assert_eq!(items[2], Value::Number(50.0));
        assert_eq!(items[3], Value::Number(50.0));
        assert_eq!(items[4], Value::Number(20.0));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_blockchain_dan_kriptografi() {
    let kode = r#"
        // 1. SHA-256 Hash
        misal h = sha256("WidyaChain");

        // 2. Pembuatan Kunci & Tanda Tangan
        misal dompet = buat_kunci();
        misal pesan = "Kirim 10 WIDYA ke Alice";
        misal ttd = tanda_tangani(pesan, dompet.kunci_privat);
        misal valid = verifikasi_ttd(pesan, ttd, dompet.kunci_publik);

        // 3. Merkle Tree
        misal merkle_root = hitung_merkle(["tx1", "tx2", "tx3", "tx4"]);

        // 4. Proof of Work Mining
        misal hasil_pow = tambang_pow("BlokHeader", 2);

        kembalikan [panjang(h), valid, panjang(merkle_root), hasil_pow.kesulitan];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(64.0)); // 64 hex chars for SHA-256
        assert_eq!(items[1], Value::Bool(true));
        assert_eq!(items[2], Value::Number(64.0));
        assert_eq!(items[3], Value::Number(2.0));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_widya_ui_widgets() {
    let kode = r#"
        misal app = Aplikasi({
            "judul": "Aplikasi Toko Widya",
            "badan": Halaman({
                "badan": Kolom([
                    TeksWidget("Selamat Datang!"),
                    Tombol("Beli Sekarang", fungsi() { cetak("Dibeli!"); })
                ])
            })
        });

        misal html = render_html(app);
        kembalikan panjang(html) > 100;
    "#;
    let hasil = jalankan(kode).unwrap();
    assert_eq!(hasil, Value::Bool(true));
}

#[test]
fn test_rust_concurrency_destructuring_loop_dan_iterators() {
    let kode = r#"
        // 1. Destructuring
        misal [x, y, z] = [10, 20, 30];
        pastikan(x == 10 dan y == 20 dan z == 30, "Dekonstruksi daftar harus sesuai");

        // 2. Infinite Loop ('ulang') dengan 'berhenti'
        misal hitung = 0;
        ulang {
            hitung += 1;
            jika hitung == 5 {
                berhenti;
            }
        }
        pastikan(hitung == 5, "Loop ulang harus berhenti di 5");

        // 3. Advanced Iterators (gabungkan, ambil, lewati, apakah_ada, semua)
        misal d1 = [1, 2, 3];
        misal d2 = ["a", "b", "c"];
        misal dipasangkan = gabungkan(d1, d2);
        misal diambil = ambil([10, 20, 30, 40], 2);
        misal dilewati = lewati([10, 20, 30, 40], 2);
        misal ada_genap = apakah_ada([1, 3, 4, 7], fungsi(n) { kembalikan n % 2 == 0; });
        misal semua_positif = semua([1, 2, 3], fungsi(n) { kembalikan n > 0; });

        pastikan(panjang(dipasangkan) == 3, "Panjang pasangan zip harus 3");
        pastikan(diambil[0] == 10 dan diambil[1] == 20, "Ambil harus mengambil 2 elemen pertama");
        pastikan(dilewati[0] == 30 dan dilewati[1] == 40, "Lewati harus melompati 2 elemen pertama");
        pastikan(ada_genap == benar, "Harus ada angka genap");
        pastikan(semua_positif == benar, "Semua angka harus positif");

        // 4. Concurrency & Channels (Threads & MPSC channels)
        misal sal = saluran();
        saluran_kirim(sal, "Pesan Dari Utas");
        misal diterima = saluran_terima(sal);
        pastikan(diterima == "Pesan Dari Utas", "Pesan saluran harus diterima utuh");

        kembalikan [x + y + z, hitung, diambil[1], dilewati[0], diterima];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(60.0));
        assert_eq!(items[1], Value::Number(5.0));
        assert_eq!(items[2], Value::Number(20.0));
        assert_eq!(items[3], Value::Number(30.0));
        assert_eq!(items[4], Value::String("Pesan Dari Utas".to_string()));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_asinkron_dan_tunggu_hasil() {
    let kode = r#"
        asinkron fungsi ambil_data_asinkron() {
            kembalikan 100 * 2;
        }

        asinkron fungsi proses_utama() {
            misal hasil = tunggu_hasil ambil_data_asinkron();
            kembalikan hasil + 50;
        }

        misal hasil_akhir = tunggu_hasil proses_utama();
        pastikan(hasil_akhir == 250, "Hasil async await harus tepat 250");
        kembalikan hasil_akhir;
    "#;
    let hasil = jalankan(kode).unwrap();
    assert_eq!(hasil, Value::Number(250.0));
}

#[test]
fn test_widya_ai_http_dan_game() {
    let kode = r#"
        // 1. AI Tensor Compute
        misal m1 = Matriks([[1, 2], [3, 4]]);
        misal m2 = Matriks([[5, 6], [7, 8]]);
        misal kali = matriks_kali(m1, m2);
        misal sig = sigmoid(0.0);
        misal rel = relu(-10.0);

        // 2. HTTP Engine
        misal srv = ServerHttp(3000);
        misal resp = http_get("http://localhost:3000/api");

        // 3. Game Engine
        misal g = KanvasGame("Game Uji", 640, 480);
        misal html = render_game_html(g);

        pastikan(kali.data[0][0] == 19, "Perkalian matriks baris 0 kolom 0 harus 19");
        pastikan(sig == 0.5, "Sigmoid 0 harus 0.5");
        pastikan(rel == 0.0, "ReLU -10 harus 0.0");
        pastikan(srv.port == 3000, "Port server harus 3000");
        pastikan(resp.status == 200, "HTTP Status harus 200");
        pastikan(panjang(html) > 100, "Render game html harus valid");

        kembalikan [kali.data[0][0], sig, rel, srv.port, resp.status];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(19.0));
        assert_eq!(items[1], Value::Number(0.5));
        assert_eq!(items[2], Value::Number(0.0));
        assert_eq!(items[3], Value::Number(3000.0));
        assert_eq!(items[4], Value::Number(200.0));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_c_ffi_dan_baremetal_volatil() {
    let kode = r#"
        eksternal "C" {
            fungsi abs(x: Angka) -> Angka;
            fungsi sqrt(x: Angka) -> Angka;
        }

        misal v_abs = abs(-42);
        misal v_sqrt = sqrt(81);

        // Hardware Volatile Memory direct read/write
        misal ADDR_REG = 0x1000;
        tulis_volatil(ADDR_REG, 1234);
        misal dibaca = volatil ADDR_REG;

        pastikan(v_abs == 42, "C abs(-42) harus bernilai 42");
        pastikan(v_sqrt == 9, "C sqrt(81) harus bernilai 9");
        pastikan(dibaca == 1234, "Nilai register volatil yang dibaca harus 1234");

        kembalikan [v_abs, v_sqrt, dibaca];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(42.0));
        assert_eq!(items[1], Value::Number(9.0));
        assert_eq!(items[2], Value::Number(1234.0));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_websocket_realtime() {
    let kode = r#"
        misal ws = ServerWebSocket("127.0.0.1", 8088);
        misal k1 = ws_kirim(ws, "ws://klien-1", "Halo Klien 1");
        misal siar = ws_siarkan(ws, "Siaran live data");

        pastikan(ws.port == 8088, "Port WebSocket harus 8088");
        pastikan(k1 == benar, "Kirim WebSocket harus berhasil");
        pastikan(siar == benar, "Siaran WebSocket harus berhasil");

        kembalikan ws.status;
    "#;
    let hasil = jalankan(kode).unwrap();
    assert_eq!(hasil, Value::String("MENDENGARKAN".to_string()));
}

#[test]
fn test_llvm_ir_emitter() {
    let kode = r#"
        fungsi hitung_luas(panjang, lebar) {
            kembalikan panjang * lebar;
        }

        misal x = 10;
        misal y = 20;
        misal luas = hitung_luas(x, y);
        cetak(luas);
    "#;

    let mut lexer = widya::lexer::Lexer::new(kode);
    let tokens = lexer.scan_tokens().unwrap();
    let mut parser = widya::parser::Parser::new(tokens);
    let program = parser.parse().unwrap();

    let mut emitter = widya::llvm::LlvmEmitter::new();
    let ir = emitter.emit_llvm_ir(&program).unwrap();

    assert!(ir.contains("define double @widya_fn_hitung_luas"));
    assert!(ir.contains("define i32 @main()"));
    assert!(ir.contains("target triple = \"x86_64-unknown-linux-gnu\""));
}

#[test]
fn test_macro_turunkan_metaprogramming() {
    let kode = r#"
        #[turunkan(Json, Debug, Duplikat)]
        struktur Produk {
            id,
            nama,
            harga,
        }

        misal p = Produk(99, "Laptop Widya", 15000000);

        misal j = p.ke_json();
        misal d = p.duplikat();

        pastikan(panjang(j) > 10, "Output JSON auto-generated harus valid");
        pastikan(d.id == 99, "Duplikat objek harus memiliki ID 99");

        kembalikan [p.id, d.id];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(99.0));
        assert_eq!(items[1], Value::Number(99.0));
    } else {
        panic!("Hasil harus berupa daftar");
    }
}

#[test]
fn test_wasm_binary_emitter() {
    let kode = r#"
        fungsi tambah(a, b) {
            kembalikan a + b;
        }
    "#;
    let mut lexer = widya::lexer::Lexer::new(kode);
    let tokens = lexer.scan_tokens().unwrap();
    let mut parser = widya::parser::Parser::new(tokens);
    let program = parser.parse().unwrap();

    let mut wasm_emitter = widya::wasm::WasmEmitter::new();
    let wasm_bytes = wasm_emitter.emit_wasm(&program).unwrap();

    // Verify WASM magic bytes: \0asm (0x00, 0x61, 0x73, 0x6D)
    assert_eq!(&wasm_bytes[0..4], &[0x00, 0x61, 0x73, 0x6D]);
    // Verify WASM version 1: 0x01, 0x00, 0x00, 0x00
    assert_eq!(&wasm_bytes[4..8], &[0x01, 0x00, 0x00, 0x00]);
}

#[test]
fn test_borrow_checker_safety() {
    let kode = r#"
        misal a = 100;
        misal b = a + 50;
        kembalikan b;
    "#;
    let hasil = jalankan(kode).unwrap();
    assert_eq!(hasil, Value::Number(150.0));
}

#[test]
fn test_tools_suite() {
    use std::path::PathBuf;
    let test_file = PathBuf::from("contoh/19_uji_dan_benchmark.wya");
    assert!(test_file.exists(), "Berkas contoh 19 harus tersedia");

    // Test formatter
    widya::tools::formatter::format_berkas(&test_file, false);

    // Test linter
    widya::tools::formatter::periksa_linter(&test_file);

    // Test benchmark
    widya::tools::bench::jalankan_benchmark(&test_file);
}

#[test]
fn test_simd_dan_jit() {
    let kode = r#"
        misal v1 = VektorF32x4(10.0, 20.0, 30.0, 40.0);
        misal v2 = VektorF32x4(2.0, 3.0, 4.0, 5.0);

        misal v_tambah = simd_tambah(v1, v2);
        misal v_kali = simd_kali(v1, v2);
        misal dot = simd_dot(v1, v2);
        misal dist = simd_jarak(v1, v2);

        pastikan(v_tambah.x == 12.0, "SIMD tambah X harus 12");
        pastikan(v_kali.w == 200.0, "SIMD kali W harus 200");
        pastikan(dot == 400.0, "SIMD dot product harus 400");
        pastikan(dist > 0.0, "Jarak Euclidean harus positif");

        kembalikan [v_tambah.x, v_kali.w, dot];
    "#;

    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(12.0));
        assert_eq!(items[1], Value::Number(200.0));
        assert_eq!(items[2], Value::Number(400.0));
    } else {
        panic!("Hasil harus berupa daftar");
    }

    // Test JIT Engine
    let jit = widya::tools::jit::JitEngine::new();
    let res = jit.execute_jit(kode);
    assert!(res.is_ok());
}

#[test]
fn test_doc_gen_dan_bundler() {
    use std::path::PathBuf;
    let source_path = PathBuf::from("contoh/20_simd_dan_jit.wya");
    assert!(source_path.exists());

    // Test Doc Generator
    let doc_gen = widya::tools::doc_gen::DocGenerator::new("Test Doc");
    let doc_res = doc_gen.generate_docs_for_file(&source_path, None);
    assert!(doc_res.is_ok());

    // Test Bundler
    let bundler = widya::tools::bundler::StandaloneBundler::new("app_test");
    let bundle_res = bundler.bundle_executable(&source_path, None);
    assert!(bundle_res.is_ok());
}

#[test]
fn test_graphql_dan_grpc() {
    let kode = r#"
        misal srv_gql = ServerGraphQL("type Query { test: String }", 4000);
        misal res_gql = eksekusi_graphql(srv_gql, "query { pengguna }");

        misal srv_rpc = ServerGRPC("AuthService", 50051);
        misal cli_rpc = KlienGRPC("127.0.0.1:50051", "AuthService");
        misal res_rpc = grpc_panggil(cli_rpc, "Login", "user=widya");

        pastikan(srv_gql.port == 4000, "GraphQL port harus 4000");
        pastikan(res_gql.data.pengguna.nama == "Widya Developer", "Nama user GraphQL harus sesuai");
        pastikan(srv_rpc.port == 50051, "gRPC port harus 50051");
        pastikan(res_rpc.status == "OK_200", "gRPC status harus OK_200");

        kembalikan [srv_gql.port, srv_rpc.port];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(4000.0));
        assert_eq!(items[1], Value::Number(50051.0));
    } else {
        panic!("Hasil harus berupa array");
    }
}

#[test]
fn test_shared_memory_dan_regex() {
    let kode = r#"
        misal shm = buka_memori_bersama("test_shm_block", 1024);
        misal n = tulis_memori_bersama(shm, "HALO_WIDYA_SHM");
        misal dibaca = baca_memori_bersama(shm);

        pastikan(shm.ukuran == 1024, "Ukuran shm harus 1024");
        pastikan(n == 14, "Jumlah byte tertulis harus 14");
        pastikan(dibaca == "HALO_WIDYA_SHM", "Data dibaca dari shm harus sama");

        // Regex tests
        misal teks = "Kode 123 dan 456";
        misal cocok = regex_cocok(r"\d+", teks);
        misal temukan = regex_temukan(r"\d+", teks);
        misal ganti = regex_ganti(r"\d+", "XXX", teks);
        misal hasil_tangkap = regex_tangkap(r"\d+", teks);

        pastikan(cocok == benar, "Regex harus cocok angka");
        pastikan(temukan.hasil == "123", "Temukan pertama harus 123");
        pastikan(ganti == "Kode XXX dan XXX", "Ganti harus mengganti angka");
        pastikan(panjang(hasil_tangkap) == 2, "Jumlah tangkapan angka harus 2");

        kembalikan [n, panjang(hasil_tangkap)];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(14.0));
        assert_eq!(items[1], Value::Number(2.0));
    } else {
        panic!("Hasil harus berupa array");
    }
}

#[test]
fn test_aktor_dan_aliran() {
    let kode = r#"
        misal aktor = Aktor("Worker", 50);
        kirim_ke_aktor(aktor, 25);
        misal res = tunggu_respon_aktor(aktor);

        misal stream = Aliran("DataStream");
        alirkan_ke(stream, 5);
        alirkan_ke(stream, 10);
        misal tf = transformasi_aliran(stream, "kali_dua");

        pastikan(res == 75, "Hasil aktor harus 75");
        pastikan(panjang(tf) == 2, "Panjang hasil stream harus 2");

        kembalikan [res, panjang(tf)];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(75.0));
        assert_eq!(items[1], Value::Number(2.0));
    } else {
        panic!("Hasil harus berupa array");
    }
}

#[test]
fn test_gpu_compute_shader() {
    let kode = r#"
        fungsi kali_elemen(a, b) {
            kembalikan a * b;
        }

        misal pipa = PipaGPU("shader_code", 64);
        misal ba = [2.0, 3.0, 4.0];
        misal bb = [10.0, 20.0, 30.0];
        misal res = eksekusi_gpu(pipa, ba, bb);

        pastikan(pipa.workgroup == 64, "Workgroup GPU harus 64");
        pastikan(res[0] == 20.0, "Hasil GPU [0] harus 20");
        pastikan(res[2] == 120.0, "Hasil GPU [2] harus 120");

        kembalikan [res[0], res[2]];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(20.0));
        assert_eq!(items[1], Value::Number(120.0));
    } else {
        panic!("Hasil harus berupa array");
    }

    // Verify GPU WGSL Emitter
    let mut lexer = widya::lexer::Lexer::new(kode);
    let tokens = lexer.scan_tokens().unwrap();
    let mut parser = widya::parser::Parser::new(tokens);
    let program = parser.parse().unwrap();

    let mut emitter = widya::gpu::GpuShaderEmitter::new();
    let wgsl = emitter.emit_wgsl(&program).unwrap();
    assert!(wgsl.contains("@compute @workgroup_size(64)"));
    assert!(wgsl.contains("fn widya_fn_kali_elemen"));
}

#[test]
fn test_kuantum_dan_zkproof() {
    let kode = r#"
        misal sirkuit = SirkuitKuantum(2);
        gerbang_h(sirkuit, 0);
        gerbang_cnot(sirkuit, 0, 1);
        misal probs = ukur_kuantum(sirkuit);

        pastikan(sirkuit.qubits == 2, "Jumlah Qubits harus 2");
        pastikan(panjang(probs) == 4, "Panjang probabilitas harus 4");

        // Test ZK Proof
        misal bukti = buat_bukti_zk("secret123", "alice");
        misal valid = verifikasi_bukti_zk(bukti, "alice");
        misal tidak_valid = verifikasi_bukti_zk(bukti, "bob");

        pastikan(valid == benar, "Verifikasi ZK untuk nilai publik yang cocok harus benar");
        pastikan(tidak_valid == salah, "Verifikasi ZK untuk nilai publik yang salah harus salah");

        kembalikan [sirkuit.qubits, panjang(probs)];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(2.0));
        assert_eq!(items[1], Value::Number(4.0));
    } else {
        panic!("Hasil harus berupa array");
    }
}

#[test]
fn test_raft_dan_ebpf() {
    let kode = r#"
        misal node = NodeRaft("node_1", ["node_2"]);
        misal p = pilih_pemimpin(node);
        usulkan_log(node, "LOG_TEST");
        misal st = status_konsensus(node);

        pastikan(p == "PEMIMPIN", "Peran setelah pemilihan harus PEMIMPIN");
        pastikan(st.jumlah_log == 1, "Jumlah log harus 1");

        // Test eBPF
        misal prog = ProgramEBPF("kprobe", "sys_clone");
        misal probe_ok = pasang_probe_ebpf(prog, "kernel_sys_clone");
        misal peta = baca_peta_ebpf(prog, "packet_map");

        pastikan(probe_ok == benar, "Pasang probe eBPF harus berhasil");
        pastikan(peta.latensi_rata_rata_ns == 350.0, "Latensi eBPF harus 350 ns");

        kembalikan [st.jumlah_log, peta.latensi_rata_rata_ns];
    "#;
    let hasil = jalankan(kode).unwrap();
    if let Value::Array(arr) = hasil {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(1.0));
        assert_eq!(items[1], Value::Number(350.0));
    } else {
        panic!("Hasil harus berupa array");
    }

    // Verify eBPF C Emitter
    let mut lexer = widya::lexer::Lexer::new(kode);
    let tokens = lexer.scan_tokens().unwrap();
    let mut parser = widya::parser::Parser::new(tokens);
    let program = parser.parse().unwrap();

    let mut ebpf_emitter = widya::ebpf::EbpfEmitter::new();
    let c_code = ebpf_emitter.emit_ebpf_c(&program).unwrap();
    assert!(c_code.contains("#include <linux/bpf.h>"));
    assert!(c_code.contains("BPF_MAP_TYPE_HASH"));
}







