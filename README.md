# 🇮🇩 Widya-Lang (Bahasa Pemrograman Widya)

**Widya-Lang** adalah bahasa pemrograman modern, mandiri (*self-hosted & native compilable*), lintas platform (**Windows, Linux, macOS, BSD, Android/iOS, & WebAssembly**), ekspresif, dan ramah pengguna dengan sintaks berbasis **Bahasa Indonesia**.

---

## 🌐 Dukungan Lintas Platform (Cross-Platform)

Widya-Lang dirancang dengan arsitektur portabel standar POSIX dan Win32:

| Sistem Operasi | Arsitektur Target | Format Biner | Status |
| :--- | :--- | :--- | :--- |
| **Windows** | `x86_64`, `i686`, `aarch64` | `widya.exe` (PE) | ✅ Didukung Penuh |
| **Linux (Ubuntu/Debian/Arch/Fedora)** | `x86_64-unknown-linux-gnu` | `widya` (ELF) | ✅ Didukung Penuh |
| **Linux (Alpine / Musl Static)** | `x86_64-unknown-linux-musl` | `widya` (Zero-Dependency Static) | ✅ Didukung Penuh |
| **macOS (Apple Silicon M1/M2/M3/M4)** | `aarch64-apple-darwin` | `widya` (Mach-O) | ✅ Didukung Penuh |
| **macOS (Intel Mac)** | `x86_64-apple-darwin` | `widya` (Mach-O) | ✅ Didukung Penuh |
| **Web Browser (Online Sandbox)** | `wasm32-unknown-unknown` | `widya.wasm` | ✅ Didukung |

---

## ⚡ Cara Instalasi Cepat

### 🐧 Di Linux & 🍎 Di macOS
Cukup jalankan satu baris perintah di Terminal Anda:
```bash
git clone https://github.com/widya-lang/widya.git
cd widya
chmod +x install.sh && ./install.sh
```

### 🪟 Di Windows
Jalankan di PowerShell:
```powershell
git clone https://github.com/widya-lang/widya.git
cd widya
.\install.ps1
```

---

## 🛠️ Cara Melakukan Cross-Compilation (Kompilasi Silang)

Anda dapat mengompilasi biner untuk sistem operasi lain langsung dari komputer Anda saat ini:

```bash
# 1. Menambahkan target kompilasi (misal untuk Linux Musl / macOS)
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-apple-darwin

# 2. Mengompilasi biner untuk Linux dari Windows
cargo build --release --target x86_64-unknown-linux-musl

# 3. Mengompilasi biner untuk Mac M1/M2/M3 dari Windows
cargo build --release --target aarch64-apple-darwin
```

---

## 💻 Panduan Perintah CLI

```bash
# 1. Menjalankan skrip langsung (Interpreter)
widya run contoh/01_halo_dunia.wya

# 2. Mengompilasi menjadi .EXE Mandiri (Native Direct Binary)
widya kompilasi contoh/02_fibonacci.wya -o fibonacci.exe

# 3. Mengompilasi langsung ke biner WebAssembly (.wasm) & HTML Web Runner
widya wasm contoh/18_makro_dan_ownership.wya -o app.wasm

# 4. Mengompilasi kode ke standar LLVM IR (.ll) untuk optimasi Clang/LTO
widya llvm contoh/16_c_ffi_dan_baremetal.wya

# 5. Mengompilasi program ke WebGPU Shading Language (WGSL) Compute Shader
widya gpu contoh/25_gpu_compute_shader.wya

# 6. Mengompilasi kode program ke Linux Kernel eBPF C Source
widya ebpf contoh/27_raft_dan_ebpf.wya

# 7. Menjalankan dengan Pemantau Perubahan Berkas Otomatis (Live Hot-Reload)
widya tonton contoh/01_halo_dunia.wya

# 8. Menjalankan dengan Mesin Optimasi Just-In-Time (JIT Execution)
widya jit contoh/20_simd_dan_jit.wya

# 9. Memaketkan kode menjadi Single Standalone Binary atau Mobile Webview App
widya kemas contoh/21_aplikasi_multiplatform.wya
widya kemas contoh/21_aplikasi_multiplatform.wya --mobile

# 10. Menjalankan Integrated Test Suite (#[uji])
widya uji contoh/19_uji_dan_benchmark.wya

# 11. Menjalankan Micro-Benchmarking Suite (#[tolak_ukur])
widya tolak-ukur contoh/19_uji_dan_benchmark.wya

# 12. Auto-Format Kode & Linter Statis
widya format contoh/19_uji_dan_benchmark.wya --tulis
widya periksa contoh/19_uji_dan_benchmark.wya

# 13. Mengompilasi Dokumentasi API HTML Otomatis dari doc comments (///)
widya dok contoh/20_simd_dan_jit.wya -o dokumen_widya

# 14. Membuka IDE Mandiri Widya Studio (Modern Monaco Desktop/Web)
widya studio

# 15. Publikasi & Manajemen Paket Global (WPM)
widya inisialisasi ProyekSaya
widya publikasi
widya pasang
```

---

## 🎨 Widya Studio (IDE Khusus Mandiri) & Ekstensi VS Code

Tersedia dua opsi alat pengembangan terpadu (IDE) untuk programmer Widya:

1. **Widya Studio IDE (Mandiri / Standalone)**:
   - Cukup jalankan perintah:
     ```bash
     widya studio
     ```
   - Otomatis membuka IDE modern berbasis Monaco Editor (mesin inti VS Code) lengkap dengan fitur:
     - **Pewarnaan Sintaks Khusus Widya** (*Dark Theme Catppuccin*)
     - **Penjelajah Berkas & Proyek (File Explorer)**
     - **Konsol Output & Eksekusi Terintegrasi (Tombol Run F5)**
     - **Diagnostics & Error Checker Real-time**

