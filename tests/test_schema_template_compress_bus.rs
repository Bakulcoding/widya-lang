use widya::jalankan;
use widya::value::Value;

#[test]
fn test_schema_dan_template() {
    let code = r##"
        misal schema = {
            "nama": { "wajib": benar, "tipe": "teks", "min_panjang": 3 },
            "umur": { "wajib": benar, "tipe": "angka", "min": 18 }
        };

        misal data_ok = { "nama": "Alice", "umur": 20 };
        misal data_bad = { "nama": "A", "umur": 15 };

        tetap res_ok = validasi_skema(data_ok, schema);
        tetap res_bad = validasi_skema(data_bad, schema);

        misal tmpl = "Halo {{nama}}, umurmu {{umur}} tahun!";
        tetap rendered = render_template(tmpl, data_ok);

        kembalikan [
            res_ok.valid == benar,
            panjang(res_ok.galat) == 0,
            res_bad.valid == salah,
            panjang(res_bad.galat) == 2,
            rendered == "Halo Alice, umurmu 20 tahun!"
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
fn test_compress_dan_eventbus() {
    let code = r##"
        tetap orig = "AAAAABBBCC";
        tetap rle = kompres_rle(orig);
        tetap dec_rle = dekompres_rle(rle);

        tetap lzw = kompres_lzw("ABCABCABC");
        tetap dec_lzw = dekompres_lzw(lzw);

        misal bus = BusEvent();
        misal count = 0;
        
        bus_dengarkan(bus, "e1", fungsi(p) { kembalikan 1; });
        bus_dengarkan_sekali(bus, "e1", fungsi(p) { kembalikan 2; });

        tetap c1 = bus_pancarkan(bus, "e1", "data");
        tetap c2 = bus_pancarkan(bus, "e1", "data");

        kembalikan [
            rle == "5A3B2C",
            dec_rle == orig,
            dec_lzw == "ABCABCABC",
            c1 == 2,
            c2 == 1
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
