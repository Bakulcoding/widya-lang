# Buku Panduan Lengkap Standar Modul Industri Widya-Lang

Widya-Lang menyediakan **21 Modul Standar Industri Tingkat Tinggi** yang dirancang untuk mengatasi seluruh spektrum rekayasa perangkat lunak modern dari level sistem biner, cloud-native microservices, sistem terdistribusi, hingga komputasi sains dan AI.

---

## 📑 Daftar Isi Modul

1. [Kriptografi, Privasi & Keamanan](#1-kriptografi-privasi--keamanan)
2. [Data Engineering & Big Data Analytics](#2-data-engineering--big-data-analytics)
3. [Cloud Native & Resilient Microservices](#3-cloud-native--resilient-microservices)
4. [Edge Computing & Internet of Things (IoT)](#4-edge-computing--internet-of-things-iot)
5. [Fintech, Order Book & Kuantitatif](#5-fintech-order-book--kuantitatif)
6. [Kecerdasan Buatan & RAG Stack](#6-kecerdasan-buatan--rag-stack)
7. [Konsensus Sistem Terdistribusi & CRDT](#7-konsensus-sistem-terdistribusi--crdt)
8. [Keamanan Siber, Forensik & SIEM](#8-keamanan-siber-forensik--siem)
9. [DevOps, Kontainer OCI & Preservasi Digital](#9-devops-kontainer-oci--preservasi-digital)
10. [Robotika, Topologi GIS & Orbit Kepler](#10-robotika-topologi-gis--orbit-kepler)
11. [Pemrosesan Sinyal Digital (DSP) & Akustik](#11-pemrosesan-sinyal-digital-dsp--akustik)
12. [Logika Formal, SAT/SMT Solver & Astrofisika](#12-logika-formal-satsmt-solver--astrofisika)
13. [Database Driver, Connection Pool & WAL](#13-database-driver-connection-pool--wal)
14. [Observabilitas Kubernetes & Prometheus](#14-observabilitas-kubernetes--prometheus)
15. [Autentikasi Enterprise & RBAC Matrix](#15-autentikasi-enterprise--rbac-matrix)
16. [Runtime Asinkron Berbasis Aktor](#16-runtime-asinkron-berbasis-aktor)
17. [Streaming I/O Berkas Ukuran Besar](#17-streaming-io-berkas-ukuran-besar)
18. [Protokol Biner PostgreSQL (pgwire)](#18-protokol-biner-postgresql-pgwire)
19. [Event Streaming Bus (Partitioned Commit Log)](#19-event-streaming-bus-partitioned-commit-log)
20. [Codec Protobuf & GraphQL Resolver](#20-codec-protobuf--graphql-resolver)
21. [Manajer Paket & SemVer Resolver](#21-manajer-paket--semver-resolver)
22. [Protokol Transaksi Perbankan ISO 8583](#22-protokol-transaksi-perbankan-iso-8583)
23. [Kriptografi Pasca-Kuantum (NIST PQC KEM & Signatures)](#23-kriptografi-pasca-kuantum-nist-pqc-kem--signatures)
24. [Cloud Infrastructure-as-Code & Policy-as-Code](#24-cloud-infrastructure-as-code--policy-as-code)
25. [Standar Rekam Medis HL7 FHIR](#25-standar-rekam-medis-hl7-fhir)
26. [Otomotif CAN Bus & UDS Diagnostic (ISO 14229)](#26-otomotif-can-bus--uds-diagnostic-iso-14229)
27. [Otomasi SCADA Industri OPC-UA & Modbus TCP](#27-otomasi-scada-industri-opc-ua--modbus-tcp)
28. [Identitas Digital Terdesentralisasi W3C DID & VC](#28-identitas-digital-terdesentralisasi-w3c-did--vc)
29. [Standar Finansial Global ISO 20022](#29-standar-finansial-global-iso-20022)
30. [Navigasi Maritim AIS & NMEA 0183](#30-navigasi-maritim-ais--nmea-0183)
31. [Avionika ARINC 429 Instrumen Kokpit Pesawat](#31-avionika-arinc-429-instrumen-kokpit-pesawat)
32. [SPKLU Pengisian Daya Kendaraan Listrik (OCPP 2.0.1)](#32-spklu-pengisian-daya-kendaraan-listrik-ocpp-201)
33. [Smart Grid Gardu Induk IEC 61850 GOOSE](#33-smart-grid-gardu-induk-iec-61850-goose)
34. [Telekomunikasi 5G Core Diameter OCS Charging](#34-telekomunikasi-5g-core-diameter-ocs-charging)
35. [Citra Radiologi Medis DICOM PACS](#35-citra-radiologi-medis-dicom-pacs)
36. [Geofisika Seismik SEG-Y](#36-geofisika-seismik-seg-y)
37. [Robotika Industri ROS2 DDS CDR](#37-robotika-industri-ros2-dds-cdr)
38. [Antariksa ESA ECSS PUS Satelit](#38-antariksa-esa-ecss-pus-satelit)
39. [Manajemen Baterai Kendaraan Listrik (BMS)](#39-manajemen-baterai-kendaraan-listrik-bms)
40. [Konstruksi Digital BIM & ISO 16739 IFC](#40-konstruksi-digital-bim--iso-16739-ifc)
41. [Bioinformatika Genomika VCF](#41-bioinformatika-genomika-vcf)
42. [Software-Defined Radio (SDR) VITA 49 VRT](#42-software-defined-radio-sdr-vita-49-vrt)
43. [Persinyalan Kereta Cepat ETCS](#43-persinyalan-kereta-cepat-etcs)
44. [Logistik Rantai Pasok GS1 EPCIS](#44-logistik-rantai-pasok-gs1-epcis)

---

### 1. Kriptografi, Privasi & Keamanan
- **Berkas**: `modul/privasi_keamanan.wya`
- **Fungsi Utama**:
  - `buat_brankas_rahasia(nama_layanan)`: Inisialisasi cryptographic vault.
  - `brankas_simpan_rahasia(brankas, id_kunci, plaintext)`: Simpan rahasia terenkripsi ChaCha20/Poly1305 + Argon2id hash.
  - `brankas_ambil_rahasia(brankas, id_kunci)`: Dekripsi rahasia dengan validasi integritas.
  - `inisialisasi_komputasi_rahasia_fhe()`: Setup BFV Fully Homomorphic Encryption.
  - `komputasi_total_terenkripsi(ctx, daftar_angka)`: Agregasi penjumlahan homomorfik tanpa membuka data rahasia.
  - `buat_bukti_audit_zk(tx_id, nilai, ambang)`: Verifikasi zero-knowledge proof audit regulasi AML.
  - `audit_privasi_differensial(data, epsilon)`: Tambahkan Laplace noise untuk statistik publik yang aman.

### 2. Data Engineering & Big Data Analytics
- **Berkas**: `modul/data_engineering.wya`
- **Fungsi Utama**:
  - `buat_pipeline_data(nama_pipeline)`: Inisialisasi pipeline ETL/ELT.
  - `jalankan_etl_sederhana(sumber_data, skema, batas_anomali)`: Pemrosesan DataFrame, SMA tren, dan deteksi anomali.
  - `peramalan_tren_bisnis(deret_nilai, horizon)`: Peramalan tren eksponensial.

### 3. Cloud Native & Resilient Microservices
- **Berkas**: `modul/microservice_stack.wya`
- **Fungsi Utama**:
  - `buat_mesh_gateway(nama, port, endpoints)`: Setup API gateway dengan load balancer & circuit breaker.
  - `gateway_rute_permintaan(gw, path, payload)`: Rute permintaan dengan rate limiting Token-Bucket dan OpenTelemetry span tracking.

### 4. Edge Computing & Internet of Things (IoT)
- **Berkas**: `modul/edge_iot.wya`
- **Fungsi Utama**:
  - `buat_node_edge_iot(id_node, pin_sensor)`: Inisialisasi perangkat edge dengan GPIO, battery telemetry, dan ring buffer offline.
  - `baca_dan_proses_sensor_edge(node, val_sensor)`: Pembacaan sensor dan penyimpanan otomatis ke buffer saat offline.

### 5. Fintech, Order Book & Kuantitatif
- **Berkas**: `modul/fintech_settlement.wya`
- **Fungsi Utama**:
  - `buat_matching_engine_bursa(simbol)`: Mesin pencocokan pesanan L2 Limit Order Book.
  - `tambah_order(engine, id, tipe, harga, jumlah)`: Tambah pesanan Beli/Jual dan eksekusi matching otomatis.
  - `buat_liquidity_pool(token_a, token_b, cadangan_a, cadangan_b)`: AMM Uniswap Constant Product ($x \cdot y = k$).
  - `hitung_harga_opsi_black_scholes(s, k, r, t, v)`: Valuasi opsi Call/Put & Greeks Risk.

### 6. Kecerdasan Buatan & RAG Stack
- **Berkas**: `modul/ai_rag_stack.wya`
- **Fungsi Utama**:
  - `buat_vektor_penyimpanan(nama)`: Inisialisasi vector store semantik.
  - `indeks_dokumen(store, id, teks, embedding)`: Penyimpanan dokumen & vektor embedding.
  - `sintesis_rag_jawaban(store, query, vec_query, top_k)`: Cosine similarity retrieval dan perakitan prompt kontekstual.

### 7. Konsensus Sistem Terdistribusi & CRDT
- **Berkas**: `modul/sistem_terdistribusi.wya`
- **Fungsi Utama**:
  - `buat_klaster_konsensus(id_node, rekan_list)`: Inisialisasi node Raft Consensus.
  - `raft_usulkan_perubahan(node, kunci, nilai)`: Replikasi log terdistribusi.
  - `buat_crdt_sinkronisasi()`: Multi-master collaborative sync (G-Counter / PN-Counter).

### 8. Keamanan Siber, Forensik & SIEM
- **Berkas**: `modul/keamanan_siber_forensik.wya`
- **Fungsi Utama**:
  - `buat_pusat_operasi_keamanan()`: Inisialisasi SOC SIEM & YARA rules engine.
  - `analisa_keamanan_log(soc, logs, threshold)`: Deteksi serangan web (SQLi/XSS) & anomali IP bruteforce.
  - `periksa_file_biner(soc, bytes)`: Pencocokan signature malware YARA & kalkulasi entropi Shannon.

### 9. DevOps, Kontainer OCI & Preservasi Digital
- **Berkas**: `modul/cloud_devops_arsip.wya`
- **Fungsi Utama**:
  - `buat_pipeline_devops(nama)`: Runner otomatisasi tahap CI/CD (Lint, Test, Build, Deploy).
  - `hasilkan_spesifikasi_kontainer_oci(nama_img, versi)`: Generator manifest format OCI Image Container.
  - `buat_arsip_bagit(direktori, daftar_berkas)`: Kemasan preservasi arsip digital standar IETF RFC 8493 BagIt.

### 10. Robotika, Topologi GIS & Orbit Kepler
- **Berkas**: `modul/spasial_dan_robotika.wya`
- **Fungsi Utama**:
  - `buat_pengendali_robot(l1, l2)`: Kinematika maju & balik (Inverse Kinematics) 2-DOF Robot Arm.
  - `periksa_geofence_titik(poligon_koordinat, titik_uji)`: Ray-casting point-in-polygon GIS.
  - `hitung_posisi_orbit_kepler(a, e, anomali_rata2)`: Kalkulasi posisi orbit mekanika antariksa.

### 11. Pemrosesan Sinyal Digital (DSP) & Akustik
- **Berkas**: `modul/dsp_dan_multimedia.wya`
- **Fungsi Utama**:
  - `spektrum_frekuensi_fft(sinyal_waktu)`: Analisis domain frekuensi via 1D Discrete Fourier Transform.
  - `terapkan_filter_biquad(sinyal, a0, a1, a2, b1, b2)`: Filter digital IIR Low-Pass / High-Pass audio.
  - `konvolusi_audio(sinyal, impuls_respon)`: Simulasi resonansi reverb & akustik spasial.

### 12. Logika Formal, SAT/SMT Solver & Astrofisika
- **Berkas**: `modul/formal_dan_sains_komputasi.wya`
- **Fungsi Utama**:
  - `pecahkan_sat_dpll(klausa_cnf)`: DPLL Boolean Satisfiability Solver.
  - `pecahkan_smt_qf_lra(kendala_linear)`: SMT Linear Real Arithmetic Solver.
  - `simulasi_langkah_barnes_hut(benda_langit, delta_waktu)`: Gravitasi N-Body $O(N \log N)$ astrofisika.

### 13. Database Driver, Connection Pool & WAL
- **Berkas**: `modul/database_driver.wya`
- **Fungsi Utama**:
  - `buat_kolam_koneksi(driver, host, port, db, maks)`: Connection pool manager relasional & NoSQL.
  - `db_eksekusi_transaksi_acid(pool, tx_id, operasi_list)`: Transaksi ACID dengan jaminan Write-Ahead Log (WAL).
  - `buat_cache_driver()`: In-memory KV caching dengan expirasi waktu (TTL).

### 14. Observabilitas Kubernetes & Prometheus
- **Berkas**: `modul/observability_k8s.wya`
- **Fungsi Utama**:
  - `buat_eksportir_metrik()`: Prometheus OpenMetrics counter/gauge exporter.
  - `metrik_tambah_counter(eksportir, nama, val)` & `metrik_set_gauge(eksportir, nama, val)`.
  - `periksa_probe_k8s_liveness(status_db, utilisasi_memori)`: Kubernetes `/healthz` & `/readyz` probes.

### 15. Autentikasi Enterprise & RBAC Matrix
- **Berkas**: `modul/autentikasi_enterprise.wya`
- **Fungsi Utama**:
  - `buat_manajer_rbac()`: Matriks otorisasi peran berhierarki (Admin, Editor, Viewer).
  - `terbitkan_token_sesi(rbac, id, nama, peran, durasi)`: Penerbitan token JWT sesi bertanda tangan.
  - `validasi_dan_periksa_izin(rbac, token, aksi)`: Verifikasi tanda tangan kriptografis dan izin aksi RBAC.

### 16. Runtime Asinkron Berbasis Aktor
- **Berkas**: `modul/async_runtime.wya`
- **Fungsi Utama**:
  - `buat_kolam_tugas_asinkron(nama_pool)`: Inisialisasi thread pool aktor non-blocking.
  - `jalankan_tugas_asinkron(pool, beban_kerja)`: Pengiriman tugas asinkron via antrean pesan.
  - `tunggu_hasil_tugas(pool, handle)`: Pengambilan hasil tugas tanpa memblokir thread utama.

### 17. Streaming I/O Berkas Ukuran Besar
- **Berkas**: `modul/streaming_io.wya`
- **Fungsi Utama**:
  - `buat_pembaca_aliran_berkas(path, ukuran_chunk)`: Streaming pembacaan file besar bebas OOM.
  - `aliran_baca_blok_berikutnya(aliran)`: Pengambilan chunk data berikutnya secara bertahap.
  - `jalankan_pipeline_transformasi_aliran(sumber, tujuan, transformer)`: Pipeline transformasi data aliran (Map/Filter/Reduce).

### 18. Protokol Biner PostgreSQL (pgwire)
- **Berkas**: `modul/protokol_postgres.wya`
- **Fungsi Utama**:
  - `buat_pg_startup_packet(user, db)`: Framing StartupMessage v3.0 PostgreSQL.
  - `buat_pg_query_packet(sql)`: Framing paket Simple Query `'Q'`.
  - `parse_pg_backend_frame(tipe, payload)`: Parser balasan frame backend (`RowDescription`, `DataRow`, `CommandComplete`).
  - `eksekusi_pg_mock_pipeline(sql, db_state)`: Simulasi interaksi soket wire database.

### 19. Event Streaming Bus (Partitioned Commit Log)
- **Berkas**: `modul/event_streaming_bus.wya`
- **Fungsi Utama**:
  - `buat_streaming_topic(nama, partisi)`: Inisialisasi partisi log streaming ala Apache Kafka.
  - `publish_event(topik, key, payload)`: Append event ke partisi log berdasar hash key.
  - `buat_consumer_group(group_id, topik)`: Consumer group dengan pelacak offset commit log.
  - `poll_events(consumer_group, topik, max_batch)`: Batch polling event dengan auto-checkpointing offset.

### 20. Codec Protobuf & GraphQL Resolver
- **Berkas**: `modul/codec_protobuf_graphql.wya`
- **Fungsi Utama**:
  - `protobuf_encode_string(field_num, teks)`: Enkoder Protobuf Varint Tag-Length-Value tipe 2 (string).
  - `protobuf_encode_int32(field_num, angka)`: Enkoder Protobuf Varint tipe 0 (integer).
  - `graphql_parse_fields(query_str)`: Parser deklarasi field permintaan GraphQL.
  - `graphql_resolve_query(query_str, dataset_row)`: Eksekusi dan resolusi field skema GraphQL.

### 21. Manajer Paket & SemVer Resolver
- **Berkas**: `modul/paket_manajer_resolver.wya`
- **Fungsi Utama**:
  - `semver_cocok(versi, aturan_req)`: Pencocokan aturan Semantic Versioning (`^1.2.0`, `>=2.0.0`, `==1.0.0`).
  - `resolve_dependency_tree(manifest_deps, registry_pkgs)`: Penelusuran pohon dependensi dan pembuatan lockfile dengan verifikasi hash SHA256.

### 22. Protokol Transaksi Perbankan ISO 8583
- **Berkas**: `modul/protokol_fintech_iso8583.wya`
- **Fungsi Utama**:
  - `buat_pesan_iso8583(mti)`: Inisialisasi payload pesan ISO 8583 (0200, 0210, 0800).
  - `iso8583_set_field(pesan, field_num, nilai)` & `iso8583_ambil_field(pesan, field_num)`: Manipulasi field bitmap perbankan.
  - `iso8583_pack_payload(pesan)`: Serialisasi bitmap wire payload.
  - `iso8583_proses_transaksi_pembayaran(pan, nominal, terminal_id, trace_num)`: Pemrosesan transaksi pembayaran core banking switch & QRIS.

### 23. Kriptografi Pasca-Kuantum (NIST PQC KEM & Signatures)
- **Berkas**: `modul/quantum_post_quantum_kem.wya`
- **Fungsi Utama**:
  - `pqc_generate_kem_keypair(tingkat_keamanan)`: Pembuatan keypair kisi (Lattice) ML-KEM-512/768/1024.
  - `pqc_kem_enkapsulasi(kunci_publik)` & `pqc_kem_dekapsulasi(ciphertext, sk, pk)`: Enkapsulasi dan dekapsulasi kunci bersama kebal komputer kuantum.
  - `pqc_tanda_tangan_lattice(pesan, sk)` & `pqc_verifikasi_tanda_tangan_lattice(pesan, ttd, pk)`: Tanda tangan digital kisi ML-DSA.

### 24. Cloud Infrastructure-as-Code & Policy-as-Code
- **Berkas**: `modul/cloud_iac_terraform_spec.wya`
- **Fungsi Utama**:
  - `buat_spesifikasi_iac(nama_proyek)`: Inisialisasi blueprint infrastruktur cloud multi-provider.
  - `iac_tambah_resource(spec, tipe, id, konfigurasi)`: Penambahan resource cloud (S3, VM, Database, VPC).
  - `iac_evaluasi_kepatuhan_kebijakan(spec)`: Evaluasi kepatuhan aturan keamanan dan tata kelola ala Open Policy Agent (OPA).
  - `iac_hasilkan_hcl_terraform(spec)`: Generator sintaks blueprint Terraform HCL native.

### 25. Standar Rekam Medis HL7 FHIR
- **Berkas**: `modul/standar_kesehatan_hl7_fhir.wya`
- **Fungsi Utama**:
  - `buat_fhir_patient(id, nama, tgl_lahir, gender)`: Pembuatan resource Patient standar FHIR R4.
  - `buat_fhir_observation(id, id_pasien, loinc, display, val, unit)`: Observasi parameter vital / lab pasien.
  - `buat_fhir_bundle(id, list_res)`: Bundling transaksi rekam medis interoperabel.
  - `fhir_validasi_rekam_medis(bundle)`: Validasi kepatuhan format rekam medis elektronik HL7 FHIR.

### 26. Otomotif CAN Bus & UDS Diagnostic (ISO 14229)
- **Berkas**: `modul/otomotif_canbus_uds.wya`
- **Fungsi Utama**:
  - `buat_can_frame(id, is_ext, data)`: Frame Controller Area Network standard/extended.
  - `uds_build_service_request(sid, sub_fn, params)`: Pembuatan paket diagnostik ECU OBD-II / UDS.
  - `uds_parse_diagnostic_response(frame)`: Parser positive/negative response service diagnostik otomotif.

### 27. Otomasi SCADA Industri OPC-UA & Modbus TCP
- **Berkas**: `modul/scada_opcua_modbus_tcp.wya`
- **Fungsi Utama**:
  - `buat_modbus_tcp_request(tx_id, unit_id, fc, start, qty)`: Header MBAP & PDU Modbus TCP register query.
  - `parse_modbus_tcp_response(req, raw_values)`: Parser telemetri sensor PLC/RTU pabrik.
  - `buat_opcua_node_read_request(ns, id)` & `parse_opcua_data_value(node, val, status)`: Pembacaan node OPC Unified Architecture.

### 28. Identitas Digital Terdesentralisasi W3C DID & VC
- **Berkas**: `modul/identitas_terdesentralisasi_did.wya`
- **Fungsi Utama**:
  - `buat_did_document(did, tipe_kunci, pubkey_mb)`: Dokumen identitas W3C DID.
  - `terbitkan_verifiable_credential(issuer, subject, claims, secret_key)`: Penerbitan sertifikat digital berbukti kriptografis.
  - `verifikasi_verifiable_credential(vc, secret_key)`: Verifikasi integritas tanda tangan digital klaim identitas.

### 29. Standar Finansial Global ISO 20022
- **Berkas**: `modul/fintech_iso20022_xml.wya`
- **Fungsi Utama**:
  - `buat_iso20022_pacs008_transfer(msg_id, inst_id, amount, deb_acct, deb_nm, cdt_acct, cdt_nm)`: Pembentukan transaksi transfer SWIFT MX / BI-FAST pacs.008.
  - `iso20022_validasi_pacs008(pacs_obj)`: Validasi struktur dan nominal pembayaran standar ISO 20022.

### 30. Navigasi Maritim AIS & NMEA 0183
- **Berkas**: `modul/maritim_ais_nmea0183.wya`
- **Fungsi Utama**:
  - `parse_nmea_gprmc(kalimat)`: Parser kalimat navigasi GPS maritim `$GPRMC` (waktu, koordinat, kecepatan knot).
  - `decode_ais_posisi_kapal(mmsi, lat, lon, sog, cog, nama)`: Decoding transponder AIS pelacakan armada kapal laut.

### 31. Avionika ARINC 429 Instrumen Kokpit Pesawat
- **Berkas**: `modul/avionika_arinc429_nav.wya`
- **Fungsi Utama**:
  - `arinc429_encode_word(label, ssm, payload, sdi)`: Encoding 32-bit ARINC 429 avionics data word.
  - `arinc429_decode_altimeter(word)`: Decoding data tekanan barometrik dan ketinggian altimeter kokpit.

### 32. SPKLU Pengisian Daya Kendaraan Listrik (OCPP 2.0.1)
- **Berkas**: `modul/ev_charging_ocpp_iso15118.wya`
- **Fungsi Utama**:
  - `ocpp_authorize_id_tag(id_tag, station_id)`: Otentikasi kartu RFID/Tag pengisian SPKLU.
  - `ocpp_meter_values_report(evse_id, daya_kw, tegangan_volt, soc)`: Pelaporan telemetri daya aktif & persentase baterai SoC EV.

### 33. Smart Grid Gardu Induk IEC 61850 GOOSE
- **Berkas**: `modul/smartgrid_iec61850_goose.wya`
- **Fungsi Utama**:
  - `iec61850_buat_goose_control_block(gocb_ref, app_id, dataset)`: Inisialisasi GoCB multicast proteksi listrik.
  - `iec61850_kirim_sinyal_trip_proteksi(gocb, breaker_id, trip)`: Transmisi sinyal trip pemutus daya berkecepatan tinggi (<4ms).

### 34. Telekomunikasi 5G Core Diameter OCS Charging
- **Berkas**: `modul/telco_5g_diameter_gtp.wya`
- **Fungsi Utama**:
  - `diameter_buat_ccr_charging_request(session_id, imsi, service_id, rating_group)`: Credit-Control-Request (CCR) tarif seluler 5G.
  - `diameter_proses_cca_charging_response(ccr, kuota_mb)`: Alokasi kuota data internet pengguna secara real-time.

### 35. Citra Radiologi Medis DICOM PACS
- **Berkas**: `modul/medis_dicom_imaging.wya`
- **Fungsi Utama**:
  - `dicom_parse_dataset_tags(nama, id, modalitas, tgl, rows, cols)`: Metadata tagging citra radiologi klinis (CT, MRI, X-Ray).
  - `dicom_hitung_windowing_hu(pixel, center, width)`: Kalkulasi kontras Hounsfield Unit (HU) jaringan lunak / tulang.

### 36. Geofisika Seismik SEG-Y
- **Berkas**: `modul/geofisika_segy_seismik.wya`
- **Fungsi Utama**:
  - `segy_buat_binary_file_header(job_id, line_num, dt, ns)`: Header biner format standar survei seismik geologi.
  - `segy_proses_seismic_trace(trace_num, rx, ry, samples)`: Kalkulasi Root Mean Square (RMS) deteksi anomali hidrokarbon/minyak bumi.

### 37. Robotika Industri ROS2 DDS CDR
- **Berkas**: `modul/robotika_cdr_ros2_dds.wya`
- **Fungsi Utama**:
  - `ros2_serialize_twist_message(lx, ly, lz, ax, ay, az)`: Serialisasi pesan gerak kecepatan robot `geometry_msgs/msg/Twist`.
  - `ros2_dispatch_cmd_vel(node, msg)`: Publikasi kontrol pergerakan AGV/manipulator ke topik `/cmd_vel`.

### 38. Antariksa ESA ECSS PUS Satelit
- **Berkas**: `modul/antariksa_spacepackets_ecss.wya`
- **Fungsi Utama**:
  - `ecss_pus_buat_telecommand(svc, subsvc, apid, seq, payload)`: Header Telecommand (TC) standar ECSS-E-ST-70-41C.
  - `ecss_pus_proses_telemetry(apid, svc, subsvc, params)`: Pemrosesan paket telemetri (TM) status kesehatan satelit.

### 39. Manajemen Baterai Kendaraan Listrik (BMS)
- **Berkas**: `modul/baterai_bms_canopen.wya`
- **Fungsi Utama**:
  - `bms_estimasi_kondisi_baterai(tegangan_sel_list, arus, suhu)`: Estimasi tegangan pack baterai, delta cell balancing, dan proteksi termal.

### 40. Konstruksi Digital BIM & ISO 16739 IFC
- **Berkas**: `modul/konstruksi_ifc_bim_step.wya`
- **Fungsi Utama**:
  - `ifc_buat_elemen_dinding(guid, nama, p, t, l, mat)`: Model 3D struktural dinding beton `IfcWallStandardCase`.
  - `ifc_rekap_kebutuhan_material_proyek(daftar_elem)`: Rekapitulasi volume material kubikasi beton proyek cerdas.

### 41. Bioinformatika Genomika VCF
- **Berkas**: `modul/bioinformatika_sam_bam_vcf.wya`
- **Fungsi Utama**:
  - `vcf_parse_variant_record(chr, pos, id, ref, alt, qual, info)`: Analisis mutasi sekuensing DNA genetik dan signifikansi klinis.

### 42. Software-Defined Radio (SDR) VITA 49 VRT
- **Berkas**: `modul/sdr_vita49_radio_transport.wya`
- **Fungsi Utama**:
  - `vita49_buat_iq_data_packet(stream_id, seq, freq, rate, i_s, q_s)`: Streaming paket sampel data sinyal I/Q radio transport VRT.

### 43. Persinyalan Kereta Cepat ETCS
- **Berkas**: `modul/kereta_etcs_euroradio.wya`
- **Fungsi Utama**:
  - `etcs_buat_movement_authority(nid, loc, eoa, speed)`: Otoritas pergerakan kereta ETCS Full Supervision dan pengereman otomatis.

### 44. Logistik Rantai Pasok GS1 EPCIS
- **Berkas**: `modul/logistik_gs1_epcis.wya`
- **Fungsi Utama**:
  - `gs1_buat_epcis_object_event(epc, action, biz_step, read_pt)`: Pencatatan event lacak balak (provenance) rantai pasok global.

---

## 🚀 Panduan Eksekusi Contoh Terintegrasi

Seluruh contoh siap pakai dapat langsung dijalankan menggunakan perintah:
```bash
# Menjalankan salah satu contoh solusi industri
widya contoh/153_solusi_fintech_dan_zk_confidential.wya
widya contoh/154_solusi_iot_cloud_native_microservice.wya
widya contoh/157_solusi_enterprise_database_dan_k8s_observability.wya
widya contoh/158_solusi_autentikasi_rbac_dan_streaming_io.wya
widya contoh/159_solusi_postgres_streaming_graphql_dan_paket.wya
widya contoh/160_solusi_iso8583_pqc_dan_cloud_iac.wya
widya contoh/161_solusi_fhir_canbus_scada_did_iso20022.wya
widya contoh/162_solusi_deep_industry_maritim_avionik_ev_dan_ros2.wya
widya contoh/163_solusi_space_bms_bim_genome_sdr_etcs_gs1.wya

# Menjalankan suite pengujian komprehensif
cargo test
```
