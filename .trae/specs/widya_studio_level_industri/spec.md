# Widya Studio Level Industri - Product Requirements Document (PRD)

## Overview
- **Summary**: Upgrade Widya Studio dari IDE Basic v1.0 (hanya 4 endpoint, 1 tab editor, readonly output) menjadi IDE level industri setara mini VS Code khusus Bahasa Pemrograman Widya-Lang. Menambahkan 18+ fitur level industri: Dashboard Welcome Page, Multi-Tab Editor, Save/Load file ke disk, Modul Explorer 80+ kategori industri, UI Kompilasi 6 Target (Native/Rust/LLVM/Wasm/WGSL/eBPF), Cargo Test Runner UI panel, Marker Error Inline di Monaco, REPL interaktif, Format Document, Command Palette (Ctrl+Shift+P), Git Mini Panel status, Debugger breakpoint + variables watch, AI Autocomplete via modul ai_rag_stack, Rate Limit API, Split Panel + Zen Mode, Recent Projects, Inisialisasi Proyek UI, dan Snippet insert dari vscode-extension/snippets.
- **Purpose**: Membuat Widya Studio siap dipakai software engineer INDUSTRI sehari-hari untuk mengembangkan proyek Widya-Lang 9 Layer Stack 100% (bare-metal → enterprise AI) tanpa harus pindah-pindah tool (tidak perlu keluar Studio untuk compile, test, commit, debug, setup project).
- **Target Users**: 
  1) Software Engineer Enterprise Indonesia membangun sistem DePIN, SaaS, AI On-Prem, Hardware OS
  2) Mahasiswa / akademisi melakukan riset GIS+SQL+NoSQL, FL AI Federasi, Mesh P2P, Embedded BAREMETAL GPIO
  3) Industri manufaktur / OT / ICS menggunakan SCADA OPC UA WidyaDB + WidyaOS Heterogen scheduler.

