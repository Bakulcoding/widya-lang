use crate::ast::*;
use crate::error::{Galat, Span};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;
use std::path::{Path, PathBuf};

pub struct DocGenerator {
    pub title: String,
}

pub struct FunctionDoc {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: Option<String>,
    pub doc_comment: String,
}

pub struct StructDoc {
    pub name: String,
    pub fields: Vec<String>,
    pub methods: Vec<String>,
    pub doc_comment: String,
}

pub struct ModuleDoc {
    pub file_name: String,
    pub functions: Vec<FunctionDoc>,
    pub structs: Vec<StructDoc>,
    pub general_doc: String,
}

impl DocGenerator {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }

    pub fn generate_from_source(&self, source: &str, file_name: &str) -> Result<ModuleDoc, Galat> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;

        // Extract comments from source
        let lines: Vec<&str> = source.lines().collect();
        let mut doc_comments: Vec<String> = Vec::new();
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                doc_comments.push(trimmed.trim_start_matches('/').trim().to_string());
            }
        }

        let mut functions = Vec::new();
        let mut structs = Vec::new();

        for stmt in &program.statements {
            match stmt {
                Stmt::FunctionDecl {
                    name,
                    params,
                    ..
                } => {
                    functions.push(FunctionDoc {
                        name: name.clone(),
                        params: params.clone(),
                        return_type: None,
                        doc_comment: format!("Fungsi publik `{}` pada modul {}", name, file_name),
                    });
                }
                Stmt::StructDecl { name, fields, .. } => {
                    structs.push(StructDoc {
                        name: name.clone(),
                        fields: fields.clone(),
                        methods: Vec::new(),
                        doc_comment: format!("Struktur data `{}` pada modul {}", name, file_name),
                    });
                }
                _ => {}
            }
        }

        Ok(ModuleDoc {
            file_name: file_name.to_string(),
            functions,
            structs,
            general_doc: if doc_comments.is_empty() {
                format!("Dokumentasi otomatis modul `{}`", file_name)
            } else {
                doc_comments.join("\n")
            },
        })
    }

    pub fn render_html(&self, doc: &ModuleDoc) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html lang=\"id\">\n<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("  <title>{} - Dokumentasi API</title>\n", doc.file_name));
        html.push_str("  <style>\n");
        html.push_str("    :root { --bg: #0d1117; --card-bg: #161b22; --border: #30363d; --text: #c9d1d9; --accent: #58a6ff; --badge: #238636; }\n");
        html.push_str("    body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: var(--bg); color: var(--text); margin: 0; padding: 2rem; }\n");
        html.push_str("    .container { max-width: 900px; margin: 0 auto; }\n");
        html.push_str("    h1 { color: #58a6ff; border-bottom: 1px solid var(--border); padding-bottom: 0.5rem; }\n");
        html.push_str("    h2 { color: #7ee787; margin-top: 2rem; }\n");
        html.push_str("    .card { background: var(--card-bg); border: 1px solid var(--border); border-radius: 8px; padding: 1.25rem; margin-bottom: 1rem; }\n");
        html.push_str("    .fn-sig { font-family: monospace; font-size: 1.1rem; color: #f0883e; background: #0b0e14; padding: 0.5rem 0.75rem; border-radius: 6px; }\n");
        html.push_str("    .badge { background: var(--badge); color: white; padding: 2px 8px; border-radius: 12px; font-size: 0.8rem; font-weight: bold; }\n");
        html.push_str("    .desc { margin-top: 0.75rem; color: #8b949e; line-height: 1.5; }\n");
        html.push_str("  </style>\n</head>\n<body>\n");
        html.push_str("  <div class=\"container\">\n");
        html.push_str(&format!("    <h1>📖 Dokumentasi Widya: {}</h1>\n", doc.file_name));
        html.push_str(&format!("    <p class=\"desc\">{}</p>\n", doc.general_doc));

        if !doc.structs.is_empty() {
            html.push_str("    <h2>🏛️ Struktur Data (Structs)</h2>\n");
            for st in &doc.structs {
                html.push_str("    <div class=\"card\">\n");
                html.push_str(&format!("      <div class=\"fn-sig\">struktur {} {{ {} }}</div>\n", st.name, st.fields.join(", ")));
                html.push_str(&format!("      <p class=\"desc\">{}</p>\n", st.doc_comment));
                html.push_str("    </div>\n");
            }
        }

        if !doc.functions.is_empty() {
            html.push_str("    <h2>⚡ Fungsi & Prosedur</h2>\n");
            for f in &doc.functions {
                html.push_str("    <div class=\"card\">\n");
                let ret = match &f.return_type {
                    Some(r) => format!(" -> {}", r),
                    None => "".to_string(),
                };
                html.push_str(&format!("      <div class=\"fn-sig\">fungsi {}({}){}</div>\n", f.name, f.params.join(", "), ret));
                html.push_str(&format!("      <p class=\"desc\">{}</p>\n", f.doc_comment));
                html.push_str("    </div>\n");
            }
        }

        html.push_str("  </div>\n</body>\n</html>\n");
        html
    }

    pub fn generate_docs_for_file(&self, source_path: &Path, output_dir: Option<&Path>) -> Result<PathBuf, Galat> {
        let content = fs::read_to_string(source_path)
            .map_err(|e| Galat::runtime(format!("Gagal membaca berkas: {}", e), &Span::new(1, 1)))?;
        let file_name = source_path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let doc = self.generate_from_source(&content, &file_name)?;
        let html = self.render_html(&doc);

        let out_dir = match output_dir {
            Some(d) => d.to_path_buf(),
            None => PathBuf::from("dokumen_widya"),
        };
        fs::create_dir_all(&out_dir)
            .map_err(|e| Galat::runtime(format!("Gagal membuat direktori dokumen: {}", e), &Span::new(1, 1)))?;

        let out_file = out_dir.join(format!("{}.html", source_path.file_stem().unwrap_or_default().to_string_lossy()));
        fs::write(&out_file, html)
            .map_err(|e| Galat::runtime(format!("Gagal menulis berkas dokumen HTML: {}", e), &Span::new(1, 1)))?;

        Ok(out_file)
    }
}

pub fn hasilkan_dokumentasi(path: &PathBuf, output: Option<PathBuf>) {
    println!("📚 Menghasilkan Dokumentasi API Otomatis untuk {}...", path.display());
    let doc_gen = DocGenerator::new("Dokumentasi Widya");
    match doc_gen.generate_docs_for_file(path, output.as_deref()) {
        Ok(out_path) => {
            println!("✨ Dokumentasi HTML berhasil dibuat di: {}", out_path.display());
        }
        Err(e) => {
            eprintln!("❌ Gagal menghasilkan dokumentasi: {}", e);
        }
    }
}
