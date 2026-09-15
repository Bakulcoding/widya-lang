use crate::ast::*;
use crate::error::Galat;
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, Galat> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            // Skip empty semicolons
            if self.match_token(&[TokenType::Semicolon]) {
                continue;
            }
            statements.push(self.declaration()?);
        }

        Ok(Program::new(statements))
    }

    // --- Declarations & Statements ---

    fn declaration(&mut self) -> Result<Stmt, Galat> {
        let mut attributes = Vec::new();

        // Parse attributes: #[turunkan(Json, Serialisasi)] or #[profil("edge")]
        while self.match_token(&[TokenType::Hash]) {
            let attr_span = self.previous().span.clone();
            self.consume(TokenType::LeftBracket, "Harapkan '[' setelah '#'")?;
            let attr_name = self.consume_identifier("Harapkan nama atribut")?.lexeme;
            let mut arguments = Vec::new();
            if self.match_token(&[TokenType::LeftParen]) {
                if !self.check(&TokenType::RightParen) {
                    loop {
                        let arg = if let TokenType::StringLiteral(s) = &self.peek().token_type {
                            let val = s.clone();
                            self.advance();
                            val
                        } else {
                            self.consume_identifier("Harapkan argumen atribut (identifier atau string)")?.lexeme
                        };
                        arguments.push(arg);
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Harapkan ')' setelah argumen atribut")?;
            }
            self.consume(TokenType::RightBracket, "Harapkan ']' untuk menutup atribut")?;
            attributes.push(Attribute {
                name: attr_name,
                arguments,
                span: attr_span,
            });
        }

        if self.match_token(&[TokenType::Misal]) {
            self.var_declaration(false)
        } else if self.match_token(&[TokenType::Tetap]) {
            self.var_declaration(true)
        } else if self.match_token(&[TokenType::Asinkron]) {
            self.consume(TokenType::Fungsi, "Harapkan 'fungsi' setelah kata kunci 'asinkron'")?;
            if self.check_identifier() {
                self.function_declaration(attributes)
            } else {
                let expr = self.function_expression()?;
                self.match_token(&[TokenType::Semicolon]);
                Ok(Stmt::Expression(expr))
            }
        } else if self.match_token(&[TokenType::Fungsi]) {
            if self.check_identifier() {
                self.function_declaration(attributes)
            } else {
                let expr = self.function_expression()?;
                self.match_token(&[TokenType::Semicolon]);
                Ok(Stmt::Expression(expr))
            }
        } else if self.match_token(&[TokenType::Struktur]) {
            self.struct_declaration(attributes)
        } else if self.match_token(&[TokenType::Sifat]) {
            self.trait_declaration()
        } else if self.match_token(&[TokenType::Terapkan]) {
            self.impl_declaration()
        } else if self.match_token(&[TokenType::Enum]) {
            self.enum_declaration(attributes)
        } else if self.match_token(&[TokenType::Eksternal]) {
            self.external_block()
        } else if self.match_token(&[TokenType::Impor]) {
            self.import_statement()
        } else if self.match_token(&[TokenType::Coba]) {
            self.try_catch_statement()
        } else if self.match_token(&[TokenType::Lempar]) {
            self.throw_statement()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self, is_const: bool) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();

        // Check for destructuring declaration: misal [a, b, c] = data;
        if self.match_token(&[TokenType::LeftBracket]) {
            let mut names = Vec::new();
            if !self.check(&TokenType::RightBracket) {
                loop {
                    let id = self.consume_identifier("Harapkan nama variabel dalam dekonstruksi")?;
                    names.push(id.lexeme);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightBracket, "Harapkan ']' setelah daftar variabel")?;
            self.consume(TokenType::Assign, "Harapkan '=' setelah dekonstruksi variabel")?;
            let initializer = self.expression()?;
            self.match_token(&[TokenType::Semicolon]);
            return Ok(Stmt::DestructureDecl {
                names,
                initializer,
                is_const,
                span,
            });
        }

        let name_token = self.consume_identifier("Harapkan nama variabel setelah 'misal'/'tetap'")?;
        let name = name_token.lexeme.clone();

        let initializer = if self.match_token(&[TokenType::Assign]) {
            Some(self.expression()?)
        } else if is_const {
            return Err(Galat::sintaks(
                format!("Konstanta 'tetap {}' harus memiliki nilai awal", name),
                &span,
            ));
        } else {
            None
        };

        self.match_token(&[TokenType::Semicolon]);
        Ok(Stmt::VarDecl {
            name,
            is_const,
            initializer,
            span,
        })
    }

    fn function_declaration(&mut self, attributes: Vec<Attribute>) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let name_token = self.consume_identifier("Harapkan nama fungsi")?;
        let name = name_token.lexeme.clone();

        self.consume(TokenType::LeftParen, "Harapkan '(' setelah nama fungsi")?;
        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let param = self.consume_identifier("Harapkan nama parameter fungsi")?;
                params.push(param.lexeme.clone());
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Harapkan ')' setelah daftar parameter")?;

        let body = self.block_statement_list()?;
        Ok(Stmt::FunctionDecl {
            name,
            params,
            body,
            attributes,
            span,
        })
    }

    fn struct_declaration(&mut self, attributes: Vec<Attribute>) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let name_token = self.consume_identifier("Harapkan nama struktur")?;
        let name = name_token.lexeme.clone();

        self.consume(TokenType::LeftBrace, "Harapkan '{' pada definisi struktur")?;
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon, TokenType::Comma]) {
                continue;
            }

            if self.match_token(&[TokenType::Fungsi]) {
                let method_span = self.previous().span.clone();
                let method_name = self.consume_identifier("Harapkan nama metode")?.lexeme;
                self.consume(TokenType::LeftParen, "Harapkan '(' setelah nama metode")?;
                let mut params = Vec::new();
                if !self.check(&TokenType::RightParen) {
                    loop {
                        let param = self.consume_identifier("Harapkan nama parameter")?;
                        params.push(param.lexeme.clone());
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Harapkan ')'")?;
                let body = self.block_statement_list()?;
                methods.push(StructMethod {
                    name: method_name,
                    params,
                    body,
                    span: method_span,
                });
            } else if self.check_identifier() {
                let field_token = self.advance();
                fields.push(field_token.lexeme);
                self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
            } else {
                return Err(Galat::sintaks(
                    "Harapkan nama field atau metode 'fungsi' di dalam struktur",
                    &self.peek().span,
                ));
            }
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup struktur")?;
        Ok(Stmt::StructDecl {
            name,
            fields,
            methods,
            attributes,
            span,
        })
    }

    fn trait_declaration(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let name_token = self.consume_identifier("Harapkan nama sifat (trait)")?;
        let name = name_token.lexeme;

        self.consume(TokenType::LeftBrace, "Harapkan '{' pada definisi sifat")?;
        let mut methods = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon, TokenType::Comma]) {
                continue;
            }

            self.consume(TokenType::Fungsi, "Harapkan kata kunci 'fungsi' dalam deklarasi sifat")?;
            let m_span = self.previous().span.clone();
            let m_name = self.consume_identifier("Harapkan nama metode pada sifat")?.lexeme;
            self.consume(TokenType::LeftParen, "Harapkan '(' setelah nama metode")?;
            let mut params = Vec::new();
            if !self.check(&TokenType::RightParen) {
                loop {
                    let param = self.consume_identifier("Harapkan nama parameter")?;
                    params.push(param.lexeme);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightParen, "Harapkan ')'")?;
            self.match_token(&[TokenType::Semicolon]);
            methods.push(TraitMethodSignature {
                name: m_name,
                params,
                span: m_span,
            });
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup sifat")?;
        Ok(Stmt::TraitDecl {
            name,
            methods,
            span,
        })
    }

    fn impl_declaration(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let first_id = self.consume_identifier("Harapkan nama sifat atau struktur setelah 'terapkan'")?.lexeme;

        let (trait_name, target_name) = if self.match_token(&[TokenType::Untuk]) {
            let target_id = self.consume_identifier("Harapkan nama struktur target setelah 'untuk'")?.lexeme;
            (Some(first_id), target_id)
        } else {
            (None, first_id)
        };

        self.consume(TokenType::LeftBrace, "Harapkan '{' pada blok 'terapkan'")?;
        let mut methods = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon, TokenType::Comma]) {
                continue;
            }

            self.consume(TokenType::Fungsi, "Harapkan kata kunci 'fungsi' dalam blok terapkan")?;
            let method_span = self.previous().span.clone();
            let method_name = self.consume_identifier("Harapkan nama metode")?.lexeme;
            self.consume(TokenType::LeftParen, "Harapkan '(' setelah nama metode")?;
            let mut params = Vec::new();
            if !self.check(&TokenType::RightParen) {
                loop {
                    let param = self.consume_identifier("Harapkan nama parameter")?;
                    params.push(param.lexeme);
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightParen, "Harapkan ')'")?;
            let body = self.block_statement_list()?;
            methods.push(StructMethod {
                name: method_name,
                params,
                body,
                span: method_span,
            });
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup blok terapkan")?;
        Ok(Stmt::ImplDecl {
            trait_name,
            target_name,
            methods,
            span,
        })
    }

    fn enum_declaration(&mut self, attributes: Vec<Attribute>) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let name_token = self.consume_identifier("Harapkan nama enum setelah kata kunci 'enum'")?;
        let name = name_token.lexeme.clone();

        self.consume(TokenType::LeftBrace, "Harapkan '{' pada definisi enum")?;
        let mut variants = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Comma, TokenType::Semicolon]) {
                continue;
            }

            let var_span = self.peek().span.clone();
            let var_name = self.consume_identifier("Harapkan nama varian enum")?.lexeme;

            let mut fields_count = 0;
            if self.match_token(&[TokenType::LeftParen]) {
                if !self.check(&TokenType::RightParen) {
                    loop {
                        let _field_name = self.consume_identifier("Harapkan nama field varian enum")?;
                        fields_count += 1;
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Harapkan ')' setelah parameter varian enum")?;
            }

            variants.push(EnumVariantDecl {
                name: var_name,
                fields_count,
                span: var_span,
            });

            self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup enum")?;
        Ok(Stmt::EnumDecl {
            name,
            variants,
            attributes,
            span,
        })
    }

    fn external_block(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let abi = if let TokenType::StringLiteral(s) = &self.peek().token_type {
            let val = s.clone();
            self.advance();
            val
        } else {
            "C".to_string()
        };

        self.consume(TokenType::LeftBrace, "Harapkan '{' setelah blok 'eksternal'")?;
        let mut functions = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon]) {
                continue;
            }
            self.consume(TokenType::Fungsi, "Harapkan deklarasi 'fungsi' dalam blok eksternal")?;
            let fn_token = self.consume_identifier("Harapkan nama fungsi eksternal")?;
            let fn_name = fn_token.lexeme.clone();
            let fn_span = fn_token.span.clone();

            self.consume(TokenType::LeftParen, "Harapkan '(' setelah nama fungsi eksternal")?;
            let mut params = Vec::new();
            if !self.check(&TokenType::RightParen) {
                loop {
                    let p = self.consume_identifier("Harapkan nama parameter fungsi eksternal")?;
                    params.push(p.lexeme);
                    // Optional type annotation: param: Tipe
                    if self.match_token(&[TokenType::Colon]) {
                        let _type_name = self.consume_identifier("Harapkan tipe parameter")?;
                    }
                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.consume(TokenType::RightParen, "Harapkan ')' setelah daftar parameter")?;

            // Optional return type annotation: -> Tipe
            let mut return_type = None;
            if self.match_token(&[TokenType::Minus]) {
                if self.match_token(&[TokenType::Greater]) {
                    let ret_id = self.consume_identifier("Harapkan tipe kembalian fungsi eksternal")?;
                    return_type = Some(ret_id.lexeme);
                }
            }

            self.match_token(&[TokenType::Semicolon]);
            functions.push(ExternalFunctionDecl {
                name: fn_name,
                params,
                return_type,
                span: fn_span,
            });
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup blok eksternal")?;
        Ok(Stmt::ExternalBlock {
            abi,
            functions,
            span,
        })
    }

    fn import_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let path_token = self.peek().clone();
        let path = match &path_token.token_type {
            TokenType::StringLiteral(s) => {
                self.advance();
                s.clone()
            }
            _ => {
                return Err(Galat::sintaks(
                    "Harapkan path berkas string setelah 'impor'/'muat'",
                    &path_token.span,
                ));
            }
        };

        let alias = if self.match_token(&[TokenType::Sebagai]) {
            let alias_token = self.consume_identifier("Harapkan nama alias setelah 'sebagai'")?;
            Some(alias_token.lexeme.clone())
        } else {
            None
        };

        self.match_token(&[TokenType::Semicolon]);
        Ok(Stmt::Import { path, alias, span })
    }

    fn try_catch_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let try_block = Box::new(self.block_or_single_statement()?);

        self.consume(TokenType::Tangkap, "Harapkan 'tangkap' setelah blok 'coba'")?;
        let error_var = if self.check_identifier() {
            self.advance().lexeme
        } else {
            "galat".to_string()
        };

        let catch_block = Box::new(self.block_or_single_statement()?);
        Ok(Stmt::TryCatch {
            try_block,
            error_var,
            catch_block,
            span,
        })
    }

    fn throw_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let expr = self.expression()?;
        self.match_token(&[TokenType::Semicolon]);
        Ok(Stmt::Throw { expr, span })
    }

    fn statement(&mut self) -> Result<Stmt, Galat> {
        if self.match_token(&[TokenType::Jika]) {
            self.if_statement()
        } else if self.match_token(&[TokenType::Selama]) {
            self.while_statement()
        } else if self.match_token(&[TokenType::Untuk]) {
            self.for_in_statement()
        } else if self.match_token(&[TokenType::Ulang]) {
            self.loop_statement()
        } else if self.match_token(&[TokenType::Kembalikan]) {
            self.return_statement()
        } else if self.match_token(&[TokenType::Berhenti]) {
            let span = self.previous().span.clone();
            self.match_token(&[TokenType::Semicolon]);
            Ok(Stmt::Break(span))
        } else if self.match_token(&[TokenType::Lanjut]) {
            let span = self.previous().span.clone();
            self.match_token(&[TokenType::Semicolon]);
            Ok(Stmt::Continue(span))
        } else if self.check(&TokenType::LeftBrace) {
            let span = self.peek().span.clone();
            let stmts = self.block_statement_list()?;
            Ok(Stmt::Block(stmts, span))
        } else {
            self.assignment_or_expression_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();

        // Check for 'if let' pattern: jika misal <pola> = <nilai> { ... }
        if self.match_token(&[TokenType::Misal]) {
            let pattern = self.match_pattern()?;
            self.consume(TokenType::Assign, "Harapkan '=' setelah pola pada 'jika misal'")?;
            let value = self.expression()?;
            let then_branch = Box::new(self.block_or_single_statement()?);
            let else_branch = if self.match_token(&[TokenType::Lainnya]) {
                Some(Box::new(self.block_or_single_statement()?))
            } else {
                None
            };
            return Ok(Stmt::IfLet {
                pattern,
                value,
                then_branch,
                else_branch,
                span,
            });
        }

        let condition = self.expression()?;
        let then_branch = Box::new(self.block_or_single_statement()?);

        let mut elif_branches = Vec::new();
        while self.match_token(&[TokenType::Kalau]) {
            let elif_cond = self.expression()?;
            let elif_body = self.block_or_single_statement()?;
            elif_branches.push((elif_cond, elif_body));
        }

        let else_branch = if self.match_token(&[TokenType::Lainnya]) {
            Some(Box::new(self.block_or_single_statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            elif_branches,
            else_branch,
            span,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let condition = self.expression()?;
        let body = Box::new(self.block_or_single_statement()?);
        Ok(Stmt::While {
            condition,
            body,
            span,
        })
    }

    fn for_in_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let var_token = self.consume_identifier("Harapkan nama variabel pada perulangan 'untuk'")?;
        let var_name = var_token.lexeme.clone();

        self.consume(TokenType::Dalam, "Harapkan 'dalam' setelah nama variabel pada perulangan 'untuk'")?;
        let iterable = self.expression()?;
        let body = Box::new(self.block_or_single_statement()?);

        Ok(Stmt::ForIn {
            var_name,
            iterable,
            body,
            span,
        })
    }

    fn loop_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let body = Box::new(self.block_or_single_statement()?);
        Ok(Stmt::Loop { body, span })
    }

    fn return_statement(&mut self) -> Result<Stmt, Galat> {
        let span = self.previous().span.clone();
        let value = if self.check(&TokenType::Semicolon)
            || self.check(&TokenType::RightBrace)
            || self.is_at_end()
        {
            None
        } else {
            Some(self.expression()?)
        };

        self.match_token(&[TokenType::Semicolon]);
        Ok(Stmt::Return { value, span })
    }

    fn block_statement_list(&mut self) -> Result<Vec<Stmt>, Galat> {
        self.consume(TokenType::LeftBrace, "Harapkan '{' untuk memulai blok")?;
        let mut stmts = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Semicolon]) {
                continue;
            }
            stmts.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup blok")?;
        Ok(stmts)
    }

    fn block_or_single_statement(&mut self) -> Result<Stmt, Galat> {
        if self.check(&TokenType::LeftBrace) {
            let span = self.peek().span.clone();
            let stmts = self.block_statement_list()?;
            Ok(Stmt::Block(stmts, span))
        } else {
            self.statement()
        }
    }

    fn assignment_or_expression_statement(&mut self) -> Result<Stmt, Galat> {
        let expr = self.expression()?;

        if self.match_token(&[
            TokenType::Assign,
            TokenType::PlusAssign,
            TokenType::MinusAssign,
            TokenType::StarAssign,
            TokenType::SlashAssign,
        ]) {
            let op_token = self.previous().clone();
            let op = match op_token.token_type {
                TokenType::Assign => AssignOp::Assign,
                TokenType::PlusAssign => AssignOp::PlusAssign,
                TokenType::MinusAssign => AssignOp::MinusAssign,
                TokenType::StarAssign => AssignOp::StarAssign,
                TokenType::SlashAssign => AssignOp::SlashAssign,
                _ => unreachable!(),
            };

            let target = match expr {
                Expr::Identifier(name, _) => AssignTarget::Variable(name),
                Expr::Index {
                    target,
                    index,
                    span: _,
                } => AssignTarget::Index { target, index },
                _ => {
                    return Err(Galat::sintaks(
                        "Target penugasan tidak valid (hanya variabel atau elemen indeks)",
                        expr.span(),
                    ));
                }
            };

            let value = self.expression()?;
            self.match_token(&[TokenType::Semicolon]);
            return Ok(Stmt::Assignment {
                target,
                op,
                value,
                span: op_token.span,
            });
        }

        self.match_token(&[TokenType::Semicolon]);
        Ok(Stmt::Expression(expr))
    }

    // --- Expressions (Pratt Precedence) ---

    pub fn expression(&mut self) -> Result<Expr, Galat> {
        self.or()
    }

    fn or(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.and()?;

        while self.match_token(&[TokenType::Atau]) {
            let span = self.previous().span.clone();
            let right = self.and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::Dan]) {
            let span = self.previous().span.clone();
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let token = self.previous().clone();
            let op = match token.token_type {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span: token.span,
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.range()?;

        while self.match_token(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let token = self.previous().clone();
            let op = match token.token_type {
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            let right = self.range()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span: token.span,
            };
        }

        Ok(expr)
    }

    fn range(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.term()?;

        if self.match_token(&[TokenType::DotDot]) {
            let span = self.previous().span.clone();
            let end = self.term()?;
            expr = Expr::Range {
                start: Box::new(expr),
                end: Box::new(end),
                span,
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let token = self.previous().clone();
            let op = match token.token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span: token.span,
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.power()?;

        while self.match_token(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let token = self.previous().clone();
            let op = match token.token_type {
                TokenType::Star => BinaryOp::Multiply,
                TokenType::Slash => BinaryOp::Divide,
                TokenType::Percent => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            let right = self.power()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span: token.span,
            };
        }

        Ok(expr)
    }

    fn power(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Caret]) {
            let span = self.previous().span.clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Power,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Galat> {
        if self.match_token(&[TokenType::TungguHasil]) {
            let span = self.previous().span.clone();
            let right = self.unary()?;
            return Ok(Expr::Await(Box::new(right), span));
        }

        if self.match_token(&[TokenType::Volatil]) {
            let span = self.previous().span.clone();
            let addr = self.unary()?;
            return Ok(Expr::VolatileRead {
                address: Box::new(addr),
                span,
            });
        }

        if self.match_token(&[TokenType::Minus, TokenType::Bukan]) {
            let token = self.previous().clone();
            let op = match token.token_type {
                TokenType::Minus => UnaryOp::Negate,
                TokenType::Bukan => UnaryOp::Not,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            return Ok(Expr::Unary {
                op,
                right: Box::new(right),
                span: token.span,
            });
        }

        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr, Galat> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                let span = self.previous().span.clone();
                let mut arguments = Vec::new();
                if !self.check(&TokenType::RightParen) {
                    loop {
                        arguments.push(self.expression()?);
                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Harapkan ')' setelah argumen fungsi")?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    arguments,
                    span,
                };
            } else if self.match_token(&[TokenType::LeftBracket]) {
                let span = self.previous().span.clone();
                let index = self.expression()?;
                self.consume(TokenType::RightBracket, "Harapkan ']' setelah indeks")?;
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                    span,
                };
            } else if self.match_token(&[TokenType::Dot]) {
                let span = self.previous().span.clone();
                let prop_token = self.consume_identifier("Harapkan nama properti setelah '.'")?;
                let index = Expr::String(prop_token.lexeme, span.clone());
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                    span,
                };
            } else if self.match_token(&[TokenType::Colon]) {
                // Check if it's double colon '::'
                if self.match_token(&[TokenType::Colon]) {
                    let span = self.previous().span.clone();
                    let variant_token = self.consume_identifier("Harapkan nama varian enum setelah '::'")?;
                    let index = Expr::String(variant_token.lexeme, span.clone());
                    expr = Expr::Index {
                        target: Box::new(expr),
                        index: Box::new(index),
                        span,
                    };
                } else {
                    // Single colon is not part of postfix, backtrack
                    self.current -= 1;
                    break;
                }
            } else if self.match_token(&[TokenType::Question]) {
                let span = self.previous().span.clone();
                expr = Expr::Try(Box::new(expr), span);
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, Galat> {
        let token = self.peek().clone();

        match &token.token_type {
            TokenType::Cocokkan => self.match_expression(),
            TokenType::Number(n) => {
                let val = *n;
                let span = token.span.clone();
                self.advance();
                Ok(Expr::Number(val, span))
            }
            TokenType::StringLiteral(s) => {
                let val = s.clone();
                let span = token.span.clone();
                self.advance();
                Ok(Expr::String(val, span))
            }
            TokenType::Benar => {
                let span = token.span.clone();
                self.advance();
                Ok(Expr::Boolean(true, span))
            }
            TokenType::Salah => {
                let span = token.span.clone();
                self.advance();
                Ok(Expr::Boolean(false, span))
            }
            TokenType::Nihil => {
                let span = token.span.clone();
                self.advance();
                Ok(Expr::Nil(span))
            }
            TokenType::Ini => {
                let span = token.span.clone();
                self.advance();
                Ok(Expr::This(span))
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                let span = token.span.clone();
                self.advance();
                Ok(Expr::Identifier(name, span))
            }
            TokenType::LeftParen => {
                let span = token.span.clone();
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Harapkan ')' setelah ekspresi")?;
                Ok(Expr::Grouping(Box::new(expr), span))
            }
            TokenType::LeftBracket => self.array_literal(),
            TokenType::LeftBrace => self.map_literal(),
            TokenType::Fungsi => self.function_expression(),
            _ => Err(Galat::sintaks(
                format!("Ekspresi tidak terduga '{}'", token.lexeme),
                &token.span,
            )),
        }
    }

    fn match_expression(&mut self) -> Result<Expr, Galat> {
        let span = self.consume(TokenType::Cocokkan, "Harapkan 'cocokkan'")?.span;
        let target = self.expression()?;
        self.consume(TokenType::LeftBrace, "Harapkan '{' setelah target ekspresi 'cocokkan'")?;

        let mut arms = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&[TokenType::Comma, TokenType::Semicolon]) {
                continue;
            }

            let pattern_span = self.peek().span.clone();
            let pattern = self.match_pattern()?;

            self.consume(TokenType::FatArrow, "Harapkan '=>' setelah pola cocokkan")?;

            let body = if self.check(&TokenType::LeftBrace) {
                let block_span = self.peek().span.clone();
                let stmts = self.block_statement_list()?;
                // Wrap block as an IIFE (Immediately Invoked Function Expression)
                Expr::Call {
                    callee: Box::new(Expr::FunctionExpr {
                        params: Vec::new(),
                        body: stmts,
                        span: block_span.clone(),
                    }),
                    arguments: Vec::new(),
                    span: block_span,
                }
            } else {
                let expr = self.expression()?;
                self.match_token(&[TokenType::Comma, TokenType::Semicolon]);
                expr
            };

            arms.push(MatchArm {
                pattern,
                body,
                span: pattern_span,
            });
        }

        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup blok 'cocokkan'")?;
        Ok(Expr::Match {
            target: Box::new(target),
            arms,
            span,
        })
    }

    fn match_pattern(&mut self) -> Result<MatchPattern, Galat> {
        let token = self.peek().clone();

        match &token.token_type {
            TokenType::Identifier(name) if name == "_" => {
                self.advance();
                Ok(MatchPattern::Wildcard(token.span))
            }
            TokenType::Number(n) => {
                let val = *n;
                let span = token.span.clone();
                self.advance();
                Ok(MatchPattern::Literal(Expr::Number(val, span)))
            }
            TokenType::StringLiteral(s) => {
                let val = s.clone();
                let span = token.span.clone();
                self.advance();
                Ok(MatchPattern::Literal(Expr::String(val, span)))
            }
            TokenType::Benar => {
                let span = token.span.clone();
                self.advance();
                Ok(MatchPattern::Literal(Expr::Boolean(true, span)))
            }
            TokenType::Salah => {
                let span = token.span.clone();
                self.advance();
                Ok(MatchPattern::Literal(Expr::Boolean(false, span)))
            }
            TokenType::Nihil => {
                let span = token.span.clone();
                self.advance();
                Ok(MatchPattern::Literal(Expr::Nil(span)))
            }
            TokenType::Identifier(first_id) => {
                let first_name = first_id.clone();
                let span = token.span.clone();
                self.advance();

                // Check for Enum::Variant or Variant(args...)
                if self.match_token(&[TokenType::Colon]) {
                    if self.match_token(&[TokenType::Colon]) {
                        let variant_token = self.consume_identifier("Harapkan nama varian enum setelah '::'")?;
                        let variant_name = variant_token.lexeme;
                        let mut bindings = Vec::new();
                        if self.match_token(&[TokenType::LeftParen]) {
                            if !self.check(&TokenType::RightParen) {
                                loop {
                                    let b = self.consume_identifier("Harapkan nama variabel pengikat varian")?;
                                    bindings.push(b.lexeme);
                                    if !self.match_token(&[TokenType::Comma]) {
                                        break;
                                    }
                                }
                            }
                            self.consume(TokenType::RightParen, "Harapkan ')' setelah pengikat varian")?;
                        }
                        return Ok(MatchPattern::EnumVariant {
                            enum_name: Some(first_name),
                            variant_name,
                            bindings,
                            span,
                        });
                    } else {
                        self.current -= 1;
                    }
                }

                if self.match_token(&[TokenType::LeftParen]) {
                    let mut bindings = Vec::new();
                    if !self.check(&TokenType::RightParen) {
                        loop {
                            let b = self.consume_identifier("Harapkan nama variabel pengikat varian")?;
                            bindings.push(b.lexeme);
                            if !self.match_token(&[TokenType::Comma]) {
                                break;
                            }
                        }
                    }
                    self.consume(TokenType::RightParen, "Harapkan ')' setelah pengikat varian")?;
                    return Ok(MatchPattern::EnumVariant {
                        enum_name: None,
                        variant_name: first_name,
                        bindings,
                        span,
                    });
                }

                Ok(MatchPattern::Identifier(first_name, span))
            }
            _ => Err(Galat::sintaks(
                format!("Pola tidak valid pada 'cocokkan': '{}'", token.lexeme),
                &token.span,
            )),
        }
    }

    fn array_literal(&mut self) -> Result<Expr, Galat> {
        let span = self.consume(TokenType::LeftBracket, "Harapkan '['")?.span;
        let mut elements = Vec::new();
        if !self.check(&TokenType::RightBracket) {
            loop {
                elements.push(self.expression()?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightBracket, "Harapkan ']' untuk menutup daftar")?;
        Ok(Expr::Array(elements, span))
    }

    fn map_literal(&mut self) -> Result<Expr, Galat> {
        let span = self.consume(TokenType::LeftBrace, "Harapkan '{'")?.span;
        let mut pairs = Vec::new();
        if !self.check(&TokenType::RightBrace) {
            loop {
                let key = if self.check_identifier() {
                    let id = self.advance();
                    Expr::String(id.lexeme, id.span)
                } else {
                    self.expression()?
                };

                self.consume(TokenType::Colon, "Harapkan ':' antara kunci dan nilai kamus")?;
                let val = self.expression()?;
                pairs.push((key, val));

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightBrace, "Harapkan '}' untuk menutup kamus")?;
        Ok(Expr::Map(pairs, span))
    }

    fn function_expression(&mut self) -> Result<Expr, Galat> {
        let span = self.consume(TokenType::Fungsi, "Harapkan 'fungsi'")?.span;
        self.consume(TokenType::LeftParen, "Harapkan '(' setelah 'fungsi'")?;
        let mut params = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let param = self.consume_identifier("Harapkan nama parameter fungsi")?;
                params.push(param.lexeme.clone());
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Harapkan ')' setelah daftar parameter")?;
        let body = self.block_statement_list()?;
        Ok(Expr::FunctionExpr { params, body, span })
    }

    // --- Helpers ---

    fn check_identifier(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Identifier(_))
    }

    fn consume_identifier(&mut self, err_msg: &str) -> Result<Token, Galat> {
        if self.check_identifier() {
            Ok(self.advance())
        } else {
            Err(Galat::sintaks(err_msg, &self.peek().span))
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, err_msg: &str) -> Result<Token, Galat> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(Galat::sintaks(err_msg, &self.peek().span))
        }
    }
}