## Goals
1. **G1: Zero-Context Switching** — Semua fitur toolchain Widya (run/compile 6 target/cargo test/LSP/formatter/debug/git) TERSEDIA DALAM 1 JENDELA Widya Studio tanpa keluar browser / pindah terminal.
2. **G2: Backward Compatibility 100%** — Semua endpoint API yang sudah ada (/api/run, /api/check, /api/contoh, GET /) TETAP BERJALAN persis sama seperti v1.0 agar user yang sudah integrasi / dokumentasi TIDAK PERLU diubah.
3. **G3: UX setara mini VS-Code** — Shortcut standar industri (F5 run, Ctrl+S save, Ctrl+P quick open, Ctrl+Shift+P command palette, Ctrl+Shift+F search in project, F9 toggle breakpoint, Ctrl+` toggle terminal), Split panel Zen Mode, Recent Files.
4. **G4: Coverage 9 Layer Stack 100%** — Fitur Studio relevan untuk SETIAP layer L1 baremetal (export compile thumbv7m-none-eabi), L2 OS/HV, L3 container, L4 Mesh, L5 WidyaDB 6 engine, L6 hardware khusus SGX/CUDA/OCL/HSM/FPGA, L7 backend, L8 CLI toolchain, L9 app user.

## Non-Goals
1. NG-1: TIDAK rewrite Widya Studio menjadi Electron / Tauri App — tetap Browser-based via TcpListener Rust std lib (ringan, 0 binary dependency tambah).
2. NG-2: TIDAK buat installer binary Widya Studio terpisah — tetap invoke via `widya studio --port X` di src/main.rs.
3. NG-3: TIDAK menambahkan Cargo dependency selain yang SUDAH ADA di Cargo.toml (9 dep: clap, colored, rustyline, rand, serde_json, sha2, hex, hmac, regex) — tetap std::net TCP + serde_json hanya.
4. NG-4: TIDAK implementasi user login / cloud sync — tetap local-only IDE untuk keamanan proyek industri confidential.
5. NG-5: TIDAK menghapus / merename file atau fungsi lama di studio.rs — hanya EXTEND (append kode baru, TIDAK ada edit yang merusak function signature lama handle_api_run / handle_api_check / render_ide_html yang mungkin sudah dipakai user).

## Background & Context
Widya Studio saat ini (di src/studio.rs:1-415):
- HTTP server TcpListener single-thread port default 8080, 4 endpoint: GET /, POST /api/run, POST /api/check, GET /api/contoh
- UI hanya 1 TAB editor, Monaco v0.45, Tailwind CDN, FontAwesome 6.4, explorer hanya folder contoh/ (tidak akses modul/ atau workspace folder)
- Hanya bisa RUN interpreter — tidak bisa save file, compile 6 target, cargo test, REPL input, debug
- Output konsol read-only (tidak bisa stdin ke interpreter yang membutuhkan input baca())
- Tidak ada inline error marker galat sintaks di Monaco — hanya tampil galat di output panel
- Tidak ada rate limit, jadi mudah DoS jika banyak request.

Semua modul toolchain 9 Layer SUDAH READY di repo (80 modul industri, 168 contoh file, 2 test suite Rust 33 test pass, cargo compile 6 target). Hanya butuh STITCHING ke UI Widya Studio via endpoint baru + JS client enhancement.

## Functional Requirements
### A. Backend Rust (extend src/studio.rs & src/main.rs)
- **FR-1**: 20+ endpoint API BARU ditambahkan (TANPA menghapus 4 endpoint existing)
  - Workspace: POST /api/workspace/buka_folder, GET /api/workspace/tree, POST /api/file/simpan, POST /api/file/baca, POST /api/file/hapus, POST /api/file/rename
  - Compile: POST /api/compile/{native,rust,llvm,wasm,wgsl,ebpf} → manggil fungsi yang sama di src/main.rs CLI per kompilasi target
  - Test: POST /api/cargo/test → execute `cargo test -- --nocapture 2>&1` stream output
  - REPL: POST /api/repl/step → stateful REPL interpreter (state per session)
  - Format: POST /api/format → indentasi auto + semantic formatter Widya
  - Init Project: POST /api/project/init {nama} → ekivalen `widya inisialisasi nama`
  - Dashboard: GET /api/dashboard → list recent projects, modul count, contoh count, cargo test summary terakhir
  - Explorer Modul: GET /api/modul → list 80+ modul industri dengan KATEGORI (AI/DB/OS/HW/Network dll) + file content
  - Git: GET /api/git/status, POST /api/git/commit {pesan} → mini wrapper git
  - Debug: POST /api/debug/start, POST /api/debug/step, GET /api/debug/variables → interpreter mode debug
  - AI Autocomplete: POST /api/ai/complete {source, posisi} → wrapper modul ai_rag_stack completions
  - Snippets: GET /api/snippets → baca dari vscode-extension/snippets/snippets.json
- **FR-2**: Backward Compat Endpoint Lama TETAP ADA & BERPERILAKU SAMA: handle_api_run, handle_api_check, handle_api_list_contoh, render_ide_html (tidak ada parameter baru yang required).
- **FR-3**: Rate Limiter per IP — max 120 request / menit, jika kelebihan return HTTP 429 Too Many Requests JSON (bukan crash / drop connection).
- **FR-4**: Stdin Streaming untuk interpreter — output cetak() dikirim SSE (Server-Sent Events) / chunked ke UI secara realtime (tidak buffer semua baru kirim seperti lama).

### B. Frontend Monaco (extend render_ide_html di src/studio.rs)
- **FR-5**: Dashboard Welcome Page di tab pertama (jika tidak ada file yang dibuka) — 4 kartu: [Buat Proyek Baru, Buka Folder Workspace, Buka Contoh Terbaru, Buka Modul Industri] + Recent Projects list.
- **FR-6**: Multi-Tab Editor — bisa buka 5+ file sekaligus, close tab, dirty state indicator (* jika belum disave), urutan tab bisa di-drag.
- **FR-7**: Workspace Explorer (Pohon Direktori) — selain folder contoh/, juga bisa buka SEMUA subfolder: src/, modul/, tests/, dokumen_widya/, .github/, dan workspace apapun user pilih via `POST /api/workspace/buka_folder`. Icon per file type (.wya biru, .rs oranye, .md abu, .json kuning).
- **FR-8**: Modul Explorer KATEGORI 80+ — sidebar terpisah menampilkan 80+ modul industri by kategori (Fintech/Health/AI/DB/OS/HW/GIS dll) — click nama modul langsung tab content modul.
- **FR-9**: Inline Error Marker di Monaco — galat sintaks / borrow checker / LSP diagnostic TIDAK hanya di output panel, tapi MUNCUL garis MERAH di nomor baris Monaco beserta tooltip pesan galat (via editor.deltaDecorations).
- **FR-10**: 6 Panel Output Bawah (bukan 1 seperti lama) dengan Tab Switcher: [Konsol Output, Masalah (Diagnostics), Terminal REPL Interaktif, Cargo Test Report, Kompilasi Hasil, Debug Console, Aktivitas Git, AI Suggestions] — total 8 tab output.
- **FR-11**: REPL Terminal Interaktif — input prompt `widya>` di panel TERMINAL REPL, user ketik kode widya inline 1 baris lalu Enter, hasil langsung tampil (bukan harus via file simpan dulu). State REPL persisten: variabel misal yang dideklarasikan sebelumnya TETAP ADA di request selanjutnya (session per websocket).
- **FR-12**: Cargo Test Runner UI Panel — tombol "Jalankan Semua Test", list nama test dengan: nama test, status (pass hijau / fail merah / running kuning), durasi ms, expandable stack trace jika fail. Equivalent `cargo test --test test_all9layer_core --test test_all9layer_full_stack`.
- **FR-13**: UI Kompilasi 6 Target (menu Build) — dropdown pilih target: (T1 Native ELF/PE, T2 Rust, T3 LLVM IR, T4 Wasm+HTML, T5 WGSL, T6 eBPF) + tombol Kompilasi. Setelah selesai: link download output file, lokasi path output, pesan compiler error/warning (jika ada).
- **FR-14**: Command Palette (Ctrl+Shift+P) — daftar 50+ perintah yang bisa di-search & execute: File: Simpan, File: Buka Folder, File: Cari di Proyek (Ctrl+Shift+F), Edit: Format Dokumen (Alt+Shift+F), Build: Kompilasi Native, View: Zen Mode (Ctrl+K Z), View: Toggle Terminal (Ctrl+`), Debug: Mulai Debugging (F5 alt), Source Control: Commit, Insert Snippet: ... , Workspace: Inisialisasi Proyek.
- **FR-15**: Save / Save As / Auto Save — Ctrl+S simpan file ke path workspace aktual (jika file berasal dari folder), Ctrl+Shift+S Save As pilih nama baru. Auto Save tiap 5 detik jika opsi diaktifkan di Settings (lihat FR-17).
- **FR-16**: Mini Debugger (F9 / F10 / F5 / Shift+F5) — toggle breakpoint (titik MERAH di gutter nomor baris). Start Debug (F5 alt), Step Over (F10), Continue (F5), Stop (Shift+F5). Panel Variables Watch menampilkan isi variabel misal/tetap saat pause di breakpoint.
- **FR-17**: Settings Dialog (Ctrl+,) — 10+ opsi konfigurasi user: tema (Catppuccin Dark / Latte Light / Monokai / Dracula), font size 12-20, tab size 2/4, auto save on/off, auto format on save, minimap enable/disable, word wrap enable/disable, AI autocomplete enable/disable, rate limit warning notify. Settings disimpan di localStorage browser (bukan file disk, untuk portability).
- **FR-18**: Search / Replace in Project (Ctrl+Shift+F) — cari string secara rekursif di seluruh workspace (folder yang dibuka), tampilkan list match [nama_file:baris:kolom] preview context, bisa replace all / replace satu.
- **FR-19**: Split Editor (Split Kanan / Split Bawah) + Zen Mode (Ctrl+K Z) — buka 2 file berdampingan untuk bandingkan. Zen Mode: sembunyikan sidebar + panel bawah + header, full layar hanya editor (ESC 2x keluar).
- **FR-20**: Source Control Mini Panel (ikon branch di sidebar) — tampilkan file modified / staged (git status), input pesan commit, tombol Commit (ekivalen `git add -A && git commit -m "pesan"`), tombol Push & Pull (opsional jika remote origin ada).
- **FR-21**: AI Autocomplete Inline (Ctrl+Space) — saat mengetik di editor posisi kursor, request POST /api/ai/complete berisi source code + baris/kolom, tampilkan 3 opsi snippet code suggesstion modul ai_rag_stack (pakai pattern context 80 modul terdekat).
- **FR-22**: Snippet Insert (Ctrl+Shift+P → "Insert Snippet") — list snippet dari vscode-extension/snippets/snippets.json, pilih satu maka snippet body di-insert ke posisi kursor Monaco.

