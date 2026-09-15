use widya::jalankan;
use widya::value::Value;

#[test]
fn test_decimal_dan_heap() {
    let code = r##"
        misal d1 = Desimal("100.50", 2);
        misal d2 = Desimal("50.25", 2);
        
        misal dt = desimal_tambah(d1, d2);
        misal dk = desimal_kurang(d1, d2);
        misal dmul = desimal_kali(d1, Desimal("2.00", 2));
        misal ddiv = desimal_bagi(d1, Desimal("2.00", 2), 2);
        misal rp = desimal_format_rupiah(d1);

        misal pq = AntreanPrioritas(benar);
        antrean_dorong(pq, "Rendah", 1);
        antrean_dorong(pq, "Tinggi", 10);
        antrean_dorong(pq, "Sedang", 5);

        tetap top = antrean_tarik(pq);
        tetap next = antrean_tarik(pq);
        tetap last = antrean_tarik(pq);

        kembalikan [
            desimal_ke_string(dt) == "150.75",
            desimal_ke_string(dk) == "50.25",
            desimal_ke_string(dmul) == "201.00",
            desimal_ke_string(ddiv) == "50.25",
            rp == "Rp 100,50",
            top == "Tinggi",
            next == "Sedang",
            last == "Rendah",
            antrean_kosong(pq) == benar
        ];
    "##;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        for item in items.iter() {
            assert_eq!(*item, Value::Bool(true));
        }
    } else {
        panic!("Harus mengembalikan array");
    }
}

#[test]
fn test_glob_dan_uuid5() {
    let code = r##"
        tetap m1 = cocokkan_glob("src/*.rs", "src/main.rs");
        tetap m2 = cocokkan_glob("src/*.rs", "docs/main.md");
        tetap m3 = cocokkan_glob("src/**/*.rs", "src/stdlib/mod.rs");

        tetap list = ["src/a.rs", "src/b.js", "tests/c.rs"];
        tetap filtered = saring_glob(list, "src/*.rs");

        tetap u1 = buat_uuid_v5_dns("widya-lang.org");
        tetap u2 = buat_uuid_v5_dns("widya-lang.org");
        tetap u_url = buat_uuid_v5_url("https://widya-lang.org");

        kembalikan [
            m1 == benar,
            m2 == salah,
            m3 == benar,
            panjang(filtered) == 1,
            filtered[0] == "src/a.rs",
            u1 == u2,
            panjang(u1) == 36,
            panjang(u_url) == 36
        ];
    "##;

    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        for item in items.iter() {
            assert_eq!(*item, Value::Bool(true));
        }
    } else {
        panic!("Harus mengembalikan array");
    }
}
