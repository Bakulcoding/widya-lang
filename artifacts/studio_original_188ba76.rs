use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::lsp::WidyaLsp;
use crate::parser::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::net::TcpListener;

pub fn jalankan_studio(port: u16) {
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Gagal mengikat port {}: {}", port, e);
            return;
        }
    };

    println!("===============================================================");
    println!("≡ƒîƒ WIDYA STUDIO (IDE MANDIRI LINTAS PLATFORM) AKTIF!");
    println!("≡ƒîÉ Buka browser Anda di: http://127.0.0.1:{}", port);
    println!("   Tekan Ctrl+C di terminal ini untuk keluar.");
    println!("===============================================================");

    // Otomatis buka browser default di Windows
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd")
        .args(["/C", "start", &format!("http://127.0.0.1:{}", port)])
        .spawn();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buffer = [0; 65536];
            if let Ok(bytes_read) = stream.read(&mut buffer) {
                let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                
                // Route endpoints
                if request.starts_with("GET / ") || request.starts_with("GET /index.html") {
                    let html = render_ide_html();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
                        html.len(),
                        html
                    );
                    let _ = stream.write_all(response.as_bytes());
                } else if request.starts_with("POST /api/run") {
                    let body = extract_body(&request);
                    let result_json = handle_api_run(&body);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
                        result_json.len(),
                        result_json
                    );
                    let _ = stream.write_all(response.as_bytes());
                } else if request.starts_with("POST /api/check") {
                    let body = extract_body(&request);
                    let result_json = handle_api_check(&body);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
                        result_json.len(),
                        result_json
                    );
                    let _ = stream.write_all(response.as_bytes());
                } else if request.starts_with("GET /api/contoh") {
                    let contoh_list = handle_api_list_contoh();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
                        contoh_list.len(),
                        contoh_list
                    );
                    let _ = stream.write_all(response.as_bytes());
                } else {
                    let not_found = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                    let _ = stream.write_all(not_found.as_bytes());
                }
            }
        }
    }
}

pub fn jalankan_lsp() {
    println!("Widya LSP aktif pada standard I/O. Siap menerima permintaan editor...");
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    while let Ok(n) = handle.read_to_string(&mut buffer) {
        if n == 0 { break; }
    }
}

fn extract_body(req: &str) -> String {
    if let Some(pos) = req.find("\r\n\r\n") {
        req[(pos + 4)..].to_string()
    } else {
        String::new()
    }
}

fn handle_api_run(body: &str) -> String {
    let source = if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
        val.get("source").and_then(|s| s.as_str()).unwrap_or(body).to_string()
    } else {
        body.to_string()
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            return serde_json::json!({
                "sukses": false,
                "galat": format!("{}", e)
            }).to_string();
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            return serde_json::json!({
                "sukses": false,
                "galat": format!("{}", e)
            }).to_string();
        }
    };

    let mut interpreter = Interpreter::new();
    match interpreter.interpret(&program) {
        Ok(val) => {
            serde_json::json!({
                "sukses": true,
                "hasil": val.to_string_repr()
            }).to_string()
        }
        Err(e) => {
            serde_json::json!({
                "sukses": false,
                "galat": format!("{}", e)
            }).to_string()
        }
    }
}

fn handle_api_check(body: &str) -> String {
    let diags = WidyaLsp::get_diagnostics(body);
    serde_json::to_string(&diags).unwrap_or_else(|_| "[]".to_string())
}

fn handle_api_list_contoh() -> String {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir("contoh") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("wya") {
                if let (Some(name), Ok(content)) = (path.file_name().and_then(|n| n.to_str()), fs::read_to_string(&path)) {
                    files.push(serde_json::json!({
                        "name": name,
                        "content": content
                    }));
                }
            }
        }
    }
    serde_json::to_string(&files).unwrap_or_else(|_| "[]".to_string())
}

