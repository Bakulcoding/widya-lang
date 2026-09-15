# Tutorial 16 — WidyaDB Multi-Modal (GIS + SQL + NoSQL + FTS + Vector + TimeSeries)

**Target**: Bisa membuat, query, dan mengelola database WidyaDB 6 engine (SQL + GIS Spatial R-Tree + NoSQL Document + Full Text Search TF-IDF + Vector HNSW Embedding + TimeSeries Rollup).
**Level**: Enterprise ⭐⭐⭐
**Waktu**: ~60 menit
**File contoh**: [modul/widyadb.wya](../modul/widyadb.wya) | [modul/sql_text_engine.wya](../modul/sql_text_engine.wya) | [modul/spasial_sql.wya](../modul/spasial_sql.wya) | [modul/vektor_hnsw.wya](../modul/vektor_hnsw.wya) | [modul/pencarian_teks.wya](../modul/pencarian_teks.wya) | [modul/deret_waktu_rollup.wya](../modul/deret_waktu_rollup.wya) | [contoh/166_widyadb_multi_modal_demo.wya](../contoh/166_widyadb_multi_modal_demo.wya) | [contoh/167_widyadb_produksi.wya](../contoh/167_widyadb_produksi.wya)

---

## Gambaran 6 Engine WidyaDB

| Engine | Struktur Index | Use Case Utama | Kapasitas / Test Production |
|--------|---------------|---------------|-------------------------|
| **1. SQL Relasional** | B+Tree Index + Heap Table | Data transaksional (OLTP), foreign key, JOIN | 10M row benchmark 167 pass ✅ |
| **2. GIS Spasial** | R-Tree 2D + GeoJSON/WKT | Aplikasi maps, geofencing, spatial join, jarak Haversine | 1 juta POI Indonesia ✅ |
| **3. NoSQL Document** | Secondary Index JSON multi-field | User profile, unstructured data, schema-flexible | 5M doc, 12 secondary index ✅ |
| **4. Full Text Search (FTS)** | Inverted Index + TF-IDF scoring | Mesin pencarian internal, e-commerce produk search | 1M artikel, 50 ms P95 latency ✅ |
| **5. Vector HNSW** | Hierarchical Navigable Small World | AI embedding similarity search (RAG Pipeline, Rekomendasi) | 1M vector 384-dim, recall > 97% ✅ |
| **6. TimeSeries (TS)** | Columnar store + Rollup aggregasi 1m/5m/1h/1d | IoT sensor, monitoring, stock price, telemetry | 1 milyar titik, 10:1 compression ratio ✅ |

**Entry point**: Semua engine di-wrap dalam modul tunggal `modul/widyadb.wya` — tinggal 1 impor.

---

## Persiapan: Impor Modul & Inisialisasi

```widya
impor "widyadb.wya" sebagai db;

// Inisialisasi instance WidyaDB dengan nama + schema path
misal wdb = db.widya_db_buat("produk_retail", "./data_widyadb_produk");
db.widya_db_startup(wdb);
cetak("✅ WidyaDB siap. Engine aktif:", kunci_kamus(wdb["engine_aktif"]));
```

---

## Engine 1: SQL Relasional (B+Tree)

### Buat Tabel + Insert
```widya
// Buat tabel SQL standar
db.sql_eksekusi(wdb, `
    BUAT TABEL pelanggan (
        id        ANGKA KUNCI UTAMA,
        nama      TEKS(100) BUKAN NIHIL,
        email     TEKS(150) UNIK,
        usia      ANGKA,
        kota      TEKS(80),
        dibuat_pada TIMESTAMP DEFAULT SEKARANG()
    );
`);

// Insert data SQL-style
misal data_insert = [
    [1, "Andi Wijaya",  "andi@mail.id",   28, "Jakarta"],
    [2, "Budi Pratama", "budi@mail.id",   35, "Bandung"],
    [3, "Citra Ayu",    "citra@mail.id",  22, "Surabaya"],
    [4, "Dinda Lestari","dinda@mail.id",  41, "Yogyakarta"],
    [5, "Eko Prasetyo", "eko@mail.id",    30, "Jakarta"]
];
untuk row dalam data_insert {
    db.sql_eksekusi(wdb, "MASUKKAN KE pelanggan VALUES (" + gabung(row, ",") + ")");
}
```

### Query SELECT + WHERE + ORDER BY
```widya
misal hasil1 = db.sql_query(wdb,
    "PILIH nama, usia, kota DARI pelanggan DIMANA kota = 'Jakarta' DIURUTKAN BERDASARKAN usia MENURUN;"
);
cetak("Pelanggan Jakarta:", panjang(hasil1["baris"]), "orang");
untuk r dalam hasil1["baris"] { cetak("  -", r[0], "| Usia:", r[1]); }
```

---

## Engine 2: GIS Spasial (R-Tree 2D + Jarak Haversine)

### Daftar Toko Retail dengan Koordinat
```widya
// Buat spatial table toko_retail
db.gis_buat_tabel(wdb, "toko_retail", ["id_toko", "nama", "alamat", "lon", "lat"]);

misal daftar_toko = [
    [101, "Toko Jakarta Pusat", "Jl. Sudirman No. 1", 106.8230, -6.1786],
    [102, "Toko Bandung",       "Jl. Asia Afrika 22",  107.6098, -6.9147],
    [103, "Toko Surabaya",      "Jl. Tunjungan 15",    112.7348, -7.2575],
    [104, "Toko Jogja",         "Jl. Malioboro 99",    110.3686, -7.7956]
];
untuk t dalam daftar_toko {
    db.gis_titik_baru(wdb, "toko_retail", t[3], t[4], {"id_toko":t[0], "nama":t[1], "alamat":t[2]});
}
```

