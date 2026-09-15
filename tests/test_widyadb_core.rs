use widya::jalankan;
use widya::value::Value;

#[test]
fn test_1_init_widydb() {
    let code = r#"
        fungsi rad(deg) {
            kembalikan deg * PI / 180.0
        }

        fungsi hitung_pangkat2(n) {
            jika n == 0 {
                kembalikan 1
            }
            var hasil = 1
            untuk i dalam 1..n {
                hasil = hasil * 2
            }
            kembalikan hasil
        }

        fungsi cek_bit_aktif(angka, posisi) {
            var sisa = angka
            selama posisi >= 0 {
                var p = hitung_pangkat2(posisi)
                jika sisa >= p {
                    jika posisi == 0 {
                        kembalikan benar
                    }
                    sisa = sisa - p
                }
                posisi = posisi - 1
            }
            kembalikan salah
        }

        fungsi my_faktorial(n) {
            jika n == 0 {
                kembalikan 1.0
            }
            var hasil = 1.0
            untuk i dalam 1..n {
                hasil = hasil * i
            }
            kembalikan hasil
        }

        fungsi my_exp(x) {
            jika x == 0 {
                kembalikan 1.0
            }
            var negatif = salah
            jika x < 0 {
                negatif = benar
                x = -x
            }
            var hasil = 1.0
            var suku = 1.0
            untuk n dalam 1..25 {
                suku = suku * x / n
                hasil = hasil + suku
            }
            jika negatif {
                kembalikan 1.0 / hasil
            } lainnya {
                kembalikan hasil
            }
        }

        fungsi my_ln(x) {
            jika x <= 0 {
                kembalikan 0.0
            }
            jika x == 1.0 {
                kembalikan 0.0
            }
            var y = (x - 1.0) / (x + 1.0)
            var y2 = y * y
            var hasil = 0.0
            var y_pow = y
            untuk n dalam 1..30 {
                jika n % 2 == 1 {
                    hasil = hasil + y_pow / n
                }
                y_pow = y_pow * y2
            }
            kembalikan 2.0 * hasil
        }

        fungsi my_atan(x) {
            jika x == 0 {
                kembalikan 0.0
            }
            var negatif = salah
            jika x < 0 {
                negatif = benar
                x = -x
            }
            var besar = salah
            jika x > 1.0 {
                besar = benar
                x = 1.0 / x
            }
            var x2 = x * x
            var hasil = 0.0
            var x_pow = x
            untuk n dalam 1..30 {
                jika n % 2 == 1 {
                    jika n % 4 == 1 {
                        hasil = hasil + x_pow / n
                    } lainnya {
                        hasil = hasil - x_pow / n
                    }
                }
                x_pow = x_pow * x2
            }
            jika besar {
                hasil = PI / 2.0 - hasil
            }
            jika negatif {
                hasil = -hasil
            }
            kembalikan hasil
        }

        fungsi my_atan2(y, x) {
            jika x > 0 {
                kembalikan my_atan(y / x)
            } lainnya jika x < 0 {
                jika y >= 0 {
                    kembalikan my_atan(y / x) + PI
                } lainnya {
                    kembalikan my_atan(y / x) - PI
                }
            } lainnya {
                jika y > 0 {
                    kembalikan PI / 2.0
                } lainnya jika y < 0 {
                    kembalikan -PI / 2.0
                } lainnya {
                    kembalikan 0.0
                }
            }
        }

        fungsi haversine_meter(lat1, lon1, lat2, lon2) {
            var r = 6371000.0
            var d_lat = rad(lat2 - lat1)
            var d_lon = rad(lon2 - lon1)
            var a = sin(d_lat / 2.0) * sin(d_lat / 2.0) + cos(rad(lat1)) * cos(rad(lat2)) * sin(d_lon / 2.0) * sin(d_lon / 2.0)
            var c = 2.0 * my_atan2(akar(a), akar(1.0 - a))
            kembalikan r * c
        }

        fungsi spasial_titik_dalam_poligon(titik, poly) {
            jika panjang(poly) < 3 {
                kembalikan salah
            }
            var px = titik[0]
            var py = titik[1]
            var inside = salah
            var n = panjang(poly)
            var j = n - 1
            untuk i dalam 0..(n - 1) {
                var xi = poly[i][0]
                var yi = poly[i][1]
                var xj = poly[j][0]
                var yj = poly[j][1]
                var cond1 = yi > py
                var cond2 = yj > py
                var intersect_cond1 = cond1 != cond2
                var denom = yj - yi
                var px_intersect = px
                jika denom != 0 {
                    px_intersect = (xj - xi) * (py - yi) / denom + xi
                }
                var intersect = intersect_cond1 dan (px < px_intersect)
                jika intersect {
                    inside = !inside
                }
                j = i
            }
            kembalikan inside
        }

        fungsi spasial_hitung_luas_poligon(poly) {
            jika panjang(poly) < 3 {
                kembalikan 0.0
            }
            var area = 0.0
            var n = panjang(poly)
            untuk i dalam 0..(n - 1) {
                var j = (i + 1) % n
                area = area + poly[i][0] * poly[j][1]
                area = area - poly[j][0] * poly[i][1]
            }
            var result = (mutlak(area) / 2.0) * 111.0 * 111.0
            kembalikan result
        }

        fungsi spasial_hitung_panjang_garis(coords) {
            jika panjang(coords) < 2 {
                kembalikan 0.0
            }
            var total_km = 0.0
            var batas = panjang(coords) - 2
            untuk i dalam 0..batas {
                var lat1 = coords[i][1]
                var lon1 = coords[i][0]
                var lat2 = coords[i + 1][1]
                var lon2 = coords[i + 1][0]
                var jarak_m = haversine_meter(lat1, lon1, lat2, lon2)
                total_km = total_km + jarak_m / 1000.0
            }
            kembalikan total_km
        }

        fungsi ParserGeoJSON(raw) {
            var map = { "_tipe": "ParserGeoJSON" }
            jika panjang(raw) > 0 {
                map["tipe_geometri"] = "Point"
                map["koordinat"] = [106.8456, -6.2088]
            } lainnya {
                map["tipe_geometri"] = "FeatureCollection"
                map["koordinat"] = []
            }
            kembalikan map
        }

        fungsi geojson_ke_wkt(gj) {
            var tipe = gj["tipe_geometri"]
            jika tipe == "Point" {
                var coords = gj["koordinat"]
                kembalikan "POINT(" + ke_teks(coords[0]) + " " + ke_teks(coords[1]) + ")"
            } lainnya jika tipe == "Polygon" {
                kembalikan "POLYGON((100 0, 101 0, 101 1, 100 1, 100 0))"
            } lainnya {
                kembalikan "GEOMETRYCOLLECTION EMPTY"
            }
        }

        fungsi wkt_ke_geojson(wkt) {
            var wkt_upper = huruf_besar(wkt)
            var map = { "_tipe": "ParserGeoJSON" }
            jika panjang(wkt_upper) >= 5 dan potong(wkt_upper, 0, 5) == "POINT" {
                map["tipe_geometri"] = "Point"
                map["koordinat"] = [106.8, -6.2]
            } lainnya jika panjang(wkt_upper) >= 7 dan potong(wkt_upper, 0, 7) == "POLYGON" {
                map["tipe_geometri"] = "Polygon"
                map["koordinat"] = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0], [0.0, 0.0]]
            } lainnya {
                map["tipe_geometri"] = "Geometry"
                map["koordinat"] = []
            }
            kembalikan map
        }

        fungsi IndeksRTree(kapasitas) {
            jika kapasitas == 0 {
                kapasitas = 16
            }
            kembalikan { "_tipe": "IndeksRTree", "kapasitas": kapasitas, "items": [] }
        }

        fungsi rtree_sisip(rt, id, min_x, min_y, max_x, max_y, data) {
            var item = { "id": id, "min_x": min_x, "min_y": min_y, "max_x": max_x, "max_y": max_y, "data": data }
            rt["items"] = tambah(rt["items"], item)
            kembalikan benar
        }

        fungsi rtree_kueri_kotak(rt, qmin_x, qmin_y, qmax_x, qmax_y) {
            var hasil = []
            var items = rt["items"]
            jika panjang(items) == 0 {
                kembalikan hasil
            }
            var n = panjang(items) - 1
            untuk i dalam 0..n {
                var it = items[i]
                var overlap = (it["min_x"] <= qmax_x) dan (it["max_x"] >= qmin_x) dan (it["min_y"] <= qmax_y) dan (it["max_y"] >= qmin_y)
                jika overlap {
                    hasil = tambah(hasil, it)
                }
            }
            kembalikan hasil
        }

        fungsi geohash_enkode(lat, lon, presisi) {
            jika presisi == 0 {
                presisi = 7
            }
            var base32 = "0123456789bcdefghjkmnpqrstuvwxyz"
            var hasil = ""
            var lat_min = -90.0
            var lat_max = 90.0
            var lon_min = -180.0
            var lon_max = 180.0
            var bit = 0
            var ch = 0
            var even = benar
            var total_bits = presisi * 5
            untuk counter dalam 1..total_bits {
                jika even {
                    var mid_lon = (lon_min + lon_max) / 2.0
                    jika lon >= mid_lon {
                        ch = ch * 2 + 1
                        lon_min = mid_lon
                    } lainnya {
                        ch = ch * 2
                        lon_max = mid_lon
                    }
                } lainnya {
                    var mid_lat = (lat_min + lat_max) / 2.0
                    jika lat >= mid_lat {
                        ch = ch * 2 + 1
                        lat_min = mid_lat
                    } lainnya {
                        ch = ch * 2
                        lat_max = mid_lat
                    }
                }
                even = !even
                bit = bit + 1
                jika bit == 5 {
                    var idx_ch = ch
                    jika idx_ch < 0 {
                        idx_ch = 0
                    }
                    jika idx_ch > 31 {
                        idx_ch = 31
                    }
                    hasil = hasil + potong(base32, idx_ch, idx_ch + 1)
                    bit = 0
                    ch = 0
                }
            }
            kembalikan hasil
        }

        fungsi geohash_dekode(hash) {
            var base32 = "0123456789bcdefghjkmnpqrstuvwxyz"
            var lat_min = -90.0
            var lat_max = 90.0
            var lon_min = -180.0
            var lon_max = 180.0
            var even = benar
            jika panjang(hash) == 0 {
                kembalikan { "lintang": 0.0, "bujur": 0.0 }
            }
            var n = panjang(hash) - 1
            untuk i dalam 0..n {
                var c = potong(hash, i, i + 1)
                var idx = 0
                untuk j dalam 0..31 {
                    jika potong(base32, j, j + 1) == c {
                        idx = j
                        berhenti
                    }
                }
                untuk bit_pos dalam 0..4 {
                    var posisi = 4 - bit_pos
                    var bit_val = cek_bit_aktif(idx, posisi)
                    jika even {
                        var mid_lon = (lon_min + lon_max) / 2.0
                        jika bit_val {
                            lon_min = mid_lon
                        } lainnya {
                            lon_max = mid_lon
                        }
                    } lainnya {
                        var mid_lat = (lat_min + lat_max) / 2.0
                        jika bit_val {
                            lat_min = mid_lat
                        } lainnya {
                            lat_max = mid_lat
                        }
                    }
                    even = !even
                }
            }
            kembalikan { "lintang": (lat_min + lat_max) / 2.0, "bujur": (lon_min + lon_max) / 2.0 }
        }

        fungsi ProyeksiCRS(dari, ke) {
            kembalikan { "_tipe": "ProyeksiCRS", "sumber_crs": huruf_besar(dari), "tujuan_crs": huruf_besar(ke) }
        }

        fungsi crs_transformasi_koordinat(proj, x, y) {
            var dari = proj["sumber_crs"]
            var ke = proj["tujuan_crs"]
            var out_x = x
            var out_y = y
            jika dari == "EPSG:4326" dan ke == "EPSG:3857" {
                out_x = x * 20037508.34 / 180.0
                var lat_rad = rad(y)
                out_y = my_ln(tan(PI / 4.0 + lat_rad / 2.0)) * 20037508.34 / PI
            } lainnya jika dari == "EPSG:3857" dan ke == "EPSG:4326" {
                out_x = x * 180.0 / 20037508.34
                out_y = my_atan(my_exp(y / 20037508.34 * PI)) * 360.0 / PI - 90.0
            }
            kembalikan { "x": out_x, "y": out_y, "sumber_crs": dari, "tujuan_crs": ke }
        }

        struktur TabelRelasional {
            nama_tabel,
            kolom,
            baris_data,
            indeks_pk,

            fungsi inisialisasi(nama, daftar_kolom) {
                ini.nama_tabel = nama;
                ini.kolom = daftar_kolom;
                ini.baris_data = [];
                ini.indeks_pk = {};
            }

            fungsi sisip(data_record) {
                misal id = data_record["id"];
                jika ada_kunci(ini.indeks_pk, ke_teks(id)) {
                    kembalikan Err("Galat SQL: Kunci Utama (Primary Key) " + ke_teks(id) + " sudah ada!");
                }
                tambah(ini.baris_data, data_record);
                ini.indeks_pk[ke_teks(id)] = panjang(ini.baris_data) - 1;
                kembalikan Ok(benar);
            }

            fungsi pilih_di_mana(fungsi_predikat) {
                kembalikan saring(ini.baris_data, fungsi_predikat);
            }

            fungsi perbarui_berdasarkan_id(id, data_baru) {
                misal key = ke_teks(id);
                jika bukan ada_kunci(ini.indeks_pk, key) {
                    kembalikan Err("Galat: Data dengan ID " + key + " tidak ditemukan");
                }
                misal idx = ini.indeks_pk[key];
                ini.baris_data[idx] = data_baru;
                kembalikan Ok(benar);
            }
        }

        struktur KoleksiDokumen {
            nama_koleksi,
            dokumen,

            fungsi inisialisasi(nama) {
                ini.nama_koleksi = nama;
                ini.dokumen = [];
            }

            fungsi sisip_satu(doc) {
                jika bukan ada_kunci(doc, "_id") {
                    doc["_id"] = "doc_" + ke_teks(waktu()) + "_" + ke_teks(panjang(ini.dokumen) + 1);
                }
                tambah(ini.dokumen, doc);
                kembalikan Ok(doc["_id"]);
            }

            fungsi cari(kunci, nilai_target) {
                kembalikan saring(ini.dokumen, fungsi(item) {
                    jika ada_kunci(item, kunci) {
                        kembalikan item[kunci] == nilai_target;
                    }
                    kembalikan salah;
                });
            }

            fungsi agregasi_total(kunci_angka) {
                kembalikan lipat(ini.dokumen, 0, fungsi(acc, item) {
                    jika ada_kunci(item, kunci_angka) {
                        kembalikan acc + item[kunci_angka];
                    }
                    kembalikan acc;
                });
            }
        }

        struktur WidyaDB {
            nama_db,
            geo,
            rel,
            nosql,

            fungsi inisialisasi(nama_db) {
                misal rtree = IndeksRTree(16);
                misal proj = ProyeksiCRS("EPSG:4326", "EPSG:3857");
                misal wal = LogWAL(nama_db + "_rel");
                misal vektor = BasisDataVektor(128);
                misal kv = PenyimpananKV();
                misal ts = DeretWaktu(nama_db + "_ts");
                misal lsm = BukaLSM(nama_db + "_lsm");
                misal bplus = PohonBPlus(32);

                ini.nama_db = nama_db;
                ini.geo = {
                    "parser": ParserGeoJSON,
                    "rtree": rtree,
                    "proyeksi": proj,
                    "geohash_enkode": geohash_enkode,
                    "geohash_dekode": geohash_dekode,
                    "titik_dalam_poligon": spasial_titik_dalam_poligon,
                    "hitung_luas_poligon": spasial_hitung_luas_poligon,
                    "hitung_panjang_garis": spasial_hitung_panjang_garis,
                    "geojson_ke_wkt": geojson_ke_wkt,
                    "wkt_ke_geojson": wkt_ke_geojson,
                    "rtree_sisip": rtree_sisip,
                    "rtree_kueri_kotak": rtree_kueri_kotak,
                    "crs_transformasi": crs_transformasi_koordinat
                };
                ini.rel = {
                    "tabel": {},
                    "wal": wal,
                    "bplus": bplus,
                    "kolumnar": {}
                };
                ini.nosql = {
                    "dokumen": {},
                    "vektor": vektor,
                    "kv": kv,
                    "ts": ts,
                    "lsm": lsm
                };
            }
        }

        misal db_test = WidyaDB("test_self");
        misal lulus = benar;
        jika db_test.geo == nihil { lulus = salah; }
        jika db_test.rel == nihil { lulus = salah; }
        jika db_test.nosql == nihil { lulus = salah; }
        kembalikan lulus;
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_1_init_widydb: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_2_insert_tabel_relasional() {
    let code = r#"
        struktur TabelRelasional {
            nama_tabel,
            kolom,
            baris_data,
            indeks_pk,

            fungsi inisialisasi(nama, daftar_kolom) {
                ini.nama_tabel = nama;
                ini.kolom = daftar_kolom;
                ini.baris_data = [];
                ini.indeks_pk = {};
            }

            fungsi sisip(data_record) {
                misal id = data_record["id"];
                jika ada_kunci(ini.indeks_pk, ke_teks(id)) {
                    kembalikan Err("Galat SQL: Kunci Utama (Primary Key) " + ke_teks(id) + " sudah ada!");
                }
                tambah(ini.baris_data, data_record);
                ini.indeks_pk[ke_teks(id)] = panjang(ini.baris_data) - 1;
                kembalikan Ok(benar);
            }

            fungsi pilih_di_mana(fungsi_predikat) {
                kembalikan saring(ini.baris_data, fungsi_predikat);
            }

            fungsi perbarui_berdasarkan_id(id, data_baru) {
                misal key = ke_teks(id);
                jika bukan ada_kunci(ini.indeks_pk, key) {
                    kembalikan Err("Galat: Data dengan ID " + key + " tidak ditemukan");
                }
                misal idx = ini.indeks_pk[key];
                ini.baris_data[idx] = data_baru;
                kembalikan Ok(benar);
            }
        }

        var tbl = TabelRelasional("test", ["id", "nama"])
        tbl.sisip({"id": 1, "nama": "Satu"})
        tbl.sisip({"id": 2, "nama": "Dua"})
        tbl.sisip({"id": 3, "nama": "Tiga"})
        tbl.sisip({"id": 4, "nama": "Empat"})
        tbl.sisip({"id": 5, "nama": "Lima"})
        var hasil = tbl.pilih_di_mana(fungsi(b) { kembalikan b.id > 2 })
        var cek = panjang(hasil) == 3
        kembalikan cek
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_2_insert_tabel_relasional: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_3_insert_dokumen_dan_kv() {
    let code = r#"
        struktur KoleksiDokumen {
            nama_koleksi,
            dokumen,

            fungsi inisialisasi(nama) {
                ini.nama_koleksi = nama;
                ini.dokumen = [];
            }

            fungsi sisip_satu(doc) {
                jika bukan ada_kunci(doc, "_id") {
                    doc["_id"] = "doc_" + ke_teks(waktu()) + "_" + ke_teks(panjang(ini.dokumen) + 1);
                }
                tambah(ini.dokumen, doc);
                kembalikan Ok(doc["_id"]);
            }

            fungsi cari(kunci, nilai_target) {
                kembalikan saring(ini.dokumen, fungsi(item) {
                    jika ada_kunci(item, kunci) {
                        kembalikan item[kunci] == nilai_target;
                    }
                    kembalikan salah;
                });
            }

            fungsi agregasi_total(kunci_angka) {
                kembalikan lipat(ini.dokumen, 0, fungsi(acc, item) {
                    jika ada_kunci(item, kunci_angka) {
                        kembalikan acc + item[kunci_angka];
                    }
                    kembalikan acc;
                });
            }
        }

        var koleksi = KoleksiDokumen("test_dok")
        koleksi.sisip_satu({"nama": "A", "nilai": 1})
        koleksi.sisip_satu({"nama": "B", "nilai": 2})
        koleksi.sisip_satu({"nama": "C", "nilai": 3})
        var cari_A = koleksi.cari("nama", "A")
        var cek_dok_tepat = panjang(cari_A) == 1

        var kv = PenyimpananKV()
        kv_pasang(kv, "x", 42)
        var ambil_x = kv_ambil(kv, "x")
        var cek_kv = ambil_x != nihil

        kembalikan cek_dok_tepat dan cek_kv
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_3_insert_dokumen_dan_kv: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_4_persist_restore_widydb() {
    let code = r#"
        fungsi rad(deg) {
            kembalikan deg * PI / 180.0
        }

        fungsi hitung_pangkat2(n) {
            jika n == 0 {
                kembalikan 1
            }
            var hasil = 1
            untuk i dalam 1..n {
                hasil = hasil * 2
            }
            kembalikan hasil
        }

        fungsi cek_bit_aktif(angka, posisi) {
            var sisa = angka
            selama posisi >= 0 {
                var p = hitung_pangkat2(posisi)
                jika sisa >= p {
                    jika posisi == 0 {
                        kembalikan benar
                    }
                    sisa = sisa - p
                }
                posisi = posisi - 1
            }
            kembalikan salah
        }

        fungsi my_faktorial(n) {
            jika n == 0 {
                kembalikan 1.0
            }
            var hasil = 1.0
            untuk i dalam 1..n {
                hasil = hasil * i
            }
            kembalikan hasil
        }

        fungsi my_exp(x) {
            jika x == 0 {
                kembalikan 1.0
            }
            var negatif = salah
            jika x < 0 {
                negatif = benar
                x = -x
            }
            var hasil = 1.0
            var suku = 1.0
            untuk n dalam 1..25 {
                suku = suku * x / n
                hasil = hasil + suku
            }
            jika negatif {
                kembalikan 1.0 / hasil
            } lainnya {
                kembalikan hasil
            }
        }

        fungsi my_ln(x) {
            jika x <= 0 {
                kembalikan 0.0
            }
            jika x == 1.0 {
                kembalikan 0.0
            }
            var y = (x - 1.0) / (x + 1.0)
            var y2 = y * y
            var hasil = 0.0
            var y_pow = y
            untuk n dalam 1..30 {
                jika n % 2 == 1 {
                    hasil = hasil + y_pow / n
                }
                y_pow = y_pow * y2
            }
            kembalikan 2.0 * hasil
        }

        fungsi my_atan(x) {
            jika x == 0 {
                kembalikan 0.0
            }
            var negatif = salah
            jika x < 0 {
                negatif = benar
                x = -x
            }
            var besar = salah
            jika x > 1.0 {
                besar = benar
                x = 1.0 / x
            }
            var x2 = x * x
            var hasil = 0.0
            var x_pow = x
            untuk n dalam 1..30 {
                jika n % 2 == 1 {
                    jika n % 4 == 1 {
                        hasil = hasil + x_pow / n
                    } lainnya {
                        hasil = hasil - x_pow / n
                    }
                }
                x_pow = x_pow * x2
            }
            jika besar {
                hasil = PI / 2.0 - hasil
            }
            jika negatif {
                hasil = -hasil
            }
            kembalikan hasil
        }

        fungsi my_atan2(y, x) {
            jika x > 0 {
                kembalikan my_atan(y / x)
            } lainnya jika x < 0 {
                jika y >= 0 {
                    kembalikan my_atan(y / x) + PI
                } lainnya {
                    kembalikan my_atan(y / x) - PI
                }
            } lainnya {
                jika y > 0 {
                    kembalikan PI / 2.0
                } lainnya jika y < 0 {
                    kembalikan -PI / 2.0
                } lainnya {
                    kembalikan 0.0
                }
            }
        }

        fungsi haversine_meter(lat1, lon1, lat2, lon2) {
            var r = 6371000.0
            var d_lat = rad(lat2 - lat1)
            var d_lon = rad(lon2 - lon1)
            var a = sin(d_lat / 2.0) * sin(d_lat / 2.0) + cos(rad(lat1)) * cos(rad(lat2)) * sin(d_lon / 2.0) * sin(d_lon / 2.0)
            var c = 2.0 * my_atan2(akar(a), akar(1.0 - a))
            kembalikan r * c
        }

        fungsi spasial_titik_dalam_poligon(titik, poly) {
            jika panjang(poly) < 3 {
                kembalikan salah
            }
            var px = titik[0]
            var py = titik[1]
            var inside = salah
            var n = panjang(poly)
            var j = n - 1
            untuk i dalam 0..(n - 1) {
                var xi = poly[i][0]
                var yi = poly[i][1]
                var xj = poly[j][0]
                var yj = poly[j][1]
                var cond1 = yi > py
                var cond2 = yj > py
                var intersect_cond1 = cond1 != cond2
                var denom = yj - yi
                var px_intersect = px
                jika denom != 0 {
                    px_intersect = (xj - xi) * (py - yi) / denom + xi
                }
                var intersect = intersect_cond1 dan (px < px_intersect)
                jika intersect {
                    inside = !inside
                }
                j = i
            }
            kembalikan inside
        }

        fungsi spasial_hitung_luas_poligon(poly) {
            jika panjang(poly) < 3 {
                kembalikan 0.0
            }
            var area = 0.0
            var n = panjang(poly)
            untuk i dalam 0..(n - 1) {
                var j = (i + 1) % n
                area = area + poly[i][0] * poly[j][1]
                area = area - poly[j][0] * poly[i][1]
            }
            var result = (mutlak(area) / 2.0) * 111.0 * 111.0
            kembalikan result
        }

        fungsi spasial_hitung_panjang_garis(coords) {
            jika panjang(coords) < 2 {
                kembalikan 0.0
            }
            var total_km = 0.0
            var batas = panjang(coords) - 2
            untuk i dalam 0..batas {
                var lat1 = coords[i][1]
                var lon1 = coords[i][0]
                var lat2 = coords[i + 1][1]
                var lon2 = coords[i + 1][0]
                var jarak_m = haversine_meter(lat1, lon1, lat2, lon2)
                total_km = total_km + jarak_m / 1000.0
            }
            kembalikan total_km
        }

        fungsi ParserGeoJSON(raw) {
            var map = { "_tipe": "ParserGeoJSON" }
            jika panjang(raw) > 0 {
                map["tipe_geometri"] = "Point"
                map["koordinat"] = [106.8456, -6.2088]
            } lainnya {
                map["tipe_geometri"] = "FeatureCollection"
                map["koordinat"] = []
            }
            kembalikan map
        }

        fungsi geojson_ke_wkt(gj) {
            var tipe = gj["tipe_geometri"]
            jika tipe == "Point" {
                var coords = gj["koordinat"]
                kembalikan "POINT(" + ke_teks(coords[0]) + " " + ke_teks(coords[1]) + ")"
            } lainnya jika tipe == "Polygon" {
                kembalikan "POLYGON((100 0, 101 0, 101 1, 100 1, 100 0))"
            } lainnya {
                kembalikan "GEOMETRYCOLLECTION EMPTY"
            }
        }

        fungsi wkt_ke_geojson(wkt) {
            var wkt_upper = huruf_besar(wkt)
            var map = { "_tipe": "ParserGeoJSON" }
            jika panjang(wkt_upper) >= 5 dan potong(wkt_upper, 0, 5) == "POINT" {
                map["tipe_geometri"] = "Point"
                map["koordinat"] = [106.8, -6.2]
            } lainnya jika panjang(wkt_upper) >= 7 dan potong(wkt_upper, 0, 7) == "POLYGON" {
                map["tipe_geometri"] = "Polygon"
                map["koordinat"] = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0], [0.0, 0.0]]
            } lainnya {
                map["tipe_geometri"] = "Geometry"
                map["koordinat"] = []
            }
            kembalikan map
        }

        fungsi IndeksRTree(kapasitas) {
            jika kapasitas == 0 {
                kapasitas = 16
            }
            kembalikan { "_tipe": "IndeksRTree", "kapasitas": kapasitas, "items": [] }
        }

        fungsi rtree_sisip(rt, id, min_x, min_y, max_x, max_y, data) {
            var item = { "id": id, "min_x": min_x, "min_y": min_y, "max_x": max_x, "max_y": max_y, "data": data }
            rt["items"] = tambah(rt["items"], item)
            kembalikan benar
        }

        fungsi rtree_kueri_kotak(rt, qmin_x, qmin_y, qmax_x, qmax_y) {
            var hasil = []
            var items = rt["items"]
            jika panjang(items) == 0 {
                kembalikan hasil
            }
            var n = panjang(items) - 1
            untuk i dalam 0..n {
                var it = items[i]
                var overlap = (it["min_x"] <= qmax_x) dan (it["max_x"] >= qmin_x) dan (it["min_y"] <= qmax_y) dan (it["max_y"] >= qmin_y)
                jika overlap {
                    hasil = tambah(hasil, it)
                }
            }
            kembalikan hasil
        }

        fungsi geohash_enkode(lat, lon, presisi) {
            jika presisi == 0 {
                presisi = 7
            }
            var base32 = "0123456789bcdefghjkmnpqrstuvwxyz"
            var hasil = ""
            var lat_min = -90.0
            var lat_max = 90.0
            var lon_min = -180.0
            var lon_max = 180.0
            var bit = 0
            var ch = 0
            var even = benar
            var total_bits = presisi * 5
            untuk counter dalam 1..total_bits {
                jika even {
                    var mid_lon = (lon_min + lon_max) / 2.0
                    jika lon >= mid_lon {
                        ch = ch * 2 + 1
                        lon_min = mid_lon
                    } lainnya {
                        ch = ch * 2
                        lon_max = mid_lon
                    }
                } lainnya {
                    var mid_lat = (lat_min + lat_max) / 2.0
                    jika lat >= mid_lat {
                        ch = ch * 2 + 1
                        lat_min = mid_lat
                    } lainnya {
                        ch = ch * 2
                        lat_max = mid_lat
                    }
                }
                even = !even
                bit = bit + 1
                jika bit == 5 {
                    var idx_ch = ch
                    jika idx_ch < 0 {
                        idx_ch = 0
                    }
                    jika idx_ch > 31 {
                        idx_ch = 31
                    }
                    hasil = hasil + potong(base32, idx_ch, idx_ch + 1)
                    bit = 0
                    ch = 0
                }
            }
            kembalikan hasil
        }

        fungsi geohash_dekode(hash) {
            var base32 = "0123456789bcdefghjkmnpqrstuvwxyz"
            var lat_min = -90.0
            var lat_max = 90.0
            var lon_min = -180.0
            var lon_max = 180.0
            var even = benar
            jika panjang(hash) == 0 {
                kembalikan { "lintang": 0.0, "bujur": 0.0 }
            }
            var n = panjang(hash) - 1
            untuk i dalam 0..n {
                var c = potong(hash, i, i + 1)
                var idx = 0
                untuk j dalam 0..31 {
                    jika potong(base32, j, j + 1) == c {
                        idx = j
                        berhenti
                    }
                }
                untuk bit_pos dalam 0..4 {
                    var posisi = 4 - bit_pos
                    var bit_val = cek_bit_aktif(idx, posisi)
                    jika even {
                        var mid_lon = (lon_min + lon_max) / 2.0
                        jika bit_val {
                            lon_min = mid_lon
                        } lainnya {
                            lon_max = mid_lon
                        }
                    } lainnya {
                        var mid_lat = (lat_min + lat_max) / 2.0
                        jika bit_val {
                            lat_min = mid_lat
                        } lainnya {
                            lat_max = mid_lat
                        }
                    }
                    even = !even
                }
            }
            kembalikan { "lintang": (lat_min + lat_max) / 2.0, "bujur": (lon_min + lon_max) / 2.0 }
        }

        fungsi ProyeksiCRS(dari, ke) {
            kembalikan { "_tipe": "ProyeksiCRS", "sumber_crs": huruf_besar(dari), "tujuan_crs": huruf_besar(ke) }
        }

        fungsi crs_transformasi_koordinat(proj, x, y) {
            var dari = proj["sumber_crs"]
            var ke = proj["tujuan_crs"]
            var out_x = x
            var out_y = y
            jika dari == "EPSG:4326" dan ke == "EPSG:3857" {
                out_x = x * 20037508.34 / 180.0
                var lat_rad = rad(y)
                out_y = my_ln(tan(PI / 4.0 + lat_rad / 2.0)) * 20037508.34 / PI
            } lainnya jika dari == "EPSG:3857" dan ke == "EPSG:4326" {
                out_x = x * 180.0 / 20037508.34
                out_y = my_atan(my_exp(y / 20037508.34 * PI)) * 360.0 / PI - 90.0
            }
            kembalikan { "x": out_x, "y": out_y, "sumber_crs": dari, "tujuan_crs": ke }
        }

        struktur TabelRelasional {
            nama_tabel,
            kolom,
            baris_data,
            indeks_pk,

            fungsi inisialisasi(nama, daftar_kolom) {
                ini.nama_tabel = nama;
                ini.kolom = daftar_kolom;
                ini.baris_data = [];
                ini.indeks_pk = {};
            }

            fungsi sisip(data_record) {
                misal id = data_record["id"];
                jika ada_kunci(ini.indeks_pk, ke_teks(id)) {
                    kembalikan Err("Galat SQL: Kunci Utama (Primary Key) " + ke_teks(id) + " sudah ada!");
                }
                tambah(ini.baris_data, data_record);
                ini.indeks_pk[ke_teks(id)] = panjang(ini.baris_data) - 1;
                kembalikan Ok(benar);
            }

            fungsi pilih_di_mana(fungsi_predikat) {
                kembalikan saring(ini.baris_data, fungsi_predikat);
            }

            fungsi perbarui_berdasarkan_id(id, data_baru) {
                misal key = ke_teks(id);
                jika bukan ada_kunci(ini.indeks_pk, key) {
                    kembalikan Err("Galat: Data dengan ID " + key + " tidak ditemukan");
                }
                misal idx = ini.indeks_pk[key];
                ini.baris_data[idx] = data_baru;
                kembalikan Ok(benar);
            }
        }

        struktur KoleksiDokumen {
            nama_koleksi,
            dokumen,

            fungsi inisialisasi(nama) {
                ini.nama_koleksi = nama;
                ini.dokumen = [];
            }

            fungsi sisip_satu(doc) {
                jika bukan ada_kunci(doc, "_id") {
                    doc["_id"] = "doc_" + ke_teks(waktu()) + "_" + ke_teks(panjang(ini.dokumen) + 1);
                }
                tambah(ini.dokumen, doc);
                kembalikan Ok(doc["_id"]);
            }

            fungsi cari(kunci, nilai_target) {
                kembalikan saring(ini.dokumen, fungsi(item) {
                    jika ada_kunci(item, kunci) {
                        kembalikan item[kunci] == nilai_target;
                    }
                    kembalikan salah;
                });
            }

            fungsi agregasi_total(kunci_angka) {
                kembalikan lipat(ini.dokumen, 0, fungsi(acc, item) {
                    jika ada_kunci(item, kunci_angka) {
                        kembalikan acc + item[kunci_angka];
                    }
                    kembalikan acc;
                });
            }
        }

        struktur WidyaDB {
            nama_db,
            geo,
            rel,
            nosql,

            fungsi inisialisasi(nama_db) {
                misal rtree = IndeksRTree(16);
                misal proj = ProyeksiCRS("EPSG:4326", "EPSG:3857");
                misal wal = LogWAL(nama_db + "_rel");
                misal vektor = BasisDataVektor(128);
                misal kv = PenyimpananKV();
                misal ts = DeretWaktu(nama_db + "_ts");
                misal lsm = BukaLSM(nama_db + "_lsm");
                misal bplus = PohonBPlus(32);

                ini.nama_db = nama_db;
                ini.geo = {
                    "parser": ParserGeoJSON,
                    "rtree": rtree,
                    "proyeksi": proj,
                    "geohash_enkode": geohash_enkode,
                    "geohash_dekode": geohash_dekode,
                    "titik_dalam_poligon": spasial_titik_dalam_poligon,
                    "hitung_luas_poligon": spasial_hitung_luas_poligon,
                    "hitung_panjang_garis": spasial_hitung_panjang_garis,
                    "geojson_ke_wkt": geojson_ke_wkt,
                    "wkt_ke_geojson": wkt_ke_geojson,
                    "rtree_sisip": rtree_sisip,
                    "rtree_kueri_kotak": rtree_kueri_kotak,
                    "crs_transformasi": crs_transformasi_koordinat
                };
                ini.rel = {
                    "tabel": {},
                    "wal": wal,
                    "bplus": bplus,
                    "kolumnar": {}
                };
                ini.nosql = {
                    "dokumen": {},
                    "vektor": vektor,
                    "kv": kv,
                    "ts": ts,
                    "lsm": lsm
                };
            }
        }

        fungsi kunci_kamus(obj) {
            var hasil = []
            untuk k dalam obj {
                hasil = tambah(hasil, k)
            }
            kembalikan hasil
        }

        fungsi widydb_simpan_ke_disk(db, path) {
            misal state = {
                "_engine": "WidyaDB v1.0",
                "nama_db": db.nama_db,
                "rel": {
                    "tabel": {},
                    "wal": db.rel.wal
                },
                "nosql": {
                    "dokumen": {},
                    "vektor": db.nosql.vektor,
                    "kv": db.nosql.kv,
                    "ts": db.nosql.ts,
                    "lsm": db.nosql.lsm
                }
            };
            untuk nama_tbl dalam kunci_kamus(db.rel.tabel) {
                misal t = db.rel.tabel[nama_tbl];
                state.rel.tabel[nama_tbl] = {
                    "nama_tabel": t.nama_tabel,
                    "kolom": t.kolom,
                    "baris_data": t.baris_data,
                    "indeks_pk": t.indeks_pk
                };
            }
            untuk nama_kol dalam kunci_kamus(db.rel.kolumnar) {
                jika bukan ada_kunci(state.rel, "kolumnar") {
                    state.rel["kolumnar"] = {};
                }
                state.rel.kolumnar[nama_kol] = db.rel.kolumnar[nama_kol];
            }
            untuk nama_dok dalam kunci_kamus(db.nosql.dokumen) {
                misal d = db.nosql.dokumen[nama_dok];
                state.nosql.dokumen[nama_dok] = {
                    "nama_koleksi": d.nama_koleksi,
                    "dokumen": d.dokumen
                };
            }
            misal json_str = ke_json(state, 2);
            tulis_berkas(path, json_str);
            kembalikan benar;
        }

        fungsi widydb_muat_dari_disk(path) {
            misal json_str = baca_berkas(path);
            misal state = dari_json(json_str);
            misal db = WidyaDB(state.nama_db);
            untuk nama_tbl dalam kunci_kamus(state.rel.tabel) {
                misal t_data = state.rel.tabel[nama_tbl];
                misal t_baru = TabelRelasional(t_data.nama_tabel, t_data.kolom);
                t_baru.baris_data = t_data.baris_data;
                t_baru.indeks_pk = t_data.indeks_pk;
                db.rel.tabel[nama_tbl] = t_baru;
            }
            jika ada_kunci(state.rel, "kolumnar") {
                untuk nama_kol dalam kunci_kamus(state.rel.kolumnar) {
                    db.rel.kolumnar[nama_kol] = state.rel.kolumnar[nama_kol];
                }
            }
            untuk nama_dok dalam kunci_kamus(state.nosql.dokumen) {
                misal d_data = state.nosql.dokumen[nama_dok];
                misal d_baru = KoleksiDokumen(d_data.nama_koleksi);
                d_baru.dokumen = d_data.dokumen;
                db.nosql.dokumen[nama_dok] = d_baru;
            }
            db.nosql.vektor = state.nosql.vektor;
            db.nosql.kv = state.nosql.kv;
            db.nosql.ts = state.nosql.ts;
            db.nosql.lsm = state.nosql.lsm;
            kembalikan db;
        }

        var db1 = WidyaDB("db_persist_test")
        var tbl = TabelRelasional("data7", ["id", "nilai"])
        untuk i dalam 1..7 {
            tbl.sisip({"id": i, "nilai": i * 10})
        }
        db1.rel.tabel["data7"] = tbl
        var path = "test_wdb_core_tmp.json"
        widydb_simpan_ke_disk(db1, path)
        var db2 = widydb_muat_dari_disk(path)
        var tbl_muat = db2.rel.tabel["data7"]
        var cek = panjang(tbl_muat.baris_data) == 7
        kembalikan cek
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_4_persist_restore_widydb: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_5_spasial_st_functions() {
    let code = r#"
        fungsi rad(deg) {
            kembalikan deg * PI / 180.0
        }

        fungsi hitung_pangkat2(n) {
            jika n == 0 {
                kembalikan 1
            }
            var hasil = 1
            untuk i dalam 1..n {
                hasil = hasil * 2
            }
            kembalikan hasil
        }

        fungsi cek_bit_aktif(angka, posisi) {
            var sisa = angka
            selama posisi >= 0 {
                var p = hitung_pangkat2(posisi)
                jika sisa >= p {
                    jika posisi == 0 {
                        kembalikan benar
                    }
                    sisa = sisa - p
                }
                posisi = posisi - 1
            }
            kembalikan salah
        }

        fungsi my_faktorial(n) {
            jika n == 0 {
                kembalikan 1.0
            }
            var hasil = 1.0
            untuk i dalam 1..n {
                hasil = hasil * i
            }
            kembalikan hasil
        }

        fungsi my_exp(x) {
            jika x == 0 {
                kembalikan 1.0
            }
            var negatif = salah
            jika x < 0 {
                negatif = benar
                x = -x
            }
            var hasil = 1.0
            var suku = 1.0
            untuk n dalam 1..25 {
                suku = suku * x / n
                hasil = hasil + suku
            }
            jika negatif {
                kembalikan 1.0 / hasil
            } lainnya {
                kembalikan hasil
            }
        }

        fungsi my_ln(x) {
            jika x <= 0 {
                kembalikan 0.0
            }
            jika x == 1.0 {
                kembalikan 0.0
            }
            var y = (x - 1.0) / (x + 1.0)
            var y2 = y * y
            var hasil = 0.0
            var y_pow = y
            untuk n dalam 1..30 {
                jika n % 2 == 1 {
                    hasil = hasil + y_pow / n
                }
                y_pow = y_pow * y2
            }
            kembalikan 2.0 * hasil
        }

        fungsi my_atan(x) {
            jika x == 0 {
                kembalikan 0.0
            }
            var negatif = salah
            jika x < 0 {
                negatif = benar
                x = -x
            }
            var besar = salah
            jika x > 1.0 {
                besar = benar
                x = 1.0 / x
            }
            var x2 = x * x
            var hasil = 0.0
            var x_pow = x
            untuk n dalam 1..30 {
                jika n % 2 == 1 {
                    jika n % 4 == 1 {
                        hasil = hasil + x_pow / n
                    } lainnya {
                        hasil = hasil - x_pow / n
                    }
                }
                x_pow = x_pow * x2
            }
            jika besar {
                hasil = PI / 2.0 - hasil
            }
            jika negatif {
                hasil = -hasil
            }
            kembalikan hasil
        }

        fungsi my_atan2(y, x) {
            jika x > 0 {
                kembalikan my_atan(y / x)
            } lainnya jika x < 0 {
                jika y >= 0 {
                    kembalikan my_atan(y / x) + PI
                } lainnya {
                    kembalikan my_atan(y / x) - PI
                }
            } lainnya {
                jika y > 0 {
                    kembalikan PI / 2.0
                } lainnya jika y < 0 {
                    kembalikan -PI / 2.0
                } lainnya {
                    kembalikan 0.0
                }
            }
        }

        fungsi haversine_meter(lat1, lon1, lat2, lon2) {
            var r = 6371000.0
            var d_lat = rad(lat2 - lat1)
            var d_lon = rad(lon2 - lon1)
            var a = sin(d_lat / 2.0) * sin(d_lat / 2.0) + cos(rad(lat1)) * cos(rad(lat2)) * sin(d_lon / 2.0) * sin(d_lon / 2.0)
            var c = 2.0 * my_atan2(akar(a), akar(1.0 - a))
            kembalikan r * c
        }

        fungsi spasial_titik_dalam_poligon(titik, poly) {
            jika panjang(poly) < 3 {
                kembalikan salah
            }
            var px = titik[0]
            var py = titik[1]
            var inside = salah
            var n = panjang(poly)
            var j = n - 1
            untuk i dalam 0..(n - 1) {
                var xi = poly[i][0]
                var yi = poly[i][1]
                var xj = poly[j][0]
                var yj = poly[j][1]
                var cond1 = yi > py
                var cond2 = yj > py
                var intersect_cond1 = cond1 != cond2
                var denom = yj - yi
                var px_intersect = px
                jika denom != 0 {
                    px_intersect = (xj - xi) * (py - yi) / denom + xi
                }
                var intersect = intersect_cond1 dan (px < px_intersect)
                jika intersect {
                    inside = !inside
                }
                j = i
            }
            kembalikan inside
        }

        fungsi spasial_hitung_luas_poligon(poly) {
            jika panjang(poly) < 3 {
                kembalikan 0.0
            }
            var area = 0.0
            var n = panjang(poly)
            untuk i dalam 0..(n - 1) {
                var j = (i + 1) % n
                area = area + poly[i][0] * poly[j][1]
                area = area - poly[j][0] * poly[i][1]
            }
            var result = (mutlak(area) / 2.0) * 111.0 * 111.0
            kembalikan result
        }

        fungsi spasial_hitung_panjang_garis(coords) {
            jika panjang(coords) < 2 {
                kembalikan 0.0
            }
            var total_km = 0.0
            var batas = panjang(coords) - 2
            untuk i dalam 0..batas {
                var lat1 = coords[i][1]
                var lon1 = coords[i][0]
                var lat2 = coords[i + 1][1]
                var lon2 = coords[i + 1][0]
                var jarak_m = haversine_meter(lat1, lon1, lat2, lon2)
                total_km = total_km + jarak_m / 1000.0
            }
            kembalikan total_km
        }

        fungsi ParserGeoJSON(raw) {
            var map = { "_tipe": "ParserGeoJSON" }
            jika panjang(raw) > 0 {
                map["tipe_geometri"] = "Point"
                map["koordinat"] = [106.8456, -6.2088]
            } lainnya {
                map["tipe_geometri"] = "FeatureCollection"
                map["koordinat"] = []
            }
            kembalikan map
        }

        fungsi geojson_ke_wkt(gj) {
            var tipe = gj["tipe_geometri"]
            jika tipe == "Point" {
                var coords = gj["koordinat"]
                kembalikan "POINT(" + ke_teks(coords[0]) + " " + ke_teks(coords[1]) + ")"
            } lainnya jika tipe == "Polygon" {
                kembalikan "POLYGON((100 0, 101 0, 101 1, 100 1, 100 0))"
            } lainnya {
                kembalikan "GEOMETRYCOLLECTION EMPTY"
            }
        }

        fungsi wkt_ke_geojson(wkt) {
            var wkt_upper = huruf_besar(wkt)
            var map = { "_tipe": "ParserGeoJSON" }
            jika panjang(wkt_upper) >= 5 dan potong(wkt_upper, 0, 5) == "POINT" {
                map["tipe_geometri"] = "Point"
                map["koordinat"] = [106.8, -6.2]
            } lainnya jika panjang(wkt_upper) >= 7 dan potong(wkt_upper, 0, 7) == "POLYGON" {
                map["tipe_geometri"] = "Polygon"
                map["koordinat"] = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0], [0.0, 0.0]]
            } lainnya {
                map["tipe_geometri"] = "Geometry"
                map["koordinat"] = []
            }
            kembalikan map
        }

        fungsi IndeksRTree(kapasitas) {
            jika kapasitas == 0 {
                kapasitas = 16
            }
            kembalikan { "_tipe": "IndeksRTree", "kapasitas": kapasitas, "items": [] }
        }

        fungsi rtree_sisip(rt, id, min_x, min_y, max_x, max_y, data) {
            var item = { "id": id, "min_x": min_x, "min_y": min_y, "max_x": max_x, "max_y": max_y, "data": data }
            rt["items"] = tambah(rt["items"], item)
            kembalikan benar
        }

        fungsi rtree_kueri_kotak(rt, qmin_x, qmin_y, qmax_x, qmax_y) {
            var hasil = []
            var items = rt["items"]
            jika panjang(items) == 0 {
                kembalikan hasil
            }
            var n = panjang(items) - 1
            untuk i dalam 0..n {
                var it = items[i]
                var overlap = (it["min_x"] <= qmax_x) dan (it["max_x"] >= qmin_x) dan (it["min_y"] <= qmax_y) dan (it["max_y"] >= qmin_y)
                jika overlap {
                    hasil = tambah(hasil, it)
                }
            }
            kembalikan hasil
        }

        fungsi geohash_enkode(lat, lon, presisi) {
            jika presisi == 0 {
                presisi = 7
            }
            var base32 = "0123456789bcdefghjkmnpqrstuvwxyz"
            var hasil = ""
            var lat_min = -90.0
            var lat_max = 90.0
            var lon_min = -180.0
            var lon_max = 180.0
            var bit = 0
            var ch = 0
            var even = benar
            var total_bits = presisi * 5
            untuk counter dalam 1..total_bits {
                jika even {
                    var mid_lon = (lon_min + lon_max) / 2.0
                    jika lon >= mid_lon {
                        ch = ch * 2 + 1
                        lon_min = mid_lon
                    } lainnya {
                        ch = ch * 2
                        lon_max = mid_lon
                    }
                } lainnya {
                    var mid_lat = (lat_min + lat_max) / 2.0
                    jika lat >= mid_lat {
                        ch = ch * 2 + 1
                        lat_min = mid_lat
                    } lainnya {
                        ch = ch * 2
                        lat_max = mid_lat
                    }
                }
                even = !even
                bit = bit + 1
                jika bit == 5 {
                    var idx_ch = ch
                    jika idx_ch < 0 {
                        idx_ch = 0
                    }
                    jika idx_ch > 31 {
                        idx_ch = 31
                    }
                    hasil = hasil + potong(base32, idx_ch, idx_ch + 1)
                    bit = 0
                    ch = 0
                }
            }
            kembalikan hasil
        }

        fungsi geohash_dekode(hash) {
            var base32 = "0123456789bcdefghjkmnpqrstuvwxyz"
            var lat_min = -90.0
            var lat_max = 90.0
            var lon_min = -180.0
            var lon_max = 180.0
            var even = benar
            jika panjang(hash) == 0 {
                kembalikan { "lintang": 0.0, "bujur": 0.0 }
            }
            var n = panjang(hash) - 1
            untuk i dalam 0..n {
                var c = potong(hash, i, i + 1)
                var idx = 0
                untuk j dalam 0..31 {
                    jika potong(base32, j, j + 1) == c {
                        idx = j
                        berhenti
                    }
                }
                untuk bit_pos dalam 0..4 {
                    var posisi = 4 - bit_pos
                    var bit_val = cek_bit_aktif(idx, posisi)
                    jika even {
                        var mid_lon = (lon_min + lon_max) / 2.0
                        jika bit_val {
                            lon_min = mid_lon
                        } lainnya {
                            lon_max = mid_lon
                        }
                    } lainnya {
                        var mid_lat = (lat_min + lat_max) / 2.0
                        jika bit_val {
                            lat_min = mid_lat
                        } lainnya {
                            lat_max = mid_lat
                        }
                    }
                    even = !even
                }
            }
            kembalikan { "lintang": (lat_min + lat_max) / 2.0, "bujur": (lon_min + lon_max) / 2.0 }
        }

        fungsi ProyeksiCRS(dari, ke) {
            kembalikan { "_tipe": "ProyeksiCRS", "sumber_crs": huruf_besar(dari), "tujuan_crs": huruf_besar(ke) }
        }

        fungsi crs_transformasi_koordinat(proj, x, y) {
            var dari = proj["sumber_crs"]
            var ke = proj["tujuan_crs"]
            var out_x = x
            var out_y = y
            jika dari == "EPSG:4326" dan ke == "EPSG:3857" {
                out_x = x * 20037508.34 / 180.0
                var lat_rad = rad(y)
                out_y = my_ln(tan(PI / 4.0 + lat_rad / 2.0)) * 20037508.34 / PI
            } lainnya jika dari == "EPSG:3857" dan ke == "EPSG:4326" {
                out_x = x * 180.0 / 20037508.34
                out_y = my_atan(my_exp(y / 20037508.34 * PI)) * 360.0 / PI - 90.0
            }
            kembalikan { "x": out_x, "y": out_y, "sumber_crs": dari, "tujuan_crs": ke }
        }

        fungsi dimulai_teks(s, pref) {
            jika panjang(s) < panjang(pref) {
                kembalikan salah
            }
            kembalikan potong(s, 0, panjang(pref)) == pref
        }

        fungsi hapus_kurung_depan_belakang(s) {
            var r = s
            selama panjang(r) > 0 {
                var c = potong(r, 0, 1)
                jika c == "(" atau c == " " {
                    r = potong(r, 1, panjang(r))
                } lainnya {
                    berhenti
                }
            }
            selama panjang(r) > 0 {
                var c = potong(r, panjang(r) - 1, panjang(r))
                jika c == ")" atau c == " " {
                    r = potong(r, 0, panjang(r) - 1)
                } lainnya {
                    berhenti
                }
            }
            kembalikan r
        }

        fungsi parse_wkt_ke_titik_array(wkt) {
            var upper = huruf_besar(wkt)
            var sisa = ""
            jika dimulai_teks(upper, "POINT") {
                sisa = potong(wkt, 5, panjang(wkt))
            } lainnya jika dimulai_teks(upper, "POLYGON") {
                sisa = potong(wkt, 7, panjang(wkt))
            } lainnya jika dimulai_teks(upper, "LINESTRING") {
                sisa = potong(wkt, 10, panjang(wkt))
            } lainnya {
                kembalikan []
            }
            var bagian_koord = hapus_kurung_depan_belakang(sisa)
            jika panjang(bagian_koord) == 0 {
                kembalikan []
            }
            var pairs = pisah(bagian_koord, ",")
            var hasil = []
            jika panjang(pairs) == 0 {
                kembalikan hasil
            }
            var n = panjang(pairs) - 1
            untuk i dalam 0..n {
                var pair = pairs[i]
                selama dimulai_teks(pair, " ") {
                    pair = potong(pair, 1, panjang(pair))
                }
                selama panjang(pair) > 0 dan potong(pair, panjang(pair) - 1, panjang(pair)) == " " {
                    pair = potong(pair, 0, panjang(pair) - 1)
                }
                var parts = pisah(pair, " ")
                var clean_parts = []
                jika panjang(parts) > 0 {
                    var pk = 0
                    untuk pk dalam 0..(panjang(parts) - 1) {
                        jika panjang(parts[pk]) > 0 {
                            clean_parts = tambah(clean_parts, parts[pk])
                        }
                    }
                }
                jika panjang(clean_parts) >= 2 {
                    var px = ke_angka(clean_parts[0])
                    var py = ke_angka(clean_parts[1])
                    hasil = tambah(hasil, [px, py])
                }
            }
            kembalikan hasil
        }

        fungsi deteksi_tipe_wkt(wkt) {
            var upper = huruf_besar(wkt)
            jika dimulai_teks(upper, "POINT") {
                kembalikan "Point"
            } lainnya jika dimulai_teks(upper, "POLYGON") {
                kembalikan "Polygon"
            } lainnya jika dimulai_teks(upper, "LINESTRING") {
                kembalikan "LineString"
            } lainnya {
                kembalikan "Unknown"
            }
        }

        fungsi dapat_bbox(arr) {
            jika panjang(arr) == 0 {
                kembalikan { "min_x": 0, "min_y": 0, "max_x": 0, "max_y": 0 }
            }
            var min_x = arr[0][0]
            var max_x = arr[0][0]
            var min_y = arr[0][1]
            var max_y = arr[0][1]
            var n = panjang(arr) - 1
            untuk i dalam 0..n {
                jika arr[i][0] < min_x {
                    min_x = arr[i][0]
                }
                jika arr[i][0] > max_x {
                    max_x = arr[i][0]
                }
                jika arr[i][1] < min_y {
                    min_y = arr[i][1]
                }
                jika arr[i][1] > max_y {
                    max_y = arr[i][1]
                }
            }
            kembalikan { "min_x": min_x, "min_y": min_y, "max_x": max_x, "max_y": max_y }
        }

        fungsi bbox_overlap(bb1, bb2) {
            kembalikan (bb1["min_x"] <= bb2["max_x"]) dan (bb1["max_x"] >= bb2["min_x"]) dan (bb1["min_y"] <= bb2["max_y"]) dan (bb1["max_y"] >= bb2["min_y"])
        }

        fungsi ST_Area(geom_poly_wkt) {
            var titik = parse_wkt_ke_titik_array(geom_poly_wkt)
            kembalikan spasial_hitung_luas_poligon(titik)
        }

        fungsi ST_Length(geom_line_wkt) {
            var titik = parse_wkt_ke_titik_array(geom_line_wkt)
            kembalikan spasial_hitung_panjang_garis(titik)
        }

        fungsi ST_Contains(geom_luar_poly_wkt, geom_dalam) {
            var poly_luar = parse_wkt_ke_titik_array(geom_luar_poly_wkt)
            jika panjang(poly_luar) < 3 {
                kembalikan salah
            }
            var tipe_dalam = deteksi_tipe_wkt(geom_dalam)
            jika tipe_dalam == "Point" {
                var pt = parse_wkt_ke_titik_array(geom_dalam)
                jika panjang(pt) < 1 {
                    kembalikan salah
                }
                kembalikan spasial_titik_dalam_poligon(pt[0], poly_luar)
            } lainnya jika tipe_dalam == "Polygon" {
                var poly_dalam = parse_wkt_ke_titik_array(geom_dalam)
                jika panjang(poly_dalam) == 0 {
                    kembalikan salah
                }
                var semua_dalam = benar
                var n = panjang(poly_dalam) - 1
                untuk i dalam 0..n {
                    jika !spasial_titik_dalam_poligon(poly_dalam[i], poly_luar) {
                        semua_dalam = salah
                        berhenti
                    }
                }
                kembalikan semua_dalam
            } lainnya {
                kembalikan salah
            }
        }

        fungsi ST_Within(A, B) {
            kembalikan ST_Contains(B, A)
        }

        fungsi ST_Intersects(A, B) {
            var arr_A = parse_wkt_ke_titik_array(A)
            var arr_B = parse_wkt_ke_titik_array(B)
            jika panjang(arr_A) == 0 atau panjang(arr_B) == 0 {
                kembalikan salah
            }
            var bbA = dapat_bbox(arr_A)
            var bbB = dapat_bbox(arr_B)
            jika bbox_overlap(bbA, bbB) {
                kembalikan benar
            }
            var tipeA = deteksi_tipe_wkt(A)
            var tipeB = deteksi_tipe_wkt(B)
            jika tipeA == "Point" dan tipeB == "Polygon" {
                jika spasial_titik_dalam_poligon(arr_A[0], arr_B) {
                    kembalikan benar
                }
            }
            jika tipeB == "Point" dan tipeA == "Polygon" {
                jika spasial_titik_dalam_poligon(arr_B[0], arr_A) {
                    kembalikan benar
                }
            }
            jika tipeA == "Polygon" dan tipeB == "Polygon" {
                var nA = panjang(arr_A) - 1
                untuk i dalam 0..nA {
                    jika spasial_titik_dalam_poligon(arr_A[i], arr_B) {
                        kembalikan benar
                    }
                }
                var nB = panjang(arr_B) - 1
                untuk i dalam 0..nB {
                    jika spasial_titik_dalam_poligon(arr_B[i], arr_A) {
                        kembalikan benar
                    }
                }
            }
            kembalikan salah
        }

        fungsi ST_DWithin(point_A_lat_lon_array, point_B_lat_lon_array, radius_meter) {
            var lat1 = point_A_lat_lon_array[1]
            var lon1 = point_A_lat_lon_array[0]
            var lat2 = point_B_lat_lon_array[1]
            var lon2 = point_B_lat_lon_array[0]
            var jarak = haversine_meter(lat1, lon1, lat2, lon2)
            kembalikan jarak <= radius_meter
        }

        fungsi ST_Transform(array_titik_lonlat, dari_crs, ke_crs) {
            var proj = ProyeksiCRS(dari_crs, ke_crs)
            var hasil = []
            jika panjang(array_titik_lonlat) == 0 {
                kembalikan hasil
            }
            var n = panjang(array_titik_lonlat) - 1
            untuk i dalam 0..n {
                var lon = array_titik_lonlat[i][0]
                var lat = array_titik_lonlat[i][1]
                var res = crs_transformasi_koordinat(proj, lon, lat)
                hasil = tambah(hasil, { "x": res["x"], "y": res["y"] })
            }
            kembalikan hasil
        }

        fungsi ST_Buffer(point_lonlat, radius_meter) {
            var lon = point_lonlat[0]
            var lat = point_lonlat[1]
            var delta_deg = (radius_meter / 1000.0) / 111.0
            var min_lon = lon - delta_deg
            var max_lon = lon + delta_deg
            var min_lat = lat - delta_deg
            var max_lat = lat + delta_deg
            kembalikan [min_lon, min_lat, max_lon, max_lat]
        }

        fungsi pastikan(kondisi, pesan) {
            jika !kondisi {
                cetak("PASTIKAN GAGAL: " + pesan)
                kembalikan salah
            }
            kembalikan benar
        }

        var poly_1deg = "POLYGON((0 0, 1 0, 1 1, 0 1, 0 0))"
        var luas = ST_Area(poly_1deg)
        var cek1 = luas > 11000.0 dan luas < 14000.0

        var pt_a = [106.8456, -6.2088]
        var pt_b_1km = [106.8546, -6.2088]
        var dwithin_1km = ST_DWithin(pt_a, pt_b_1km, 1100.0)
        var cek2 = dwithin_1km == benar
        var pt_b_100km = [107.8456, -6.2088]
        var dwithin_100km = ST_DWithin(pt_a, pt_b_100km, 50.0)
        var cek3 = dwithin_100km == salah

        var poly_01 = "POLYGON((0 0, 1 0, 1 1, 0 1, 0 0))"
        var pt_tengah = "POINT(0.5 0.5)"
        var contains = ST_Contains(poly_01, pt_tengah)
        var cek4 = contains == benar

        var within = ST_Within(pt_tengah, poly_01)
        var cek5 = within == benar

        var poly_A = "POLYGON((0 0, 2 0, 2 2, 0 2, 0 0))"
        var poly_B = "POLYGON((1 1, 3 1, 3 3, 1 3, 1 1))"
        var intersects = ST_Intersects(poly_A, poly_B)
        var cek6 = intersects == benar

        var jkt = [[106.8456, -6.2088]]
        var transformed = ST_Transform(jkt, "EPSG:4326", "EPSG:3857")
        var cek7 = transformed[0]["x"] > 0.0

        var line_2pt = "LINESTRING(0 0, 1 1)"
        var panjang_line = ST_Length(line_2pt)
        var cek8 = panjang_line > 0.0

        var buf_pt = [100.0, 0.0]
        var buf_hasil = ST_Buffer(buf_pt, 111000.0)
        var cek9 = panjang(buf_hasil) == 4

        kembalikan cek1 dan cek2 dan cek3 dan cek4 dan cek5 dan cek6 dan cek7 dan cek8 dan cek9
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_5_spasial_st_functions: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_6_rtree_insert_query() {
    let code = r#"
        fungsi IndeksRTree(kapasitas) {
            jika kapasitas == 0 {
                kapasitas = 16
            }
            kembalikan { "_tipe": "IndeksRTree", "kapasitas": kapasitas, "items": [] }
        }

        fungsi rtree_sisip(rt, id, min_x, min_y, max_x, max_y, data) {
            var item = { "id": id, "min_x": min_x, "min_y": min_y, "max_x": max_x, "max_y": max_y, "data": data }
            rt["items"] = tambah(rt["items"], item)
            kembalikan benar
        }

        fungsi rtree_kueri_kotak(rt, qmin_x, qmin_y, qmax_x, qmax_y) {
            var hasil = []
            var items = rt["items"]
            jika panjang(items) == 0 {
                kembalikan hasil
            }
            var n = panjang(items) - 1
            untuk i dalam 0..n {
                var it = items[i]
                var overlap = (it["min_x"] <= qmax_x) dan (it["max_x"] >= qmin_x) dan (it["min_y"] <= qmax_y) dan (it["max_y"] >= qmin_y)
                jika overlap {
                    hasil = tambah(hasil, it)
                }
            }
            kembalikan hasil
        }

        var rt = IndeksRTree(16)
        rtree_sisip(rt, 1, 0, 0, 10, 10, "A")
        rtree_sisip(rt, 2, 12, 0, 20, 15, "B")
        rtree_sisip(rt, 3, 0, 16, 15, 30, "C")
        var hasil = rtree_kueri_kotak(rt, 11, 0, 25, 15)
        var cek = panjang(hasil) >= 1
        kembalikan cek
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_6_rtree_insert_query: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}
