# Tutorial 08 — Impor Modul & Ekosistem 80+ Modul Industri

**Target**: Bisa mengimpor modul dari folder `modul/`, memahami struktur self-test bawaan setiap modul, dan memilih modul yang tepat untuk use case industri.
**Level**: Menengah ⭐⭐
**Waktu**: ~20 menit
**File contoh lengkap**: [08_impor_modul.wya](../contoh/08_impor_modul.wya) | [contoh/modul_mtk.wya](../contoh/modul_mtk.wya)

---

## 1. Konsep Modul di Widya

Setiap file `.wya` adalah **modul**. 2 cara membagi kode:
- **Modul lokal proyek**: file `.wya` yang kamu buat sendiri (contoh: `modul_mtk.wya`)
- **Modul standar industri**: file `.wya` di folder `modul/` (80+ modul siap pakai)

---

## 2. Membuat Modul Sendiri

Contoh: buat file `modul_mtk.wya` di folder yang sama dengan skrip utama:

```widya
// ====================================================
// File: modul_mtk.wya  (Modul kustom perhitungan matematika)
// ====================================================

fungsi tambah(a, b) { kembalikan a + b; }
fungsi kali(a, b)   { kembalikan a * b; }
fungsi kurang(a, b) { kembalikan a - b; }
fungsi bagi(a, b)   {
    jika b == 0 { lempar("modul_mtk.bagi(): Pembagian nol!"); }
    kembalikan a / b;
}

// Faktorial rekursif
fungsi fakt(n) {
    jika n <= 1 { kembalikan 1 };
    kembalikan n * fakt(n - 1);
}

// 👇 Self-test modul — berjalan JIKA file ini di-run LANGSUNG sebagai entry point
cetak("🧪 [modul_mtk] Self-test dimulai...");
pastikan(tambah(2,3) == 5);
pastikan(kali(4,5) == 20);
pastikan(kurang(10,3) == 7);
pastikan(fakt(5) == 120);
cetak("✅ [modul_mtk] Self-test LULUS (5/5 assertions)");
```

---

## 3. Mengimpor Modul Pakai `impor`

Buat file utama `app.wya`:
```widya
impor "modul_mtk.wya" sebagai mtk;

cetak("2 + 7 =", mtk.tambah(2, 7));  // 9
cetak("5! =", mtk.fakt(5));           // 120
cetak("100 / 8 =", mtk.bagi(100, 8)); // 12.5
```

Jalankan:
```bash
widya jalankan app.wya
```

> 📝 Catatan: `impor "path/ke/file.wya" sebagai nama_alias` — path adalah **relatif terhadap file skrip saat ini**, atau nama file di folder `modul/`.

---

## 4. Impor Modul Industri dari Folder `modul/`

Semua 80+ modul di folder `modul/` bisa diimpor langsung dengan nama file (tanpa path). Widya resolver otomatis cek: (1) relative path, (2) folder `modul/`.

### Contoh 1 — Pakai Modul Blockchain
```widya
impor "blockchain.wya" sebagai bc;

// Struktur + method otomatis tersedia
misal rantai = bc.BlockchainBaru();
bc.tambah_blok_data(rantai, "Transfer 100 WID → 0xAlice");
bc.tambah_blok_data(rantai, "Transfer 50 WID → 0xBob");
cetak("Panjang blockchain:", panjang(rantai.blocks));
cetak("Valid?", bc.cek_validitas_rantai(rantai));
```

### Contoh 2 — Pakai Modul AI RAG Stack
```widya
impor "ai_rag_stack.wya" as rag;

misal dokumen = [
    {"id":"D1","teks":"Widya-Lang adalah bahasa Indonesia untuk OS terdistribusi."},
    {"id":"D2","teks":"WidyaDB mendukung GIS, SQL, NoSQL, Vector, dan TimeSeries."}
];
misal vdb = rag.vdb_buat(dokumen, 384);
misal hasil = rag.rag_query(vdb, "Apa itu WidyaDB?", 3);
cetak("Hasil pencarian RAG:", panjang(hasil), "dokumen relevan");
```