## Non-Functional Requirements
- **NFR-1**: UX Performance — First load halaman `/` (full render Monaco + Tailwind) selesai DALAM waktu ≤ 2.0 detik (di laptop Windows i5 Gen 11 dengan internet stabil).
- **NFR-2**: API Response Time — untuk request yang tidak berhubungan compile/test (misal /api/check, /api/file/baca kecil, /api/snippets) response time P95 ≤ 200 ms tanpa blocking UI.
- **NFR-3**: Backward Compatibility 0% Break — Semua endpoint yang SUDAH ADA di v1.0 tetap ada: GET /, POST /api/run, POST /api/check, GET /api/contoh — request payload & response schema SAMA PENUH (field "sukses", "galat", "hasil" untuk run; array diagnostics untuk check; array {name,content} untuk contoh).
- **NFR-4**: Error Resilience — Setiap endpoint baru selalu return JSON 200 dengan `{"sukses": false, "galat": "pesan"}` untuk galat internal (bukan HTML 500 crash server). Server TIDAK panik / crash jika file permission error / JSON payload invalid.
- **NFR-5**: Maintainable Code Structure — Fungsi di src/studio.rs TIDAK lebih panjang dari 120 baris satu fungsi. Setiap endpoint baru punya fungsi handler terpisah dengan nama `handle_api_[nama_endpoint]`.
- **NFR-6**: Accessibility (a11y) — Semua tombol action memiliki `aria-label`, focus keyboard bisa navigasi ke 90% UI tanpa mouse (tab / shift+tab), kontras warna untuk tema default minimal WCAG AA 4.5:1.
- **NFR-7**: Security Local Isolation — Request ke luar local hanya 3 CDN (tailwindcss, monaco cdnjs, fontawesome) — TIDAK ada request lain ke internet (AI autocomplete offline via modul ai_rag_stack local, tidak pakai OpenAI API remote kecuali user setting explicit).

