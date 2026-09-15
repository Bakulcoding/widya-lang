# Tutorial 19 — Mesh Networking libp2p-style: Kademlia DHT, Raft Consensus, GossipSub

**Target**: Menjalankan jaringan P2P mesh (peer-to-peer) terdistribusi untuk sistem DePIN (Decentralized Physical Infrastructure Networks), blockchain, atau node IoT edge. Tiga protokol utama: Kademlia DHT (content discovery), Raft (consensus / leader election), GossipSub (pub/sub messaging broadcast 1 hop / many hop fanout).
**Level**: Enterprise ⭐⭐⭐
**Waktu**: ~40 menit
**File terkait**: [modul/mesh_libp2p_style.wya](../modul/mesh_libp2p_style.wya) | [modul/sistem_terdistribusi.wya](../modul/sistem_terdistribusi.wya) | [modul/privasi_keamanan.wya](../modul/privasi_keamanan.wya) | [modul/event_streaming_bus.wya](../modul/event_streaming_bus.wya)

---

## Tiga Protokol Mesh: Use Case Masing-Masing

| Protokol | Tipe Konsistensi | Latency Tipikal | Use Case di Widya Mesh |
|---------|-----------------|----------------|----------------------|
| **Kademlia DHT** (Distributed Hash Table) | Eventual Consistency | < 50 ms lookup | Penemuan kontrak, discovery node, mapping key→value global terdistribusi |
| **Raft Consensus** | Strong Consistency (Linearizable) | ~100-500 ms per commit (tergantung quorum 2F+1) | Konfigurasi global cluster, state machine state ter-replikasi, leader election scheduler |
| **GossipSub (Pub-Sub)** | At-Least-Once Delivery + Dedupe | < 100 ms (fanout D=6 hop) | Broadcast transaksi blockchain, notifikasi IoT event realtime, chat group |

---

## 1. Kademlia DHT: Peer Discovery + Key/Value Terdistribusi

Kademlia = struktur data XOR metric distance, 160-bit node ID, bucket k-bucket, FIND_NODE / FIND_VALUE RPC.

```widya
impor "mesh_libp2p_style.wya" sebagai mesh;

// ===== Inisialisasi 6 Node P2P tersebar di 6 kota Indonesia =====
misal peers = [];
peers = tambah_elemen(peers, mesh.node_dht_buat("N_JKT", "10.1.0.1", 4001));
peers = tambah_elemen(peers, mesh.node_dht_buat("N_BDG", "10.1.1.1", 4001));
peers = tambah_elemen(peers, mesh.node_dht_buat("N_SBY", "10.1.2.1", 4001));
peers = tambah_elemen(peers, mesh.node_dht_buat("N_JOG", "10.1.3.1", 4001));
peers = tambah_elemen(peers, mesh.node_dht_buat("N_MDN", "10.1.4.1", 4001));
peers = tambah_elemen(peers, mesh.node_dht_buat("N_MKS", "10.1.5.1", 4001));

// Hubungkan semua node secara mesh (bootstrap 2 seed node per peer)
misal i = 0;
selama i < panjang(peers) {
    misal j = 0;
    selama j < panjang(peers) {
        jika i != j {
            mesh.dht_sambungkan(peers[i], peers[j]);
        }
        j = j + 1;
    }
    i = i + 1;
}

// ===== Tulis data kontrak ke DHT di node N_JKT =====
misal k1 = "kontrak/widya/supply_chain/v0xabc123";
misal v1 = {"id_komoditas":"KOPI_ARABIKA_GAYO_1KG", "harga_berlangganan":125000, "ttl_block":9999999999, "merk_root":"0xMERKLE_ROOT_SHA256_64HEX"};
mesh.dht_simpan(peers[0], k1, v1);
cetak("✅ DHT simpan key:", k1, "di node", peers[0]["id"]);

// ===== Cari / Lookup key dari node N_MKS (berbeda 3 hop XOR!) =====
misal hasil_lookup = mesh.dht_cari(peers[5], k1);
cetak("🔍 DHT lookup dari node N_MKS (5 hop jauh):", hasil_lookup["ditemukan"]);  // ✅ benar
cetak("   Value:", hasil_lookup["value"]["id_komoditas"]); // ✅ "KOPI_ARABIKA_GAYO_1KG"
cetak("   XOR distance N_JKT ↔ N_MKS (hex):", hasil_lookup["xor_distance_hex"]);
cetak("   Jumlah hop RPC yang dibutuhkan:", hasil_lookup["jumlah_hop"]);
```

---

## 2. Raft Consensus: Leader Election + Replication Log (Strong Consistency)

Raft = 5 node server cluster, 1 Leader + 2 Follower active commit. Quorum write = 2F+1 = 3 node setuju → dipastikan tidak ada split brain (tidak mungkin 2 leader di term sama).

