# Rencana Implementasi: WidyaOS 100% Siap

## Prioritas Task Overview

| ID Task | Deskripsi | Priority | AC Terkait |
|---|---|---|---|
| T1 | Modul `widya_os.wya` (OS Kernel Abstraction Layer) | high | AC-R03, AC-R09, AC-U1 |
| T2 | Sistem atribut `#[profil(...)]` + conditional compilation di parser & borrow checker | high | AC-R08, NFR-05 |
| T3 | Modul `scheduler_heterogen.wya` (Compute DAG + Heterogeneous Dispatch) | high | AC-R04, AC-U2, NFR-02 |
| T4 | Perluasan `blockchain.wya`: PoA/PBFT consensus + ComputeLelang + zk-PoUW pipeline | high | AC-R05, AC-R10, AC-U3, NFR-03 |
| T5 | Modul `ai_federasi.wya`: Federated Learning + Sharded LLM + Quantization | high | AC-R06, AC-U4 |
| T6 | Modul `mesh_libp2p_style.wya`: Heartbeat Kademlia + GossipSub + NAT traversal | medium | AC-R07, AC-U5, NFR-03 |
| T7 | Contoh `contoh/165_widyaos_boot.wya` — Full-stack OS boot end-to-end | high | AC-R01, AC-U1..AC-U5 |
| T8 | Suite test `tests/test_widyaos_full_stack.rs` — coverage seluruh AC | high | AC-R02, NFR-06 |

---

## Task 1: Modul `modul/widya_os.wya` — OS Kernel Abstraction Layer

**Status: pending**

**Priority: high**

**AC Coverage**: AC-R03, AC-R09, AC-U1 (FR-02.1 – FR-02.5)

**Task-local Test Requirements (TR)**:

| TR | Tipe | Detail |
|---|---|---|
| T1-TR1 | rule | `WidyaOSKernel("edge")` dibuat; field `profil == "edge"`, `cspace`, `mmu`, `scheduler` tidak nihil. |
| T1-TR2 | rule | `os_boot(kernel, [proc1, proc2])` berjalan; setiap proc terdaftar sebagai TCB di scheduler dengan status "SIAP". |
| T1-TR3 | rule | `os_syscall(kernel, cap_valid, SYSCALL_BACA_GPIO, [pin13])` sukses; cap yang dicabut → error string berisi "CAPABILITY_DICABUT". |
| T1-TR4 | rule | `os_spawn_userspace_driver(kernel, "DriverI2C", wasm_bytes)` membuat endpoint IPC `ep_DriverI2C` di CSpace. |
| T1-TR5 | rule | `WidyaOSNode("node-edge-01", "edge")` bundling seluruh sub-sistem; field `node_kademlia`, `node_raft`, `ai_runtime`, `blockchain_wallet` terdefinisi. |
| T1-TR6 | rubric | Kualitas kelengkapan OS-KAL (skala 0–5). Ambang: 4. Bukti: ≥4 struktur + ≥4 fungsi sesuai FR-02. |

**Dependency**: Tidak ada (task daun)

---

## Task 2: Sistem `#[profil(...)]` — Parser & Borrow Checker Extension

**Status: pending**

**Priority: high**

**AC Coverage**: AC-R08, NFR-05

**Implementation Scope**:
1. Tambah tipe `Attribute` di `src/ast.rs`: `Attribute { name: String, args: Vec<String> }`
2. Modifikasi parser untuk mengenali `#[profil("...")]` sebelum `fungsi`, `struktur`, `impor`.
3. Perluasan `BorrowChecker` di `src/borrow_checker.rs` dengan method `check_profile_context(program, active_profile)`.
4. Di `modul/widya_os.wya`: contoh `fungsi` bertanda `#[profil("server")]` hanya dapat dipanggil jika profil aktif = "server"; jika edge → galat.
5. CLI tambah flag `--profil <nama>` di `src/main.rs` (ops `Jalankan`, `Kompilasi`, `Wasm`).