## Constraints
- **Technical**:
  1. Hanya pakai Rust std library + 9 dep Cargo.toml yang sudah ada (TIDAK add hyper/axum/actix — tetap TcpListener manual untuk zero cost).
  2. UI tetap inline HTML string di `render_ide_html()` (bukan file .html terpisah) agar binary widya.exe tetap single file tanpa asset.
  3. Monaco tetap v0.45 dari CDN (tidak host local), Tailwind v3 CDN, FontAwesome v6.4 CDN.
- **Business**:
  1. Budget $0 — GRATIS semua stack, tidak perlu subscription / API key berbayar.
  2. Deadline 1 iterasi (< 1 minggu implementasi di repo ini).
- **Dependencies**:
  1. Interpreter Lexer + Parser + Borrow Checker SUDAH ADA di src/lexer.rs, src/parser.rs, src/borrow_checker.rs.
  2. 6 Target compile function SUDAH ADA di src/main.rs: kompilasi native/rust/llvm/wasm/wgsl/ebpf (bisa di-ekstrak ke fungsi public di lib.rs agar bisa dipanggil dari studio.rs).
  3. 80+ modul industri di modul/ (baca dari disk via fs::read_dir kategorikan by prefix nama file).
  4. vscode-extension/snippets/snippets.json SUDAH ADA.

## Assumptions
1. User menjalankan Widya Studio di local browser (Chrome/Firefox/Edge latest) — tidak support IE/Safari 13 kebawah.
2. Git CLI TELAH TERINSTALL di PATH user untuk Source Control Mini Panel (jika tidak ada → UI menampilkan info "Git tidak terinstall, fitur commit nonaktif").
3. User running Widya Studio via `cargo run --release -- studio` atau binary widya.exe release — jadi Rust toolchain (rustc + cargo) ADA untuk fitur POST /api/cargo/test dan kompilasi target Rust/LLVM.
4. Monaco editor support zoom, multi-cursor, find replace built-in dari librarynya — tidak perlu reimplement.

## Acceptance Criteria

### AC-R1: Endpoint Backward Compat v1.0 TETAP 100% BERJALAN
- **Type**: `rule`
- **Given**: Studio dijalankan port 8080, request payload lama seperti sebelum upgrade
- **When**: Mengirim (1) GET / → dapatkan HTML (2) POST /api/run body {"source":"cetak(\"halo\");kembalikan benar;"} → (3) POST /api/check body kode error → (4) GET /api/contoh → array contoh
- **Then**: Semua return schema field SAMA PENUH: run punya "sukses" bool + "hasil" atau "galat"; check punya array diags; contoh punya array {name, content}
- **Pass Condition**: 4 endpoint v1.0 return HTTP 200 & JSON schema match 100% (dibuktikan via curl sebelum/sesudah upgrade, diff body kosong untuk 4 kasus)
- **Evidence**: Output curl request 4 endpoint diff lama vs baru field-by-field match

### AC-R2: Workspace Explorer bisa Buka Folder + Save Load File Aktual ke Disk
- **Type**: `rule`
- **Given**: User pilih folder workspace "C:\widya-lang" di Dashboard
- **When**: (1) Explorer menampilkan tree subfolder (contoh/ modul/ src/ tests/ dokumen_widya/ .github) expandable; (2) Buka file `contoh/01_halo_dunia.wya` → ubah baris tambahkan `cetak("edit studio");` → Ctrl+S
- **Then**: (a) Tree muncul benar di explorer sidebar; (b) File di-disk TERUPDATE dengan baris edit; (c) Dirty state * hilang dari nama tab setelah save
- **Pass Condition**: md5sum file sebelum/sesudah edit BERBEDA (terbukti benar-benar ditulis); tree explorer tampil 6+ item subfolder
- **Evidence**: fs::read_to_string file aktual sama persis dengan value editor.getValue(); ls explorer length ≥6