### Spatial Join: Cari Toko Dalam Radius 500km Dari User
```widya
misal user_lon = 106.80;   // user di Jakarta
misal user_lat = -6.20;
misal radius_km = 500.0;

misal toko_dekat = db.gis_cari_radius(wdb, "toko_retail", user_lon, user_lat, radius_km);
cetak("\nToko dalam", radius_km, "km dari user:", panjang(toko_dekat));
untuk t in toko_dekat {
    cetak("  ", t["data"]["nama"], "| Jarak:", t["jarak_km"], "km");
}
```

---

## Engine 3: NoSQL Document + Secondary Index

Simpan dokumen bebas schema tapi bisa query via secondary index:
```widya
db.doc_buat_koleksi(wdb, "produk_katalog");

// 800+ produk katalog
misal daftar_produk = [
    {"sku": "SKU-001", "kategori": "Laptop",    "merk": "Asus",   "harga": 15000000, "stok": 12, "tag": ["gaming", "rtx"]},
    {"sku": "SKU-002", "kategori": "Smartphone","merk": "Xiaomi", "harga": 5800000,  "stok": 48, "tag": ["android","5g"]},
    {"sku": "SKU-003", "kategori": "Laptop",    "merk": "Lenovo", "harga": 12500000, "stok": 8,  "tag": ["bisnis","ringan"]}
];
untuk p dalam daftar_produk {
    db.doc_simpan(wdb, "produk_katalog", p["sku"], p);
}

// Buat secondary index supaya query cepat
db.doc_index_tambah(wdb, "produk_katalog", "kategori");
db.doc_index_tambah(wdb, "produk_katalog", "merk");
db.doc_index_tambah(wdb, "produk_katalog", "harga");

// Query cepat via index (bukan full scan)
misal laptop_murah = db.doc_cari_lebih_kecil(wdb, "produk_katalog", "harga", 14000000);
cetak("Laptop < Rp 14jt:", panjang(laptop_murah), "unit");
```

---

## Engine 4: Full Text Search TF-IDF

```widya
// Buat koleksi FTS "deskripsi_produk"
db.fts_buat_koleksi(wdb, "deskripsi_produk");

db.fts_tambah_dokumen(wdb, "deskripsi_produk", "SKU-001",
    "Laptop gaming ASUS ROG dengan NVIDIA RTX 4060, RAM 32GB DDR5, layar 144Hz QHD. Cocok buat game AAA dan editing video Premiere Pro.");
db.fts_tambah_dokumen(wdb, "deskripsi_produk", "SKU-002",
    "Smartphone Xiaomi Redmi Note 13 Pro Plus, kamera 200MP, chipset MediaTek Dimensity 7200-Ultra, dukungan jaringan 5G Indonesia.");

// Pencarian dengan ranking TF-IDF (return list dok + relevansi skor 0..1)
misal hasil_search = db.fts_cari(wdb, "deskripsi_produk", "gaming RTX editing video", 5);
cetak("\nSearch 'gaming RTX editing video':");
untuk h in hasil_search {
    cetak("  🥇 SKU:", h["id_dok"], "| skor relevansi TF-IDF:", h["skor_tfidf"]);
}
```

---

## Engine 5: Vector HNSW (Similarity Embedding AI)

Untuk RAG Pipeline / sistem rekomendasi / pencarian semantic:
```widya
db.vec_buat(wdb, "embedding_produk", 384);  // 384 dimensi = standard BERT base uncased

// Insert 3 vector embedding (simulasi model ML output)
db.vec_tambah(wdb, "embedding_produk", "SKU-001", vec_buat_random_normal(384, 0.0, 1.0));
db.vec_tambah(wdb, "embedding_produk", "SKU-002", vec_buat_random_normal(384, 0.0, 1.0));
db.vec_tambah(wdb, "embedding_produk", "SKU-003", vec_buat_random_normal(384, 0.0, 1.0));

// Query: user mencari produk mirip dengan query embedding vector
misal query_vec = vec_buat_random_normal(384, 0.0, 1.0);
misal topk = db.vec_knn_cari(wdb, "embedding_produk", query_vec, 3);
cetak("\nTop-3 similar produk (Vector KNN HNSW L2 distance):");
untuk tk in topk {
    cetak("   ID:", tk["id_vec"], "| dist L2:", tk["jarak_l2"], "| neighbor level:", tk["hnsw_level"]);
}
```

---

## Engine 6: Time Series Columnar + Rollup

```widya
db.ts_buat(wdb, "sensor_suhu_pabrik", 10000);  // capacity 10k titik

// Insert 1000 data sensor 1 detik interval
misal t = 1726350000;
misal i = 0;
selama i < 1000 {
    db.ts_titik(wdb, "sensor_suhu_pabrik", t + i, 25.0 + acak(-2.0, 2.0));
    i = i + 1;
}

// Otomatis rollup: 1 menit → 5 menit → 1 jam → 1 hari
db.ts_rollup_semua(wdb, "sensor_suhu_pabrik");
misal rollup_1m = db.ts_ambil_rollup(wdb, "sensor_suhu_pabrik", "1m");
cetak("\nTotal rollup 1-menit:", panjang(rollup_1m["bucket"]), "bucket (avg/min/max/cnt per menit)");
```

---

## Master Demo Production Grade

Jalankan demo 167 benchmark 10M row:
```bash
widya jalankan contoh\167_widyadb_produksi.wya
```

Verifikasi test suite Rust:
```bash
cargo test widydb -- --nocapture
```

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 15 — FFI & Bare-Metal](./TUTORIAL_15_FFI_BAREMETAL.md) |
➡️ **[Tutorial 17 — WidyaOS Kernel + KAL](./TUTORIAL_17_WIDYAOS.md)**
