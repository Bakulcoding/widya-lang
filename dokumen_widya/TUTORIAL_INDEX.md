# 📚 Indeks Tutorial Widya-Lang

Selamat datang di **Pusat Dokumentasi & Tutorial Resmi Widya-Lang** — bahasa pemrograman berbasis Bahasa Indonesia untuk AI, Blockchain, IoT, OS Terdistribusi, dan 9 Layer Universal Computing Stack 100% SIAP.

---

## 🧭 Navigasi Tutorial (Urutan Belajar yang Disarankan)

### Level 1 — Dasar (Pemula) | ⭐ 30 menit
| No | Tutorial | Topik | Link File |
|----|----------|-------|-----------|
| 01 | **Instalasi Widya-Lang** | Clone repo, install ps1/sh, build cargo, verifikasi `widya.exe` | [TUTORIAL_01_INSTALASI.md](./TUTORIAL_01_INSTALASI.md) |
| 02 | **Halo Dunia & Variabel** | `cetak()`, `misal`, `tetap`, tipe data, komentar | [TUTORIAL_02_HALO_DUNIA.md](./TUTORIAL_02_HALO_DUNIA.md) |
| 03 | **Kontrol Alur** | `jika / kalau`, `selama`, `untuk X dalam Y`, break | [TUTORIAL_03_KONTROL_ALUR.md](./TUTORIAL_03_KONTROL_ALUR.md) |
| 04 | **Fungsi & Modularisasi** | `fungsi`, parameter, `kembalikan`, scope, import modul | [TUTORIAL_04_FUNGSI.md](./TUTORIAL_04_FUNGSI.md) |
| 05 | **Koleksi Data** | Array/Larik, Dict/Kamus, operasi filter/map/sort | [TUTORIAL_05_KOLEKSI_DATA.md](./TUTORIAL_05_KOLEKSI_DATA.md) |

### Level 2 — Menengah | ⭐⭐ 90 menit
| No | Tutorial | Topik | Link File |
|----|----------|-------|-----------|
| 06 | **Struktur & OOP** | `struktur`, method, `ini` keyword, constructor | [TUTORIAL_06_OOP_STRUKTUR.md](./TUTORIAL_06_OOP_STRUKTUR.md) |
| 07 | **Penanganan Galat** | `coba / tangkap`, `lempar`, Tipe `Hasil` (Result) | [TUTORIAL_07_PENANGANAN_GALAT.md](./TUTORIAL_07_PENANGANAN_GALAT.md) |
| 08 | **Impor Modul & Ekosistem** | `impor` dari folder `modul/`, 80+ modul industri tersedia | [TUTORIAL_08_IMPOR_MODUL.md](./TUTORIAL_08_IMPOR_MODUL.md) |
| 09 | **File I/O & JSON** | `tulis_berkas`, `baca_berkas`, `ke_json`, `dari_json` | [TUTORIAL_09_FILE_JSON.md](./TUTORIAL_09_FILE_JSON.md) |
| 10 | **Unit Test & Benchmark** | `pastikan` (assert), `uji()`, `benchmark()`, cargo test | [TUTORIAL_10_UJI_BENCHMARK.md](./TUTORIAL_10_UJI_BENCHMARK.md) |

### Level 3 — Kompilasi & Toolchain | ⭐⭐ 60 menit
| No | Tutorial | Topik | Link File |
|----|----------|-------|-----------|
| 11 | **CLI Widya Lengkap** | 12 perintah `widya` (run/kompilasi/wasm/llvm/gpu/ebpf/repl/lsp) | [TUTORIAL_11_CLI_LENGKAP.md](./TUTORIAL_11_CLI_LENGKAP.md) |
| 12 | **6 Target Cross Compile** | Rust/LLVM/Wasm/WGSL/eBPF/Native ELF dari satu kode | [TUTORIAL_12_KOMPILASI_6_TARGET.md](./TUTORIAL_12_KOMPILASI_6_TARGET.md) |
| 13 | **Cargo Test Suite Rust** | 60+ file `tests/*.rs` — cara jalankan, cara tambah test baru | [TUTORIAL_13_CARGO_TEST.md](./TUTORIAL_13_CARGO_TEST.md) |
| 14 | **Makro & Ownership** | Hygienic macro, Ownership borrow checker (Rust-style) | [TUTORIAL_14_MAKRO_OWNERSHIP.md](./TUTORIAL_14_MAKRO_OWNERSHIP.md) |
| 15 | **FFI & Bare-Metal** | C FFI interop, GPIO/RISC-V, `program_native.c` | [TUTORIAL_15_FFI_BAREMETAL.md](./TUTORIAL_15_FFI_BAREMETAL.md) |