### AC-R3: Multi-Tab Editor ≥5 File Buka Serentak + Dirty State
- **Type**: `rule`
- **Given**: -
- **When**: Buka 7 file berbeda via explorer (01,02,05,10,15,168 contoh + modul/widya_os.wya) → edit 2 tab di tengah tanpa save
- **Then**: (a) Tab bar menampilkan 7 tab dengan nama file; (b) 2 tab yang diedit punya asterisk * di nama (dirty indicator); (c) Click tab A → tampil content A, click tab C → content C TIDAK HILANG (persis saat terakhir edit)
- **Pass Condition**: Map<namafile, content> 7 file semua stored di sisi client JS, 2 tab * dirty, switching tab tidak reset value
- **Evidence**: console.log(Object.keys(globalTabs).length) ≥ 7; 2 nama tab berakhiran "*"

### AC-R4: Inline Error Marker di Monaco (bukan hanya output panel)
- **Type**: `rule`
- **Given**: Editor isi code yang GALAT sintaks: `fungsi () { { salah kurung }`
- **When**: Tunggu 500ms debounce auto check POST /api/check
- **Then**: (a) Di Monaco gutter baris terdapat GARIS MERAH + icon error di nomor baris yang galat; (b) Hover marker menampilkan pesan galat persis; (c) Tab Problems di panel bawah menampilkan 1 baris entry galat dengan link "klik → lompat ke baris"
- **Pass Condition**: editor.getModel().getAllDecorations().length ≥ 1 bertype error; panel Problems tab count ≥ 1
- **Evidence**: Screenshot Monaco dengan marker merah + deltaDecorations return array length ≥ 1

### AC-R5: UI Kompilasi 6 Target BISA JALANKAN 6 target berbeda hasil generate file output
- **Type**: `rule`
- **Given**: Editor buka `contoh/02_fibonacci.wya` (fungsi fibonacci valid return int)
- **When**: Menu Build → pilih 1 per 1 target [Native, Rust Source, LLVM IR, Wasm+HTML, WGSL, eBPF] → klik tombol Kompilasi masing-masing
- **Then**: Untuk SETIAP target: (a) tab "Kompilasi" menampilkan log berjalan; (b) akhir sukses → tombol "Download Output" enable; (c) path file output ada di harddisk (contoh: output_native/fibonacci.exe, output_wasm/app.wasm dll)
- **Pass Condition**: 6 file output (1 per target) EXIST di folder output_* setelah 6x compile
- **Evidence**: `ls -la output_native/ output_rust/ output_llvm/ output_wasm/ output_wgsl/ output_ebpf/` masing-masing punya ≥1 file

### AC-R6: Cargo Test Runner UI Menampilkan ≥30 Test Pass dengan Status Berwarna
- **Type**: `rule`
- **Given**: -
- **When**: Klik tombol "Jalankan Semua Test" di panel Cargo Test
- **Then**: (a) List test nama individual 1 per baris; (b) 33 test (sesuai repo) tampil PASS latar hijau; (c) Total bar bawah menampilkan "test result: ok. 33 passed; 0 failed"; durasi total detik
- **Pass Condition**: count hijau pass = 33, count merah fail = 0, summary match cargo test official
- **Evidence**: Screenshot panel Cargo Test + log cargo test 33 passed di dalam tab

### AC-R7: REPL Terminal Interaktif State Persisten (Session per Client)
- **Type**: `rule`
- **Given**: Panel Terminal REPL interaktif aktif
- **When**: (1) Ketik `misal x = 42;` enter → (2) Ketik `cetak(x * 2);` enter → (3) Ketik `kembalikan x * 10;` enter
- **Then**: Output (1) tidak ada error; output (2) menampilkan 84; output (3) return 420 → VARIABEL x tetap ADA di request ke-3 (tidak direset setiap request)
- **Pass Condition**: REPL session state HashMap<sessionid, Interpreter> di backend memory survive 3 request berturut, tidak di-new ulang
- **Evidence**: POST /api/repl/step 3 kalimat berturut return value 420 di ketiga