### Contoh 3 — Pakai Modul Mesh Networking (libp2p-style)
```widya
impor "mesh_libp2p_style.wya" sebagai mesh;

misal node1 = mesh.node_dht_buat("N1", "10.0.0.1", 4001);
misal node2 = mesh.node_dht_buat("N2", "10.0.0.2", 4002);
mesh.dht_simpan(node1, "kontrak/widya/0x01", "0xDEADBEEF");
mesh.dht_sambungkan(node2, node1);
cetak("Lookup N2:", mesh.dht_cari(node2, "kontrak/widya/0x01"));
```

---

## 5. Daftar Pintar 80+ Modul Industri (Panduan Pilih Modul)

| Butuh Apa? | Pilih Modul di `modul/` |
|-----------|------------------------|
| **OS & Kernel** | widya_os.wya, scheduler_heterogen.wya, interrupt_idt_apic.wya, hypervisor_kvm_xen.wya, resource_cgroup_v2.wya, wasmtime_wasmer_runtime.wya |
| **Database Multi-Modal** | widydb.wya, sql_text_engine.wya, spasial_sql.wya, secondary_index_doc.wya, pencarian_teks.wya, vektor_hnsw.wya, deret_waktu_rollup.wya, pgwire_server.wya |
| **Hardware Khusus L6** | sgx_enclave_simulasi.wya, hsm_pkcs11_emulasi.wya, fpga_bitstream_config.wya, cuda_ffi_runtime.wya, opencl_wrapper_platform.wya |
| **AI & Data Science** | ai_federasi.wya, ai_rag_stack.wya, data_engineering.wya, formal_dan_sains_komputasi.wya, dsp_dan_multimedia.wya |
| **Networking & Protocol** | mesh_libp2p_style.wya, sistem_terdistribusi.wya, protokol_postgres.wya, protokol_fintech_iso8583.wya, streaming_io.wya |
| **Cloud & DevOps** | microservice_stack.wya, observability_k8s.wya, cloud_devops_arsip.wya, cloud_iac_terraform_spec.wya, paket_manajer_resolver.wya, kompiler.wya |
| **Keamanan & Privasi** | privasi_keamanan.wya, keamanan_siber_forensik.wya, quantum_post_quantum_kem.wya, autentikasi_enterprise.wya, identitas_terdesentralisasi_did.wya |
| **Kesehatan (HL7 FHIR / DICOM)** | standar_kesehatan_hl7_fhir.wya, medis_dicom_imaging.wya |
| **Fintech & Perbankan** | fintech_iso8583_iso20022.wya, fintech_settlement.wya |
| **Otomotif & Transportasi** | otomotif_canbus_uds.wya, otomotif_v2x_cits_cam.wya, ev_charging_ocpp_iso15118.wya, kereta_etcs_euroradio.wya, maritim_ais_nmea0183.wya |
| **Industri Manufaktur** | scada_opcua_modbus_tcp.wya, otomasi_kimia_isa88_batch.wya, konstruksi_ifc_bim_step.wya, energi_turbin_angin_iec61400.wya, baterai_bms_canopen.wya |
| **Robotika & Luar Angkasa** | robotika_cdr_ros2_dds.wya, antariksa_spacepackets_ecss.wya, sdr_vita49_radio_transport.wya, avionika_arinc429_nav.wya, nuklir_mcnp_neutronics.wya |
| **Telekomunikasi 5G** | telco_5g_diameter_gtp.wya |
| **Bioinformatika** | bioinformatika_pdb_protein.wya, bioinformatika_sam_bam_vcf.wya |
| **Rantai Pasok & Logistik** | logistik_gs1_epcis.wya |

---

## 6. Pola Best Practice Modul

1. **Self-test di AKHIR setiap modul**: Tambahkan assertion + banner `[NAMA_MODUL] Self-test LULUS`. Ini membantu CI/CD verifikasi otomatis.
2. **Jangan circular import**: Jika A impor B, B jangan impor A.
3. **Pure function > global mutable state**: Semua fungsi modul sebaiknya deterministic, output hanya bergantung input.
4. **Nama alias = snake_case / kecil**: `impor "pgwire_server.wya" sebagai pg;` bukan `PgWire`.
5. **Satu file = satu topik domain**: Jangan campur AI RAG dengan IoT GPIO di modul yang sama.

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 07 — Penanganan Galat](./TUTORIAL_07_PENANGANAN_GALAT.md) |
➡️ **[Tutorial 09 — File I/O & JSON](./TUTORIAL_09_FILE_JSON.md)**