```widya
impor "sistem_terdistribusi.wya" sebagai td;

// 5 Node Raft untuk state machine "konfigurasi_global_widya"
misal nodes_raft = [];
var id_node_raft = ["R1","R2","R3","R4","R5"];
var ip_node_raft = ["192.168.0.11","192.168.0.12","192.168.0.13","192.168.0.14","192.168.0.15"];
var idx_r = 0;
selama idx_r < 5 {
    nodes_raft = tambah_elemen(nodes_raft, td.raft_node_buat(id_node_raft[idx_r], ip_node_raft[idx_r], 7000 + idx_r));
    idx_r = idx_r + 1;
}
td.raft_cluster_init(nodes_raft);

// ============= PEMILIHAN LEADER (ELECTION) =============
cetak("\n🗳️  Memulai Raft Election Term 1...");
misal leader = td.raft_eleksi_pemimpin(nodes_raft);
cetak("✅ Leader Terpilih:", leader["id_node"], "| Vote count:", leader["jumlah_suara"], "/5");

// ============= WRITE KONFIGURASI KE STATE MACHINE =============
misal cmd_baru = {"op":"SET","key":"max_conn_per_gateway","value":65535};
misal commit = td.raft_commit(nodes_raft, cmd_baru);
cetak("\n📝 Commit state machine:", commit["status"]);   // ✅ "COMMITTED_QUORUM_3OF5"
cetak("   Index log Raft:", commit["log_index"]);
cetak("   Leader term:", commit["term"]);

// ============== VERIFIKASI REPLIKASI DI SEMUA FOLLOWER =============
cetak("\n🔁 Verifikasi state machine sama di kelima node:");
untuk n in nodes_raft {
    misal cek = td.raft_baca_state_mesin(n, "max_conn_per_gateway");
    cetak("   Node", n["id_node"], "| role:", n["role"], "| value:", cek);
}
// Semua node return 65535 → Strong Consistency tercapai ✅
```

---

## 3. GossipSub: Pub/Sub Mesh Message Broadcast

GossipSub = (1) Random Gossip pull-based, (2) Subscription mesh D-6 (setiap peer terhubung 6 peer lain di topic mesh). Cocok untuk event realtime.

```widya
// Buat 3 Topic GossipSub
var topic_transaksi = "widya.chain.transaksi.mainnet.v1";
var topic_iot_tegis = "widya.iot.sensor.suhu_pabrik.v1";
var topic_chat_admin = "widya.internal.admin.darurat.v1";

// 6 peer sebelumnya subscribe ke topic masing-masing
i = 0;
selama i < 6 {
    mesh.gs_subscribe(peers[i], topic_transaksi);
    jika i < 3 { mesh.gs_subscribe(peers[i], topic_iot_tegis); }
    jika i < 2 { mesh.gs_subscribe(peers[i], topic_chat_admin); }
    i = i + 1;
}

// ============== PUBLISH 3 EVENT ==============
cetak("\n📡 Publish 3 event GossipSub:");

// Event transaksi blockchain → broadcast ke SEMUA 6 peer
var tx = {"hash_tx":"0xTX_SHA256_64HEX","from":"0xALICE","to":"0xBOB","value_wid":8888};
var r_tx = mesh.gs_publish(peers[0], topic_transaksi, tx);
cetak("   Tx broadcasted → fanout count:", r_tx["fanout_count"]);   // ✅ 6 peer terima

// Event IoT sensor → 3 peer
var ev_sensor = {"id_sensor":"S_001","suhu_c":31.2,"kelembaban_pct":72.5,"ts":1726352222};
var r_sensor = mesh.gs_publish(peers[1], topic_iot_tegis, ev_sensor);
cetak("   IoT sensor → subscribers count:", r_sensor["delivered_peers"]);  // ✅ 3

// Event chat admin darurat → 2 peer
var ev_darurat = {"severity":"CRITICAL","pesan":"UPS di DC JKT failure, failover ke DR site"};
var r_darurat = mesh.gs_publish(peers[0], topic_chat_admin, ev_darurat);
cetak("   Admin emergency → delivered:", r_darurat["delivered_peers"]);  // ✅ 2
```

---

## 4. Gabungan Ketiga Protokol: DePIN Use Case Lengkap

Contoh integrasi: **DePIN Solar Panel 1000 node**.

| Fase | Protokol yang Dipakai | Aksi |
|------|---------------------|------|
| 1 | Kademlia DHT | 1000 node solar panel bootstrap, menemukan peer di sekitar XOR distance <20 bit |
| 2 | Raft Consensus | 5 node validator terpilih, leader election, komit harga listrik token WID/kWh → state machine |
| 3 | GossipSub | Setiap panel publish event `{panel_id, watt, timestamp}` setiap 5 detik ke topic `depin.solar.meter.v1` |
| 4 | Kembali ke Kademlia | Hasil agregasi harian disimpan sebagai key `depin/solar/daily/<YYYY-MM-DD>` → lookup cepat audit |

Verifikasi end-to-end:
```bash
widya jalankan modul\mesh_libp2p_style.wya
widya jalankan modul\sistem_terdistribusi.wya
widya jalankan contoh\154_solusi_iot_cloud_native_microservice.wya
```

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 18 — AI Federasi & RAG](./TUTORIAL_18_AI_FEDERASI.md) |
➡️ **[Tutorial 20 — Semua 9 Layer 100% SIAP](./TUTORIAL_20_SEMUA_LAYER_100PERSEN.md)**
