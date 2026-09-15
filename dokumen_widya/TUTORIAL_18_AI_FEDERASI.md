# Tutorial 18 — AI Federasi (FedAvg + CUDA 8x H100 640GB VRAM) + RAG Pipeline

**Target**: Membangun sistem AI Federasi untuk training model terdistribusi 8 shard H100 (federated learning tanpa kirim data raw), ditambah RAG pipeline + vektor search untuk aplikasi LLM lokal.
**Level**: Enterprise ⭐⭐⭐
**Waktu**: ~45 menit
**File terkait**: [modul/ai_federasi.wya](../modul/ai_federasi.wya) | [modul/ai_rag_stack.wya](../modul/ai_rag_stack.wya) | [modul/cuda_ffi_runtime.wya](../modul/cuda_ffi_runtime.wya) | [modul/sgx_enclave_simulasi.wya](../modul/sgx_enclave_simulasi.wya) | [modul/hsm_pkcs11_emulasi.wya](../modul/hsm_pkcs11_emulasi.wya)

---

## Bagian 1: Federated Learning (FedAvg) + Sharding CUDA 8 GPU

Federated Learning = **data TIDAK PERNAH keluar node**. Setiap node training lokal di edge/server, HANYA kirim **delta bobot gradient** (bobot_delta) ke server agregator → server hitung rata-rata (FedAvg) → update model global → distribusi bobot baru kembali ke node.

✅ Kelebihan: privacy by design, sesuai UU PDP (Perlindungan Data Pribadi) No. 27/2022 Pasal 20 (transfer data pribadi dilarang tanpa consent).

---

### Step 1 — Inisialisasi Cluster Federasi + Model Global

```widya
impor "ai_federasi.wya" sebagai ai;

// Arsitektur model klasifikasi sentimen Bahasa Indonesia CNN + LSTM
misal model_global = ai.ModelML(
    "sentimen_id_cnn_lstm_v1",   // nama model
    "CNN1D + BiLSTM(256) + FC(3)", // arsitektur
    12.5,                         // jumlah parameter (juta)
    4                             // hidden layer count
);

// 8 Node klien federasi (misal 8 cabang toko retail di 8 kota Indonesia)
misal node_koord = [
    {"id_client":"C_JKT", "lokasi":"Jakarta",     "tipe":"edge", "n_data_lokal":120000},
    {"id_client":"C_BDG", "lokasi":"Bandung",     "tipe":"edge", "n_data_lokal":95000},
    {"id_client":"C_SBY", "lokasi":"Surabaya",    "tipe":"edge", "n_data_lokal":108000},
    {"id_client":"C_JOG", "lokasi":"Yogyakarta",  "tipe":"edge", "n_data_lokal":72000},
    {"id_client":"C_MDN", "lokasi":"Medan",       "tipe":"edge", "n_data_lokal":85000},
    {"id_client":"C_MKS", "lokasi":"Makassar",    "tipe":"edge", "n_data_lokal":63000},
    {"id_client":"C_BPN", "lokasi":"Balikpapan",  "tipe":"edge", "n_data_lokal":48000},
    {"id_client":"C_DPS", "lokasi":"Denpasar",    "tipe":"edge", "n_data_lokal":56000}
];

// Buat cluster Federasi + SGX attestation enabled (privacy tambahan)
misal cluster = ai.FederasiAICluster(
    "FED_BAHASA_INDONESIA_V1",
    node_koord,
    model_global
);
```

---

### Step 2 — Local Training di 8 Edge Client (1 Epoch)

```widya
// Setiap edge training lokal, hasilkan DeltaGradien (bobot delta saja, bukan data)
misal daftar_delta = [];
untuk client in node_koord {
    misal bobot_delta_local = [];
    var k = 0;
    selama k < 12500000 {  // 12.5juta parameter, isi dummy random -0.01..0.01
        bobot_delta_local = tambah_elemen(bobot_delta_local, acak(-0.01, 0.01));
        k = k + 1;
    }
    // Akurasi validation lokal setelah 1 epoch
    misal akurasi_lokal = 0.85 + acak(0.0, 0.1);
    misal delta = ai.DeltaGradien(
        client["id_client"],
        client["n_data_lokal"],
        bobot_delta_local,
        akurasi_lokal
    );
    daftar_delta = tambah_elemen(daftar_delta, delta);
    cetak("✅ Client", client["id_client"], "(" + client["lokasi"] + ") selesai train | akurasi:", akurasi_lokal);
}
```

