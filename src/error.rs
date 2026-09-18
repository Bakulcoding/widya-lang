use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Galat {
    Sintaks {
        pesan: String,
        baris: usize,
        kolom: usize,
    },
    Runtime {
        pesan: String,
        baris: usize,
        kolom: usize,
    },
    Kembalikan(crate::value::Value),
    Berhenti,
    Lanjut,
}

impl Galat {
    pub fn sintaks(pesan: impl Into<String>, span: &Span) -> Self {
        Galat::Sintaks {
            pesan: pesan.into(),
            baris: span.line,
            kolom: span.column,
        }
    }

    pub fn runtime(pesan: impl Into<String>, span: &Span) -> Self {
        Galat::Runtime {
            pesan: pesan.into(),
            baris: span.line,
            kolom: span.column,
        }
    }

    pub fn runtime_polos(pesan: impl Into<String>) -> Self {
        Galat::Runtime {
            pesan: pesan.into(),
            baris: 1,
            kolom: 1,
        }
    }

    pub fn format_dengan_sumber(&self, sumber: &str) -> String {
        match self {
            Galat::Sintaks { pesan, baris, kolom } => {
                Self::format_pesan("Galat Sintaks", pesan, *baris, *kolom, sumber)
            }
            Galat::Runtime { pesan, baris, kolom } => {
                Self::format_pesan("Galat Runtime", pesan, *baris, *kolom, sumber)
            }
            _ => format!("{:?}", self),
        }
    }

    fn format_pesan(
        tipe_galat: &str,
        pesan: &str,
        baris: usize,
        kolom: usize,
        sumber: &str,
    ) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "❌ [{}] pada baris {}, kolom {}:\n   {}\n",
            tipe_galat, baris, kolom, pesan
        ));

        let lines: Vec<&str> = sumber.lines().collect();
        if baris > 0 && baris <= lines.len() {
            let start = baris.saturating_sub(2);
            let end = (baris + 1).min(lines.len());
            for i in start..end {
                let curr_line = i + 1;
                let line_str = lines[i];
                if curr_line == baris {
                    out.push_str(&format!("\n → {:4} | {}\n", curr_line, line_str));
                    let padding = " ".repeat(kolom.saturating_sub(1));
                    out.push_str(&format!("        | {}^\n", padding));
                } else {
                    out.push_str(&format!("   {:4} | {}\n", curr_line, line_str));
                }
            }
        }

        // Smart suggestions based on message keywords
        if pesan.contains("Harapkan") || pesan.contains("Expected") {
            if pesan.contains("';'") {
                out.push_str("\n 💡 Saran: Tambahkan tanda titik koma ';' di akhir pernyataan.\n");
            } else if pesan.contains("')'") {
                out.push_str("\n 💡 Saran: Tutup kurung buka dengan pasangannya ')'.\n");
            } else if pesan.contains("'}'") {
                out.push_str("\n 💡 Saran: Pastikan blok kode ditutup dengan tanda '}'.\n");
            }
        } else if pesan.contains("belum didefinisikan") || pesan.contains("Undefined") {
            out.push_str("\n 💡 Saran: Deklarasikan variabel terlebih dahulu dengan 'misal nama = nilai;' sebelum digunakan.\n");
        }

        out
    }
}

impl fmt::Display for Galat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Galat::Sintaks { pesan, baris, kolom } => {
                write!(f, "Galat Sintaks [baris {}, kolom {}]: {}", baris, kolom, pesan)
            }
            Galat::Runtime { pesan, baris, kolom } => {
                write!(f, "Galat Runtime [baris {}, kolom {}]: {}", baris, kolom, pesan)
            }
            Galat::Kembalikan(_) => write!(f, "Pernyataan 'kembalikan' di luar fungsi"),
            Galat::Berhenti => write!(f, "Pernyataan 'berhenti' di luar perulangan"),
            Galat::Lanjut => write!(f, "Pernyataan 'lanjut' di luar perulangan"),
        }
    }
}

impl std::error::Error for Galat {}
