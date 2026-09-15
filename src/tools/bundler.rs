use crate::compiler::NativeCompiler;
use crate::error::{Galat, Span};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;
use std::path::{Path, PathBuf};

pub struct StandaloneBundler {
    pub target_name: String,
}

impl StandaloneBundler {
    pub fn new(target_name: &str) -> Self {
        Self {
            target_name: target_name.to_string(),
        }
    }

    pub fn bundle_executable(&self, source_path: &Path, output_path: Option<&Path>) -> Result<PathBuf, Galat> {
        let source = fs::read_to_string(source_path)
            .map_err(|e| Galat::runtime(format!("Gagal membaca berkas sumber: {}", e), &Span::new(1, 1)))?;

        let mut lexer = Lexer::new(&source);
        let tokens = lexer.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;

        // Generate Native Rust code for self-contained embedding
        let mut compiler = NativeCompiler::new();
        let native_code = compiler.compile_to_rust(&program)?;

        let out_rs = match output_path {
            Some(p) => p.to_path_buf(),
            None => {
                let stem = source_path.file_stem().unwrap_or_default().to_string_lossy();
                PathBuf::from(format!("{}_bundle.rs", stem))
            }
        };

        let mut wrapped_bundle = String::new();
        wrapped_bundle.push_str("/* === Widya-Lang Standalone Native / Mobile Bundle === */\n");
        wrapped_bundle.push_str("/* Generated automatically by 'widya kemas' */\n\n");
        wrapped_bundle.push_str(&native_code);

        fs::write(&out_rs, wrapped_bundle)
            .map_err(|e| Galat::runtime(format!("Gagal menulis bundle: {}", e), &Span::new(1, 1)))?;

        Ok(out_rs)
    }

    pub fn bundle_mobile_webview_app(&self, source_path: &Path, output_dir: &Path) -> Result<PathBuf, Galat> {
        fs::create_dir_all(output_dir)
            .map_err(|e| Galat::runtime(format!("Gagal membuat folder output: {}", e), &Span::new(1, 1)))?;

        let source = fs::read_to_string(source_path)
            .map_err(|e| Galat::runtime(format!("Gagal membaca berkas sumber: {}", e), &Span::new(1, 1)))?;

        let html_content = format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
  <title>{} (Widya Mobile App)</title>
  <style>
    body {{ margin: 0; padding: 0; background: #121212; color: #fff; font-family: sans-serif; display: flex; flex-direction: column; height: 100vh; }}
    .app-bar {{ background: #1f1f1f; padding: 1rem; font-weight: bold; border-bottom: 1px solid #333; text-align: center; }}
    .content {{ flex: 1; padding: 1rem; overflow-y: auto; }}
    .card {{ background: #242424; border-radius: 12px; padding: 1rem; margin-bottom: 1rem; border: 1px solid #333; }}
    button {{ width: 100%; background: #6200ee; color: white; border: none; padding: 12px; border-radius: 8px; font-weight: bold; font-size: 1rem; cursor: pointer; }}
  </style>
</head>
<body>
  <div class="app-bar">📱 Widya Mobile WebApp</div>
  <div class="content">
    <div class="card">
      <h3>🚀 Aplikasi Siap Distribusi (Android / iOS / Desktop)</h3>
      <p>Aplikasi ini dikemas secara otomatis dari kode sumber Widya-Lang.</p>
    </div>
    <div class="card">
      <h4>Kode Sumber Termasuk:</h4>
      <pre style="color: #03dac6; overflow-x: auto;">{}</pre>
    </div>
    <button onclick="kirimEventNative()">Kirim Event ke Native Bridge</button>
  </div>
  <script>
    function kirimEventNative() {{
      alert("Pesan dikirim melalui Widya Mobile Bridge!");
    }}
  </script>
</body>
</html>"#, self.target_name, source);

        let out_index = output_dir.join("index.html");
        fs::write(&out_index, html_content)
            .map_err(|e| Galat::runtime(format!("Gagal menulis index.html: {}", e), &Span::new(1, 1)))?;

        Ok(out_index)
    }
}

pub fn kemas_aplikasi(path: &std::path::PathBuf, output: Option<std::path::PathBuf>, mobile: bool) {
    let nama = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let bundler = StandaloneBundler::new(&nama);

    if mobile {
        println!("📱 Mengemas aplikasi Widya untuk Mobile/Webview: {}", path.display());
        let target_dir = output.unwrap_or_else(|| std::path::PathBuf::from(format!("{}_mobile_app", nama)));
        match bundler.bundle_mobile_webview_app(path, &target_dir) {
            Ok(out_file) => {
                println!("🎉 Mobile Package berhasil dibuat di: {}", out_file.display());
            }
            Err(e) => {
                eprintln!("❌ Gagal mengemas mobile app: {}", e);
            }
        }
    } else {
        println!("📦 Mengemas aplikasi Widya menjadi Single Standalone Executable Source: {}", path.display());
        match bundler.bundle_executable(path, output.as_deref()) {
            Ok(out_file) => {
                println!("🎉 Standalone Bundle berhasil dibuat di: {}", out_file.display());
            }
            Err(e) => {
                eprintln!("❌ Gagal mengemas standalone: {}", e);
            }
        }
    }
}
