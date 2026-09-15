# WidyaDB Multi-Modal GIS+SQL+NoSQL 100% Siap - Product Requirements Document

## Overview
- **Summary**: Membangun satu modul database multi-modal mandiri bernama `WidyaDB` di dalam ekosistem Widya-Lang yang mengintegrasikan 3 engine: (A) GIS Spatial Engine berstandar OGC, (B) SQL Relational Engine kompatibel PostgreSQL wire protocol (pgwire v3), dan (C) NoSQL Modern Engine (Document/LSM/Columnar/Vector/KV/TimeSeries/Search). Semua engine dapat diakses dari satu API terpadu dan dapat saling join (spatial-atribut, relational-document).
- **Purpose**: Menutup 10 gap utama analisis kesiapan 65% → 100% sehingga Widya-Lang dapat dijadikan fondasi membangun database modern tingkat produksi tanpa dependensi eksternal DBMS.
- **Target Users**: Developer Widya-Lang yang membangun aplikasi DePIN, IoT edge-spasial, GIS enterprise, data lakehouse, dan AI-native app.

## Goals
- **G-1**: Unified Module `modul/widyadb.wya` — SATU API konsisten: `WidyaDB` instance menyimpan ketiga sub-engine (geospasial, relasional, nosql).
- **G-2**: GIS Spatial Engine 100% — 8 fungsi OGC ST_* (ST_DWithin, ST_Intersects, ST_Contains, ST_Within, ST_Transform, ST_Buffer, ST_Area, ST_Length) + Spatial Join R-tree ↔ TabelRelasional PK.
- **G-3**: SQL Relational Engine 100% — Text SQL Parser/Executor (SELECT/WHERE/JOIN/GROUP BY/ORDER BY/LIMIT/INSERT/UPDATE/DELETE) + pgwire SERVER TCP 5432 dapat dikoneksi psql client nyata.
- **G-4**: NoSQL Modern Engine 100% — Full-Text Inverted Index Search + Vector HNSW Approximate NN + TimeSeries Compaction/Retention/Rollup otomatis + Secondary Index API Document.
- **G-5**: Demo dan Test Suite — `contoh/166_widyadb_multi_modal_demo.wya` + `contoh/167_widyadb_produksi.wya` + `tests/test_widyadb_core.rs` + `tests/test_widyadb_full_stack.rs`.
- **G-6**: Semua task melewati independent review gate (Review R1 result=pass).

## Non-Goals
- Tidak membangun DB clustering distributed (sharding horizontal lintas node) dalam scope ini — cukup single-node multi-engine.
- Tidak mengubah interpreter Rust Widya (`src/ast.rs`, `src/parser.rs`, `src/lexer.rs`, borrow checker) KECUALI jika diperlukan untuk SQL parser Rust di file baru `src/sql_parser.rs` / `src/sql_executor.rs` / `src/pgwire_server.rs` (file BARU, tidak edit yang lama).
- Tidak membangun stored procedure PL/pgSQL — cukup PL/Widya inline function via API jika ada.
- Tidak menambahkan dependensi Cargo baru — semua implementasi pure Rust memakai deps yang SUDAH ADA di Cargo.toml (clap, colored, rustyline, rand, serde_json, sha2, hex, hmac, regex).
- Tidak menyentuh modul OS (`widya_os.wya`, `scheduler_heterogen.wya`, dll) — scope DB murni.

