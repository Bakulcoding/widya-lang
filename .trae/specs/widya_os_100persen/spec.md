# Spesifikasi: Widya-Lang 100% Siap untuk OS Terdistribusi Heterogen (WidyaOS)

## 1. Masalah & Konteks

Saat ini Widya-Lang sudah memiliki **84% kapabilitas** untuk membangun OS terdistribusi berbasis microkernel:
- ✅ Simulasi API mikrokernel (IPC capability, TCB scheduler, MMU)
- ✅ Wasm/WASI emitter + sandbox, GPU WGSL, eBPF
- ✅ Raft, Kademlia DHT, CRDT, Vector Clock
- ✅ Blockchain + zkProof + FHE, RAG/LLM/SNN AI
- ❌ **Belum ada integrasi vertikal menyeluruh** — modul-modul berdiri sendiri

Tujuan spesifikasi ini: **menutup 16% gap sehingga Widya-Lang dapat langsung digunakan untuk mem-boot dan meng-orchestrasi node OS terdistribusi (edge + server + blockchain + DePIN AI) tanpa ad-hoc glue code.**

---

## 2. Pengguna & Tujuan

| Pengguna | Tujuan |
|---|---|
| Developer OS WidyaOS | Menulis kernel, driver userspace, scheduler terdistribusi dalam Widya-Lang murni |
| Operator Node Edge | Menjalankan WidyaOS Profile Edge (16-128 MB RAM) pada SBC/MCU |
| Operator Node Server | Menjalankan WidyaOS Profile Server (NUMA-aware, GPU/NPU) di rack server |
| DePIN Worker | Menyewakan komputasi via Compute Market dengan bukti zk-PoUW |
| DePIN Consumer | Melelang komputasi heterogen (CPU/Wasm/GPU) melalui smart contract |

### Tujuan Utama (Goals)
1. **Kernel Profile System** — Dua profil kompilasi (Edge/Server) dengan conditional compilation native.
2. **Unified OS Kernel Abstraction Layer (OS-KAL)** — Satu modul `widya_os.wya` yang mem-bundle seluruh pilar OS menjadi satu API tunggal.
3. **Heterogeneous Distributed Scheduler** — DAG task scheduler yang otomatis dispatch workload ke Wasm sandbox / MicroVM / GPU shader sesuai resource node.
4. **PoA/BFT Private Chain + zk-PoUW DePIN** — Ganti PoW menjadi konsensus enterprise + pipeline verifiable compute market.
5. **Federated AI Orchestrator** — End-to-end: edge inference → gradient ring-buffer → FedAvg aggregation server → sharded LLM inference.
6. **Mesh Networking Node Discovery** — Integrasi topologi libp2p-style (Kademlia + libp2p gossipsub) dengan churn-tolerant peer table.
7. **End-to-End OS Boot Demo** — Contoh program `165_widyaos_boot.wya` yang mensimulasikan full-stack booting node WidyaOS dari init sampai join cluster.

### Non-Goals
- ❌ Menulis kode assembly bootloader baremetal x86_64/ARM64 nyata (ini luar scope compiler-level; cukup API abstraction + simulasi terverifikasi).
- ❌ Production blockchain mainnet (cukup private PoA testnet API).
- ❌ Replikasi seL4/Zircon source code (cukup API parity + capability enforcement di level interpreter/borrow-checker).

---

## 3. Persyaratan Fungsional (Functional Requirements)

### FR-01: Sistem Profil Kompilasi `#[profil(...)]`
- FR-01.1: Atribut `#[profil("edge")]` dan `#[profil("server")]` dapat dilampirkan pada `fungsi`, `struktur`, dan `modul`.
- FR-01.2: Saat kompilasi dengan flag `--profil edge`, hanya simbol bertanda `edge` atau tanpa profil yang dimuat; simbol `server` dihilangkan (dead code elimination).
- FR-01.3: Profile Edge otomatis mengaktifkan fitur: `no_std_compat`, `heap_limit_mb=128`, `embedded_hal`, `ai_quantized_int8`.
- FR-01.4: Profile Server otomatis mengaktifkan fitur: `numa_scheduler`, `gpu_npu_accel`, `io_uring_like`, `big_heap`.