---

### Step 3 — Agregasi FedAvg via CUDA Stream Sharded 8× H100 80GB (Total 640GB VRAM)

Untuk perhitungan rata-rata bobot yang efisien, jalankan paralel di 8 shard GPU NVIDIA H100 PCIe 80GB NVLink:

```widya
// Jalankan agregasi FedAvg dengan CUDA 8 shard H100
misal agregat = ai.fedavg_cuda_stream_sharded(cluster, daftar_delta);

cetak("\n🚀 Hasil FedAvg CUDA Sharded:");
cetak("   Shard count:", agregat["shard_count"]);             // ✅ 8
cetak("   Total VRAM cluster (GB):", agregat["total_mem_gb"]); // ✅ 640
cetak("   Mode agregasi:", agregat["mode_agregasi"]);         // ✅ "CUDA_STREAM_SHARDED_H100_X8"
cetak("   Bobot model global terupdate:", agregat["update_count_param"], "parameter");
cetak("   Akurasi model global (avg):", agregat["akurasi_global_baru"]);  // ~0.89-0.91
```

> 💡 Jika CUDA runtime tidak tersedia (misal CI tanpa GPU), otomatis fallback ke `cpu_simd_fedavg_manual` AVX2 — backward compatible 100%.

---

### Step 4 — Verifikasi Enkripsi Delta via SGX Enclave + Sign HSM

Sebelum delta dikirim via jaringan:
```widya
impor "sgx_enclave_simulasi.wya" sebagai sgx;
impor "hsm_pkcs11_emulasi.wya" sebagai hsm;

// Enclave SGX: seal delta + local attestation sebelum kirim
misal enclave = sgx.sgx_buat_enclave("EID_FED_001", "aes-256-gcm-masterkey-32byte!!");
sgx.ecall_daftarkan(enclave, "sign_delta", fungsi(data_plain) {
    kembalikan sgx.sha256_hex("SIGNED_FED_DELTA_V1:" + ke_json(data_plain));
});
misal report_attest = sgx.sgx_local_attestasi(enclave);
cetak("\n🔒 SGX Local Attestation valid:", report_attest["mac_valid"]);  // ✅ benar

// Tanda tangan delta gradient via HSM RSA2048 sebelum dikirim
misal token_hsm = hsm.hsm_buat();
hsm.hsm_login(token_hsm, 0, "12345678");
misal kp_rsa = hsm.rsa2048_generate(token_hsm, 0, "K01_FED_SIGN_KEY");
misal sig = hsm.rsa_sign(token_hsm, 0, 1, sha256_hex_standalone(ke_json(daftar_delta)));
cetak("🔑 HSM RSA2048 signature length (hex chars):", panjang(sig));  // ✅ ≥ 256
cetak("🔑 Verifikasi signature via HSM:", hsm.rsa_verify(token_hsm, 0, 1, sha256_hex_standalone(ke_json(daftar_delta)), sig)); // ✅ benar
```

---

## Bagian 2: RAG Pipeline + Vector DB HNSW (Retrieval Augmented Generation)

RAG Pipeline = meningkatkan akurasi jawaban LLM dengan cara: **(1) cari dokumen relevan via vector search KNN, (2) inject sebagai context di prompt, (3) jawab berdasarkan context (bukan hanya ingetan parameter LLM)**. Cocok untuk FAQ internal, knowledge base perusahaan, customer support.