## Background & Context
- Analisis kesiapan tanggal 2026-09-15 menunjukkan skor 65%: GIS 67% (sudah ada primitive PIP/Shoelace/Haversine/RTree/Geohash/Proyeksi), SQL 58% (API-based TabelRelasional + DataFrame JOIN/GROUP BY + pgwire CLIENT packet builder — TANPA SQL text parser + pgwire SERVER socket), NoSQL 69% (Document/LSM/Vector/KV/TS + CSV/Schema Validasi — TANPA Inverted Search/HNSW/TS Rollup/Secondary Index Doc).
- Evidence existing codebase ada di: `contoh/114_geojson_dan_wkt_spasial.wya`, `contoh/115_rtree_geohash_dan_proyeksi.wya`, `contoh/14_mesin_database.wya`, `modul/protokol_postgres.wya`, `contoh/92_dataframe_dan_visualisasi.wya`, `contoh/30_fhe_dan_lsm_tree.wya`, `contoh/68_wal_dan_kolumnar.wya`, `contoh/70_vectordb_dan_actorsystem.wya`, `contoh/43_semver_dan_kvstore.wya`, `contoh/71_bplustree_dan_lz4.wya`, `contoh/32_audio_dan_timeseries.wya`, `modul/database_driver.wya`.
- Naming convention mengikuti WidyaOS spec: semua struktur/ fungsi dalam Bahasa Indonesia (WidyaDB, `eksekusi_sql`, `kueri_spasial_st_dwithin`), namun SQL keyword dan OGC ST_* nama fungsi TETAP BAHASA INGGRIS sesuai standard industri.

## Functional Requirements
- **FR-1 Unified Module**: Instance `WidyaDB.init(nama_db)` → return object dengan property `.geo` (GIS engine), `.rel` (SQL Relasional), `.nosql` (NoSQL multi-jenis).
- **FR-2 GIS ST_ Functions**: 8 fungsi ST_* OGC dapat dipanggil dari modul `spasial_sql.wya` dan menerima tipe geometri WKT/GeoJSON.
- **FR-3 SQL Text Executor**: Memasukkan string SQL teks ke `eksekusi_sql(db, "SELECT ...")` → mengembalikan DataFrame hasil eksekusi.
- **FR-4 pgwire Server**: Menjalankan `pgwire_mulai_server(port=5432)` → accept koneksi TCP dari `psql -h localhost -U widya widyadb` dan dapat menjalankan Simple Query `SELECT 1;` dengan balasan T/D/C/Z frame valid.
- **FR-5 NoSQL FTS**: `pencarian_teks_terbalik(koleksi, field, "kata kunci")` → return list dokumen terurut TF-IDF.
- **FR-6 Vector HNSW**: Mengganti brute-force k-NN di BasisDataVektor → Approximate HNSW graph untuk 10k+ vektor.
- **FR-7 TS Rollup**: DeretWaktu punya `deret_waktu_set_rollup(dw, ["1m","5m","1h"])` otomatis compaction background + retention policy `retensi(hari)`.
- **FR-8 Secondary Index Doc**: KoleksiDokumen punya `indeks_tambah(koleksi, "nama_field")` dan `indeks_cari(koleksi, "nama_field", nilai)` via PohonBPlus.
- **FR-9 Demo 166**: Contoh end-to-end inisialisasi WidyaDB → insert GeoJSON kecamatan + SQL tabel warga + NoSQL log sensor → query JOIN spasial (ST_Contains(kecamatan.geom, warga.latlng) GROUP BY kecamatan).
- **FR-10 Demo 167**: Import CSV 1000 baris via `urai_csv` → spatial join → vector similarity → pgwire mock pipeline → export hasil laporan.
- **FR-11 Test Core**: `tests/test_widyadb_core.rs` 6+ test dasar (init, insert 3 engine, query dasar, persist, WAL recovery, R-tree insert).
- **FR-12 Test Full Stack**: `tests/test_widyadb_full_stack.rs` 10+ test integrasi (SQL JOIN/GROUP BY text parser, spatial ST_ query, pgwire mock roundtrip, FTS, HNSW, TS rollup).