### FR-02: OS Kernel Abstraction Layer (Modul `widya_os.wya`)
- FR-02.1: Struktur `WidyaOSKernel` dengan field `profil: "edge"|"server"`, `cspace: MikrokernelCapSpace`, `mmu: TabelHalamanMMU`, `scheduler: PenjadwalMikrokernel`.
- FR-02.2: Fungsi `os_boot(kernel, init_processes)` menjalankan urutan: init_CSpace → init_MMU → init_TCB → spawn init_processes.
- FR-02.3: Fungsi `os_syscall(kernel, caller_cap, syscall_num, args)` memeriksa capability sebelum mengeksekusi syscall (revocation otomatis jika tidak sah).
- FR-02.4: Fungsi `os_spawn_userspace_driver(kernel, nama_driver, binary_wasm)` memuat driver sebagai Wasm sandbox di userspace dengan IPC endpoint terbatas.
- FR-02.5: Struktur `WidyaOSNode` membundel `kernel + node_kademlia + node_raft + ai_runtime + blockchain_wallet` menjadi satu instance node lengkap.

### FR-03: Heterogeneous Compute DAG Scheduler
- FR-03.1: Struktur `ComputeDAG` dengan node bertipe: `TUGAS_WASM`, `TUGAS_GPU_WGSL`, `TUGAS_MICROVM`.
- FR-03.2: Fungsi `dag_tambah_tugas(dag, id_tugas, tipe, kebutuhan_cpu, kebutuhan_ram, kode)` menambah node tugas dengan dependensi.
- FR-03.3: Fungsi `scheduler_dispatch_heterogen(dag, klaster_node)` otomatis memilih node yang sesuai:
  - Edge node (RAM ≤128MB) → hanya `TUGAS_WASM` ringan.
  - Server node dengan GPU → prioritas `TUGAS_GPU_WGSL`.
  - Server node tanpa GPU → `TUGAS_MICROVM` dan `TUGAS_WASM`.
- FR-03.4: Fungsi `scheduler_dapatkan_rencana(dag)` mengembalikan rencana DAG eksekusi dengan mapping `tugas_id → node_id`.

### FR-04: Private PoA/BFT Blockchain + zk-PoUW Compute Market
- FR-04.1: Modifikasi `RantaiBlok` agar mendukung mode `konsensus: "PoW" | "PoA" | "PBFT"`.
- FR-04.2: Struktur `PoAValidator` dengan field `alamat`, `pubkey`, `reputasi_skor`, `status_aktif`.
- FR-04.3: Pada mode PoA: hanya validator terdaftar yang dapat memproduksi blok; round-robin leader selection.
- FR-04.4: Struktur `ComputeLelang` (auction) dengan field: `id`, `penyewa_alamat`, `tipe_tugas`, `sla_waktu_detik`, `harga_maks_widya_per_detik`.
- FR-04.5: Fungsi `pouw_buat_bukti(tugas_id, hasil_hash, eksekusi_detak)` menghasilkan zk-SNARK bukti eksekusi.
- FR-04.6: Fungsi `pouw_verifikasi_dan_bayar(rantai, bukti, validator)`: jika bukti valid → otomatis transfer escrow ke worker + bonus reputasi.

### FR-05: Federated AI Orchestrator End-to-End
- FR-05.1: Struktur `FederasiAICluster` dengan field: `pusat_koordinator`, `edge_clients: []`, `model_arsitektur`, `putaran_fedavg`.
- FR-05.2: `fed_train_lokal(edge_client, data_lokal, model_global_cp)`: training lokal INT8 quantized → mengembalikan delta_gradien terkompresi.
- FR-05.3: `fed_agregasi_fedavg(koordinator, daftar_delta)`: Federated Averaging dengan bobot berdasarkan ukuran dataset klien.
- FR-05.4: `sharded_llm_inference(model_shards, prompt_nodes)`: pipeline parallelism LLM di mana setiap node server memegang beberapa layer model; hasil digabung bertahap.
- FR-05.5: `ai_quantize_model(model, presisi: "INT4"|"INT8")` → model terkuantisasi untuk edge.

### FR-06: Mesh Networking Churn-Tolerant
- FR-06.1: Perluasan `TabelKademlia` dengan peer liveness heartbeat (`kademlia_detak_heartbeat(dht, node_id)`).
- FR-06.2: Struktur `GossipSubTopik` untuk pub/sub terdistribusi (mirip libp2p gossipsub).
- FR-06.3: `gossip_publikasi(topik, pesan)` dan `gossip_langganan(topik, callback_fn)`.
- FR-06.4: `mesh_nat_traversal_coba(node_a, node_b)` → simulasi hole punching STUN-style.

