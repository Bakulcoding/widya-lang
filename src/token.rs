use crate::error::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords (Kata Kunci Dasar)
    Misal,      // misal / let
    Tetap,      // tetap / const
    Fungsi,     // fungsi / fn
    Kembalikan, // kembalikan / return
    Jika,       // jika / if
    Kalau,      // kalau / elif
    Lainnya,    // lainnya / else
    Selama,     // selama / while
    Untuk,      // untuk / for
    Dalam,      // dalam / in
    Ulang,      // ulang / loop (Rust-style infinite loop)
    Berhenti,   // berhenti / break
    Lanjut,     // lanjut / continue
    Benar,      // benar / true
    Salah,      // salah / false
    Nihil,      // nihil / null
    Dan,        // dan / and / &&
    Atau,       // atau / or / ||
    Bukan,      // bukan / not / !

    // Concurrency & Asynchronous Keywords (Rust-like async/await)
    Asinkron,    // asinkron / async
    TungguHasil, // tunggu_hasil / await

    // Modul & Penanganan Galat & OOP & Rust-like Enums, Traits / Sifat, Pattern Matching
    Impor,      // impor / muat / import
    Sebagai,    // sebagai / as
    Coba,       // coba / try
    Tangkap,    // tangkap / catch
    Lempar,     // lempar / throw
    Struktur,   // struktur / kelas / class
    Sifat,      // sifat / trait / interface
    Terapkan,   // terapkan / implementasi / impl
    Enum,       // enum / varian
    Cocokkan,   // cocokkan / sesuaikan / match
    Baru,       // baru / new
    Ini,        // ini / this / self

    // Interop C-FFI & Bare-metal Embedded Keywords
    Eksternal,   // eksternal / extern / foreign
    Volatil,     // volatil / volatile / direct memory

    // Memory Safety & Borrow Checker Keywords
    Pinjam,      // pinjam / borrow
    Mutabel,     // mutabel / mut

    // Literals
    Identifier(String),
    Number(f64),
    StringLiteral(String),

    // Arithmetic & Assignment Operators
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    Caret,        // ^ (pangkat)
    Ampersand,    // & (referensi/pinjam)
    Hash,         // # (makro atribut misal #[turunkan])
    Assign,       // =
    PlusAssign,   // +=
    MinusAssign,  // -=
    StarAssign,   // *=
    SlashAssign,  // /=

    // Comparison Operators
    EqualEqual,   // ==
    BangEqual,    // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
    FatArrow,     // =>

    // Delimiters & Punctuation
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Colon,        // :
    Dot,          // .
    DotDot,       // .. (rentang/range)
    Question,     // ? (operator propagasi galat/error try operator)
    Semicolon,    // ;

    // Special
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Misal => write!(f, "misal"),
            TokenType::Tetap => write!(f, "tetap"),
            TokenType::Fungsi => write!(f, "fungsi"),
            TokenType::Kembalikan => write!(f, "kembalikan"),
            TokenType::Jika => write!(f, "jika"),
            TokenType::Kalau => write!(f, "kalau"),
            TokenType::Lainnya => write!(f, "lainnya"),
            TokenType::Selama => write!(f, "selama"),
            TokenType::Untuk => write!(f, "untuk"),
            TokenType::Dalam => write!(f, "dalam"),
            TokenType::Ulang => write!(f, "ulang"),
            TokenType::Berhenti => write!(f, "berhenti"),
            TokenType::Lanjut => write!(f, "lanjut"),
            TokenType::Benar => write!(f, "benar"),
            TokenType::Salah => write!(f, "salah"),
            TokenType::Nihil => write!(f, "nihil"),
            TokenType::Dan => write!(f, "dan"),
            TokenType::Atau => write!(f, "atau"),
            TokenType::Bukan => write!(f, "bukan"),
            TokenType::Asinkron => write!(f, "asinkron"),
            TokenType::TungguHasil => write!(f, "tunggu_hasil"),
            TokenType::Impor => write!(f, "impor"),
            TokenType::Sebagai => write!(f, "sebagai"),
            TokenType::Coba => write!(f, "coba"),
            TokenType::Tangkap => write!(f, "tangkap"),
            TokenType::Lempar => write!(f, "lempar"),
            TokenType::Struktur => write!(f, "struktur"),
            TokenType::Sifat => write!(f, "sifat"),
            TokenType::Terapkan => write!(f, "terapkan"),
            TokenType::Enum => write!(f, "enum"),
            TokenType::Cocokkan => write!(f, "cocokkan"),
            TokenType::Baru => write!(f, "baru"),
            TokenType::Ini => write!(f, "ini"),
            TokenType::Eksternal => write!(f, "eksternal"),
            TokenType::Volatil => write!(f, "volatil"),
            TokenType::Pinjam => write!(f, "pinjam"),
            TokenType::Mutabel => write!(f, "mutabel"),
            TokenType::Ampersand => write!(f, "&"),
            TokenType::Hash => write!(f, "#"),
            TokenType::Identifier(s) => write!(f, "pengenal('{}')", s),
            TokenType::Number(n) => write!(f, "angka({})", n),
            TokenType::StringLiteral(s) => write!(f, "teks(\"{}\")", s),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Star => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Percent => write!(f, "%"),
            TokenType::Caret => write!(f, "^"),
            TokenType::Assign => write!(f, "="),
            TokenType::PlusAssign => write!(f, "+="),
            TokenType::MinusAssign => write!(f, "-="),
            TokenType::StarAssign => write!(f, "*="),
            TokenType::SlashAssign => write!(f, "/="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::FatArrow => write!(f, "=>"),
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Comma => write!(f, ","),
            TokenType::Colon => write!(f, ":"),
            TokenType::Dot => write!(f, "."),
            TokenType::DotDot => write!(f, ".."),
            TokenType::Question => write!(f, "?"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Eof => write!(f, "<akhir berkas>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: impl Into<String>, span: Span) -> Self {
        Self {
            token_type,
            lexeme: lexeme.into(),
            span,
        }
    }
}
