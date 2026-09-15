use widya::jalankan;
use widya::value::Value;

#[test]
fn test_widya_benchmark() {
    let code = r#"
        fungsi tambah() {
            misal s = 0;
            untuk i dalam 0..100 {
                s += i;
            }
            kembalikan s;
        }

        misal res = waktu_eksekusi(tambah, []);
        misal bench = tolak_ukur_fungsi(tambah, [], 5);
        kembalikan [res["hasil"], bench["iterasi"]];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(5050.0));
        assert_eq!(items[1], Value::Number(5.0));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_mime() {
    let code = r#"
        misal m_json = ekstensi_ke_mime("json");
        misal m_pdf = ekstensi_ke_mime(".pdf");
        misal is_txt = apakah_mime_teks(m_json);
        misal ext_png = mime_ke_ekstensi("image/png");
        kembalikan [m_json, m_pdf, is_txt, ext_png];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::String("application/json".to_string()));
        assert_eq!(items[1], Value::String("application/pdf".to_string()));
        assert_eq!(items[2], Value::Bool(true));
        assert_eq!(items[3], Value::String("png".to_string()));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_counting_bloom() {
    let code = r#"
        misal cbf = CountingBloomFilter(50, 3);
        cbloom_tambah(cbf, "kucing");
        cbloom_tambah(cbf, "anjing");

        misal punya_kucing = cbloom_mungkin_ada(cbf, "kucing");
        misal punya_burung = cbloom_mungkin_ada(cbf, "burung");

        misal hapus_kucing = cbloom_hapus(cbf, "kucing");
        misal kucing_stlh_hapus = cbloom_mungkin_ada(cbf, "kucing");
        misal anjing_msh_ada = cbloom_mungkin_ada(cbf, "anjing");

        kembalikan [punya_kucing, punya_burung, hapus_kucing, kucing_stlh_hapus, anjing_msh_ada];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Bool(true));
        assert_eq!(items[1], Value::Bool(false));
        assert_eq!(items[2], Value::Bool(true));
        assert_eq!(items[3], Value::Bool(false));
        assert_eq!(items[4], Value::Bool(true));
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}

#[test]
fn test_widya_matrix_math() {
    let code = r#"
        misal m = matriks_dari_array([[1, 2], [3, 4]]);
        misal det = matriks_determinan(m);
        misal inv = matriks_invers(m);
        misal ident = matriks_kali(m, inv);
        misal ident_arr = matriks_ke_array(ident);
        kembalikan [det, ident_arr[0][0], ident_arr[0][1], ident_arr[1][0], ident_arr[1][1]];
    "#;
    let res = jalankan(code).unwrap();
    if let Value::Array(arr) = res {
        let items = arr.borrow();
        assert_eq!(items[0], Value::Number(-2.0));
        // Check identity matrix elements within float tolerance
        if let Value::Number(v00) = items[1] {
            assert!((v00 - 1.0).abs() < 1e-5);
        }
        if let Value::Number(v01) = items[2] {
            assert!(v01.abs() < 1e-5);
        }
        if let Value::Number(v10) = items[3] {
            assert!(v10.abs() < 1e-5);
        }
        if let Value::Number(v11) = items[4] {
            assert!((v11 - 1.0).abs() < 1e-5);
        }
    } else {
        panic!("Expected Array, got {:?}", res);
    }
}