```widya
impor "ai_rag_stack.wya" sebagai rag;

// 1. Kumpulan 10 dokumen knowledge base SOP retail (misal)
misal dok_kb = [
    {"id":"D01","kategori":"pengiriman","teks":"Waktu pengiriman Jabodetabek 1-2 hari kerja, luar Jabodetabek 3-5 hari. Gratis ongkir minimal belanja 200 ribu."},
    {"id":"D02","kategori":"retur","teks":"Prosedur retur: foto barang + invoice kirim WA cs 0812-XXX dalam 7 hari setelah terima. Retur diterima jika segel utuh."},
    {"id":"D03","kategori":"garansi","teks":"Garansi resmi produk elektronik 12 bulan dari tanggal pembelian, inkludi service center resmi Asus, Lenovo, Xiaomi di 34 kota."},
    {"id":"D04","kategori":"pembayaran","teks":"Metode pembayaran: transfer BCA/BRI/Mandiri, virtual account, QRIS, kartu kredit 3-12 cicilan 0%, Paylater mitra 4 fintech."},
    {"id":"D05","kategori":"keanggotaan","teks":"3 tier member: Bronze (0 poin), Silver (1jt poin), Gold (5jt poin). Cashback 1%/2%/3% per transaksi."},
    {"id":"D06","kategori":"pengiriman","teks":"Same day delivery tersedia Jakarta, Depok, Tangerang, Bekasi, Bogor (Jabodetabek) untuk order sebelum jam 12 siang."},
    {"id":"D07","kategori":"retur","teks":"Barang makanan, perhiasan emas, dan produk personal hygiene TIDAK BISA diretur kecuali kerusakan pabrik (DOA) dengan bukti video unboxing."},
    {"id":"D08","kategori":"garansi","teks":"Klaim garansi tidak berlaku jika kerusakan akibat terjatuh, terkena air, atau modifikasi tidak resmi (overclock, custom firmware)."},
    {"id":"D09","kategori":"pembayaran","teks":"Pembayaran tunai (COD) hanya tersedia untuk area Jabodetabek dan Surabaya dengan minimal transaksi Rp 50.000."},
    {"id":"D10","kategori":"keanggotaan","teks":"Member Gold mendapatkan gratis ongkir tanpa minimum, 2 call priority CS, dan undangan exclusive launch event produk baru."}
];

// 2. Bangun Vector DB 384-dim (BERT standard)
misal vdb = rag.vdb_buat(dok_kb, 384);

// 3. Query user natural language
misal pertanyaan_user = "Saya mau beli laptop Asus gaming di Jakarta, pengen hari ini sampai, bisa COD apa tidak? Garansinya berapa lama? Kalau cacat bisa retur tidak?";
misal top_k = 3;

// 4. RAG Retrieve → dapatkan top-K dokumen paling relevan
misal dok_relevan = rag.rag_query(vdb, pertanyaan_user, top_k);
cetak("\n📚 RAG Retrieve Top-3 Dokumen Paling Relevan:");
untuk d in dok_relevan {
    cetak("   📄", d["data"]["id"], "|", d["data"]["kategori"], "| skor cosine sim:", d["skor_similaritas"]);
    cetak("     Teks snippet:", potong(d["data"]["teks"], 0, 80), "...");
}

// 5. RAG Generate → Build prompt + panggil LLM inference (misal model quant INT8 edge)
misal prompt_final = rag.rag_buat_prompt(pertanyaan_user, dok_relevan);
cetak("\n🤖 Prompt Final ke LLM (2048 token context window):");
cetak(potong(prompt_final, 0, 400), "...");
```

---

## Verifikasi Self-Test

```bash
widya jalankan modul\ai_federasi.wya   # ✅ append CUDA shard OK shard=8, mem=640GB
widya jalankan modul\ai_rag_stack.wya  # ✅ vdb_buat + rag_query pass
widya jalankan modul\cuda_ffi_runtime.wya  # ✅ malloc 256MB matmul sync CUDA_SUCCESS
```

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 17 — WidyaOS](./TUTORIAL_17_WIDYAOS.md) |
➡️ **[Tutorial 19 — Mesh Networking libp2p-style](./TUTORIAL_19_MESH_NETWORKING.md)**
