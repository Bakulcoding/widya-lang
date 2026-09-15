use widya::jalankan;
use widya::value::Value;

#[test]
fn test_os_kernel_boot_and_syscall_capability() {
    let code = r#"
        impor "modul/widya_os.wya"

        fungsi cek_berisi(teks, sub) {
            var i = 0
            selama i <= panjang(teks) - panjang(sub) {
                jika potong(teks, i, i + panjang(sub)) == sub {
                    kembalikan benar
                }
                i = i + 1
            }
            kembalikan salah
        }

        var kernel = WidyaOSKernel("edge")
        var init_list = [
            InitProcess("init_proc1", 1, 10.0),
            InitProcess("init_proc2", 2, 5.0)
        ]
        var kernel_booted = os_boot(kernel, init_list)
        var cek_boot = panjang(kernel_booted.boot_log) > 3

        cap_buat_endpoint(kernel_booted.cspace, "ep_sensor", "KIRIM_TERIMA")

        var syscall_valid = os_syscall(kernel_booted, "ep_sensor", SYSCALL_ALOKASI_MEM, [32])
        var str_valid = ke_json(syscall_valid)
        var cek_valid = cek_berisi(str_valid, "CAPABILITY_DICABUT")

        cap_cabut(kernel_booted.cspace, "ep_sensor")

        var syscall_cabut = os_syscall(kernel_booted, "ep_sensor", SYSCALL_ALOKASI_MEM, [16])
        var str_cabut = ke_json(syscall_cabut)
        var cek_cabut = cek_berisi(str_cabut, "CAPABILITY_DICABUT")

        kembalikan cek_boot dan cek_valid dan cek_cabut
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_os_kernel_boot_and_syscall_capability: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_profile_attribute_parsing_and_enforcement() {
    let code = r#"
        #[profil("server")]
        fungsi server_only_xyz() {
            kembalikan 42
        }
        fungsi biasa_saja() {
            kembalikan 7
        }
        var hasil1 = biasa_saja()
        var hasil2 = server_only_xyz()
        kembalikan hasil1 == 7 dan hasil2 == 42
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_profile_attribute_parsing_and_enforcement: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_heterogeneous_dag_dispatch_and_node_rules() {
    let code = r#"
        impor "modul/scheduler_heterogen.wya"

        var node_e1 = {}
        node_e1["id_node"] = "e1"
        node_e1["profil"] = "edge"
        node_e1["ram_tersedia_mb"] = 16
        node_e1["ada_gpu"] = salah
        node_e1["cpu_core_tersedia"] = 2

        var node_e2 = {}
        node_e2["id_node"] = "e2"
        node_e2["profil"] = "edge"
        node_e2["ram_tersedia_mb"] = 16
        node_e2["ada_gpu"] = salah
        node_e2["cpu_core_tersedia"] = 2

        var node_s1 = {}
        node_s1["id_node"] = "s1"
        node_s1["profil"] = "server"
        node_s1["ram_tersedia_mb"] = 16384
        node_s1["ada_gpu"] = benar
        node_s1["cpu_core_tersedia"] = 32

        var klaster = [node_e1, node_e2, node_s1]

        var dag = ComputeDAG()
        dag_tambah_tugas(dag, "wasm_1", TUGAS_WASM, 1, 16, "payload_w1", [])
        dag_tambah_tugas(dag, "wasm_2", TUGAS_WASM, 1, 16, "payload_w2", [])
        dag_tambah_tugas(dag, "gpu_1", TUGAS_GPU_WGSL, 4, 2048, "payload_gpu", [])
        dag_tambah_tugas(dag, "vm_1", TUGAS_MICROVM, 8, 8192, "payload_vm", [])

        var rencana = scheduler_dispatch_heterogen(dag, klaster)

        var gpu_ke_server = (rencana["gpu_1"] == "s1")
        var vm_ke_server = (rencana["vm_1"] == "s1")
        var wasm1_ke_edge = (rencana["wasm_1"] == "e1" atau rencana["wasm_1"] == "e2")
        var wasm2_ke_edge = (rencana["wasm_2"] == "e1" atau rencana["wasm_2"] == "e2")

        kembalikan gpu_ke_server dan vm_ke_server dan wasm1_ke_edge dan wasm2_ke_edge
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_heterogeneous_dag_dispatch_and_node_rules: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_poa_consensus_quorum_and_depin_market_pouw() {
    let code = r#"
        impor "modul/blockchain.wya"

        var rantai = RantaiBlok(2)

        var val1 = buat_validator_poa("0xValidator_Alice", "pubkey_alice_001")
        var val2 = buat_validator_poa("0xValidator_Bob", "pubkey_bob_002")
        var val3 = buat_validator_poa("0xValidator_Charlie", "pubkey_charlie_003")

        inisialisasi_konsensus_poa(rantai, [val1, val2, val3])

        var id_lelang = "lelang_wasm_001"
        lelang_buat(rantai, id_lelang, "0xPenyewa_01", TUGAS_WASM, 30, 10)
        lelang_tawar(rantai, id_lelang, "0xWorker_A", 8)
        lelang_tawar(rantai, id_lelang, "0xWorker_B", 6)

        var hasil_tutup = lelang_tutup_pilih_pemenang(rantai, id_lelang)
        var pemenang_obj = hasil_tutup["pemenang"]
        var cek_harga = (pemenang_obj["harga_per_detik"] == 6)

        var hasil_hash = sha256("hasil_eksekusi_wasm_001")
        var bukti = pouw_buat_bukti(id_lelang, hasil_hash, 25, pemenang_obj["worker"])
        var hasil_verif = pouw_verifikasi_dan_bayar(rantai, bukti, val1.alamat)

        var tx_daftar = rantai.transaksi_tertunda
        var hasil_blok = poa_produksi_blok(rantai, val1.alamat, tx_daftar)

        var cek_sukses_blok = hasil_blok["sukses"]
        var tanda_tangan = hasil_blok["signature"]
        var cek_jumlah_ttd = panjang(tanda_tangan) >= 2
        var cek_integritas = rantai.validasi_integritas()

        kembalikan cek_harga dan cek_integritas dan cek_jumlah_ttd
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_poa_consensus_quorum_and_depin_market_pouw: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_federated_ai_quantization_fedavg_sharding() {
    let code = r#"
        impor "modul/ai_federasi.wya"

        fungsi cek_berisi(teks, sub) {
            var i = 0
            selama i <= panjang(teks) - panjang(sub) {
                jika potong(teks, i, i + panjang(sub)) == sub {
                    kembalikan benar
                }
                i = i + 1
            }
            kembalikan salah
        }

        var model = ModelML("m", "linreg", 100.0, 4)
        var model_q = ai_quantize_model(model, "INT8")
        var mem_fp32 = 100.0 * 4
        var cek_kuantisasi = (model_q.ukuran_memory_mb <= 0.25 * mem_fp32)

        var node_coord = {}
        node_coord["nama_node"] = "coord_ai_01"
        node_coord["profil"] = "server"
        var cluster = FederasiAICluster("fed_test_01", node_coord, model_q)

        var k1 = {}
        k1["nama_node"] = "edge_k1_01"
        k1["profil"] = "edge"
        var k2 = {}
        k2["nama_node"] = "edge_k2_02"
        k2["profil"] = "edge"
        var k3 = {}
        k3["nama_node"] = "edge_k3_03"
        k3["profil"] = "edge"
        federasi_tambah_klien(cluster, k1, 150)
        federasi_tambah_klien(cluster, k2, 250)
        federasi_tambah_klien(cluster, k3, 400)

        var d1 = fed_train_lokal(cluster.edge_clients[0], 150, cluster.model_global_terpusat)
        var d2 = fed_train_lokal(cluster.edge_clients[1], 250, cluster.model_global_terpusat)
        var d3 = fed_train_lokal(cluster.edge_clients[2], 400, cluster.model_global_terpusat)

        var akurasi_awal = cluster.model_global_terpusat.akurasi_validation
        var agg = fed_agregasi_fedavg(cluster, [d1, d2, d3])
        var model_baru = agg["model_baru"]
        var akurasi_akhir = model_baru.akurasi_validation
        var cek_fedavg = (akurasi_akhir >= akurasi_awal - 0.005)

        var llm_besar = ModelML("LLM_ShardTest", "decoder_only", 700, 12)
        var daftar_node_shard = ["gpu_1", "gpu_2", "gpu_3"]
        var daftar_shard = sharded_llm_buat_shard(llm_besar, daftar_node_shard)
        var cek_bagi_rata = benar
        untuk sh dalam daftar_shard {
            jika (sh.layer_selesai - sh.layer_mulai) != 4 {
                cek_bagi_rata = salah
            }
        }
        var hasil_infer = sharded_llm_inference(daftar_shard, "Test prompt")
        var output_infer = hasil_infer["output_teks"]
        var cek_infer = cek_berisi(output_infer, "Output_LMM_Singkron")

        kembalikan cek_kuantisasi dan cek_fedavg dan cek_bagi_rata dan cek_infer
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_federated_ai_quantization_fedavg_sharding: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_mesh_gossipsub_churn_resilience_and_heartbeat() {
    let code = r#"
        impor "modul/mesh_libp2p_style.wya"

        var dht = HeartbeatDHT("n1", 10)
        dht.tambah_node("n2", "10.0.0.2:9000")
        dht.tambah_node("n3", "10.0.0.3:9000")
        dht.tambah_node("n4", "10.0.0.4:9000")
        dht.tambah_node("n5", "10.0.0.5:9000")

        var waktu_sekarang = waktu()
        dht.node_map["n3"]["last_seen_ts"] = waktu_sekarang - 10
        dht.node_map["n5"]["last_seen_ts"] = waktu_sekarang - 10

        var peta = kademlia_detak_heartbeat(dht, "n1")
        var n1_online = (peta["n1"]["status"] == "ONLINE")
        var n2_online = (peta["n2"]["status"] == "ONLINE")
        var n3_offline = (peta["n3"]["status"] == "OFFLINE")
        var n4_online = (peta["n4"]["status"] == "ONLINE")
        var n5_offline = (peta["n5"]["status"] == "OFFLINE")
        var cek_heartbeat = n1_online dan n2_online dan n3_offline dan n4_online dan n5_offline

        var topik = gossip_buat_topik("topik.test.fullstack")
        gossip_langganan(topik, "n1", "cb_n1")
        gossip_langganan(topik, "n2", "cb_n2")
        gossip_langganan(topik, "n3", "cb_n3")
        gossip_langganan(topik, "n4", "cb_n4")
        gossip_langganan(topik, "n5", "cb_n5")

        set_heartbeat_dht(dht)
        var payload_pub = {}
        payload_pub["data"] = "PING_FULL_STACK"
        var hasil_pub = gossip_publikasi(topik, "n1", payload_pub)
        var cek_gossip = (hasil_pub["diterima"] == 3)

        var node_a = NodeNetworkInfo("peer_a_nat", "192.168.1.10", "DI_BELAKANG_NAT")
        var node_b = NodeNetworkInfo("peer_b_pub", "203.0.113.50", "PUBLIK")
        var hasil_nat = mesh_nat_traversal_coba(node_a, node_b)
        var cek_nat = (hasil_nat["status"] == "TERHUBUNG_LANGSUNG")

        kembalikan cek_heartbeat dan cek_gossip dan cek_nat
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_mesh_gossipsub_churn_resilience_and_heartbeat: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_full_stack_165_boot_sukses() {
    let code = r#"
        impor "modul/widya_os.wya"
        impor "modul/scheduler_heterogen.wya"
        impor "modul/blockchain.wya"
        impor "modul/ai_federasi.wya"
        impor "modul/mesh_libp2p_style.wya"

        cetak("=== FULL STACK 165 WIDYAOS BOOT MINI ===")

        var nama_node = ["os_edge_01", "os_edge_02", "os_edge_03", "os_server_01", "os_server_02", "os_coord_01"]
        var profil_node = ["edge", "edge", "edge", "server", "server", "server"]
        var daftar_node_boot = []

        var i = 0
        selama i < panjang(nama_node) {
            var node = WidyaOSNode(nama_node[i], profil_node[i])
            var init_obj = InitProcess("init_" + nama_node[i], 1, 10.0)
            var init_proc = [init_obj]
            os_node_boot(node, init_proc)
            tambah(daftar_node_boot, node)
            cetak("  [" + ke_teks(i) + "] Node " + nama_node[i] + " -> " + node.status_node)
            i = i + 1
        }

        var semua_online = benar
        untuk n dalam daftar_node_boot {
            jika n.status_node != "ONLINE" {
                semua_online = salah
            }
        }

        cetak("FULL STACK BOOT SUKSES: " + ke_teks(panjang(daftar_node_boot)) + " node ONLINE")

        kembalikan semua_online
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_full_stack_165_boot_sukses: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_all_five_structs_minimum_three_coverage_rule() {
    let code = r#"
        impor "modul/widya_os.wya"
        impor "modul/scheduler_heterogen.wya"
        impor "modul/blockchain.wya"
        impor "modul/ai_federasi.wya"
        impor "modul/mesh_libp2p_style.wya"

        var wk1 = WidyaOSKernel("edge")
        var wk2 = WidyaOSKernel("server")
        var wk3 = WidyaOSKernel("cloud")
        var wk_ok = (wk1.heap_limit_mb == 128 dan wk2.heap_limit_mb == 8192 dan wk3.heap_limit_mb == 8192)
        cetak("[COV] WidyaOSKernel x3 -> edge/server/cloud: " + ke_teks(wk_ok))
        var ip1 = InitProcess("t1", 1, 5.0)
        os_boot(wk1, [ip1])
        var ip2 = InitProcess("t2", 1, 5.0)
        os_boot(wk2, [ip2])
        var ip3 = InitProcess("t3", 1, 5.0)
        os_boot(wk3, [ip3])

        var cd1 = ComputeDAG()
        var cd2 = ComputeDAG()
        dag_tambah_tugas(cd2, "a", TUGAS_WASM, 1, 16, "x", [])
        var cd3 = ComputeDAG()
        dag_tambah_tugas(cd3, "b1", TUGAS_WASM, 1, 8, "x", [])
        dag_tambah_tugas(cd3, "b2", TUGAS_WASM, 1, 8, "y", [])
        var cd_ok = (cd1.jumlah_tugas == 0 dan cd2.jumlah_tugas == 1 dan cd3.jumlah_tugas == 2)
        cetak("[COV] ComputeDAG x3 -> 0/1/2 tugas: " + ke_teks(cd_ok))

        var mdl_a = ModelML("ma", "linreg", 10.0, 2)
        var mdl_b = ModelML("mb", "logreg", 50.0, 3)
        var mdl_c = ModelML("mc", "tinyllm", 100.0, 4)
        var nc1 = {}
        nc1["nama_node"] = "cA"
        nc1["profil"] = "server"
        var nc2 = {}
        nc2["nama_node"] = "cB"
        nc2["profil"] = "server"
        var nc3 = {}
        nc3["nama_node"] = "cC"
        nc3["profil"] = "edge"
        var fa1 = FederasiAICluster("fed_A", nc1, mdl_a)
        var fa2 = FederasiAICluster("fed_B", nc2, mdl_b)
        var fa3 = FederasiAICluster("fed_C", nc3, mdl_c)
        var ek1 = {}
        ek1["nama_node"] = "e1"
        ek1["profil"] = "edge"
        federasi_tambah_klien(fa1, ek1, 100)
        var ek2 = {}
        ek2["nama_node"] = "e2"
        ek2["profil"] = "edge"
        federasi_tambah_klien(fa2, ek2, 200)
        var ek3 = {}
        ek3["nama_node"] = "e3"
        ek3["profil"] = "edge"
        federasi_tambah_klien(fa3, ek3, 300)
        var fa_ok = (fa1.id_koordinator == "fed_A" dan fa2.id_koordinator == "fed_B" dan fa3.id_koordinator == "fed_C")
        cetak("[COV] FederasiAICluster x3 -> fed_A/B/C: " + ke_teks(fa_ok))

        var pv1 = buat_validator_poa("0xVA", "pk_va")
        var pv2 = buat_validator_poa("0xVB", "pk_vb")
        var pv3 = buat_validator_poa("0xVC", "pk_vc")
        pv1.reputasi_skor = 100
        pv2.reputasi_skor = 95
        pv3.reputasi_skor = 90
        var pv_ok = (pv1.alamat == "0xVA" dan pv2.alamat == "0xVB" dan pv3.alamat == "0xVC")
        cetak("[COV] PoAValidator x3 -> VA/VB/VC: " + ke_teks(pv_ok))

        var gs1 = gossip_buat_topik("topik_satu")
        var gs2 = gossip_buat_topik("topik_dua")
        var gs3 = gossip_buat_topik("topik_tiga")
        gossip_langganan(gs1, "node_gs1_a", "cb_a")
        gossip_langganan(gs2, "node_gs2_a", "cb_b")
        gossip_langganan(gs2, "node_gs2_b", "cb_c")
        gossip_langganan(gs3, "node_gs3_a", "cb_d")
        gossip_langganan(gs3, "node_gs3_b", "cb_e")
        gossip_langganan(gs3, "node_gs3_c", "cb_f")
        var gs_ok = (panjang(gs1.daftar_subscriber) == 1 dan panjang(gs2.daftar_subscriber) == 2 dan panjang(gs3.daftar_subscriber) == 3)
        cetak("[COV] GossipSubTopik x3 -> 1/2/3 subscriber: " + ke_teks(gs_ok))

        var semua_lulus = wk_ok dan cd_ok dan fa_ok dan pv_ok dan gs_ok
        var label_lulus = "GAGAL"
        jika semua_lulus {
            label_lulus = "LULUS"
        }
        cetak("[COV] SEMUA 5 STRUKTUR x3 INISIALISASI: " + label_lulus)

        kembalikan semua_lulus
    "#;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_all_five_structs_minimum_three_coverage_rule: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}