fn render_ide_html() -> String {
    r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Widya Studio - IDE Mandiri Bahasa Pemrograman Widya</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="stylesheet" data-name="vs/editor/editor.main" href="https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/editor/editor.main.min.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css">
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; }
        .tab-active { border-bottom: 2px solid #6366f1; background-color: #1e1e2e; color: #fff; }
    </style>
</head>
<body class="bg-[#181825] text-gray-200 h-screen flex flex-col overflow-hidden select-none">

    <!-- Top Navigation Bar -->
    <header class="bg-[#11111b] border-b border-[#313244] h-12 flex items-center justify-between px-4">
        <div class="flex items-center space-x-3">
            <span class="text-xl">≡ƒç«≡ƒç⌐</span>
            <span class="font-bold text-lg bg-gradient-to-r from-red-500 via-pink-500 to-indigo-400 bg-clip-text text-transparent">Widya Studio</span>
            <span class="text-xs bg-indigo-900/60 text-indigo-300 border border-indigo-700/50 px-2 py-0.5 rounded">v1.0 IDE</span>
        </div>

        <div class="flex items-center space-x-2">
            <button id="btn-run" class="flex items-center space-x-2 bg-emerald-600 hover:bg-emerald-500 text-white font-medium px-4 py-1.5 rounded shadow transition">
                <i class="fa-solid fa-play text-xs"></i>
                <span>Jalankan (F5)</span>
            </button>
            <button id="btn-clear" class="bg-[#313244] hover:bg-[#45475a] text-gray-300 px-3 py-1.5 rounded text-sm transition">
                <i class="fa-solid fa-trash-can mr-1"></i> Bersihkan Konsol
            </button>
        </div>

        <div class="flex items-center space-x-3 text-sm text-gray-400">
            <span id="lsp-status" class="flex items-center space-x-1 text-emerald-400">
                <i class="fa-solid fa-circle-check text-xs"></i>
                <span class="text-xs">LSP Aktif</span>
            </span>
            <span class="border-l border-gray-700 pl-3 text-xs">UTF-8 | Widya-Lang</span>
        </div>
    </header>

    <!-- Main Workspace -->
    <div class="flex-1 flex overflow-hidden">
        <!-- Sidebar File Explorer -->
        <aside class="w-64 bg-[#11111b] border-r border-[#313244] flex flex-col">
            <div class="p-3 border-b border-[#313244] font-semibold text-xs tracking-wider uppercase text-gray-400 flex justify-between items-center">
                <span><i class="fa-solid fa-folder-open mr-2 text-indigo-400"></i> Penjelajah Proyek</span>
                <i class="fa-solid fa-arrows-rotate cursor-pointer hover:text-white" onclick="loadContohList()"></i>
            </div>
            <div id="file-list" class="flex-1 overflow-y-auto p-2 space-y-1 text-sm">
                <div class="text-xs text-gray-500 p-2 italic">Memuat berkas contoh...</div>
            </div>
        </aside>

        <!-- Editor & Terminal Split Panel -->
        <main class="flex-1 flex flex-col overflow-hidden">
            <!-- Tabs Bar -->
            <div class="bg-[#181825] border-b border-[#313244] flex items-center px-2 space-x-1 h-9">
                <div id="current-tab-name" class="tab-active px-3 py-1.5 text-xs rounded-t flex items-center space-x-2">
                    <i class="fa-solid fa-code text-indigo-400"></i>
                    <span>13_fitur_rust.wya</span>
                </div>
            </div>

            <!-- Monaco Code Editor -->
            <div class="flex-1 relative">
                <div id="monaco-container" class="w-full h-full"></div>
            </div>

            <!-- Bottom Console / Terminal Output -->
            <div class="h-56 bg-[#11111b] border-t border-[#313244] flex flex-col">
                <div class="px-4 py-2 bg-[#181825] border-b border-[#313244] text-xs font-semibold text-gray-400 flex items-center justify-between">
                    <div class="flex items-center space-x-3">
                        <span class="text-white"><i class="fa-solid fa-terminal text-emerald-400 mr-1.5"></i> Konsol Keluaran (Output Terminal)</span>
                    </div>
                    <span id="execution-time" class="text-xs text-gray-500">Siap dieksekusi</span>
                </div>
                <div id="console-output" class="flex-1 p-3 font-mono text-xs overflow-y-auto text-emerald-400 space-y-1 whitespace-pre-wrap">
Selamat Datang di Widya Studio!
Tekan tombol 'Jalankan (F5)' di atas untuk mengeksekusi kode sumber Widya Anda.
                </div>
            </div>
        </main>
    </div>

    <!-- Load Monaco Editor -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/loader.min.js"></script>
    <script>
        let editor = null;
        let activeFileName = "13_fitur_rust.wya";

        require.config({ paths: { 'vs': 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs' }});
        require(['vs/editor/editor.main'], function() {
            // Register Widya Language Grammar
            monaco.languages.register({ id: 'widya' });

            monaco.languages.setMonarchTokensProvider('widya', {
                keywords: [
                    'misal', 'tetap', 'fungsi', 'fn', 'struktur', 'struct', 'sifat', 'trait',
                    'terapkan', 'impl', 'enum', 'varian', 'impor', 'muat', 'sebagai',
                    'jika', 'kalau', 'lainnya', 'selama', 'untuk', 'dalam', 'ulang', 'loop',
                    'kembalikan', 'berhenti', 'lanjut', 'coba', 'tangkap', 'lempar', 'cocokkan', 'match',
                    'dan', 'atau', 'bukan', 'ini', 'diri', 'benar', 'salah', 'nihil'
                ],
                builtins: [
                    'cetak', 'tulis', 'baca', 'tipe', 'ke_angka', 'ke_teks', 'ke_boolean',
                    'Ok', 'Err', 'Ada', 'Kosong', 'buka', 'buka_atau', 'apakah_ok', 'apakah_err', 'pastikan',
                    'petakan', 'saring', 'lipat', 'untuk_setiap', 'temukan', 'gabungkan', 'ambil', 'lewati',
                    'apakah_ada', 'semua', 'buat_utas', 'gabung_utas', 'saluran', 'saluran_kirim', 'saluran_terima',
                    'panjang', 'tambah', 'hapus', 'sisip', 'gabung', 'pisah', 'huruf_besar', 'huruf_kecil', 'potong',
                    'kunci', 'nilai', 'ada_kunci', 'urutkan', 'balik', 'akar', 'pangkat', 'sin', 'cos', 'tan',
                    'Aplikasi', 'Halaman', 'Kolom', 'Baris', 'TeksWidget', 'Tombol', 'BidangTeks', 'Kartu', 'render_html'
                ],
                tokenizer: {
                    root: [
                        [/[a-zA-Z_][a-zA-Z0-9_]*/, {
                            cases: {
                                '@keywords': 'keyword',
                                '@builtins': 'type.identifier',
                                '@default': 'identifier'
                            }
                        }],
                        [/\/\/.*$/, 'comment'],
                        [/\/\*/, 'comment', '@comment'],
                        [/"([^"\\]|\\.)*"/, 'string'],
                        [/\d+(\.\d+)?/, 'number'],
                        [/[{}()\[\]]/, '@brackets'],
                        [/(=>|::|\?|\+|\-|\*|\/|==|!=|<=|>=|<|>|=)/, 'operator']
                    ],
                    comment: [
                        [/[^\/*]+/, 'comment'],
                        [/\*\//, 'comment', '@pop'],
                        [/[\/*]/, 'comment']
                    ]
                }
            });

            // Define Monokai/Catppuccin Dark Theme
            monaco.editor.defineTheme('widya-dark', {
                base: 'vs-dark',
                inherit: true,
                rules: [
                    { token: 'keyword', foreground: 'cba6f7', fontStyle: 'bold' },
                    { token: 'type.identifier', foreground: '89b4fa' },
                    { token: 'identifier', foreground: 'cdd6f4' },
                    { token: 'string', foreground: 'a6e3a1' },
                    { token: 'number', foreground: 'fab387' },
                    { token: 'comment', foreground: '6c7086', fontStyle: 'italic' },
                    { token: 'operator', foreground: 'f38ba8' }
                ],
                colors: {
                    'editor.background': '#181825',
                    'editor.foreground': '#cdd6f4',
                    'editorLineNumber.foreground': '#585b70',
                    'editorCursor.foreground': '#f5e0dc',
                    'editor.selectionBackground': '#45475a'
                }
            });

            // Init Editor Instance
            editor = monaco.editor.create(document.getElementById('monaco-container'), {
                value: `// ==========================================================\n// DEMO FITUR RUST DI WIDYA-LANG DALAM WIDYA STUDIO IDE\n// ==========================================================\n\nenum StatusPaket {\n    Diproses,\n    Dikirim(kurir, resi),\n    Selesai\n}\n\nmisal paket = StatusPaket::Dikirim("JNE Express", "WIDYA-12345");\n\n// Pencocokan Pola (Pattern Matching)\nmisal info = cocokkan paket {\n    StatusPaket::Diproses => "Sedang diproses di gudang.",\n    StatusPaket::Dikirim(kurir, resi) => "Dikirim via " + kurir + " [" + resi + "]",\n    StatusPaket::Selesai => "Telah diterima."\n};\n\ncetak("≡ƒôª Status Pengiriman:", info);\n\n// Functional Iterator & Destructuring\nmisal [a, b, c] = [10, 20, 30];\nmisal genap = saring([1, 2, 3, 4, 5, 6], fungsi(n) { kembalikan n % 2 == 0; });\ncetak("Γ£¿ Genap:", genap);\ncetak("≡ƒÄ» Total Dekonstruksi:", a + b + c);\n`,
                language: 'widya',
                theme: 'widya-dark',
                automaticLayout: true,
                fontSize: 14,
                minimap: { enabled: true },
                tabSize: 4
            });

            // Keyboard Shortcut F5 to Run
            window.addEventListener('keydown', function(e) {
                if (e.key === 'F5') {
                    e.preventDefault();
                    runCode();
                }
            });

            loadContohList();
        });

        async function loadContohList() {
            try {
                const res = await fetch('/api/contoh');
                const files = await res.json();
                const container = document.getElementById('file-list');
                container.innerHTML = '';
                files.forEach(f => {
                    const item = document.createElement('div');
                    item.className = 'p-2 rounded hover:bg-[#313244] cursor-pointer flex items-center space-x-2 text-xs transition';
                    item.innerHTML = `<i class="fa-solid fa-file-code text-indigo-400"></i><span class="truncate">${f.name}</span>`;
                    item.onclick = () => {
                        activeFileName = f.name;
                        document.getElementById('current-tab-name').querySelector('span').innerText = f.name;
                        editor.setValue(f.content);
                    };
                    container.appendChild(item);
                });
            } catch (e) {
                console.error("Gagal memuat berkas contoh", e);
            }
        }

        async function runCode() {
            if (!editor) return;
            const source = editor.getValue();
            const outputEl = document.getElementById('console-output');
            const timeEl = document.getElementById('execution-time');

            outputEl.innerHTML = "ΓÅ│ Menjalankan kode sumber Widya...";
            outputEl.className = "flex-1 p-3 font-mono text-xs overflow-y-auto text-gray-300 space-y-1 whitespace-pre-wrap";

            const start = performance.now();
            try {
                const res = await fetch('/api/run', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ source })
                });
                const data = await res.json();
                const duration = (performance.now() - start).toFixed(2);
                timeEl.innerText = `Selesai dalam ${duration} ms`;

                if (data.sukses) {
                    outputEl.innerHTML = `Γ£à [Eksekusi Berhasil]\nNilai Kembalian: ${data.hasil || 'nihil'}`;
                    outputEl.className = "flex-1 p-3 font-mono text-xs overflow-y-auto text-emerald-400 space-y-1 whitespace-pre-wrap";
                } else {
                    outputEl.innerHTML = `Γ¥î [Galat Runtime / Sintaks]:\n${data.galat}`;
                    outputEl.className = "flex-1 p-3 font-mono text-xs overflow-y-auto text-red-400 space-y-1 whitespace-pre-wrap";
                }
            } catch (err) {
                outputEl.innerHTML = `Γ¥î Terjadi kesalahan komunikasi dengan engine Widya: ${err}`;
                outputEl.className = "flex-1 p-3 font-mono text-xs overflow-y-auto text-red-400 space-y-1 whitespace-pre-wrap";
            }
        }

        document.getElementById('btn-run').onclick = runCode;
        document.getElementById('btn-clear').onclick = () => {
            document.getElementById('console-output').innerHTML = "Konsol dibersihkan.";
        };
    </script>
</body>
</html>
"#
    .to_string()
}