### Level 4 — Ekosistem Enterprise & 9 Layer Stack | ⭐⭐⭐ 3-6 jam
| No | Tutorial | Topik | Link File |
|----|----------|-------|-----------|
| 16 | **WidyaDB Multi-Modal** | GIS/SQL/NoSQL/FTS/Vector/TS — [widyadb.wya](../modul/widyadb.wya) | [TUTORIAL_16_WIDYADB.md](./TUTORIAL_16_WIDYADB.md) |
| 17 | **WidyaOS Kernel + KAL** | Capability syscalls, Scheduler Heterogen, IDT/Hypervisor | [TUTORIAL_17_WIDYAOS.md](./TUTORIAL_17_WIDYAOS.md) |
| 18 | **AI Federasi + Federated Learning** | FedAvg + CUDA 8x H100 shard, RAG Pipeline, ModelML | [TUTORIAL_18_AI_FEDERASI.md](./TUTORIAL_18_AI_FEDERASI.md) |
| 19 | **Networking Mesh (libp2p-style)** | DHT Kademlia, Raft Consensus, GossipSub P2P | [TUTORIAL_19_MESH_NETWORKING.md](./TUTORIAL_19_MESH_NETWORKING.md) |
| 20 | **9 Layer Universal Stack 100% SIAP** | Demo 168, 810/810 POINT, deploy production | [TUTORIAL_20_SEMUA_LAYER_100PERSEN.md](./TUTORIAL_20_SEMUA_LAYER_100PERSEN.md) |