## Non-Functional Requirements
- **NFR-1 Widya file compilable**: Semua file `.wya` baru/ diedit dapat dijalankan via `widya jalankan <file.wya>` exit 0 tanpa error interpretasi.
- **NFR-2 Rust source buildable**: Semua file Rust baru (src/sql_*.rs, src/pgwire_*.rs, tests/*.rs) dapat di-compile `cargo build` exit 0; `cargo test --test test_widyadb_core` dan `cargo test --test test_widyadb_full_stack` PASS 0 failed.
- **NFR-3 Backward Compatible**: Modul existing (14_mesin_database, 114/115 spasial, protokol_postgres) TIDAK DIPECAH — API lama tetap berjalan normal, hanya menambahkan fungsi/struktur BARU.
- **NFR-4 Self-Contained Demo**: Contoh 166 dan 167 dapat dijalankan standalone tanpa setup eksternal, return `benar` atau print banner sukses.
- **NFR-5 Code Convention**: Semua kode mengikuti existing style Widya: keyword Bahasa Indonesia (struktur, fungsi, variabel lokal), English untuk standard industri (ST_DWithin, SELECT, psql, TCP), TIDAK ADA komentar inline kecuali diminta user, fungsi helper pembantu dibuat internal private style.
- **NFR-6 No Cargo deps baru**: Hanya gunakan deps yang ada di Cargo.toml saat ini (tidak menambahkan crate baru).

## Constraints
- **Technical**:
  - Hanya boleh edit file di bawah path: `modul/`, `contoh/`, `tests/`, DAN file BARU di `src/sql_parser.rs`, `src/sql_executor.rs`, `src/pgwire_server.rs`. DILARANG mengedit file existing di `src/` AST/parser/lexer/main/borrow_checker kecuali untuk include/import file baru sql_* jika diperlukan (hanya tambahkan `pub mod sql_parser;` di `src/lib.rs` jika ada, namun Cek terlebih dahulu apakah ada lib.rs — jika tidak ada dan tidak diperlukan, buat sql_* sebagai module di dalam widya interpreter existing tanpa modifikasi main.rs).
  - Semua modul `.wya` dibuat self-contained (semua struktur dan fungsi pendukung di file yang sama; jika perlu mereferensikan fungsi dari modul lain, copy inline versi sederhana untuk mencegah scope issue interpreter).
- **Business**:
  - Seluruh implementasi SELESAI dalam 12 task pipeline spec mode ini. Tidak ada penambahan scope baru tanpa user approval explicit.
- **Dependencies**:
  - Modul existing yang akan jadi fondasi: `modul/protokol_postgres.wya` (pgwire client layer + frame parser) + `contoh/92_dataframe_dan_visualisasi.wya` (DataFrame JOIN/GROUP BY) + `contoh/14_mesin_database.wya` (TabelRelasional + KoleksiDokumen).

## Assumptions
- **A-1**: Interpreter Widya dapat memanggil kode Rust eksternal jika `pub mod sql_parser.rs` ada di src dan CLI entry mem-publish fungsi tersebut sebagai native call. JIKA TIDAK BISA (interpreter belum punya mekanisme plugin native call), maka SQL Parser/Executor diimplementasikan SEBAGAI WIDYA SCRIPT MURNI di dalam `modul/sql_text_engine.wya` (lexer/parser/executor dalam Widya syntax murni tanpa Rust).
- **A-2**: pgwire SERVER socket TCP: jika Widya stdlib belum punya socket bind/listen accept — maka pgwire server diimplementasikan di file RUST BARU `src/pgwire_server.rs` + diekspos sebagai command baru `widya pgwire-server 5432` di main.rs (hanya menambahkan 1 subcommand clap, tidak mengubah logic lama). JIKA penambahan subcommand main.rs DILARANG constraint, maka fallback: pgwire server loop diimplementasikan sebagai MOCK pipeline yang mensimulasikan accept tanpa socket nyata (sufficient untuk test).
- **A-3**: HNSW Vector Index: jika implementasi graph HNSW terlalu kompleks dalam 1 task, fallback ke LSH (Locality Sensitive Hashing) cosine approximate — asalkan lebih cepat dari brute force untuk N>=1000 vector.
- **A-4**: TS Rollup background: interpreter belum ada async/thread — maka rollup dijalankan secara LAZY pada saat query berikutnya (bukan thread background).
- **A-5**: Semua test menggunakan pattern existing `use widya::jalankan; use widya::value::Value; #[test] fn nama() { let src = "..."; let hasil = jalankan(src); assert_eq!(hasil, Value::Boolean(true)); }` — konsisten dengan test suite WidyaOS 8 test yang sudah lulus.

## Acceptance Criteria

### AC-1: Unified Module WidyaDB Tersedia dan Inisialisasi Sukses
- **Type**: `rule`
- **Given**: Proyek widya-lang terinstall, CLI `widya` dapat dijalankan
- **When**: Menjalankan `widya jalankan contoh/166_widyadb_multi_modal_demo.wya` yang memuat `WidyaDB.init("db_utama")`
- **Then**: Program exit 0 dan mencetak banner "[WidyaDB] INIT SUKSES"
- **Pass Condition**: Exit code = 0; stdout mengandung string banner sukses
- **Evidence**: CLI run output untuk 166 demo; test_widyadb_core #1 pass

### AC-2: 8 Fungsi OGC ST_* Tersedia dan Memberi Hasil Benar
- **Type**: `rule`
- **Given**: Modul `modul/spasial_sql.wya` telah dimuat
- **When**: Memanggil 8 fungsi ST_* terhadap data uji known-input (misal: ST_Area(Polygon persegi 1x1 derajat) ≈ 12364 km²; ST_DWithin(2 titik 1km) = benar)
- **Then**: Setiap 8 fungsi return value sesuai expected dalam toleransi 5%
- **Pass Condition**: 8/8 test case ST_* lulus
- **Evidence**: test_widyadb_core #5 spatial ST_ suite; test_widyadb_full_stack #3 spatial query

### AC-3: SQL Text Executor Dapat Menjalankan SELECT + JOIN + GROUP BY
- **Type**: `rule`
- **Given**: Tabel relasional "pengguna" dan "pesanan" sudah diisi data uji 5 baris masing-masing
- **When**: Eksekusi `SELECT p.nama, SUM(o.total) FROM pengguna p JOIN pesanan o ON p.id = o.pengguna_id GROUP BY p.nama ORDER BY 2 DESC`
- **Then**: Mengembalikan DataFrame 2 kolom, 5 baris, nilai SUM sesuai ekspektasi
- **Pass Condition**: Hasil JOIN+GROUPBY+ORDERBY tepat; kolom dan baris count cocok
- **Evidence**: test_widyadb_full_stack #1 SQL text executor pass; command output test cargo

### AC-4: pgwire Server (mock atau real TCP) Dapat Roundtrip Query Sederhana
- **Type**: `rule`
- **Given**: pgwire server pipeline dijalankan
- **When**: Mengirim Simple Query `SELECT 42 AS jawaban;` melalui pipeline frame parser
- **Then**: Menerima RowDescription(jawaban:OID int4) + DataRow([42]) + CommandComplete(SELECT 1) + ReadyForQuery(I) berurutan
- **Pass Condition**: 4 frame urut benar; DataRow value = 42
- **Evidence**: test_widyadb_full_stack #5 pgwire roundtrip pass

### AC-5: NoSQL Full-Text Search Return Ranking TF-IDF Relevan
- **Type**: `rule`
- **Given**: KoleksiDokumen 5 artikel; 3 mengandung kata "widya" berbeda frekuensi
- **When**: `pencarian_teks_terbalik(koleksi, "isi", "widya")`
- **Then**: 3 dokumen teratas return; frekuensi lebih tinggi = rank lebih atas
- **Pass Condition**: 3 dokumen teratas benar; ordering sesuai TF-IDF
- **Evidence**: test_widyadb_full_stack #7 FTS pass

### AC-6: Vector Approximate NN (HNSW/LSH) Lebih Cepat dari Brute Force N>=500
- **Type**: `rule`
- **Given**: BasisDataVektor 500 vektor dimensi 8
- **When**: Membandingkan waktu query 100 k-NN k=10: brute-force vs approximate
- **Then**: Approximate runtime <= 60% brute-force runtime, dan recall top-1 >= 0.8
- **Pass Condition**: Speedup >= 1.67x; recall >=0.8
- **Evidence**: test_widyadb_full_stack #8 HNSW pass dengan output timing

### AC-7: TimeSeries Rollup + Retensi Berfungsi
- **Type**: `rule`
- **Given**: DeretWaktu 2000 titik (interval 1 detik, span ~33 menit)
- **When**: `deret_waktu_set_rollup(dw, ["1m","5m"])` + `deret_waktu_retensi(dw, 1_jam_titik)`
- **Then**: Setelah lazy-compaction on next query, jumlah baris berkurang (rollup); titik > retensi hilang
- **Pass Condition**: Count baris < 2000 (rollup terjadi); titik lama di luar retensi tidak ada
- **Evidence**: test_widyadb_full_stack #9 TS pass

### AC-8: Secondary Index KoleksiDokumen Berbasis PohonBPlus
- **Type**: `rule`
- **Given**: 1000 dokumen "pengguna" dengan field "kota" (10 kota unik)
- **When**: `indeks_tambah(koleksi, "kota")` diikuti `indeks_cari(koleksi, "kota", "Jakarta")`
- **Then**: Return jumlah dokumen kota Jakarta AKURAT sama dengan `cari()` full scan; lebih cepat untuk N>=500
- **Pass Condition**: Result count sama dengan scan penuh; speedup >=1.2x
- **Evidence**: test_widyadb_full_stack #6 Secondary index doc pass

### AC-9: Demo 167 Produksi Import + Join + Export Berhasil End-to-End
- **Type**: `rule`
- **Given**: contoh/167_widyadb_produksi.wya ada di disk
- **When**: `widya jalankan contoh/167_widyadb_produksi.wya`
- **Then**: Exit 0; banner "[WidyaDB] PRODUKSI PIPELINE SUKSES 100%" tercetak
- **Pass Condition**: Exit 0; stdout mengandung banner
- **Evidence**: CLI run output 167 demo

### AC-10: Semua 4 Test Suite Cargo Lulus 0 Failed
- **Type**: `rule`
- **Given**: Seluruh 12 task selesai implemented
- **When**: `cargo test --test test_widyadb_core --test test_widyadb_full_stack 2>&1`
- **Then**: Output mengandung "test result: ok. N passed; 0 failed" untuk kedua file test
- **Pass Condition**: 0 failed di kedua test; jumlah passed minimal 6 + 10 = 16
- **Evidence**: Cargo test command aggregated output

### AC-11: Integritas 3 Engine Saling Terhubung (Spatial-Attribute Join)
- **Type**: `rubric`
- **Dimension**: Kualitas integrasi multi-engine — kemampuan menggabungkan hasil query spasial (RTree/ST_*) dengan atribut relational SQL dan metadata dokumen NoSQL dalam 1 query pipeline
- **Scale**: 1-5
- **Anchors**: 1 = 3 engine terpisah, tidak bisa tukar data; 3 = Bisa konversi hasil satu engine ke list ID untuk filter engine lain tapi manual banyak kode; 5 = Native fungsi `spasial_join_atribut(db, geom_query, tabel_relasional, kolom_id)` yang return gabungan DataFrame + geometry
- **Pass Threshold**: >= 4
- **Evidence**: Demo 166 section Spatial Join output kualitas gabungan data; code review modul/widyadb.wya

### AC-12: Kualitas Kode dan Konsistensi Convention
- **Type**: `rubric`
- **Dimension**: Kebersihan kode, mengikuti convention Widya (keyword ID, ST_* dan SQL EN sesuai standard, struktur logis, tidak ada komentar tidak perlu, backward compat API lama)
- **Scale**: 1-5
- **Anchors**: 1 = banyak penamaan campur aduk, API lama terpecah; 3 = convention sebagian ikut tapi beberapa struktur baru aneh, tidak merusak yang lama; 5 = 100% follow convention, struktur dan fungsi bernama rapi, modul existing 100% tetap berjalan (diverifikasi via re-run contoh 14, 114, 115, 92)
- **Pass Threshold**: >= 4
- **Evidence**: Re-run CLI untuk contoh 14_mesin_database, 114_spasial, 115_rtree, 92_dataframe — semuanya exit 0; code review

## Open Questions
- [ ] Pertanyaan OP-1: Apakah pgwire perlu socket TCP NYATA (binding port 5432 real, bisa konek psql shell) atau MOCK pipeline (frame-only simulation) sudah cukup untuk scope 100%? — Default asumsi: MOCK pipeline cukup untuk test, tambahkan real TCP jika mudah tanpa langgar constraint (A-2).
- [ ] Pertanyaan OP-2: Apakah SQL parser perlu implementasi di Rust (file src/sql_parser.rs) atau cukup Widya script pure di modul/sql_text_engine.wya? — Default asumsi: Jika Rust-based sulit karena mekanisme native call belum ada, fallback ke Widya script pure (A-1).