**Task-local Test Requirements (TR)**:

| TR | Tipe | Detail |
|---|---|---|
| T2-TR1 | rule | Kode `#[profil("server")] fungsi server_only() { cetak("Hanya server") }` dikenali parser; AST berisi attribute. |
| T2-TR2 | rule | Eksekusi dengan `--profil edge` memanggil `server_only()` → runtime error berisi "TIDAK_TERSEDIA_DI_PROFIL_EDGE". |
| T2-TR3 | rule | Eksekusi dengan `--profil server` memanggil `server_only()` → berhasil cetak output. |
| T2-TR4 | rule | Fungsi tanpa atribut profil bisa dipanggil di kedua profil. |

**Dependency**: Tidak ada. Perubahan di `src/ast.rs`, `src/parser.rs`, `src/borrow_checker.rs`, `src/main.rs`.

---

## Task 3: Modul `modul/scheduler_heterogen.wya` — Compute DAG + Heterogeneous Dispatch

**Status: pending**

**Priority: high**

**AC Coverage**: AC-R04, AC-U2 (FR-03.1 – FR-03.4)

**Task-local Test Requirements (TR)**:

| TR | Tipe | Detail |
|---|---|---|
| T3-TR1 | rule | `dag_tambah_tugas(dag, "t1", "TUGAS_WASM", 0.5, 16, wasm_kode)` menambah 1 node; dependensi default `[]`. |
| T3-TR2 | rule | Dispatch ke cluster `[edge_ram32, edge_ram64, server_gpu16gb]`: `TUGAS_GPU_WGSL` harus ke `server_gpu16gb`. |
| T3-TR3 | rule | Dispatch `TUGAS_WASM` ke edge node dengan RAM cukup (16MB task → edge_ram32 diterima, edge tidak cukup dilewati). |
| T3-TR4 | rule | Hasil `scheduler_dapatkan_rencana(dag)` mapping tugas→node deterministik untuk input seed sama. |
| T3-TR5 | rubric | 10 test acak dispatch valid (task cocok resource node). Skor 0–5. Ambang: 4. Bukti: ≥9/10 mapping benar. |

**Dependency**: T1 (menggunakan `WidyaOSNode` sebagai cluster).

---

## Task 4: Perluasan `modul/blockchain.wya` — PoA/PBFT + Compute Market + zk-PoUW

**Status: pending**

**Priority: high**

**AC Coverage**: AC-R05, AC-R10, AC-U3, NFR-03 (FR-04.1 – FR-04.6)

**Implementation Scope**:
1. Tambah field `mode_konsensus` dan `daftar_validator_poa` ke `RantaiBlok`.
2. `tambang_transaksi_tertunda` → untuk mode PoA: pilih validator round-robin, tanda tangani block.
3. Struktur `ComputeLelang` + `tambah_lelang` + `tawar_lelang` + `tutup_lelang_dan_pilih_pemenang`.
4. `pouw_buat_bukti` menggunakan hash chain (nonce + hasil) untuk simulasi zk-SNARK kecil.
5. `pouw_verifikasi_dan_bayar` → cek bukti hash + transfer escrow dari alamat penyewa → worker + kurangi escrow balance.

**Task-local Test Requirements (TR)**:

| TR | Tipe | Detail |
|---|---|---|
| T4-TR1 | rule | `RantaiBlok` dibuat dengan `mode_konsensus: "PoA"` dan 3 validator; block diproduksi hanya oleh validator aktif. |
| T4-TR2 | rule | Blok dengan <2 signature (quorum <2/3) ditolak; `validasi_integritas` mengembalikan false. |
| T4-TR3 | rule | `ComputeLelang` dengan 3 bidder; pemenang = bidder dengan harga tertinggi di bawah `harga_maks`. |
| T4-TR4 | rule | `pouw_buat_bukti("t1", hash_hasil, 120)` → bukti.komitmen tidak kosong. |
| T4-TR5 | rule | `pouw_verifikasi_dan_bayar` sukses: saldo worker += `harga × durasi`; saldo escrow berkurang sama. |
| T4-TR6 | rubric | Flow end-to-end Lelang → Dispatch → zk-PoUW → Payout. Skala 0–5. Ambang: 4. Bukti: 4/5 step pass. |