### AC-R8: Command Palette (Ctrl+Shift+P) ≥30 Action Bisa di Search & Eksekusi
- **Type**: `rule`
- **Given**: Focus di editor
- **When**: Tekan Ctrl+Shift+P → muncul input palette → ketik pencarian "Simpan" → tampil 5+ hasil → enter "File: Simpan (Ctrl+S)"
- **Then**: (a) Palette muncul overlay hitam transparan di tengah; (b) List action di-filter by search string; (c) Eksekusi action "Simpan" → behavior persis Ctrl+S (save aktif); (d) Ada minibadge shortcut key di kanan setiap item
- **Pass Condition**: Action array length ≥ 30 items; search filter "Simpan" return ≥3; execute action tidak galat
- **Evidence**: console.log(COMMAND_LIST.length) ≥ 30; after execute save, file saved (sesuai AC-R2 bukti)

### AC-R9: Modul Explorer KATEGORI 80+ Tampil per Industri Sektor
- **Type**: `rule`
- **Given**: Sidebar panel Modul (icon box) terbuka
- **When**: Lihat daftar kategori (Fintech, Medis, Otomotif, GIS Spasial, AI/ML, OS Kernel, Hardware Khusus, Mesh Networking, DevOps, Manufaktur SCADA dll) → click kategori Fintech → expand → click file fintech_iso20022_xml.wya
- **Then**: (a) Kategori count ≥ 12 sektor industri berbeda; (b) Saat expand Fintech: 2+ file modul muncul; (c) Click file → tab editor baru buka content modul aktual
- **Pass Condition**: Kategori count ≥ 12; total files di semua kategori = 80+; click 1 modul sukses load ke tab
- **Evidence**: console.log(Object.keys(kategoriModul).length) ≥ 12; totalFiles = sum(kategori.values().length) ≥ 80

### AC-R10: Settings Dialog (Ctrl+,) Tersimpan di LocalStorage
- **Type**: `rule`
- **Given**: Settings dialog terbuka
- **When**: Ubah tema menjadi "Latte Light", font size jadi 16, auto save enable → tutup dialog → refresh page F5 → buka settings lagi
- **Then**: (a) Sebelum refresh: tema berganti langsung (latte = latar terang); (b) Sesudah F5 reload: setting nilai TETAP = Light, 16, auto save = true (terload dari localStorage)
- **Pass Condition**: localStorage.getItem("widya_studio_settings") JSON punya ketiga key set dengan value sesuai
- **Evidence**: JSON.parse(localStorage.getItem(...)).theme == "latte" && fontSize == 16 && autoSave = true

### AC-R11: Rate Limiter 120 req/menit per IP return 429
- **Type**: `rule`
- **Given**: IP client 127.0.0.1 hit endpoint berulang
- **When**: Kirim 121 POST /api/check beruntun dalam < 1 menit
- **Then**: Request ke 1-120 return HTTP 200; Request ke 121 return HTTP 429 Too Many Requests dengan JSON body `{"sukses":false, "galat":"Rate limit terlampaui: 120 req/menit per IP"}`
- **Pass Condition**: Response code ke 121 = 429; body JSON berisi string match
- **Evidence**: curl loop 121 → head -n 1 response 121 = "HTTP/1.1 429 Too Many Requests"

### AC-R12: Inisialisasi Proyek Baru dari Dashboard (Widya Package Manager)
- **Type**: `rule`
- **Given**: Dashboard Welcome Page terbuka
- **When**: Klik kartu 1 "Buat Proyek Baru" → input nama = "proyek_saya_industri" → tombol "Inisialisasi"
- **Then**: Di harddisk, folder "./proyek_saya_industri/" TERBUAT dengan struktur: `proyek_saya_industri/src/main.wya`, `proyek_saya_industri/Widya.toml` (konfigurasi paket), `proyek_saya_industri/README.md` → 3 file exist
- **Pass Condition**: 3 file proyek semuanya exist & readable; Widya.toml ada field nama_proyek = "proyek_saya_industri"
- **Evidence**: cat Widya.toml field nama_proyek match

### AC-R13: Format Document (Alt+Shift+F) Indentasi Konsisten 4 Spasi
- **Type**: `rule`
- **Given**: Editor diisi kode indentasi kacau (mixed tab+spasi+tidak ada newline)
- **When**: Alt+Shift+F → "Format Dokumen" action execute via POST /api/format
- **Then**: Kode TERFORMAT dengan aturan: (1) Semua indent inside {} 4 spasi; (2) setelah } ada 1 newline; (3) panjang baris galat >120 kolom di-wrap; (4) Keyword alignment rata kiri fungsi/jika di level 0
- **Pass Condition**: Diff sebelum/sesudah format ada perubahan ≥5 baris indentasi; tidak ada tab character \t tersisa
- **Evidence**: Diff text before-after count 5+ line; "\t".count(formatted) = 0