### 🎨 Widya Studio IDE Level Industri (Sekaligus Area Kerja) | ⭐ 5 menit
| No | Tutorial | Topik | Link File |
|----|----------|-------|-----------|
| 12 | **Widya Studio — IDE Level Industri** | Jalankan `widya studio --port 9000`; 20+ fitur: Multi-tab Monaco, Inline Error Marker, Command Palette 35+ aksi, Kompilasi 6 target UI, Cargo Test panel, Mini Debugger, REPL persisten, Git mini, AI Autocomplete, Snippet, Generate Dokumentasi, Split/Zen, Cari rekursif workspace, Rate Limit 120/menit | [TUTORIAL_11_CLI_LENGKAP.md#12-widya-studio](TUTORIAL_11_CLI_LENGKAP.md) |
| — | **6 Target Cross Compile** | Tutorial 12 lengkap: Rust/LLVM/Wasm/WGSL/eBPF/Native ELF dari satu kode — bisa dijalankan dari UI studio (dropdown ⚙️ Build) maupun CLI | [TUTORIAL_12_KOMPILASI_6_TARGET.md](./TUTORIAL_12_KOMPILASI_6_TARGET.md) |

---

## 🎯 Contoh Kode Siap Pakai (168 File di Folder `contoh/`)

Dari `01_halo_dunia.wya` sampai `168_semua_layer_100persen.wya` — contoh lengkap untuk topik:
- **01-21**: Dasar → menengah (standar library)
- **100-149**: Spesifik industri (IoT, Game, DevOps, Forensik, Bioinformatika, AI Kognitif, Antariksa)
- **150-164**: Solusi terpadu vertikal (Fintech PQC, FHIR Health, CAN-Bus Otomotif, GIS, Cloud)
- **165-168**: Master demo flagship:
  - [165_widyaos_boot.wya](../contoh/165_widyaos_boot.wya) — Boot OS penuh + banner KERNEL
  - [166_widyadb_multi_modal_demo.wya](../contoh/166_widyadb_multi_modal_demo.wya) — Demo 6 engine DB
  - [167_widyadb_produksi.wya](../contoh/167_widyadb_produksi.wya) — Production grade 10M row benchmark
  - [168_semua_layer_100persen.wya](../contoh/168_semua_layer_100persen.wya) — ⭐ **Master Demo: 9 Layer 100% 810/810 POINT**

Cara jalankan semua contoh:
```bash
widya jalankan contoh/XX_nama_contoh.wya
```

---

## 🧩 80+ Modul Industri Siap Impor (Folder `modul/`)

Semua modul self-contained tanpa dependensi eksternal, tinggal `impor`:

| Kategori | Contoh Modul |
|----------|--------------|
| **AI & Data** | ai_federasi.wya, ai_rag_stack.wya, data_engineering.wya, vektor_hnsw.wya |
| **Database** | widydb.wya, sql_text_engine.wya, spasial_sql.wya, pgwire_server.wya, secondary_index_doc.wya, deret_waktu_rollup.wya, pencarian_teks.wya |
| **OS & Kernel** | widya_os.wya, scheduler_heterogen.wya, interrupt_idt_apic.wya, hypervisor_kvm_xen.wya, resource_cgroup_v2.wya, wasmtime_wasmer_runtime.wya |
| **Hardware Khusus L6** | sgx_enclave_simulasi.wya, hsm_pkcs11_emulasi.wya, fpga_bitstream_config.wya, cuda_ffi_runtime.wya, opencl_wrapper_platform.wya |
| **Networking** | mesh_libp2p_style.wya, sistem_terdistribusi.wya, event_streaming_bus.wya, protokol_postgres.wya, pgwire_server.wya |
| **Standar Global** | standar_kesehatan_hl7_fhir.wya, fintech_iso8583_iso20022.wya, identitas_terdesentralisasi_did.wya, otomotif_canbus_uds.wya, telco_5g_diameter_gtp.wya |
| **Industri Vertikal** | robotika_cdr_ros2_dds.wya, medis_dicom_imaging.wya, scada_opcua_modbus_tcp.wya, kereta_etcs_euroradio.wya, maritim_ais_nmea0183.wya, antariksa_spacepackets_ecss.wya, nuklir_mcnp_neutronics.wya |
| **Keamanan** | privasi_keamanan.wya, keamanan_siber_forensik.wya, quantum_post_quantum_kem.wya, autentikasi_enterprise.wya |
| **DevOps & Cloud** | microservice_stack.wya, observability_k8s.wya, cloud_devops_arsip.wya, cloud_iac_terraform_spec.wya, paket_manajer_resolver.wya, kompiler.wya |

---

## ❓ Cara Mendapatkan Bantuan

- **Folder contoh/**: 168 file runnable — jawaban ada di contoh
- **Folder modul/**: Lihat struktur dan self-test setiap modul untuk pola API
- **Cargo Test**: `cargo test` — 60+ test suite dengan pattern r##" inline
- **REPL**: Jalankan `widya` tanpa parameter → interactive shell
- **LSP**: VSCode extension di folder `vscode-extension/` — syntax highlighting + autocomplete

---

## ⚡ Quick Start 5 Menit — Tanpa Baca Dokumentasi

```powershell
# Windows PowerShell
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang
.\install.ps1

# Jalankan master demo — lihat banner 810/810!
widya jalankan contoh\168_semua_layer_100persen.wya

# Atau mulai dari dasar
widya jalankan contoh\01_halo_dunia.wya
widya jalankan contoh\06_oop_dan_struktur.wya
```

---

🇮🇩 **Widya-Lang — Bahasa Ibu untuk Teknologi Masa Depan**