### FR-07: Full-Stack Boot Demo & Integration Test
- FR-07.1: Contoh program `contoh/165_widyaos_boot.wya` yang menjalankan:
  1. Boot 1 node Server Koordinator + 3 node Edge + 2 node Validator PoA.
  2. Tiap edge mengirim telemetri sensor via CoAP ring-buffer.
  3. Scheduler mendispatch tugas Wasm inferensi suhu ke 3 edge.
  4. Federated Learning: edge train → aggregate di server.
  5. Lelang komputasi GPU: zk-PoUW + bayar via smart contract escrow.
  6. Semua node sync state via Kademlia + GossipSub.
- FR-07.2: File test `tests/test_widyaos_full_stack.rs` yang meng-cover 100% AC di atas.

---

## 4. Persyaratan Non-Fungsional (Non-Functional Requirements)

### NFR-01: Keamanan & Safety
- rule: Semua syscall `os_syscall` harus melalui capability check; jika capability dicabut atau tidak sesuai, syscall gagal dan capability reporter mencatat insiden.
- rule: Borrow checker harus mencegah `use-after-move` pada object `CapSpace` dan `TCB` (tidak boleh ada dangling reference ke TCB yang sudah di-terminate).

### NFR-02: Performa & Jejak Memori
- rubric (Edge Profile Footprint): Total memori statis untuk `WidyaOSKernel` edge profile ≤ 8 MB object pada interpreter (indikator footprint). Skor 0–2: `2` = ≤8 MB; `1` = 9–16 MB; `0` = >16 MB. Ambang lulus: 2.
- rubric (Scheduler Latency): `scheduler_dispatch_heterogen` untuk 100 tugas + 20 node selesai ≤ 250 ms. Skor 0–2: `2` = ≤250 ms; `1` = 251–500 ms; `0` = >500 ms. Ambang lulus: ≥1.

### NFR-03: Konsistensi Terdistribusi
- rule: Untuk 3 validator PoA, blok yang diajukan harus ditandatangani ≥2 validator sebelum commit (quorum 2f+1 dengan f=1).
- rule: Setelah `pouw_verifikasi_dan_bayar` sukses, saldo worker bertambah tepat `harga × durasi` dan saldo escrow berkurang jumlah yang sama (ledger balance preserved).

### NFR-04: Determinisme & Reprodusibilitas
- rule: Eksekusi `dag_tambah_tugas` + `scheduler_dispatch_heterogen` untuk seed input yang sama harus menghasilkan mapping `tugas_id→node_id` yang identik.
- rule: `os_boot` dengan input `init_processes` yang sama harus menghasilkan urutan TCB eksekusi deterministik.

### NFR-05: Portabilitas Antar-Profil
- rule: Kode `WidyaOSKernel` yang ditulis tanpa profil spesifik harus dapat berjalan tanpa modifikasi di profil `edge` DAN `server` (interface parity).

### NFR-06: Test Coverage
- rule: Setiap struktur utama (`WidyaOSKernel`, `ComputeDAG`, `FederasiAICluster`, `PoAValidator`, `GossipSubTopik`) memiliki minimal 3 test case terpisah di suite test.

---

## 5. Batasan, Dependensi, Asumsi

### Batasan
- Tidak menghasilkan binary baremetal yang dapat boot di perangkat nyata; scope terbatas pada abstraction layer + simulator OS terverifikasi.
- zk-PoUW menggunakan hash-based cryptographic proof (sesuai contoh 26), bukan groth16 atau circuit compiler yang sebenarnya.

