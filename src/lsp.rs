use crate::error::Galat;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

/// Widya Language Server Protocol (LSP) Engine
pub struct WidyaLsp {
    interpreter: Interpreter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub message: String,
    pub severity: DiagnosticSeverity,
    pub code: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Keyword = 14,
    Snippet = 15,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hover {
    pub contents: String,
    pub range: Option<Range>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub range: Range,
    pub signature: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Function = 12,
    Variable = 13,
    Constant = 14,
}

impl WidyaLsp {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }
    
    /// Parse and generate full real-time syntax & semantic diagnostics
    pub fn get_diagnostics(source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        let mut lexer = Lexer::new(source);
        let tokens = match lexer.scan_tokens() {
            Ok(toks) => toks,
            Err(e) => {
                let (line, col, msg) = Self::extract_error_info(&e);
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line, character: col },
                        end: Position { line, character: col + 1 },
                    },
                    message: format!("Sintaks Lexer: {}", msg),
                    severity: DiagnosticSeverity::Error,
                    code: Some("lexer-error".to_string()),
                    source: Some("widya".to_string()),
                });
                return diagnostics;
            }
        };

        let mut parser = Parser::new(tokens);
        if let Err(e) = parser.parse() {
            let (line, col, msg) = Self::extract_error_info(&e);
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line, character: col },
                    end: Position { line, character: col + 1 },
                },
                message: format!("Sintaks Parser: {}", msg),
                severity: DiagnosticSeverity::Error,
                code: Some("parser-error".to_string()),
                source: Some("widya".to_string()),
            });
        }

        diagnostics
    }
    
    /// Provide intelligent autocompletions for keywords, stdlib functions, and active variables
    pub fn get_completions(&self, _position: Position, _source: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        
        let keywords = vec![
            ("misal", "Deklarasi variabel lokal yang dapat diubah", "misal ${1:nama} = ${2:nilai};"),
            ("tetap", "Deklarasi konstanta yang tidak dapat diubah", "tetap ${1:NAMA} = ${2:nilai};"),
            ("fungsi", "Definisi fungsi baru", "fungsi ${1:nama}(${2:params}) {\n    ${3:kembalikan nilai;}\n}"),
            ("jika", "Percabangan kondisi jika", "jika ${1:kondisi} {\n    ${2}\n}"),
            ("lainnya", "Blok alternatif else", "lainnya {\n    ${1}\n}"),
            ("kalau", "Percabangan kondisi else if", "kalau ${1:kondisi} {\n    ${2}\n}"),
            ("untuk", "Perulangan for in", "untuk ${1:item} dalam ${2:daftar} {\n    ${3}\n}"),
            ("selama", "Perulangan while", "selama ${1:kondisi} {\n    ${2}\n}"),
            ("ulang", "Perulangan tak terbatas", "ulang {\n    ${1}\n}"),
            ("cocokkan", "Pattern matching ekspresi", "cocokkan ${1:target} {\n    ${2:pola} => ${3:aksi},\n}"),
            ("kembalikan", "Mengembalikan nilai dari fungsi", "kembalikan ${1:nilai};"),
            ("struktur", "Mendefinisikan tipe data struktur", "struktur ${1:Nama} {\n    ${2:field},\n}"),
            ("sifat", "Mendefinisikan interface/trait", "sifat ${1:NamaSifat} {\n    fungsi ${2:metode}();\n}"),
            ("terapkan", "Mengimplementasikan trait untuk struktur", "terapkan ${1:NamaSifat} untuk ${2:NamaStruktur} {\n    ${3}\n}"),
            ("coba", "Blok penanganan eksepsi", "coba {\n    ${1}\n} tangkap (${2:err}) {\n    ${3}\n}"),
            ("lempar", "Melempar galat/eksepsi", "lempar ${1:pesan};"),
            ("asinkron", "Fungsi asinkron non-blocking", "asinkron fungsi ${1:nama}() {\n    ${2}\n}"),
            ("tunggu_hasil", "Menunggu hasil ekspresi asinkron", "tunggu_hasil ${1:ekspresi}"),
            ("benar", "Nilai boolean benar", "benar"),
            ("salah", "Nilai boolean salah", "salah"),
            ("nihil", "Nilai kosong/null", "nihil"),
        ];
        
        for (keyword, description, snippet) in keywords {
            items.push(CompletionItem {
                label: keyword.to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some(description.to_string()),
                documentation: Some(format!("Kata kunci Widya-Lang: `{}`", keyword)),
                insert_text: Some(snippet.to_string()),
            });
        }
        
        let builtins = vec![
            ("cetak", "Mencetak nilai ke konsol standar", "cetak(${1:nilai});"),
            ("tulis", "Mencetak nilai tanpa baris baru", "tulis(${1:nilai});"),
            ("ke_teks", "Mengonversi nilai apapun ke String", "ke_teks(${1:nilai})"),
            ("ke_angka", "Mengonversi nilai ke Angka", "ke_angka(${1:nilai})"),
            ("panjang", "Menghitung panjang string, array, atau map", "panjang(${1:koleksi})"),
            ("pastikan_sama", "Test assertion: memastikan dua nilai identik", "pastikan_sama(${1:ditemukan}, ${2:diharapkan});"),
            ("pastikan_benar", "Test assertion: memastikan kondisi bernilai benar", "pastikan_benar(${1:kondisi});"),
            ("pastikan_salah", "Test assertion: memastikan kondisi bernilai salah", "pastikan_salah(${1:kondisi});"),
            ("Aplikasi", "WidyaUI: Komponen root aplikasi mobile/web", "Aplikasi({\"judul\": \"${1:App}\", \"badan\": ${2:halaman}})"),
            ("Halaman", "WidyaUI: Halaman view kontainer", "Halaman({\"badan\": ${1:kolom}})"),
            ("Kolom", "WidyaUI: Tata letak vertikal", "Kolom([${1}])"),
            ("Baris", "WidyaUI: Tata letak horizontal", "Baris([${1}])"),
            ("Tombol", "WidyaUI: Tombol aksi interaktif", "Tombol(\"${1:Label}\", fungsi() {\n    ${2}\n})"),
            ("TeksWidget", "WidyaUI: Widget label teks", "TeksWidget(\"${1:Teks}\")"),
        ];
        
        for (func, description, snippet) in builtins {
            items.push(CompletionItem {
                label: func.to_string(),
                kind: CompletionItemKind::Function,
                detail: Some(description.to_string()),
                documentation: Some(format!("Fungsi Standar Library Widya: `{}`", func)),
                insert_text: Some(snippet.to_string()),
            });
        }
        
        let vars = self.interpreter.environment.borrow().get_all_local();
        for (name, value) in vars {
            items.push(CompletionItem {
                label: name.clone(),
                kind: CompletionItemKind::Variable,
                detail: Some(format!(": {}", value.type_name())),
                documentation: None,
                insert_text: Some(name),
            });
        }
        
        items
    }
    
    /// Provide rich Markdown hover documentation for keywords, builtins, and variables
    pub fn get_hover(&self, position: Position, source: &str) -> Option<Hover> {
        let word = self.get_word_at_position(position, source)?;
        
        let keywords: HashMap<&str, (&str, &str)> = [
            ("misal", ("Deklarasi Variabel", "misal nama = \"Widya\";")),
            ("tetap", ("Konstanta Tidak Dapat Diubah", "tetap PI = 3.14159;")),
            ("fungsi", ("Definisi Fungsi", "fungsi hitung(x, y) {\n    kembalikan x + y;\n}")),
            ("jika", ("Percabangan Kondisional", "jika skor >= 80 {\n    cetak(\"Lulus\");\n}")),
            ("untuk", ("Perulangan Koleksi", "untuk item dalam daftar {\n    cetak(item);\n}")),
            ("cocokkan", ("Pattern Matching", "cocokkan nilai {\n    1 => \"Satu\",\n    _ => \"Lainnya\"\n}")),
            ("cetak", ("Pencetakan Konsol", "cetak(\"Halo Dunia!\");")),
        ].into_iter().collect();
        
        if let Some((description, example)) = keywords.get(word.as_str()) {
            return Some(Hover {
                contents: format!("### **{}**\n{}\n```widya\n{}\n```", word, description, example),
                range: None,
            });
        }
        
        let vars = self.interpreter.environment.borrow().get_all_local();
        if let Some(value) = vars.get(&word) {
            return Some(Hover {
                contents: format!("### Variabel `{}`: `{}`\nNilai: `{}`", word, value.type_name(), value.to_debug_repr()),
                range: None,
            });
        }
        
        None
    }
    
    /// Extract word under cursor position
    fn get_word_at_position(&self, position: Position, source: &str) -> Option<String> {
        let lines: Vec<&str> = source.lines().collect();
        if position.line == 0 || position.line > lines.len() {
            return None;
        }
        
        let line = lines[position.line - 1];
        let chars: Vec<char> = line.chars().collect();
        
        if position.character == 0 || position.character > chars.len() {
            return None;
        }
        
        let mut start = position.character.saturating_sub(1);
        let mut end = position.character;
        
        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
            start -= 1;
        }
        
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }
        
        let word: String = chars[start..end].iter().collect();
        if word.is_empty() {
            None
        } else {
            Some(word)
        }
    }
    
    /// Go to Definition provider
    pub fn find_definition(&self, position: Position, source: &str) -> Option<Location> {
        let word = self.get_word_at_position(position, source)?;
        
        let lines: Vec<&str> = source.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with(&format!("misal {}", word))
                || trimmed.starts_with(&format!("tetap {}", word))
                || trimmed.starts_with(&format!("fungsi {}", word))
                || trimmed.starts_with(&format!("struktur {}", word))
                || trimmed.starts_with(&format!("enum {}", word))
            {
                let col = line.find(&word).unwrap_or(0);
                return Some(Location {
                    uri: "current_file.wya".to_string(),
                    range: Range {
                        start: Position { line: idx + 1, character: col + 1 },
                        end: Position { line: idx + 1, character: col + 1 + word.len() },
                    },
                });
            }
        }
        
        None
    }
    
    /// Find all Symbol References in document
    pub fn find_references(&self, position: Position, source: &str) -> Vec<Location> {
        let word = match self.get_word_at_position(position, source) {
            Some(w) => w,
            None => return Vec::new(),
        };
        
        let mut locations = Vec::new();
        let lines: Vec<&str> = source.lines().collect();
        
        for (idx, line) in lines.iter().enumerate() {
            let mut search_start = 0;
            while let Some(col) = line[search_start..].find(&word) {
                let actual_col = search_start + col;
                
                let before_ok = actual_col == 0 || 
                    !line.chars().nth(actual_col - 1).map(|c| c.is_alphanumeric() || c == '_').unwrap_or(false);
                let after_ok = actual_col + word.len() >= line.len() || 
                    !line.chars().nth(actual_col + word.len()).map(|c| c.is_alphanumeric() || c == '_').unwrap_or(false);
                
                if before_ok && after_ok {
                    locations.push(Location {
                        uri: "current_file.wya".to_string(),
                        range: Range {
                            start: Position { line: idx + 1, character: actual_col + 1 },
                            end: Position { line: idx + 1, character: actual_col + 1 + word.len() },
                        },
                    });
                }
                
                search_start = actual_col + word.len();
            }
        }
        
        locations
    }
    
    /// Extract Document Outline & Code Symbols
    pub fn get_document_symbols(&self, source: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = source.lines().collect();
        
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("fungsi ") {
                if let Some(name) = trimmed.strip_prefix("fungsi ") {
                    let name = name.split('(').next().unwrap_or("").trim();
                    if !name.is_empty() {
                        symbols.push(Symbol {
                            name: name.to_string(),
                            kind: SymbolKind::Function,
                            range: Range {
                                start: Position { line: idx + 1, character: 1 },
                                end: Position { line: idx + 1, character: line.len() },
                            },
                            signature: Some(format!("fungsi {}(...)", name)),
                            documentation: None,
                        });
                    }
                }
            } else if trimmed.starts_with("struktur ") {
                if let Some(name) = trimmed.strip_prefix("struktur ") {
                    let name = name.split('{').next().unwrap_or("").trim();
                    if !name.is_empty() {
                        symbols.push(Symbol {
                            name: name.to_string(),
                            kind: SymbolKind::Class,
                            range: Range {
                                start: Position { line: idx + 1, character: 1 },
                                end: Position { line: idx + 1, character: line.len() },
                            },
                            signature: Some(format!("struktur {}", name)),
                            documentation: None,
                        });
                    }
                }
            } else if trimmed.starts_with("misal ") || trimmed.starts_with("tetap ") {
                let rest = if trimmed.starts_with("misal ") {
                    trimmed.strip_prefix("misal ").unwrap()
                } else {
                    trimmed.strip_prefix("tetap ").unwrap()
                };
                let name = rest.split('=').next().unwrap_or("").trim();
                if !name.is_empty() {
                    symbols.push(Symbol {
                        name: name.to_string(),
                        kind: SymbolKind::Variable,
                        range: Range {
                            start: Position { line: idx + 1, character: 1 },
                            end: Position { line: idx + 1, character: line.len() },
                        },
                        signature: None,
                        documentation: None,
                    });
                }
            }
        }
        
        symbols
    }
    
    fn extract_error_info(error: &Galat) -> (usize, usize, String) {
        match error {
            Galat::Sintaks { baris, kolom, pesan } |
            Galat::Runtime { baris, kolom, pesan } => (*baris, *kolom, pesan.clone()),
            _ => (1, 1, format!("{}", error)),
        }
    }
    
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "name": "widya-language-server",
            "version": "0.1.0",
            "capabilities": {
                "hoverProvider": true,
                "completionProvider": true,
                "definitionProvider": true,
                "referencesProvider": true,
                "documentSymbolProvider": true
            }
        })
    }
}

impl Default for WidyaLsp {
    fn default() -> Self {
        Self::new()
    }
}
