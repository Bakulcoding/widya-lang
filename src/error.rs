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
            let baris_teks = lines[baris - 1];
            out.push_str(&format!("\n   {:4} | {}\n", baris, baris_teks));
            let padding = " ".repeat(kolom.saturating_sub(1));
            out.push_str(&format!("        | {}^\n", padding));
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