2. **Ekstensi Resmi Visual Studio Code (`vscode-extension/`)**:
   - Terletak di direktori [`vscode-extension`](file:///c:/widya-lang/vscode-extension):
     - `syntaxes/widya.tmLanguage.json` (Grammar TextMate lengkap)
     - `snippets/snippets.json` (Snippets otomatis fungsi, enum, sifat, cocokkan, UI)
     - `language-configuration.json` (Autoclose bracket & komentar)
   - Dapat dipasang ke VS Code cukup dengan menyalin folder `vscode-extension` ke folder `.vscode/extensions/` komputer Anda.

---

## 📂 Katalog Berkas Contoh

| Berkas | Deskripsi |
| :--- | :--- |
| [`contoh/01_halo_dunia.wya`](file:///c:/widya-lang/contoh/01_halo_dunia.wya) | Dasar bahasa, variabel, dan tipe data |
| [`contoh/02_fibonacci.wya`](file:///c:/widya-lang/contoh/02_fibonacci.wya) | Deret Fibonacci rekursif dan iteratif |
| [`contoh/06_oop_dan_struktur.wya`](file:///c:/widya-lang/contoh/06_oop_dan_struktur.wya) | OOP (Struktur Rekening Bank, metode, `ini`) |
| [`contoh/07_penanganan_galat.wya`](file:///c:/widya-lang/contoh/07_penanganan_galat.wya) | Penanganan eksepsi (`coba`, `tangkap`, `lempar`) |
| [`contoh/08_impor_modul.wya`](file:///c:/widya-lang/contoh/08_impor_modul.wya) | Import modul multi-berkas |
| [`contoh/09_aplikasi_flutter.wya`](file:///c:/widya-lang/contoh/09_aplikasi_flutter.wya) | **WidyaUI (Flutter)**: Widget Tree deklaratif |
| [`contoh/10_blockchain_dan_mining.wya`](file:///c:/widya-lang/contoh/10_blockchain_dan_mining.wya) | **WidyaChain**: Mining PoW & Transaksi Dompet |
| [`contoh/11_smart_contract_token.wya`](file:///c:/widya-lang/contoh/11_smart_contract_token.wya) | **Smart Contract**: Token WIDYA-20 |
| [`contoh/12_kompiler_mandiri.wya`](file:///c:/widya-lang/contoh/12_kompiler_mandiri.wya) | **Kompiler Mandiri**: Self-hosted compiler |
| [`contoh/13_fitur_rust.wya`](file:///c:/widya-lang/contoh/13_fitur_rust.wya) | **Fitur Rust**: Pola `Result` (Ok/Err), `Option` (Ada/Kosong), `pastikan` (assert) |
| [`contoh/14_mesin_database.wya`](file:///c:/widya-lang/contoh/14_mesin_database.wya) | **WidyaDB Engine**: Relasional (PostgreSQL) & Dokumen (MongoDB/NoCode) |
| [`contoh/15_ekosistem_canggih.wya`](file:///c:/widya-lang/contoh/15_ekosistem_canggih.wya) | **WidyaAI & Http & Game**: Tensor Matriks, REST API, dan 2D Game Canvas |
| [`contoh/16_c_ffi_dan_baremetal.wya`](file:///c:/widya-lang/contoh/16_c_ffi_dan_baremetal.wya) | **C-FFI & Bare-Metal**: Interop C ABI (`eksternal "C"`) dan Akses Memori Volatil |
| [`contoh/17_websocket_realtime.wya`](file:///c:/widya-lang/contoh/17_websocket_realtime.wya) | **WebSocket Real-Time Engine**: Full-duplex live stream, broadcast & chat |
| [`contoh/18_makro_dan_ownership.wya`](file:///c:/widya-lang/contoh/18_makro_dan_ownership.wya) | **World-Class Metaprogramming & Safety**: Makro Procedural `#[turunkan]` & Borrow Checker |
| [`contoh/19_uji_dan_benchmark.wya`](file:///c:/widya-lang/contoh/19_uji_dan_benchmark.wya) | **World-Class Tooling Suite**: Unit Test Runner (`#[uji]`), Benchmark (`#[tolak_ukur]`), & DocGen (`///`) |
| [`contoh/20_simd_dan_jit.wya`](file:///c:/widya-lang/contoh/20_simd_dan_jit.wya) | **WidyaSIMD & JIT Engine**: Komputasi Vektor Paralel (`VektorF32x4`) & Optimasi Runtime |
| [`contoh/21_aplikasi_multiplatform.wya`](file:///c:/widya-lang/contoh/21_aplikasi_multiplatform.wya) | **Multiplatform & Mobile Bridge**: Bundler Desktop/Mobile Webview & Native IPC Bridge |
| [`contoh/22_graphql_dan_grpc.wya`](file:///c:/widya-lang/contoh/22_graphql_dan_grpc.wya) | **Enterprise API**: GraphQL Query Execution & gRPC Microservices RPC Engine |
| [`contoh/23_shared_memory_dan_regex.wya`](file:///c:/widya-lang/contoh/23_shared_memory_dan_regex.wya) | **Low-Level Performance**: Zero-Copy Shared Memory IPC (`WidyaShm`) & Native Regex Engine |
| [`contoh/24_aktor_dan_aliran.wya`](file:///c:/widya-lang/contoh/24_aktor_dan_aliran.wya) | **Actor & Reactive Streams**: Actor Model Concurrency (`WidyaActor`) & Reactive Data Stream |
| [`contoh/25_gpu_compute_shader.wya`](file:///c:/widya-lang/contoh/25_gpu_compute_shader.wya) | **Hardware Acceleration**: WebGPU WGSL Compute Shaders & GPU Matrix Parallel Pipeline |
| [`contoh/26_kuantum_dan_zkproof.wya`](file:///c:/widya-lang/contoh/26_kuantum_dan_zkproof.wya) | **Next-Gen Frontier**: Quantum Circuit Simulator (`WidyaQuantum`) & Zero-Knowledge Proofs |
| [`contoh/27_raft_dan_ebpf.wya`](file:///c:/widya-lang/contoh/27_raft_dan_ebpf.wya) | **Distributed & Kernel Ops**: Distributed Raft Consensus (`WidyaRaft`) & Linux Kernel eBPF |
| [`contoh/28_neural_dan_3d.wya`](file:///c:/widya-lang/contoh/28_neural_dan_3d.wya) | **AI & Spatial Computing**: Deep Learning LLM Self-Attention (`WidyaNeural`) & WebGL 3D Scene (`Widya3D`) |
| [`contoh/29_crdt_dan_debugger.wya`](file:///c:/widya-lang/contoh/29_crdt_dan_debugger.wya) | **Real-Time & Tooling**: CRDT Collaborative Document Sync (`WidyaCRDT`) & Time-Travel Reversible Debugger (`widya debug`) |
| [`contoh/30_fhe_dan_lsm_tree.wya`](file:///c:/widya-lang/contoh/30_fhe_dan_lsm_tree.wya) | **Privacy & Storage Engine**: Fully Homomorphic Encryption (`WidyaFHE`) & LSM-Tree Engine (`WidyaLSM`) |
| [`contoh/31_geo_dan_hotpatch.wya`](file:///c:/widya-lang/contoh/31_geo_dan_hotpatch.wya) | **Spatial & Runtime Ops**: Geospatial GIS & Haversine (`WidyaGeo`) & Live Function Hot-Patching (`WidyaHotPatch`) |
| [`contoh/32_audio_dan_timeseries.wya`](file:///c:/widya-lang/contoh/32_audio_dan_timeseries.wya) | **Audio & Analytics**: Sintesis audio DSP, ADSR shaping, ekspor WAV, serta analisis deret waktu & peramalan tren (`WidyaAudio` & `WidyaTimeSeries`) |
| [`contoh/33_bloom_dan_graphengine.wya`](file:///c:/widya-lang/contoh/33_bloom_dan_graphengine.wya) | **Probabilistic & Graph**: Bloom Filter, HyperLogLog, Dijkstra Shortest Path, & PageRank (`WidyaBloom` & `WidyaGraphEngine`) |
| [`contoh/34_diff_dan_genetika.wya`](file:///c:/widya-lang/contoh/34_diff_dan_genetika.wya) | **Diffing & Evolusi**: Myers Diffing, Unified Patch, & Algoritma Genetika (`WidyaDiff` & `WidyaGenetic`) |
| [`contoh/35_semantic_dan_smartcontract.wya`](file:///c:/widya-lang/contoh/35_semantic_dan_smartcontract.wya) | **AI & Smart Contract**: Vector Cosine Search & Virtual Deterministic Contract Ledger (`WidyaSemanticSearch` & `WidyaSmartContract`) |
| [`contoh/36_fisika_dan_ocr.wya`](file:///c:/widya-lang/contoh/36_fisika_dan_ocr.wya) | **Physics & Vision**: Simulasi Dinamika Benda Tegar 2D & Pengenalan Karakter Matriks OCR (`WidyaPhysics2D` & `WidyaOCR`) |
| [`contoh/37_regexjit_dan_markdown.wya`](file:///c:/widya-lang/contoh/37_regexjit_dan_markdown.wya) | **Regex DFA & Rich Text**: Mesin Regex DFA Cepat & Parsing Dokumen Markdown AST / HTML / ANSI (`WidyaRegexJIT` & `WidyaMarkdown`) |
| [`contoh/38_dns_dan_circuitbreaker.wya`](file:///c:/widya-lang/contoh/38_dns_dan_circuitbreaker.wya) | **Networking & Resilience**: In-Memory DNS Resolver / Multi-Record & Microservice Circuit Breaker (`WidyaDNS` & `WidyaCircuitBreaker`) |
| [`contoh/39_trie_dan_arsip_tar.wya`](file:///c:/widya-lang/contoh/39_trie_dan_arsip_tar.wya) | **Data Structure & Archival**: Kamus Autocomplete Prefix Tree & Pembuatan/Ekstraksi Arsip Tarball USTAR (`WidyaTrie` & `WidyaTar`) |
| [`contoh/40_mqtt_dan_cron.wya`](file:///c:/widya-lang/contoh/40_mqtt_dan_cron.wya) | **IoT & Scheduling**: MQTT Message Broker Pub/Sub dengan wildcard (`+`, `#`) & Cron Scheduler Parser (`WidyaMQTT` & `WidyaCron`) |
| [`contoh/41_ratelimiter_dan_qr.wya`](file:///c:/widya-lang/contoh/41_ratelimiter_dan_qr.wya) | **Traffic Shaping & Visual Matrix**: Algoritma Token Bucket Rate Limiting & Pembangkit Matriks 2D QR Code ANSI/SVG (`WidyaRateLimiter` & `WidyaQR`) |
| [`contoh/42_uuid_dan_base64.wya`](file:///c:/widya-lang/contoh/42_uuid_dan_base64.wya) | **Identity & Binary Encoding**: Pembangkit/Parser UUID v4/v7 berurutan waktu & Enkoder Base64/Base64URL/Hex (`WidyaUUID` & `WidyaBase64`) |
| [`contoh/43_semver_dan_kvstore.wya`](file:///c:/widya-lang/contoh/43_semver_dan_kvstore.wya) | **Versioning & Fast Storage**: Parser SemVer 2.0.0, Comparator & In-Memory Key-Value Store dengan TTL & Prefix Scan (`WidyaSemver` & `WidyaKV`) |
| [`contoh/44_jwt_dan_warna.wya`](file:///c:/widya-lang/contoh/44_jwt_dan_warna.wya) | **Authentication & Visual Spaces**: Tanda tangan/Verifikasi JSON Web Token (HMAC-SHA256) & Konversi HEX/RGB/HSL dan Kontras Keterbacaan WCAG 2.1 (`WidyaJWT` & `WidyaColor`) |
| [`contoh/45_csv_dan_fsm.wya`](file:///c:/widya-lang/contoh/45_csv_dan_fsm.wya) | **Tabular Processing & State Engine**: Parsing & Serialisasi RFC 4180 CSV/TSV serta Orkestrasi Status Finite State Machine (`WidyaCSV` & `WidyaStateMachine`) |
| [`contoh/46_otp_dan_fuzzy.wya`](file:///c:/widya-lang/contoh/46_otp_dan_fuzzy.wya) | **2FA Security & Fuzzy Search**: Pembangkit/Verifikator Google Authenticator 2FA TOTP/HOTP & Pencarian Teks Mirip Levenshtein Distance (`WidyaOTP` & `WidyaLevenshtein`) |
| [`contoh/47_lrucache_dan_bitset.wya`](file:///c:/widya-lang/contoh/47_lrucache_dan_bitset.wya) | **Memory Caching & Bit Arrays**: In-Memory Cache Kebijakan Penggusuran LRU & Vektor Bit Kompak Izin Akses Bitwise (`WidyaLRUCache` & `WidyaBitSet`) |
| [`contoh/48_desimal_dan_heap.wya`](file:///c:/widya-lang/contoh/48_desimal_dan_heap.wya) | **Financial Math & Priority Queue**: Aritmatika Desimal Titik-Tetap Bebas Galat Biner/Format Rupiah & Binary Max/Min Heap (`WidyaDecimal` & `WidyaPriorityQueue`) |
| [`contoh/49_glob_dan_uuid5.wya`](file:///c:/widya-lang/contoh/49_glob_dan_uuid5.wya) | **Pattern Matching & Identity**: Pencocokan Pola Jalur Berkas Wildcard POSIX/Unix & Identitas Deterministik Berbasis Namespace RFC 4122 (`WidyaGlob` & `WidyaUUIDv5`) |
| [`contoh/50_skema_dan_template.wya`](file:///c:/widya-lang/contoh/50_skema_dan_template.wya) | **Data Validation & Templating**: Validator Skema Struktur Data API Deklaratif & Mesin Template Teks Interpolasi `{{ }}` (`WidyaSchemaValidator` & `WidyaTemplate`) |
| [`contoh/51_kompresi_dan_eventbus.wya`](file:///c:/widya-lang/contoh/51_kompresi_dan_eventbus.wya) | **Compression & Event Dispatching**: Kompresi Data In-Memory RLE & LZW serta Centralized Pub/Sub Event Dispatcher (`WidyaCompress` & `WidyaEventBus`) |
| [`contoh/52_tolakukur_dan_mime.wya`](file:///c:/widya-lang/contoh/52_tolakukur_dan_mime.wya) | **Performance & Media Types**: Micro-Benchmarking Fungsi & Deteksi/Resolusi Tipe MIME Berkas (`WidyaBenchmark` & `WidyaMime`) |
| [`contoh/53_countingbloom_dan_matriks.wya`](file:///c:/widya-lang/contoh/53_countingbloom_dan_matriks.wya) | **Probabilistic Sets & Linear Algebra**: Deletable Counting Bloom Filter & Operasi Determinan/Invers Matriks 2D (`WidyaCountingBloom` & `WidyaMatrixMath`) |
| [`contoh/54_ringbuffer_dan_bitops.wya`](file:///c:/widya-lang/contoh/54_ringbuffer_dan_bitops.wya) | **Streaming Buffers & Low-Level Math**: Circular Ring Buffer FIFO & Manipulasi Bit Tingkat Rendah / Packing (`WidyaRingBuffer` & `WidyaBitwiseOps`) |
| [`contoh/55_hyperloglog_dan_versi.wya`](file:///c:/widya-lang/contoh/55_hyperloglog_dan_versi.wya) | **Big Data Estimation & Package Resolver**: Algoritma Kardinalitas HLL & Mesin Evaluasi Rentang Versi SemVer (`WidyaHLL` & `WidyaVersionRange`) |
| [`contoh/56_bigint_dan_cron.wya`](file:///c:/widya-lang/contoh/56_bigint_dan_cron.wya) | **Arbitrary Precision & Scheduling**: Aritmatika Bilangan Bulat Tak Terbatas & Mesin Evaluasi Ekspresi Cron (`WidyaBigInt` & `WidyaCronParser`) |
| [`contoh/57_soundex_dan_ipcidr.wya`](file:///c:/widya-lang/contoh/57_soundex_dan_ipcidr.wya) | **Phonetic Matching & Networking**: Pengindeksan Fonetik Bunyi Nama & Kalkulator Subnet IP/CIDR (`WidyaSoundex` & `WidyaIPCIDR`) |
| [`contoh/58_statistik_dan_markov.wya`](file:///c:/widya-lang/contoh/58_statistik_dan_markov.wya) | **Descriptive Analytics & Stochastic Modeling**: Statistik Mean/Median/Modus/StdDev & Rantai Markov Probabilistik (`WidyaStats` & `WidyaMarkov`) |
| [`contoh/59_bencode_dan_simhash.wya`](file:///c:/widya-lang/contoh/59_bencode_dan_simhash.wya) | **P2P Protocols & Deduplication**: Serialisasi BitTorrent Bencoding RFC 1077 & Locality-Sensitive Hashing Fingerprint (`WidyaBencode` & `WidyaSimHash`) |
| [`contoh/60_uuidv6_dan_trie.wya`](file:///c:/widya-lang/contoh/60_uuidv6_dan_trie.wya) | **Monotonic Identity & Search**: UUID v6 Terurut Waktu Gregorian / COMB & Kamus Autocomplete Prefix Tree (`WidyaUUIDv6` & `WidyaTriePrefix`) |
| [`contoh/61_bitbuffer_dan_levenshtein.wya`](file:///c:/widya-lang/contoh/61_bitbuffer_dan_levenshtein.wya) | **Bit Streaming & Matrix Alignment**: Arbitrary Bit-Level Stream Reader/Writer & Matriks Perbedaan Jarak Edit (`WidyaBitBuffer` & `WidyaLevenshteinMatrix`) |
| [`contoh/62_geometri2d_dan_murmur.wya`](file:///c:/widya-lang/contoh/62_geometri2d_dan_murmur.wya) | **Computational Geometry & Hashing**: Geometri 2D, Garis Berpotongan, Luas Poligon Shoelace, Convex Hull & MurmurHash3 (`WidyaGeometry2D` & `WidyaMurmurHash`) |
| [`contoh/63_pecahan_dan_ini.wya`](file:///c:/widya-lang/contoh/63_pecahan_dan_ini.wya) | **Rational Math & Configuration**: Bilangan Rasional Pecahan Presisi Eksak dengan Reduksi FPB & Parser/Serializer File INI (`WidyaBigRat` & `WidyaIniConfig`) |
| [`contoh/64_avl_dan_geohash.wya`](file:///c:/widya-lang/contoh/64_avl_dan_geohash.wya) | **Balanced Trees & Spatial Indexing**: Pohon Biner Seimbang Otomatis AVL Tree & Geohash GPS Spasial 8 Tetangga (`WidyaAVL` & `WidyaGeoHash`) |
| [`contoh/65_cincinhash_dan_toml.wya`](file:///c:/widya-lang/contoh/65_cincinhash_dan_toml.wya) | **Distributed Hashing & TOML Config**: Cincin Hash Partisi Terdistribusi (*Consistent Hash Ring*) & Parser/Serializer Format TOML (`WidyaConsistentHash` & `WidyaTOML`) |
| [`contoh/66_skiplist_dan_huffman.wya`](file:///c:/widya-lang/contoh/66_skiplist_dan_huffman.wya) | **Probabilistic Indexing & Entropy Coding**: Struktur Indeks Daftar Lompat (*Skip List*) & Kompresi Entropi Bitstream Huffman (`WidyaSkipList` & `WidyaHuffman`) |
| [`contoh/67_merkle_dan_crondaemon.wya`](file:///c:/widya-lang/contoh/67_merkle_dan_crondaemon.wya) | **Cryptographic Audit & Task Daemon**: Pohon Hash Kriptografis Merkle Proof & Orkestrator Daemon Cron Berkala (`WidyaMerkleTree` & `WidyaCronDaemon`) |
| [`contoh/68_wal_dan_kolumnar.wya`](file:///c:/widya-lang/contoh/68_wal_dan_kolumnar.wya) | **Storage Engine & Analytics**: Write-Ahead Logging Crash Recovery (`WidyaWAL`) & Columnar Storage Aggregation (`WidyaParquetLite`) |
| [`contoh/69_roaring_dan_nacl.wya`](file:///c:/widya-lang/contoh/69_roaring_dan_nacl.wya) | **Compressed Sets & AEAD Security**: Compressed Roaring Bitmaps (`WidyaRoaringBitmap`) & Kriptografi ChaCha20-Poly1305 (`WidyaNaCl`) |
| [`contoh/70_vectordb_dan_actorsystem.wya`](file:///c:/widya-lang/contoh/70_vectordb_dan_actorsystem.wya) | **AI Search & Actor Concurrency**: Basis Data Vektor $k$-NN Cosine Similarity (`WidyaVectorDB`) & Orkestrasi Aktor Mailbox (`WidyaActorSystem`) |
| [`contoh/71_bplustree_dan_lz4.wya`](file:///c:/widya-lang/contoh/71_bplustree_dan_lz4.wya) | **Indexing & Fast Streaming**: Indeks Database B+ Tree Berurutan (`WidyaBPTree`) & Kompresi Blok Kilat LZ4 (`WidyaLZ4Codec`) |
| [`contoh/72_lsh_dan_raft.wya`](file:///c:/widya-lang/contoh/72_lsh_dan_raft.wya) | **Deduplication & Distributed Systems**: MinHash Locality-Sensitive Hashing (`WidyaLSH`) & Konsensus Terdistribusi Raft (`WidyaRaftLite`) |
| [`contoh/73_rtree_dan_zstd.wya`](file:///c:/widya-lang/contoh/73_rtree_dan_zstd.wya) | **Spatial GIS & High-Ratio Compression**: Indeks Spasial 2D Pohon R (`WidyaSpatialRTree`) & Kompresi Multi-Tingkat Zstandard (`WidyaZstdLite`) |
| [`contoh/74_mmr_dan_gossip.wya`](file:///c:/widya-lang/contoh/74_mmr_dan_gossip.wya) | **Blockchain Audit & P2P Networks**: Akumulator Merkle Mountain Range (`WidyaMerkleMountainRange`) & Protokol Gossip P2P (`WidyaGossip`) |
| [`contoh/75_brotli_dan_diffprivacy.wya`](file:///c:/widya-lang/contoh/75_brotli_dan_diffprivacy.wya) | **Web Compression & Data Privacy**: Kompresi Teks Konteks Brotli (`WidyaBrotliLite`) & Privasi Diferensial Laplace (`WidyaDifferentialPrivacy`) |
| [`contoh/76_hlc_dan_quadtree.wya`](file:///c:/widya-lang/contoh/76_hlc_dan_quadtree.wya) | **Distributed Clocks & Spatial 2D**: Hybrid Logical Clock (`WidyaHLC`) & Partisi Spasial Pohon Quad (`WidyaQuadTree`) |
| [`contoh/77_shamir_dan_snappy.wya`](file:///c:/widya-lang/contoh/77_shamir_dan_snappy.wya) | **Threshold Cryptography & Streaming**: Shamir (k, n) Secret Sharing (`WidyaShamirSecret`) & Kompresi Streaming Snappy (`WidyaSnappyLite`) |
| [`contoh/78_kademlia_dan_mpt.wya`](file:///c:/widya-lang/contoh/78_kademlia_dan_mpt.wya) | **P2P DHT & State Trie**: Kademlia Routing Table (`WidyaKademlia`) & Hex-Prefix Merkle Patricia Trie (`WidyaMerklePatriciaTrie`) |
| [`contoh/79_zfp_dan_verkletree.wya`](file:///c:/widya-lang/contoh/79_zfp_dan_verkletree.wya) | **Precision Compression & Vector Commitments**: Kompresi Float Presisi ZFP (`WidyaZFP`) & Pohon Status Komitmen Vektor (`WidyaVerkleTree`) |
| [`contoh/80_bully_dan_merkledag.wya`](file:///c:/widya-lang/contoh/80_bully_dan_merkledag.wya) | **Cluster Election & Content DAG**: Pemilihan Pemimpin Bully (`WidyaBullyElection`) & Graf Berarah Merkle-DAG (`WidyaMerkleDAG`) |
| [`contoh/81_gorilla_dan_barycentric.wya`](file:///c:/widya-lang/contoh/81_gorilla_dan_barycentric.wya) | **Time-Series Codec & Interpolation**: Kompresi XOR Deret Waktu Gorilla (`WidyaGorillaCompressor`) & Interpolasi Rasional Barycentric (`WidyaBarycentricInterpolation`) |
| [`contoh/82_paxos_dan_tdigest.wya`](file:///c:/widya-lang/contoh/82_paxos_dan_tdigest.wya) | **Consensus & Streaming Quantiles**: Protokol Konsensus Terdistribusi Klasik Paxos (`WidyaPaxos`) & Estimasi Kuantil Streaming T-Digest (`WidyaTDigest`) |
| [`contoh/83_chaskey_dan_fenwick.wya`](file:///c:/widya-lang/contoh/83_chaskey_dan_fenwick.wya) | **Embedded Crypto & Dynamic Prefix**: Otentikasi Pesan Ringan IoT Chaskey MAC (`WidyaChaskeyMAC`) & Pohon Indeks Biner Fenwick (`WidyaFenwickTree`) |
| [`contoh/84_vclock_dan_radixtree.wya`](file:///c:/widya-lang/contoh/84_vclock_dan_radixtree.wya) | **Distributed Causality & Routing**: Pelacakan Kausalitas Jam Vektor (`WidyaVectorClock`) & Pohon Awalan Ringkas Patricia Radix Tree (`WidyaRadixTree`) |
| [`contoh/85_blake3_dan_rle.wya`](file:///c:/widya-lang/contoh/85_blake3_dan_rle.wya) | **Tree Hashing & Stream Compression**: Tree-Hash Kriptografi Paralel BLAKE3 (`WidyaBLAKE3`) & Kompresi Adaptif Run-Length (`WidyaRunLengthCodec`) |
| [`contoh/86_treap_dan_argon2id.wya`](file:///c:/widya-lang/contoh/86_treap_dan_argon2id.wya) | **Balanced Trees & Password Security**: Pohon Cartesian Acak Treap (`WidyaTreap`) & Hash Kata Sandi Keras-Memori Argon2id (`WidyaArgon2id`) |
| [`contoh/87_brotlidict_dan_kademliarpc.wya`](file:///c:/widya-lang/contoh/87_brotlidict_dan_kademliarpc.wya) | **Microservice Compression & P2P RPC**: Kompresi Kamus Statis Brotli (`WidyaBrotliDictionary`) & Protokol Pesan Kademlia RPC (`WidyaKademliaRPC`) |
| [`contoh/88_gasengine_dan_evmabi.wya`](file:///c:/widya-lang/contoh/88_gasengine_dan_evmabi.wya) | **Smart Contract & EVM Interop**: Mesin Gas Smart Contract Deterministik (`WidyaGasEngine`) & Jembatan ABI Ethereum (`WidyaEVMBridge`) |
| [`contoh/89_secp256k1_dan_defiamm.wya`](file:///c:/widya-lang/contoh/89_secp256k1_dan_defiamm.wya) | **Elliptic Curves & DeFi AMM**: Dompet Kripto ECDSA Secp256k1 (`WidyaSecp256k1`) & Pool Likuiditas Automated Market Maker (`WidyaDeFiVault`) |
| [`contoh/90_loadbalancer_dan_apirouter.wya`](file:///c:/widya-lang/contoh/90_loadbalancer_dan_apirouter.wya) | **Cloud-Native Traffic & Gateway**: Penyeimbang Beban Multi-Strategi (`WidyaLoadBalancer`) & API Gateway Routing Engine (`WidyaAPIGatewayRouter`) |
| [`contoh/91_distlock_dan_opentelemetry.wya`](file:///c:/widya-lang/contoh/91_distlock_dan_opentelemetry.wya) | **Distributed Lock & Tracing**: Mutex Terdistribusi Berbasis Lease (`WidyaDistributedLock`) & Distributed APM Tracing (`WidyaOpenTelemetry`) |
| [`contoh/92_dataframe_dan_visualisasi.wya`](file:///c:/widya-lang/contoh/92_dataframe_dan_visualisasi.wya) | **Data Apps & Charting**: Pemrosesan Data Tabular DataFrame, Filter, GroupBy, Join (`WidyaDataFrame`) & Pembangkit Grafik SVG/HTML (`WidyaDataVisualizer`) |
| [`contoh/93_ragpipeline_dan_mlclassifier.wya`](file:///c:/widya-lang/contoh/93_ragpipeline_dan_mlclassifier.wya) | **AI & Machine Learning**: Retrieval-Augmented Generation RAG Prompt Synthesis (`WidyaRAGPipeline`) & Klasifikasi Decision Tree / K-Means (`WidyaMLClassifier`) |
| [`contoh/94_dma_dan_atomik.wya`](file:///c:/widya-lang/contoh/94_dma_dan_atomik.wya) | **Bare-Metal & Lock-Free**: Akses Buffer DMA Memory-Mapped (`WidyaRing0MMIO`) & Operasi Hardware Atomic CAS (`WidyaAtomicOps`) |
| [`contoh/95_arena_dan_rawsocket.wya`](file:///c:/widya-lang/contoh/95_arena_dan_rawsocket.wya) | **Systems & Kernel Networking**: Alokator Memori Arena Monolitik O(1) (`WidyaMemoryArena`) & Raw TCP/UDP Socket (`WidyaSysSocket`) |
| [`contoh/96_jendela_dan_menu_desktop.wya`](file:///c:/widya-lang/contoh/96_jendela_dan_menu_desktop.wya) | **Desktop GUI & Menu**: Pengelolaan Siklus Hidup Jendela Desktop Native (`WidyaDesktopWindow`), Bilah Menu & Baki Sistem Tray (`WidyaDesktopMenu`) |
| [`contoh/97_dialog_dan_sinyal_reaktif.wya`](file:///c:/widya-lang/contoh/97_dialog_dan_sinyal_reaktif.wya) | **Native Dialogs & Reactive UI**: Dialog Berkas/Pesan OS Native (`WidyaNativeDialog`) & Manajemen Status Reaktif Signals/Effects (`WidyaReactiveState`) |
| [`contoh/98_gps_dan_sensor_mobile.wya`](file:///c:/widya-lang/contoh/98_gps_dan_sensor_mobile.wya) | **Mobile Location & Sensors**: GPS Geolocation, Perhitungan Jarak Haversine, Geofencing (`WidyaMobileLocation`), Akselerometer 3D, Giroskop & Haptik (`WidyaMobileSensors`) |
| [`contoh/99_baterai_dan_biometrik_mobile.wya`](file:///c:/widya-lang/contoh/99_baterai_dan_biometrik_mobile.wya) | **Mobile Power & Biometrics**: Pemantauan Baterai & Daya (`WidyaMobileBatteryPower`), Autentikasi Face ID/Fingerprint (`WidyaMobileBiometrics`) & KeyStore Terenkripsi (`WidyaMobileStorage`) |
| [`contoh/100_gpio_dan_bus_i2c_spi.wya`](file:///c:/widya-lang/contoh/100_gpio_dan_bus_i2c_spi.wya) | **IoT & Embedded Hardware Bus**: Kontrol Pin Digital GPIO & PWM Motor/LED (`WidyaEmbeddedGPIO`), Transaksi Register I2C (`WidyaBusI2C`), & Bus Duplex SPI (`WidyaBusSPI`) |
| [`contoh/101_uart_dan_coap_iot.wya`](file:///c:/widya-lang/contoh/101_uart_dan_coap_iot.wya) | **Serial Protocol & Lightweight CoAP**: Komunikasi Serial UART / RS-485, Modbus CRC16 (`WidyaBusUART`) & Protokol IoT Nirkabel Ringan CoAP RFC 7252 (`WidyaCoAPProtocol`) |
| [`contoh/102_ecs_dan_fisika_game.wya`](file:///c:/widya-lang/contoh/102_ecs_dan_fisika_game.wya) | **Game Engine ECS & Physics 2D**: Arsitektur Entity Component System (`WidyaECS`) & Simulasi Fisika Rigid Body, Gravitasi, Tabrakan AABB (`WidyaPhysics2D`) |
| [`contoh/103_tilemap_dan_astar_game.wya`](file:///c:/widya-lang/contoh/103_tilemap_dan_astar_game.wya) | **Tilemap, Pathfinding & Audio**: Grid Peta Ubin 2D (`WidyaTilemap`), Rute AI A* Pathfinding (`WidyaPathfindingAStar`), SFX Audio & Frame Loop (`WidyaGameAudio`) |
| [`contoh/104_devops_pipeline_dan_kontainer.wya`](file:///c:/widya-lang/contoh/104_devops_pipeline_dan_kontainer.wya) | **DevOps Pipeline & Containers**: Eksekusi Tahap CI/CD Pipeline (`WidyaDevOpsPipeline`) & Generator Manifest Docker Compose / OCI (`WidyaContainerConfig`) |
| [`contoh/105_env_vault_dan_webhook_alert.wya`](file:///c:/widya-lang/contoh/105_env_vault_dan_webhook_alert.wya) | **Dotenv Vault & Alerting**: Parsing .env & Interpolasi Rahasia (`WidyaEnvVault`), Probe Health Check Microservices, & Notifier Webhook (`WidyaWebhookNotifier`) |
| [`contoh/106_pcap_dan_entropi_forensik.wya`](file:///c:/widya-lang/contoh/106_pcap_dan_entropi_forensik.wya) | **Network & Binary Forensics**: Analisis Paket Jaringan PCAP (`WidyaPCAPParser`), Deteksi Magic Byte Header Berkas & Kalkulasi Shannon Entropy (`WidyaFileForensics`) |
| [`contoh/107_yara_dan_siem_keamanan.wya`](file:///c:/widya-lang/contoh/107_yara_dan_siem_keamanan.wya) | **Cybersecurity & SIEM Analysis**: Pencocokan Signature Malware Aturan YARA (`WidyaYARAMatcher`) & Deteksi Serangan Anomali Log SIEM (`WidyaLogAnalyzer`) |
| [`contoh/108_wasm_biner_dan_memori.wya`](file:///c:/widya-lang/contoh/108_wasm_biner_dan_memori.wya) | **WebAssembly Core & Memory**: Pembangkit Modul Biner Wasm (`WidyaWasmModule`), Validasi Header Magic & Alokasi Linear Memory 64 KiB (`WidyaWasmMemory`) |
| [`contoh/109_wasi_dan_edge_worker.wya`](file:///c:/widya-lang/contoh/109_wasi_dan_edge_worker.wya) | **WASI & Serverless Edge**: Antarmuka Standar WASI Snapshot Preview 1 (`WidyaWASIEngine`), JS FFI Bridge (`WidyaWasmJSBridge`) & V8 Edge Worker (`WidyaEdgeWorker`) |
| [`contoh/110_orderbook_dan_indikator_kuantitatif.wya`](file:///c:/widya-lang/contoh/110_orderbook_dan_indikator_kuantitatif.wya) | **Order Book L2 & Quantitative Analysis**: Matching Engine Buku Pesanan L2 (`WidyaOrderBookL2`), Indikator Teknis EMA, RSI & VWAP (`WidyaFinancialIndicators`) |
| [`contoh/111_blackscholes_dan_protokol_fix.wya`](file:///c:/widya-lang/contoh/111_blackscholes_dan_protokol_fix.wya) | **Option Pricing & FIX Protocol**: Valuasi Derivatif Black-Scholes Greeks Delta/Gamma/Vega (`WidyaBlackScholes`) & Pesan Bursa FIX Protocol 4.4 (`WidyaFIXProtocol`) |
| [`contoh/112_fasta_dan_translasi_dna.wya`](file:///c:/widya-lang/contoh/112_fasta_dan_translasi_dna.wya) | **Genomics & DNA Translation**: Parser Format FASTA/FASTQ (`WidyaFASTAQParser`), Rasio GC-Content, & Translasi Kodon mRNA ke Protein (`WidyaDNATranslator`) |
| [`contoh/113_penjajaran_sekuens_dan_kmer.wya`](file:///c:/widya-lang/contoh/113_penjajaran_sekuens_dan_kmer.wya) | **Sequence Alignment & K-Mer**: Penjajaran Global Needleman-Wunsch & Lokal Smith-Waterman (`WidyaSequenceAlignment`), Spektra K-Mer & Jarak Hamming (`WidyaKmerCounter`) |
| [`contoh/114_geojson_dan_wkt_spasial.wya`](file:///c:/widya-lang/contoh/114_geojson_dan_wkt_spasial.wya) | **Spatial Science & Vector Geometry**: Parser Format GeoJSON / OGC WKT (`WidyaGeoJSON`), Uji Point-in-Polygon Ray Casting & Luas Poligon (`WidyaSpatialTopology`) |
| [`contoh/115_rtree_geohash_dan_proyeksi.wya`](file:///c:/widya-lang/contoh/115_rtree_geohash_dan_proyeksi.wya) | **Spatial Indexing & Projections**: Indeks Bounding Box R-Tree (`WidyaSpatialIndexing`), Hirarki Geohash & Proyeksi EPSG:4326 $\leftrightarrow$ EPSG:3857 (`WidyaCRSProjection`) |
| [`contoh/116_mavlink_dan_ahrs_drone.wya`](file:///c:/widya-lang/contoh/116_mavlink_dan_ahrs_drone.wya) | **Autonomous Drone & AHRS**: Protokol Telemetri Drone MAVLink v2 (`WidyaMAVLinkProtocol`) & Fusi Sensor IMU Filter Madgwick Kuaternion (`WidyaAHRSQuaternion`) |
| [`contoh/117_kinematika_robot_dan_orbit.wya`](file:///c:/widya-lang/contoh/117_kinematika_robot_dan_orbit.wya) | **Robotics & Orbital Mechanics**: Forward & Inverse Kinematics Manipulator 2-DOF (`WidyaKinematicsRobotics`) & Propagasi Orbit Satelit Keplerian (`WidyaOrbitalKepler`) |
| [`contoh/118_dsp_fft_dan_filter_audio.wya`](file:///c:/widya-lang/contoh/118_dsp_fft_dan_filter_audio.wya) | **DSP & Audio Engineering**: Fast Fourier Transform FFT/IFFT (`WidyaFFT`), Filter Digital IIR Biquad (`WidyaFilterDSP`) & Konvolusi Reverb |
| [`contoh/119_spektrogram_dan_bingkai_video.wya`](file:///c:/widya-lang/contoh/119_spektrogram_dan_bingkai_video.wya) | **Spectral Analysis & Video**: Spektrogram Audio STFT (`WidyaSpectralAnalysis`), Dynamic Peak Compressor & Frame Video YUV/RGB (`WidyaVideoFrameProcessor`) |
| [`contoh/120_cfd_navier_stokes_dan_panas.wya`](file:///c:/widya-lang/contoh/120_cfd_navier_stokes_dan_panas.wya) | **CFD & Heat Transfer**: Pemecah Fluida Inkompresibel Navier-Stokes 2D (`WidyaNavierStokes2D`) & Difusi Konduksi Termal Laplacian (`WidyaHeatTransfer`) |
| [`contoh/121_sph_fluida_dan_gelombang.wya`](file:///c:/widya-lang/contoh/121_sph_fluida_dan_gelombang.wya) | **SPH Particles & Wave Propagation**: Partikel Hidrodinamika Terhalus SPH (`WidyaSPHFluid`) & Propagasi Gelombang Hiperbolik 2D (`WidyaWaveEquation`) |
| [`contoh/122_openxr_dan_pelacakan_tangan.wya`](file:///c:/widya-lang/contoh/122_openxr_dan_pelacakan_tangan.wya) | **OpenXR & Hand Tracking**: Sesi Immersive VR Stereoskopik (`WidyaOpenXR`), Pelacakan Tulang Tangan 26-Joint & Deteksi Gestur Pinch/Grip (`WidyaHandTracking`) |
| [`contoh/123_ar_plane_dan_audio_spasial.wya`](file:///c:/widya-lang/contoh/123_ar_plane_dan_audio_spasial.wya) | **AR Surfaces & Spatial Audio**: Deteksi Bidang Datar & Hit-Test (`WidyaARPlaneDetection`), Audio Spasial 3D HRTF & Ekspor WebXR HTML (`WidyaSpatialAudio`) |
| [`contoh/124_neuron_spiking_dan_stdp.wya`](file:///c:/widya-lang/contoh/124_neuron_spiking_dan_stdp.wya) | **Neuromorphic & SNN**: Model Neuron Biologis Leaky Integrate-and-Fire (`WidyaSpikingNeuron`) & Belajar Hebbian Plastisitas STDP (`WidyaSTDPPlasticity`) |
| [`contoh/125_eeg_gelombang_otak_dan_bci.wya`](file:///c:/widya-lang/contoh/125_eeg_gelombang_otak_dan_bci.wya) | **BCI & Brainwaves**: Dekomposisi Pita Gelombang Otak EEG $\alpha, \beta, \theta$ (`WidyaEEGSignalProcessing`) & Dekoder Niat Motorik BCI (`WidyaBCIDecoder`) |
| [`contoh/126_hdl_dan_sintesis_verilog.wya`](file:///c:/widya-lang/contoh/126_hdl_dan_sintesis_verilog.wya) | **EDA & RTL Synthesis**: Sintesis Perangkat Keras RTL Register/ALU, Ekspor IEEE 1364 Verilog & IEEE 1076 VHDL (`WidyaHDLSynthesizer`) |
| [`contoh/127_netlist_sta_dan_gelombang_vcd.wya`](file:///c:/widya-lang/contoh/127_netlist_sta_dan_gelombang_vcd.wya) | **Gate Netlist, STA & VCD**: Simulasi Logika Rangkaian Tingkat Gerbang (`WidyaGateNetlist`), Static Timing Analysis Lintasan Kritis (`WidyaStaticTimingAnalysis`), & Perekam Gelombang VCD (`WidyaVCDWaveform`) |
| [`contoh/128_mikrokernel_dan_kapabilitas_ipc.wya`](file:///c:/widya-lang/contoh/128_mikrokernel_dan_kapabilitas_ipc.wya) | **Microkernel & Capability IPC**: Ruang Kapabilitas CSpace & Endpoint IPC Sinkron seL4-style (`WidyaMicrokernelCapability`), Penjadwal Preemptive Round-Robin TCB (`WidyaMicrokernelScheduler`) |
| [`contoh/129_unikernel_dan_mmu_halaman.wya`](file:///c:/widya-lang/contoh/129_unikernel_dan_mmu_halaman.wya) | **Unikernel & Edge MMU**: Citra Bootable Multiboot Single Address Space (`WidyaUnikernelImage`) & Translasi Tabel Halaman MMU 2-Level (`WidyaEdgeMMU`) |
| [`contoh/130_penyimpanan_data_dna.wya`](file:///c:/widya-lang/contoh/130_penyimpanan_data_dna.wya) | **DNA Data Storage & Bio-Codec**: Pengkodean Biner-ke-DNA dengan Batasan Homopolimer, Rasio GC-Content, & Koreksi Reed-Solomon (`WidyaDNADataStorage`, `WidyaReedSolomonBioCodec`) |
| [`contoh/131_biokomputasi_dan_dsd_molekuler.wya`](file:///c:/widya-lang/contoh/131_biokomputasi_dan_dsd_molekuler.wya) | **Biocomputing & DSD Kinetics**: Simulasi Reaksi DNA Strand Displacement (DSD) & Gerbang Logika Molekuler Biokimia (`WidyaStrandDisplacement`, `WidyaMolecularLogicGates`) |
| [`contoh/132_sat_dan_smt_solver.wya`](file:///c:/widya-lang/contoh/132_sat_dan_smt_solver.wya) | **Formal Verification: SAT & SMT**: Pemecah Boolean CNF DPLL (`WidyaSATSolver`) & Pemecah Kendala Linear Real SMT-LIB2 (`WidyaSMTSolverLinear`) |
| [`contoh/133_hoare_logic_dan_pembukti_teorema.wya`](file:///c:/widya-lang/contoh/133_hoare_logic_dan_pembukti_teorema.wya) | **Hoare Logic & FOL Theorem Prover**: Verifikasi Program Weakest Precondition $\{P\}\ C\ \{Q\}$ (`WidyaHoareLogicVerifier`) & Pembukti Teorema Refutasi Resolusi Orde Pertama (`WidyaFirstOrderTheoremProver`) |
| [`contoh/134_fits_astronomi_dan_wcs.wya`](file:///c:/widya-lang/contoh/134_fits_astronomi_dan_wcs.wya) | **Astroinformatics: FITS & WCS**: Format Citra CCD FITS HDU & Transformasi Koordinat Langit World Coordinate System RA/Dec J2000 (`WidyaFITSParser`) |
| [`contoh/135_eksoplanet_dan_interferometri_radio.wya`](file:///c:/widya-lang/contoh/135_eksoplanet_dan_interferometri_radio.wya) | **Exoplanets & Radio Astronomy**: Fotometri Transit Eksoplanet Mandel-Agol (`WidyaExoplanetTransit`), Sintesis Apertur UV Interferometri Radio (`WidyaRadioInterferometry`), & Astrometri Gaia (`WidyaStellarAstrometry`) |
| [`contoh/136_struktur_kristal_dan_dinamika_molekul.wya`](file:///c:/widya-lang/contoh/136_struktur_kristal_dan_dinamika_molekul.wya) | **Material Informatics: Crystals & MD**: Sel Satuan Kristalografi 3D, Ekspor CIF (`WidyaCrystalStructure`) & Dinamika Molekul Lennard-Jones Velocity-Verlet (`WidyaMolecularDynamicsLJ`) |
| [`contoh/137_diagram_fase_dan_deskriptor_material.wya`](file:///c:/widya-lang/contoh/137_diagram_fase_dan_deskriptor_material.wya) | **Phase Diagrams & Perovskites**: Termodinamika Energi Bebas Gibbs CALPHAD, Aturan Tuas Paduan Biner (`WidyaPhaseDiagramThermodynamics`) & Faktor Toleransi Goldschmidt Perovskit (`WidyaMaterialDescriptor`) |
| [`contoh/138_terahertz_dan_permukaan_cerdas_ris.wya`](file:///c:/widya-lang/contoh/138_terahertz_dan_permukaan_cerdas_ris.wya) | **6G Terahertz & RIS Metasurface**: Propagasi Kanal Sub-THz/THz dengan Redaman Molekuler H2O (`WidyaTeraHertzChannel`) & Optimasi Beamforming Metasurface Array (`WidyaReconfigurableIntelligentSurface`) |
| [`contoh/139_isac_sensing_dan_oran_ric.wya`](file:///c:/widya-lang/contoh/139_isac_sensing_dan_oran_ric.wya) | **ISAC & Open RAN (O-RAN RIC)**: Radar Doppler & Komunikasi Dual-Functional (`WidyaISACSensing`) & Near-RT RIC xApp Traffic Steering (`WidyaORANArchitecture`) |
| [`contoh/140_agent_based_dan_dinamika_sistem.wya`](file:///c:/widya-lang/contoh/140_agent_based_dan_dinamika_sistem.wya) | **Complex Systems: ABM & System Dynamics**: Model Segregasi Agen Spasial Schelling (`WidyaAgentBasedModel`) & Integrasi Stok/Aliran Meadows RK4 (`WidyaSystemDynamics`) |
| [`contoh/141_jaring_ekologi_dan_game_theory.wya`](file:///c:/widya-lang/contoh/141_jaring_ekologi_dan_game_theory.wya) | **Ecological Web & Game Theory**: Jaring Trofik Makanan Lotka-Volterra Multispesies (`WidyaEcologicalNetwork`) & Dinamika Replikator ESS Teori Permainan Evolusi (`WidyaEvolutionaryGameTheory`) |
| [`contoh/142_oais_arsip_dan_bagit_packaging.wya`](file:///c:/widya-lang/contoh/142_oais_arsip_dan_bagit_packaging.wya) | **Digital Preservation: OAIS & BagIt**: Siklus Paket SIP/AIP/DIP & PREMIS Provenance (`WidyaOAISArchive`) & Kemasan Paket Standar IETF RFC 8493 Manifest (`WidyaBagItPackaging`) |
| [`contoh/143_warc_web_dan_pembersih_bitrot.wya`](file:///c:/widya-lang/contoh/143_warc_web_dan_pembersih_bitrot.wya) | **WARC Web Archive & Bit-Rot Scrubbing**: ISO 28500 Web ARChive Format Fixity (`WidyaWARCWebArchive`) & Pembersih Bit-Rot Media Dingin Paritas XOR (`WidyaBitRotScrubber`) |
| [`contoh/144_claytronics_dan_perakitan_mandiri.wya`](file:///c:/widya-lang/contoh/144_claytronics_dan_perakitan_mandiri.wya) | **Programmable Matter & Catoms**: Rekonfigurasi Bentuk Materi 3D Claytronics Catoms (`WidyaClaytronicsCatom`) & Perakitan Mandiri Molekuler Winfree aTAM (`WidyaSelfAssembly`) |
| [`contoh/145_nanorobotika_dan_komunikasi_molekuler.wya`](file:///c:/widya-lang/contoh/145_nanorobotika_dan_komunikasi_molekuler.wya) | **Nanorobotics & MolCom**: Kawanan Nanorobot Kemotaksis Penghantar Obat Tumor (`WidyaNanorobotSwarm`) & Komunikasi Molekuler Difusi CSK (`WidyaMolecularCommunication`) |
| [`contoh/146_tmr_radiasi_dan_telemetri_ccsds.wya`](file:///c:/widya-lang/contoh/146_tmr_radiasi_dan_telemetri_ccsds.wya) | **Rad-Hard & Space Telemetry**: Triple Modular Redundancy (TMR) SEU Radiation Voter (`WidyaTMRRedundancy`) & Enkapsulasi Paket CCSDS 131.0-B Space Packet CRC-16 (`WidyaCCSDSTelemetry`) |
| [`contoh/147_dtn_antarplanet_dan_fdir_wahana.wya`](file:///c:/widya-lang/contoh/147_dtn_antarplanet_dan_fdir_wahana.wya) | **Deep Space Probes: DTN & FDIR**: Interplanetary Internet Delay-Tolerant Networking RFC 9171 BPv7 (`WidyaDTNBundleProtocol`) & Pemulihan Otonom FDIR Safe Mode (`WidyaFaultTolerantFSM`) |
| [`contoh/148_global_workspace_dan_actr_kognitif.wya`](file:///c:/widya-lang/contoh/148_global_workspace_dan_actr_kognitif.wya) | **AGI Runtimes: GWT & ACT-R**: Baars Global Workspace Theory Conscious Broadcasting (`WidyaGlobalWorkspace`) & Memori Deklaratif/Prosedural ACT-R (`WidyaACTRMemory`) |
| [`contoh/149_soar_decision_dan_hiperdimensi_hdc.wya`](file:///c:/widya-lang/contoh/149_soar_decision_dan_hiperdimensi_hdc.wya) | **SOAR & HDC Vector Symbolic**: Siklus Keputusan & Resolusi Impasse SOAR (`WidyaSOARArchitecture`) & Aljabar Ruang Hiperdimensi Bind/Bundle HDC (`WidyaHyperdimensionalComputing`) |
| [`contoh/150_grammar_dsl_dan_makro_quasiquote.wya`](file:///c:/widya-lang/contoh/150_grammar_dsl_dan_makro_quasiquote.wya) | **Grammar DSL & Macros**: Generator Parser Tata Bahasa EBNF/PEG (`WidyaGrammarDSL`) & Ekspansi Makro Higienis Quasiquote Code Splicing (`WidyaMacroQuasiquote`) |
| [`contoh/151_bootstrap_kompilator_dan_inferensi_tipe.wya`](file:///c:/widya-lang/contoh/151_bootstrap_kompilator_dan_inferensi_tipe.wya) | **Compiler Bootstrap & Type Inference**: Verifikasi Bootstrap Stage0/1 Self-Hosting (`WidyaCompilerBootstrap`) & Inferensi Tipe Statis Hindley-Milner Algorithm W (`WidyaTypeCheckerInference`) |
| [`contoh/152_barneshut_nbody_dan_relativitas_lubang_hitam.wya`](file:///c:/widya-lang/contoh/152_barneshut_nbody_dan_relativitas_lubang_hitam.wya) | **Astrophysics & General Relativity**: Gravitasi Barnes-Hut $O(N \log N)$ (`WidyaBarnesHutNBody`) & Geodesik Schwarzschild Relativitas Umum (`WidyaGeneralRelativity`) |
| [`contoh/153_solusi_fintech_dan_zk_confidential.wya`](file:///c:/widya-lang/contoh/153_solusi_fintech_dan_zk_confidential.wya) | **Industrial Privacy & Fintech**: Audit AML Zero-Knowledge, Rekonsiliasi Homomorfik FHE, & Vault Terenkripsi |
| [`contoh/154_solusi_iot_cloud_native_microservice.wya`](file:///c:/widya-lang/contoh/154_solusi_iot_cloud_native_microservice.wya) | **Industrial IoT & Microservices**: Resilient Service Mesh (Load Balancer, Circuit Breaker, OTel Tracing) & Telemetri Edge |
| [`contoh/155_solusi_keamanan_siber_dan_sistem_terdistribusi.wya`](file:///c:/widya-lang/contoh/155_solusi_keamanan_siber_dan_sistem_terdistribusi.wya) | **Cybersecurity & Distributed Systems**: SOC SIEM Log Analysis, YARA Malware Defense & Replikasi Konsensus Raft Log |
| [`contoh/156_solusi_spasial_robotika_dan_sains.wya`](file:///c:/widya-lang/contoh/156_solusi_spasial_robotika_dan_sains.wya) | **Robotics, DSP & Computational Science**: Kinematika Balik Manipulator 2-DOF, FFT Spektrum Frekuensi, & Solver SMT QF_LRA |
| [`contoh/157_solusi_enterprise_database_dan_k8s_observability.wya`](file:///c:/widya-lang/contoh/157_solusi_enterprise_database_dan_k8s_observability.wya) | **Enterprise DB & K8s Observability**: Database Connection Pool ACID Transaction, Redis TTL Cache, & Prometheus K8s Probes |
| [`contoh/158_solusi_autentikasi_rbac_dan_streaming_io.wya`](file:///c:/widya-lang/contoh/158_solusi_autentikasi_rbac_dan_streaming_io.wya) | **Enterprise Auth & Stream Processing**: Role-Based Access Control (RBAC) Matrix, Actor Task Engine, & Chunked File Stream Pipeline |
| [`contoh/159_solusi_postgres_streaming_graphql_dan_paket.wya`](file:///c:/widya-lang/contoh/159_solusi_postgres_streaming_graphql_dan_paket.wya) | **Wire Protocols & Package Integrity**: Framing Biner PostgreSQL pgwire, Event Log Streaming Terpartisi, Protobuf TLV / GraphQL, & SemVer Lockfile |
| [`contoh/160_solusi_iso8583_pqc_dan_cloud_iac.wya`](file:///c:/widya-lang/contoh/160_solusi_iso8583_pqc_dan_cloud_iac.wya) | **Frontier Banking & PQC Cloud**: Transaksi Core Banking ISO 8583, Kriptografi Pasca-Kuantum NIST KEM, & Cloud IaC Policy-as-Code |
| [`contoh/161_solusi_fhir_canbus_scada_did_iso20022.wya`](file:///c:/widya-lang/contoh/161_solusi_fhir_canbus_scada_did_iso20022.wya) | **Global Standards**: Rekam Medis HL7 FHIR, Diagnostik Otomotif CAN/UDS, Otomasi SCADA PLC, W3C DID, & Finansial ISO 20022 |
| [`contoh/162_solusi_deep_industry_maritim_avionik_ev_dan_ros2.wya`](file:///c:/widya-lang/contoh/162_solusi_deep_industry_maritim_avionik_ev_dan_ros2.wya) | **Deep Industry**: Maritim AIS NMEA, Avionika ARINC 429, SPKLU EV OCPP, Smart Grid GOOSE, 5G Diameter, DICOM, SEG-Y, & ROS2 |
| [`contoh/163_solusi_space_bms_bim_genome_sdr_etcs_gs1.wya`](file:///c:/widya-lang/contoh/163_solusi_space_bms_bim_genome_sdr_etcs_gs1.wya) | **Frontier Space, Bio, Civil & Mobility**: ESA ECSS Satelit, EV Battery BMS, BIM IFC Konstruksi, Genomika VCF, VITA49 SDR, Kereta ETCS, & GS1 EPCIS |

---

## 📦 Paket Standar Industri Tingkat Tinggi (`modul/`)

Widya-Lang menyediakan 44 abstraksi tingkat tinggi siap pakai untuk kebutuhan produksi industri di seluruh spektrum rekayasa perangkat lunak modern:

| Modul | Deskripsi & Kemampuan Lapangan |
| :--- | :--- |
| [`modul/antariksa_spacepackets_ecss.wya`](file:///c:/widya-lang/modul/antariksa_spacepackets_ecss.wya) | Telemetri & Telekomando Satelit Standar ESA ECSS PUS (ECSS-E-ST-70-41C). |
| [`modul/baterai_bms_canopen.wya`](file:///c:/widya-lang/modul/baterai_bms_canopen.wya) | Sistem Manajemen Baterai EV BMS & CANopen CiA 454 (Active Balancing, SoC, SoH). |
| [`modul/konstruksi_ifc_bim_step.wya`](file:///c:/widya-lang/modul/konstruksi_ifc_bim_step.wya) | Model Digital Konstruksi Gedung BIM & ISO 16739 IFC (Rekapitulasi Volume Material). |
| [`modul/bioinformatika_sam_bam_vcf.wya`](file:///c:/widya-lang/modul/bioinformatika_sam_bam_vcf.wya) | Analisis Mutasi DNA & Sekuensing Genomika VCF (Variant Call Format). |
| [`modul/sdr_vita49_radio_transport.wya`](file:///c:/widya-lang/modul/sdr_vita49_radio_transport.wya) | Software-Defined Radio (SDR) & Radio Astronomi ANSI/VITA 49 VRT Data Streaming. |
| [`modul/kereta_etcs_euroradio.wya`](file:///c:/widya-lang/modul/kereta_etcs_euroradio.wya) | Sistem Persinyalan Kereta Cepat Otomatis ETCS Level 2/3 (Movement Authority). |
| [`modul/logistik_gs1_epcis.wya`](file:///c:/widya-lang/modul/logistik_gs1_epcis.wya) | Lacak Balak Rantai Pasok Global Standar GS1 EPCIS (Provenance Event Object). |
| [`modul/maritim_ais_nmea0183.wya`](file:///c:/widya-lang/modul/maritim_ais_nmea0183.wya) | Navigasi Maritim AIS Transponder & Parser GPS NMEA 0183 ($GPRMC). |
| [`modul/avionika_arinc429_nav.wya`](file:///c:/widya-lang/modul/avionika_arinc429_nav.wya) | Avionika Kokpit ARINC 429 Word Encoding/Decoding & Altimeter Barometrik. |
| [`modul/ev_charging_ocpp_iso15118.wya`](file:///c:/widya-lang/modul/ev_charging_ocpp_iso15118.wya) | SPKLU Pengisian Daya EV OCPP 2.0.1 (RFID Auth, Meter Values, V2G SoC). |
| [`modul/smartgrid_iec61850_goose.wya`](file:///c:/widya-lang/modul/smartgrid_iec61850_goose.wya) | Smart Grid IEC 61850 GOOSE Multicast Trip Proteksi Gardu Induk Listrik (<4ms). |
| [`modul/telco_5g_diameter_gtp.wya`](file:///c:/widya-lang/modul/telco_5g_diameter_gtp.wya) | Telekomunikasi 5G Core Diameter RFC 6733 CCR/CCA Online Charging System (OCS). |
| [`modul/medis_dicom_imaging.wya`](file:///c:/widya-lang/modul/medis_dicom_imaging.wya) | Citra Radiologi Medis DICOM PACS Dataset Metadata Tagging & Windowing HU. |
| [`modul/geofisika_segy_seismik.wya`](file:///c:/widya-lang/modul/geofisika_segy_seismik.wya) | Format Seismik SEG-Y Rev 1/2 Survei Geofisika & Deteksi Anomali Hidrokarbon. |
| [`modul/robotika_cdr_ros2_dds.wya`](file:///c:/widya-lang/modul/robotika_cdr_ros2_dds.wya) | Robot Operating System (ROS2) DDS OMG CDR Wire Serialization (`/cmd_vel`). |
| [`modul/standar_kesehatan_hl7_fhir.wya`](file:///c:/widya-lang/modul/standar_kesehatan_hl7_fhir.wya) | Rekam Medis Elektronik Interoperabel Standar HL7 FHIR R4 (Patient, Observation, Transaction Bundle). |
| [`modul/otomotif_canbus_uds.wya`](file:///c:/widya-lang/modul/otomotif_canbus_uds.wya) | Jaringan Bus Otomotif CAN 2.0B / CAN-FD & Layanan Diagnostik ECU UDS (ISO 14229 / OBD-II). |
| [`modul/scada_opcua_modbus_tcp.wya`](file:///c:/widya-lang/modul/scada_opcua_modbus_tcp.wya) | Otomasi Industri Pabrik Smart SCADA (Modbus TCP Register Telemetry & OPC Unified Architecture Binary). |
| [`modul/identitas_terdesentralisasi_did.wya`](file:///c:/widya-lang/modul/identitas_terdesentralisasi_did.wya) | Identitas KTP Digital Terdesentralisasi W3C DID & Penerbitan / Verifikasi Verifiable Credentials (VC). |
| [`modul/fintech_iso20022_xml.wya`](file:///c:/widya-lang/modul/fintech_iso20022_xml.wya) | Standar Pesan Finansial Global ISO 20022 (SWIFT MX / BI-FAST pacs.008 Customer Credit Transfer). |
| [`modul/protokol_fintech_iso8583.wya`](file:///c:/widya-lang/modul/protokol_fintech_iso8583.wya) | Protokol Transaksi Perbankan ISO 8583 (MTI 0200/0210, Bitmap Pack/Unpack, Otorisasi ATM, EDC & QRIS Switch). |
| [`modul/quantum_post_quantum_kem.wya`](file:///c:/widya-lang/modul/quantum_post_quantum_kem.wya) | Kriptografi Pasca-Kuantum NIST PQC (Lattice Key Encapsulation Mechanism ML-KEM & Digital Signatures ML-DSA). |
| [`modul/cloud_iac_terraform_spec.wya`](file:///c:/widya-lang/modul/cloud_iac_terraform_spec.wya) | Generator Blueprint Terraform HCL & Evaluator Kepatuhan Keamanan Multi-Cloud Policy-as-Code (OPA style). |
| [`modul/protokol_postgres.wya`](file:///c:/widya-lang/modul/protokol_postgres.wya) | Driver Framing Protokol Biner PostgreSQL v3.0 (`StartupMessage`, Simple Query `'Q'`, `RowDescription`, `DataRow`, `CommandComplete`). |
| [`modul/event_streaming_bus.wya`](file:///c:/widya-lang/modul/event_streaming_bus.wya) | Event-Driven Partitioned Append-Only Commit Log & Consumer Group Offset Checkpoint (Kafka-like Bus). |
| [`modul/codec_protobuf_graphql.wya`](file:///c:/widya-lang/modul/codec_protobuf_graphql.wya) | Protobuf Varint Tag-Length-Value Binary Codec & Eksekusi Skema Query Resolver GraphQL. |
| [`modul/paket_manajer_resolver.wya`](file:///c:/widya-lang/modul/paket_manajer_resolver.wya) | Semantic Versioning Dependency Resolver (`^`, `>=`, `==`) & Pembuat Lockfile Terverifikasi Hash SHA256. |
| [`modul/privasi_keamanan.wya`](file:///c:/widya-lang/modul/privasi_keamanan.wya) | Brankas rahasia kriptografis, enkripsi ChaCha20/Poly1305, audit ZKP, agregasi homomorfik FHE, & privasi diferensial. |
| [`modul/data_engineering.wya`](file:///c:/widya-lang/modul/data_engineering.wya) | Pipeline ETL/ELT tabular, data quality profiling, moving average, deteksi anomali deret waktu & peramalan tren. |
| [`modul/microservice_stack.wya`](file:///c:/widya-lang/modul/microservice_stack.wya) | Resilient mesh gateway, penyeimbang beban multi-strategi, circuit breaker, rate limiter token-bucket & OpenTelemetry tracing. |
| [`modul/edge_iot.wya`](file:///c:/widya-lang/modul/edge_iot.wya) | Hub telemetri sensor edge, manajemen daya baterai, buffer ring lokal offline-resilient, & paket CoAP/GPIO. |
| [`modul/fintech_settlement.wya`](file:///c:/widya-lang/modul/fintech_settlement.wya) | Limit order book matching engine L2, liquidity pool AMM, & kalkulasi valuasi risiko derivatif opsi Black-Scholes Greeks. |
| [`modul/ai_rag_stack.wya`](file:///c:/widya-lang/modul/ai_rag_stack.wya) | Pipeline Retrieval-Augmented Generation (RAG), indexing semantik vector, kueri konteks relevan & prompt augmentasi. |
| [`modul/sistem_terdistribusi.wya`](file:///c:/widya-lang/modul/sistem_terdistribusi.wya) | Konsensus Raft terdistribusi, sinkronisasi dokumen CRDT multi-master, & pelacakan kausalitas Jam Vektor (Vector Clocks). |
| [`modul/keamanan_siber_forensik.wya`](file:///c:/widya-lang/modul/keamanan_siber_forensik.wya) | Pusat Operasi Keamanan (SOC), pendeteksi serangan SIEM log, pencocokan aturan YARA, & analisis Shannon Entropy. |
| [`modul/cloud_devops_arsip.wya`](file:///c:/widya-lang/modul/cloud_devops_arsip.wya) | Otomatisasi CI/CD stage runner, generator konfigurasi OCI kontainer, & preservasi kemasan digital IETF RFC 8493 BagIt. |
| [`modul/spasial_dan_robotika.wya`](file:///c:/widya-lang/modul/spasial_dan_robotika.wya) | Stasiun kontrol kinematika maju/balik manipulator robotik 2-DOF, topologi spasial poligon GeoJSON & mekanika orbit. |
| [`modul/dsp_dan_multimedia.wya`](file:///c:/widya-lang/modul/dsp_dan_multimedia.wya) | Fast Fourier Transform (FFT & IFFT), filter digital IIR Biquad (Low-Pass/High-Pass), & simulasi akustik konvolusi. |
| [`modul/formal_dan_sains_komputasi.wya`](file:///c:/widya-lang/modul/formal_dan_sains_komputasi.wya) | Pemecah Boolean SAT DPLL, pembukti kendala linear real SMT-LIB2, & simulasi gravitasi n-body astrofisika Barnes-Hut. |
| [`modul/database_driver.wya`](file:///c:/widya-lang/modul/database_driver.wya) | Pool koneksi database enterprise, eksekusi transaksi ACID bergaransi WAL, & cache Redis in-memory dengan TTL. |
| [`modul/observability_k8s.wya`](file:///c:/widya-lang/modul/observability_k8s.wya) | Ekspor metrik Prometheus format standar (Counter/Gauge) & probe status kesehatan Kubernetes (Liveness/Readiness). |
| [`modul/autentikasi_enterprise.wya`](file:///c:/widya-lang/modul/autentikasi_enterprise.wya) | Manajemen otorisasi Role-Based Access Control (RBAC) & validasi token sesi JWT enterprise. |
| [`modul/async_runtime.wya`](file:///c:/widya-lang/modul/async_runtime.wya) | Penjadwal tugas komputasi non-blocking berbasis Actor Message Passing terisolasi. |
| [`modul/streaming_io.wya`](file:///c:/widya-lang/modul/streaming_io.wya) | Pemrosesan aliran potongan berkas data besar (*chunked stream*) bebas Out-Of-Memory. |














---

## 🦀 Perbandingan Lengkap: Widya-Lang vs Rust

Widya-Lang mengadopsi prinsip keandalan, keamanan memori, dan ekspresi fungsional dari Rust dengan sintaks Bahasa Indonesia yang bersahabat:

| Fitur / Paradigma | Di Bahasa Rust 🦀 | Di Widya-Lang 🇮🇩 | Keterangan & Keunggulan |
| :--- | :--- | :--- | :--- |
| **Algebraic Data Types (Enum)** | `enum Status { Aktif, Galat(String) }` | `enum Status { Aktif, Galat(pesan) }` | Tipe data varian bertenaga dengan payload data. |
| **Pattern Matching** | `match val { Pola => aksi }` | `cocokkan val { Pola => aksi }` | Pencocokan pola ekspresif dengan binding otomatis. |
| **If-Let Pattern** | `if let Some(x) = opt { ... }` | `jika misal Ada(x) = opt { ... }` | Ekstraksi pola cepat tanpa blok match penuh. |
| **Asynchronous I/O (Async/Await)** | `async fn f() -> T` & `.await` | `asinkron fungsi f()` & `tunggu_hasil` | Eksekusi operasi I/O dan konkurensi non-blocking. |
| **Error Try Operator (`?`)** | `let x = baca()?` | `misal x = baca()?` | Propagasi galat otomatis (*early return* jika Err). |
| **Traits / Sifat (Polymorphism)** | `trait Sifat { fn x(&self); }` | `sifat Sifat { fungsi x(); }` | Deklarasi antarmuka kelakuan (*shared behavior*). |
| **Implementasi Sifat (Impl)** | `impl Sifat for Tipe { ... }` | `terapkan Sifat untuk Tipe { ... }` | Penerapan polimorfisme modular pada struktur. |
| **Pattern Destructuring** | `let [a, b, c] = arr;` | `misal [a, b, c] = arr;` | Dekonstruksi elemen koleksi / array langsung ke variabel. |
| **Infinite Loop Expression** | `loop { ... break; }` | `ulang { ... berhenti; }` | Perulangan tak hingga idiomatis tanpa kondisi buatan. |
| **Functional Iterators** | `.map()`, `.filter()`, `.fold()`, `.zip()`, `.take()`, `.skip()` | `petakan()`, `saring()`, `lipat()`, `gabungkan()`, `ambil()`, `lewati()` | Pemrosesan koleksi data bergaya fungsional. |
| **Iterator Predicates** | `.any()`, `.all()`, `.find()` | `apakah_ada()`, `semua()`, `temukan()` | Pengecekan predikat elemen koleksi secara deklaratif. |
| **Concurrency & Utas (Threads)** | `std::thread::spawn(f)` | `buat_utas(f)` & `gabung_utas(u)` | Pembuatan thread terisolasi untuk eksekusi paralel. |
| **MPSC Channel Communication**| `std::sync::mpsc::channel()` | `saluran()`, `saluran_kirim()`, `saluran_terima()` | Komunikasi pesan aman antar alur eksekusi tanpa race condition. |
| **Package Manager Resmi** | `cargo new` & `cargo add` | `widya inisialisasi` & `widya tambah` | Pengelolaan dependensi dan struktur proyek standar (`widya.toml`). |
| **Pola Nilai Opsional** | `Option<T>` (`Some(v)` / `None`) | `Ada(v)` / `Kosong()` | Mencegah crash *null-pointer exception*. |
| **Pola Penanganan Galat** | `Result<T, E>` (`Ok(v)` / `Err(e)`) | `Ok(v)` / `Err(e)` | Penanganan hasil kalkulasi aman tanpa panic. |
| **Ekstraksi Data Aman** | `val.unwrap()` / `unwrap_or(default)` | `buka(val)` / `buka_atau(val, def)` | Ekstraksi eksplisit dari `Option` & `Result`. |
| **Pengecekan Status** | `res.is_ok()` / `res.is_err()` | `apakah_ok(res)` / `apakah_err(res)` | Verifikasi kevalidan data secara instan. |
| **Penegasan Keamanan** | `assert!(kondisi, pesan)` | `pastikan(kondisi, pesan)` | Verifikasi invariansi program di runtime. |
| **Immutability by Default** | `let` vs `let mut` / `const` | `tetap` (konstan) vs `misal` | Menjamin nilai konstan tidak dapat dimutasi. |
| **First-Class Closures** | `\|x\| x + 1` | `fungsi(x) { kembalikan x + 1; }` | Closure leksikal penuh dan higher-order function. |
| **Kemandirian Native** | Menghasilkan ELF / PE langsung | `widyac.wya` -> ELF / PE Native | Zero-dependency standalone binary output. |

---

## 📜 Lisensi
Lisensi MIT. Bebas dimodifikasi, didistribusikan, dan dikembangkan di semua platform OS.