**Dependency**: Tidak ada (perluasan file existing `modul/blockchain.wya`).

---

## Task 5: Modul `modul/ai_federasi.wya` — Federated Learning + Sharded LLM + Quantization

**Status: pending**

**Priority: high**

**AC Coverage**: AC-R06, AC-U4 (FR-05.1 – FR-05.5)

**Task-local Test Requirements (TR)**:

| TR | Tipe | Detail |
|---|---|---|
| T5-TR1 | rule | `FederasiAICluster` dibuat dengan 1 koordinator + 3 edge; list `edge_clients` panjang=3. |
| T5-TR2 | rule | `ai_quantize_model(model_asli, "INT8")` → model_quant.presisi == "INT8" dan ukuran_memory ≤ 25% model_asli. |
| T5-TR3 | rule | `fed_train_lokal(edge_client_1, data_1, model_global)` → delta_gradien tidak nihil; ukuran data_lokal = 100 → bobot_agregasi = 100/total. |
| T5-TR4 | rule | `fed_agregasi_fedavg(koordinator, [delta1, delta2, delta3])` → model_baru akurasi ≥ model_global (tidak menurun drastis). |
| T5-TR5 | rule | `sharded_llm_inference([shard_layer0_3, shard_layer4_7, shard_layer8_11], prompt)` → token output gabungan valid. |
| T5-TR6 | rubric | Flow 5 langkah: Quantize → 3× LocalTrain → FedAvg → ShardedInfer. Skala 0–5. Ambang: 4. Bukti: 4/5 step pass. |

**Dependency**: T1 (menggunakan `WidyaOSNode` sebagai edge client)

---

## Task 6: Modul `modul/mesh_libp2p_style.wya` — Heartbeat Kademlia + GossipSub + NAT Traversal

**Status: pending**

**Priority: medium**

**AC Coverage**: AC-R07, AC-U5, NFR-03 (FR-06.1 – FR-06.4)

**Task-local Test Requirements (TR)**:

| TR | Tipe | Detail |
|---|---|---|
| T6-TR1 | rule | `kademlia_detak_heartbeat(dht, node_saya)` → node entry `last_seen_ts` ter-update; node offline >3 detik ditandai `status = "OFFLINE"`. |
| T6-TR2 | rule | `GossipSubTopik("telemetri_suhu")` dibuat; 3 node subscribe. |
| T6-TR3 | rule | `gossip_publikasi(topik, {suhu: 30})` → seluruh 3 node online menerima callback (counter callback bertambah 1). |
| T6-TR4 | rule | 2 node dari 5 di-set OFFLINE (churn); `gossip_publikasi` berikutnya → 3 node yang masih ONLINE menerima pesan (offline tidak menerima). |
| T6-TR5 | rule | `mesh_nat_traversal_coba(node_a_di_belakang_nat, node_b_publik)` → simulasi STUN: node_a memperoleh alamat publik reflektif; status koneksi "TERHUBUNG" atau fallback "RELAY". |
| T6-TR6 | rubric | Resilience churn 5 nodes: OFFLINE 2 → 3 ONLINE dapat pesan. Skala 0–5. Ambang: 4. Bukti: ≥5/6 pesan sampai. |

**Dependency**: Tidak ada (memperluas Kademlia dari contoh 78).

---

## Task 7: Contoh Program `contoh/165_widyaos_boot.wya` — Full-Stack OS Boot End-to-End

