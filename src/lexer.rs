use crate::error::{Galat, Span};
use crate::token::{Token, TokenType};

pub struct Lexer {
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    start_column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            start_column: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Galat> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            self.start_column = self.column;
            if let Some(token) = self.scan_token()? {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(
            TokenType::Eof,
            "",
            Span::new(self.line, self.column),
        ));

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<Token>, Galat> {
        let c = self.advance();

        match c {
            ' ' | '\r' | '\t' => Ok(None),
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(None)
            }
            '(' => Ok(Some(self.make_token(TokenType::LeftParen))),
            ')' => Ok(Some(self.make_token(TokenType::RightParen))),
            '{' => Ok(Some(self.make_token(TokenType::LeftBrace))),
            '}' => Ok(Some(self.make_token(TokenType::RightBrace))),
            '[' => Ok(Some(self.make_token(TokenType::LeftBracket))),
            ']' => Ok(Some(self.make_token(TokenType::RightBracket))),
            ',' => Ok(Some(self.make_token(TokenType::Comma))),
            ':' => Ok(Some(self.make_token(TokenType::Colon))),
            ';' => Ok(Some(self.make_token(TokenType::Semicolon))),
            '?' => Ok(Some(self.make_token(TokenType::Question))),
            '^' => Ok(Some(self.make_token(TokenType::Caret))),
            '%' => Ok(Some(self.make_token(TokenType::Percent))),
            '.' => {
                if self.match_char('.') {
                    Ok(Some(self.make_token(TokenType::DotDot)))
                } else {
                    Ok(Some(self.make_token(TokenType::Dot)))
                }
            }
            '+' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::PlusAssign)))
                } else {
                    Ok(Some(self.make_token(TokenType::Plus)))
                }
            }
            '-' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::MinusAssign)))
                } else {
                    Ok(Some(self.make_token(TokenType::Minus)))
                }
            }
            '*' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::StarAssign)))
                } else {
                    Ok(Some(self.make_token(TokenType::Star)))
                }
            }
            '/' => {
                if self.match_char('/') {
                    // Single line comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(None)
                } else if self.match_char('*') {
                    // Multi line comment
                    self.multi_line_comment()?;
                    Ok(None)
                } else if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::SlashAssign)))
                } else {
                    Ok(Some(self.make_token(TokenType::Slash)))
                }
            }
            '!' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::BangEqual)))
                } else {
                    Ok(Some(self.make_token(TokenType::Bukan)))
                }
            }
            '=' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::EqualEqual)))
                } else if self.match_char('>') {
                    Ok(Some(self.make_token(TokenType::FatArrow)))
                } else {
                    Ok(Some(self.make_token(TokenType::Assign)))
                }
            }
            '<' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::LessEqual)))
                } else {
                    Ok(Some(self.make_token(TokenType::Less)))
                }
            }
            '>' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::GreaterEqual)))
                } else {
                    Ok(Some(self.make_token(TokenType::Greater)))
                }
            }
            '#' => Ok(Some(self.make_token(TokenType::Hash))),
            '&' => {
                if self.match_char('&') {
                    Ok(Some(self.make_token(TokenType::Dan)))
                } else {
                    Ok(Some(self.make_token(TokenType::Ampersand)))
                }
            }
            '|' => {
                if self.match_char('|') {
                    Ok(Some(self.make_token(TokenType::Atau)))
                } else {
                    Err(Galat::sintaks(
                        "Karakter '|' tidak valid; gunakan 'atau' atau '||'",
                        &self.current_span(),
                    ))
                }
            }
            '"' => self.string('"').map(Some),
            '\'' => self.string('\'').map(Some),
            'r' if self.peek() == '"' => {
                self.advance(); // consume '"'
                self.raw_string().map(Some)
            }
            c if c.is_ascii_digit() => self.number().map(Some),
            c if c.is_alphabetic() || c == '_' => self.identifier().map(Some),
            _ => Err(Galat::sintaks(
                format!("Karakter tidak dikenal: '{}'", c),
                &self.current_span(),
            )),
        }
    }

    fn multi_line_comment(&mut self) -> Result<(), Galat> {
        let start_span = self.current_span();
        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance(); // consume '*'
                self.advance(); // consume '/'
                return Ok(());
            }
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.advance();
        }
        Err(Galat::sintaks(
            "Komentar multibaris tidak ditutup (harus diakhiri '*/')",
            &start_span,
        ))
    }

    fn string(&mut self, quote: char) -> Result<Token, Galat> {
        let start_span = self.current_span();
        let mut value = String::new();

        while self.peek() != quote && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            if self.peek() == '\\' {
                self.advance(); // consume '\'
                match self.advance() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    '0' => value.push('\0'),
                    other => {
                        value.push('\\');
                        value.push(other);
                    }
                }
            } else {
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return Err(Galat::sintaks(
                format!("Teks string tidak ditutup (harus diakhiri {})", quote),
                &start_span,
            ));
        }

        self.advance(); // The closing quote
        let lexeme: String = self.chars[self.start..self.current].iter().collect();
        Ok(Token::new(
            TokenType::StringLiteral(value),
            lexeme,
            Span::new(self.line, self.start_column),
        ))
    }

    fn raw_string(&mut self) -> Result<Token, Galat> {
        let start_span = self.current_span();
        let mut value = String::new();

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            value.push(self.advance());
        }

        if self.is_at_end() {
            return Err(Galat::sintaks(
                "Teks raw string (r\"...\") tidak ditutup (harus diakhiri \")",
                &start_span,
            ));
        }

        self.advance(); // The closing quote '"'
        let lexeme: String = self.chars[self.start..self.current].iter().collect();
        Ok(Token::new(
            TokenType::StringLiteral(value),
            lexeme,
            Span::new(self.line, self.start_column),
        ))
    }

    fn number(&mut self) -> Result<Token, Galat> {
        // Hexadecimal support: 0x... or 0X...
        if self.chars[self.start] == '0' && (self.peek() == 'x' || self.peek() == 'X') {
            self.advance(); // consume 'x' or 'X'
            let hex_start = self.current;
            while self.peek().is_ascii_hexdigit() {
                self.advance();
            }
            let hex_str: String = self.chars[hex_start..self.current].iter().collect();
            match u64::from_str_radix(&hex_str, 16) {
                Ok(val) => return Ok(self.make_token(TokenType::Number(val as f64))),
                Err(_) => {
                    let span = Span::new(self.line, self.start_column);
                    return Err(Galat::sintaks(format!("Format angka heksadesimal tidak valid: 0x{}", hex_str), &span));
                }
            }
        }

        // Binary support: 0b... or 0B...
        if self.chars[self.start] == '0' && (self.peek() == 'b' || self.peek() == 'B') {
            self.advance(); // consume 'b' or 'B'
            let bin_start = self.current;
            while self.peek() == '0' || self.peek() == '1' {
                self.advance();
            }
            let bin_str: String = self.chars[bin_start..self.current].iter().collect();
            match u64::from_str_radix(&bin_str, 2) {
                Ok(val) => return Ok(self.make_token(TokenType::Number(val as f64))),
                Err(_) => {
                    let span = Span::new(self.line, self.start_column);
                    return Err(Galat::sintaks(format!("Format angka biner tidak valid: 0b{}", bin_str), &span));
                }
            }
        }

        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // Consume the "."
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // Look for an exponent part (e or E)
        if self.peek() == 'e' || self.peek() == 'E' {
            let next = self.peek_next();
            if next.is_ascii_digit() || next == '+' || next == '-' {
                self.advance(); // consume 'e' or 'E'
                if self.peek() == '+' || self.peek() == '-' {
                    self.advance(); // consume sign
                }
                while self.peek().is_ascii_digit() {
                    self.advance();
                }
            }
        }

        let num_str: String = self.chars[self.start..self.current].iter().collect();
        match num_str.parse::<f64>() {
            Ok(num) => Ok(self.make_token(TokenType::Number(num))),
            Err(_) => Err(Galat::sintaks(
                format!("Angka tidak valid: '{}'", num_str),
                &self.current_span(),
            )),
        }
    }

    fn identifier(&mut self) -> Result<Token, Galat> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.chars[self.start..self.current].iter().collect();
        let token_type = match text.as_str() {
            "misal" | "let" | "var" => TokenType::Misal,
            "tetap" | "konst" | "const" => TokenType::Tetap,
            "fungsi" | "fn" | "def" => TokenType::Fungsi,
            "kembalikan" | "return" => TokenType::Kembalikan,
            "jika" | "if" => TokenType::Jika,
            "kalau" | "elif" => TokenType::Kalau,
            "lainnya" | "selain_itu" | "jika_tidak" | "else" => TokenType::Lainnya,
            "selama" | "while" => TokenType::Selama,
            "untuk" | "for" => TokenType::Untuk,
            "dalam" | "in" | "di" => TokenType::Dalam,
            "ulang" | "loop" => TokenType::Ulang,
            "berhenti" | "break" => TokenType::Berhenti,
            "lanjut" | "continue" => TokenType::Lanjut,
            "benar" | "true" => TokenType::Benar,
            "salah" | "false" => TokenType::Salah,
            "nihil" | "kosong" | "null" | "nil" => TokenType::Nihil,
            "dan" | "and" => TokenType::Dan,
            "atau" | "or" => TokenType::Atau,
            "bukan" | "tidak" | "not" => TokenType::Bukan,
            "asinkron" | "async" => TokenType::Asinkron,
            "tunggu_hasil" | "await" => TokenType::TungguHasil,
            "impor" | "muat" | "import" => TokenType::Impor,
            "sebagai" | "as" => TokenType::Sebagai,
            "coba" | "try" => TokenType::Coba,
            "tangkap" | "catch" => TokenType::Tangkap,
            "lempar" | "throw" => TokenType::Lempar,
            "struktur" | "kelas" | "struct" | "class" => TokenType::Struktur,
            "sifat" | "trait" | "interface" => TokenType::Sifat,
            "terapkan" | "implementasi" | "impl" => TokenType::Terapkan,
            "enum" | "varian" | "pilihan" | "opsi" => TokenType::Enum,
            "cocokkan" | "sesuaikan" | "match" | "cocok" => TokenType::Cocokkan,
            "baru" | "new" => TokenType::Baru,
            "ini" | "this" | "self" => TokenType::Ini,
            "eksternal" | "extern" => TokenType::Eksternal,
            "volatil" | "volatile" => TokenType::Volatil,
            "pinjam" | "borrow" => TokenType::Pinjam,
            "mutabel" | "mut" => TokenType::Mutabel,
            _ => TokenType::Identifier(text.clone()),
        };

        Ok(self.make_token(token_type))
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let lexeme: String = self.chars[self.start..self.current].iter().collect();
        Token::new(token_type, lexeme, Span::new(self.line, self.start_column))
    }

    fn advance(&mut self) -> char {
        let c = self.chars[self.current];
        self.current += 1;
        self.column += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.chars[self.current] != expected {
            false
        } else {
            self.current += 1;
            self.column += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.chars.len() {
            '\0'
        } else {
            self.chars[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn current_span(&self) -> Span {
        Span::new(self.line, self.start_column)
    }
}