### AC-R14: Mini Debugger Breakpoint + Step + Variables Watch
- **Type**: `rule`
- **Given**: Kode editor: `misal a=1; misal b=2; misal c=a+b; cetak(c); kembalikan c;`
- **When**: (1) Klik gutter baris 3 (c=a+b) → SET breakpoint merah; (2) F5 alt Start Debug; (3) Pause di baris 3 → panel Variables menampilkan `a=1, b=2` (c belum dihitung); (4) F10 Step Over sekali; (5) Variables update `c=3`; (6) F5 Continue → selesai, cetak(3) tampil di Debug Console
- **Then**: (a) Breakpoint visual marker red muncul; (b) Pause state benar (pause sebelum hitung c); (c) Nilai variabel sesuai state; (d) Step over update c ke 3; (e) Selesai run total
- **Pass Condition**: Panel debug state = paused di step 3, vars watch 2 entry → setelah F10 vars watch 3 entry (c muncul 3)
- **Evidence**: Screenshot debug panel + debug log step print a,b,c state

### AC-R15: Split Editor + Zen Mode Toggle Berlaku
- **Type**: `rule`
- **Given**: Dua tab file dibuka
- **When**: (1) Command Palette "View: Split Kanan" → editor split menjadi 2 kolom (kiri=tab1, kanan=tab2); (2) Command Palette "View: Zen Mode (Ctrl+K Z)" → sidebar + panel bawah + header hide; (3) Tekan ESC 2x berturut → UI kembali normal
- **Then**: (a) Split count DOM container editor = 2 (flex 2 kolom); (b) Zen Mode: class `zen-mode` ter-apply di body → sidebar display:none + panel display:none + header display:none; (c) ESC 2x class dihapus → semuanya muncul kembali
- **Pass Condition**: document.querySelectorAll(".monaco-container").length == 2 ketika split; (sidebar hidden + panel hidden + header hidden) = true ketika zen
- **Evidence**: Check DOM length & style display property

### AC-R16: Source Control Mini Panel (Git Status + Commit)
- **Type**: `rule`
- **Given**: 1 file dimodifikasi (misal ubah 1 baris contoh/01_halo_dunia.wya)
- **When**: Buka panel Source Control (ikon branch kiri sidebar) → (a) Lihat file modified "contoh/01_halo_dunia.wya" dengan icon M kuning; (b) Ketik pesan commit "test commit studio panel"; (c) Tekan tombol commit checkmark
- **Then**: (a) Modified count = 1 file; (b) Setelah commit → `git log -n1 --oneline` menampilkan hash + pesan sesuai input user; (c) File modified TIDAK ada di M list lagi (sudah ter-commit)
- **Pass Condition**: git log -n1 match pesan; after commit modified count = 0
- **Evidence**: Output git log -n1 & git status clean after commit