### Dependensi
- Reuse 100% modul yang sudah ada:
  [modul/blockchain.wya](file:///c:/widya-lang/modul/blockchain.wya),
  [modul/sistem_terdistribusi.wya](file:///c:/widya-lang/modul/sistem_terdistribusi.wya),
  [modul/ai_rag_stack.wya](file:///c:/widya-lang/modul/ai_rag_stack.wya),
  [modul/edge_iot.wya](file:///c:/widya-lang/modul/edge_iot.wya),
  [contoh/128_mikrokernel_dan_kapabilitas_ipc.wya](file:///c:/widya-lang/contoh/128_mikrokernel_dan_kapabilitas_ipc.wya),
  [contoh/129_unikernel_dan_mmu_halaman.wya](file:///c:/widya-lang/contoh/129_unikernel_dan_mmu_halaman.wya),
  [contoh/72_lsh_dan_raft.wya](file:///c:/widya-lang/contoh/72_lsh_dan_raft.wya),
  [contoh/78_kademlia_dan_mpt.wya](file:///c:/widya-lang/contoh/78_kademlia_dan_mpt.wya),
  [contoh/26_kuantum_dan_zkproof.wya](file:///c:/widya-lang/contoh/26_kuantum_dan_zkproof.wya).

### Asumsi
- Semua data diproses di level interpreter Widya (tidak perlu Rust native bindings).
- Satu unit `detak` scheduler ekivalen dengan satu pemanggilan fungsi (tidak ada real threading OS-level).

---

## 6. Kriteria Penerimaan (Acceptance Criteria)

### Kategori rule:
- **AC-R01**: Compile & run `contoh/165_widyaos_boot.wya` tanpa error; output berisi 7 step boot + `[WidyaOS] FULL STACK BOOT SUKSES`.
- **AC-R02**: `tests/test_widyaos_full_stack.rs` seluruh test case `#[test]` lulus.
- **AC-R03**: `modul/widya_os.wya` mendefinisikan seluruh struktur dan fungsi pada FR-02 (WidyaOSKernel, WidyaOSNode, os_boot, os_syscall, os_spawn_userspace_driver).
- **AC-R04**: `modul/scheduler_heterogen.wya` mendefinisikan ComputeDAG + dispatch heterogen dengan aturan pemilihan node sesuai FR-03.3.
- **AC-R05**: Modifikasi `modul/blockchain.wya` menambah mode `PoA` + `PBFT` dengan quorum validator; struktur `ComputeLelang` dan `pouw_*` tersedia.
- **AC-R06**: `modul/ai_federasi.wya` menyediakan `FederasiAICluster`, `fed_train_lokal`, `fed_agregasi_fedavg`, `sharded_llm_inference`, `ai_quantize_model`.
- **AC-R07**: `modul/mesh_libp2p_style.wya` menyediakan `GossipSubTopik`, heartbeat, NAT traversal.
- **AC-R08**: File atribut `#[profil]` diterima parser; borrow checker mengenali profil (test: fungsi `#[profil("server")]` tidak dapat dipanggil pada profil edge).
- **AC-R09**: `os_syscall` dengan capability yang dicabut mengembalikan error `GALAT_CAPABILITY_DICABUT` dan tidak mengeksekusi operasi.
- **AC-R10**: PoA block production tanpa 2/3 signature validator ditolak (tidak masuk rantai).

### Kategori rubric:
- **AC-U1 (OS-KAL Completeness)**: Skala 0–5. `5` = Seluruh FR-02 ter-implement + 5+ sample driver; `4` = FR-02 lengkap tapi ≤2 driver; `3` = ≥80% FR-02; `2` = 50–79%; `1` = <50%; `0` = tidak ada. Ambang: 4.
- **AC-U2 (DAG Scheduler Correctness)**: Skala 0–5. `5` = 10/10 test plan mapping valid; `4` = 9/10; `3` = 8/10; `2` = 6–7/10; `1` = ≤5/10. Ambang: 4.
- **AC-U3 (DePIN Compute Market Flow)**: Skala 0–5. `5` = Lelang → Dispatch → zk-PoUW → Escrow payout seluruhnya end-to-end pass; `4` = 4/5 step; `3` = 3/5; `2` = 2/5; `1` = ≤1. Ambang: 4.
- **AC-U4 (Federated AI Flow)**: Skala 0–5. `5` = Quantize → 3 client train lokal → FedAvg aggregate → sharded inference pass; `4` = 4/5; `3` = 3/5; `2` = ≤2. Ambang: 4.
- **AC-U5 (Mesh Networking Resilience)**: Skala 0–5. `5` = 5 node churn (offline 2 node) + gossip pesan tetap sampai ke 3 node live; `4` = 5/6 pesan sampai; `3` = 4/6; `2` = ≤3. Ambang: 4.