**Status: pending**

**Priority: high**

**AC Coverage**: AC-R01, seluruh AC-U (U1-U5)

**Implementation Scope (urutan step di dalam program)**:
```
0. Cetak banner [WidyaOS] v1.0 Distributed Heterogeneous OS
1. Inisialisasi cluster: WidyaOSNode("server-kord", "server") + 3× edge("edge-01".."edge-03","edge") + 2× validator("val-01","val-02","server")
2. Boot masing-masing node (os_boot)
3. Mesh discovery: tiap node join Kademlia DHT + GossipSub "topik_telemetri", "topik_lelang"
4. 3 edge node baca sensor suhu simulasi → kirim via CoAP ring-buffer (modul edge_iot.wya)
5. Scheduler heterogen: dispatch 3 tugas Wasm "deteksi_anomali_suhu" ke 3 edge
6. Federated Learning: edge train lokal model_int8 → aggregate FedAvg di server-kord
7. Sharded LLM inference: 2 server validator hold layer shard → jawab prompt "Laporan status cluster?"
8. Compute Market: Buat lelang tugas GPU "visualisasi_heatmap_suhu_3d" → bid → pemenang → zk-PoUW → bayar escrow
9. PoA: Tambang blok berisi transaksi payout ke worker
10. Verifikasi state akhir: Cek saldo worker bertambah, Cek rantai blok valid, Cek 3 edge online via heartbeat
11. Cetak [WidyaOS] FULL STACK BOOT SUKSES → keluar
```

**Task-local Test Requirements (TR)**:

| TR | Tipe | Detail |
|---|---|---|
| T7-TR1 | rule | Program dijalankan tanpa unhandled error; seluruh 11 step dieksekusi. |
| T7-TR2 | rule | Output string terminal mengandung substring `[WidyaOS] FULL STACK BOOT SUKSES`. |
| T7-TR3 | rule | Step 8 (DePIN): saldo worker akhir > saldo worker awal. |
| T7-TR4 | rule | Step 9 (PoA): blok baru di-commit; rantai_blok.panjang = 2 (genesis + 1 blok payout). |
| T7-TR5 | rule | Step 6 (FedAvg): model_teragregasi.akurasi ≥ model_global_awal.akurasi. |

**Dependency**: T1, T3, T4, T5, T6 (membutuhkan seluruh modul).

---

## Task 8: Suite Test Integrasi `tests/test_widyaos_full_stack.rs`

**Status: pending**

**Priority: high**

**AC Coverage**: AC-R02, NFR-06

**Implementation Scope**:
File test berisi `#[test]` untuk setiap AC-R rule dan AC-U rubric dengan evidence:
- `test_os_kernel_boot_and_syscall` (T1 TR)
- `test_profile_attribute_parsing_and_enforcement` (T2 TR)
- `test_heterogeneous_dag_dispatch` (T3 TR)
- `test_poa_consensus_and_depin_market` (T4 TR)
- `test_federated_ai_and_sharded_llm` (T5 TR)
- `test_mesh_gossip_and_churn_resilience` (T6 TR)
- `test_full_stack_165_boot_sukses` (T7 TR via jalankan string kode)

Setiap test memanggil `widya::jalankan(code_str)` dan assert `Result == Value::Bool(true)` atau assert output substring.

**Task-local Test Requirements (TR)**:

| TR | Tipe | Detail |
|---|---|---|
| T8-TR1 | rule | `cargo test --test test_widyaos_full_stack` berjalan; 0 FAIL. |
| T8-TR2 | rule | Minimal 7 terpisah `#[test]` functions (satu per komponen utama). |
| T8-TR3 | rule | Setiap struktur utama (WidyaOSKernel, ComputeDAG, FederasiAICluster, PoAValidator, GossipSubTopik) muncul di minimal 3 test case berbeda. |

**Dependency**: T1–T7 (seluruh modul dan contoh).