### AC-U1: UX Keseluruhan setara mini VS-Code (Rubric 1-5)
- **Type**: `rubric`
- **Dimension**: Kemiripan alur kerja Widya Studio dengan kebiasaan engineer yang pakai VS Code sehari-hari (shortcut, panel layout, explorer behavior, command palette)
- **Scale**: 1-5
- **Anchors**:
  1 = Layout acak, shortcut tidak ada yang match VS Code standar, perlu belajar 3+ jam baru bisa pakai
  3 = Layout mirip tapi hanya 50% shortcut standar tersedia (F5 & Ctrl+S ada, Ctrl+Shift+P tidak / tidak lengkap)
  5 = SHORTCUT 80% match VS Code standar untuk 50+ action (Ctrl+S, Ctrl+P, Ctrl+`, Ctrl+Shift+P, Ctrl+Shift+F, F5, F9, F10, Alt+Shift+F, Ctrl+, Ctrl+K Z), explorer tree behave persis VS Code, multi-tab drag, dirty indicator * persis
- **Pass Threshold**: >= 4
- **Evidence**: Test checklist 50 shortcut & layout compare VS Code official → persentase match

### AC-U2: Dokumentasi Inline Studio Self-Explanatory (Rubric 1-5)
- **Type**: `rubric`
- **Dimension**: User baru tanpa baca TUTORIAL_11/12 BISA memakai 80% fitur level industri HANYA dari tooltip, icon, placeholder input, dan action label yang ADA DI DALAM UI Widya Studio (tanpa buka tab lain)
- **Scale**: 1-5
- **Anchors**:
  1 = Tidak ada tooltip, banyak tombol tanpa label icon hanya, user butuh open dokumentasi .md untuk paham
  3 = Tooltip hanya 50% tombol, 30% command ada label yang jelas
  5 = SETIAP action punya tooltip hover (1-2 kalimat penjelasan dalam Bahasa Indonesia), welcome page dashboard ada "Tips Cepat" 5 tips, command palette ada deskripsi pendek, setting punya help text, indikator status jelas (LSP Aktif/ Rate Limit/ Autosave Aktif)
- **Pass Threshold**: >= 4
- **Evidence**: Audit seluruh button/action/command di UI → ≥ 90% punya tooltip atau label jelas

### AC-U3: Integrasi 9 Layer Stack 100% Coverage di Fitur Studio (Rubric 1-5)
- **Type**: `rubric`
- **Dimension**: Seberapa baik Widya Studio memfasilitasi pengembangan untuk SETIAP layer dari L1 Bare-Metal s/d L9 User App (9 layer × 90 = 810 poin), TIDAK hanya layer aplikasi doang
- **Scale**: 1-5
- **Anchors**:
  1 = Hanya bisa run interpreter (L9 apps) — tidak ada fitur untuk baremetal/OS/compile/hardware khusus
  3 = Ada 5 layer yang fitur explicit di UI (L7-L9 + L5 DB + L4 Mesh), 4 layer lain masih harus keluar studio
  5 = KE-9 LAYER punya fitur explicit di UI level industri:
      • L1: Build target baremetal + documentation STM32 export BIN (via LLVM/Rust)
      • L2: Modul Explorer ada interrupt_idt_apic.wya + hypervisor_kvm_xen.wya (L2) + Debugger trace OS kernel
      • L3: Compile Wasm runner + resource cgroup V2 template + Dockerfile generate
      • L4: Modul mesh_libp2p_style + DHT demo di contoh
      • L5: Explorer modul 6 engine WidyaDB + SQL/GIS/FTS/Vector/NoSQL/TS
      • L6: Modul 5 kategori hardware (SGX/CUDA/OCL/HSM/FPGA) snippet insert
      • L7: Backend scaffolding (microservice_stack.wya snippet) + compile native PE/ELF
      • L8: Seluruh CLI (kompilasi×6 + cargo test + LSP + format + init) ADA di UI Studio (TIDAK perlu ketik CLI)
      • L9: Halaman app widget Flutter-like widya UI render preview
- **Pass Threshold**: >= 4
- **Evidence**: Checklist per layer (9 layer) → setidaknya 8 layer punya ≥1 fitur explicit di UI; total ≥ 720 poin (90×8)

### AC-U4: Code Maintainability & Rust Best Practice (Rubric 1-5)
- **Type**: `rubric`
- **Dimension**: Kualitas kode backend src/studio.rs & frontend HTML/JS inline
- **Scale**: 1-5
- **Anchors**:
  1 = Semua handler global 1 fungsi 1000+ baris, tidak ada komentar struktur, magic number, copy-paste 5x kode yang sama
  3 = Fungsi terpisah tapi masih ada 2-3 copy-paste pattern, beberapa handler > 200 baris
  5 = (Rust) Setiap handler endpoint = fungsi terpisah `handle_api_[nama]` ≤ 120 baris, error handling pakai match/if-let Result, tidak ada .unwrap() di path user-facing (cuma internal test). (JS) Semua async pakai try/catch, tidak ada global variable berceceran (pakai module scope object globalStudioState), function ≤ 80 baris.
- **Pass Threshold**: >= 4
- **Evidence**: Audit Rust: count semua fungsi handle_api_* ≥15 & max lines ≤ 120; Audit JS: count unwrap() user-facing = 0 (hanya unit test yang boleh)

## Open Questions
- [ ] Q1: Apakah AI Autocomplete OFFLINE WAJIB menggunakan modul ai_rag_stack internal (dengan HNSW vector 80 modul saja) atau user boleh setting explicit API Key OpenAI / Anthropic via Settings? Default = OFFLINE only untuk keamanan industri.
- [ ] Q2: Apakah Source Control Mini Panel wajib support `git push` dan `git pull` (risiko kredensial) atau cukup hanya local `git add && git commit`? Default = Commit only, Push/Pull via terminal REPL (lebih aman).
- [ ] Q3: Apakah tab AI Suggestions (FR-10) perlu diaktifkan DEFAULT untuk user pertama? Default = OFF (nyala via Settings toggle AI).
- [ ] Q4: Output file compile target (AC-R5) mau disimpan ke folder proyek (recommended: /target/widya_compile/) OR folder lokal user explicit via dialog? Default = `./target/widya_compile/{target}/` agar tidak menambah polusi root (sesuai struktur .gitignore /target).
