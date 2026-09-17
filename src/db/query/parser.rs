// ============================================================================
// SQL Parser & AST Generator - TAHAP 3.1
// ============================================================================
// Parser untuk SQL subset dengan AST generation
// Features:
// - Tokenizer/lexer
// - Parser untuk SELECT, INSERT, UPDATE, DELETE, CREATE TABLE
// - AST structure untuk query representation
// ============================================================================

use std::collections::VecDeque;

// ============================================================================
// Token Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
    CREATE,
    TABLE,
    DROP,
    ALTER,
    WHERE,
    AND,
    OR,
    NOT,
    FROM,
    INTO,
    VALUES,
    SET,
    AS,
    ON,
    JOIN,
    INNER,
    LEFT,
    RIGHT,
    FULL,
    GROUP,
    BY,
    HAVING,
    ORDER,
    LIMIT,
    OFFSET,
    PRIMARY,
    FOREIGN,
    KEY,
    REFERENCES,
    NULL,
    NOT_NULL,
    DEFAULT,
    UNIQUE,
    INDEX,
    CHECK,
    
    // Identifiers & Literals
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    Star,
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Punctuation
    LParen,
    RParen,
    Comma,
    Dot,
    Semicolon,
    Colon,
    
    // Special
    EOF,
    Error(String),
}

impl TokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenType::SELECT
                | TokenType::INSERT
                | TokenType::UPDATE
                | TokenType::DELETE
                | TokenType::CREATE
                | TokenType::TABLE
                | TokenType::DROP
                | TokenType::ALTER
                | TokenType::WHERE
                | TokenType::AND
                | TokenType::OR
                | TokenType::NOT
                | TokenType::FROM
                | TokenType::INTO
                | TokenType::VALUES
                | TokenType::SET
                | TokenType::AS
                | TokenType::ON
                | TokenType::JOIN
                | TokenType::INNER
                | TokenType::LEFT
                | TokenType::RIGHT
                | TokenType::FULL
                | TokenType::GROUP
                | TokenType::BY
                | TokenType::HAVING
                | TokenType::LIMIT
                | TokenType::OFFSET
                | TokenType::PRIMARY
                | TokenType::FOREIGN
                | TokenType::KEY
                | TokenType::REFERENCES
                | TokenType::NULL
                | TokenType::NOT_NULL
                | TokenType::DEFAULT
                | TokenType::UNIQUE
                | TokenType::INDEX
                | TokenType::CHECK
        )
    }
}

// ============================================================================
// Tokenizer (Lexer)
// ============================================================================

pub struct Tokenizer {
    input: String,
    position: usize,
    peek_position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
            peek_position: 0,
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return TokenType::EOF;
        }

        let ch = self.peek_char();

        // Check for two-character operators first
        if self.peek_char() == '=' && self.peek_char_at(1).map(|c| c == '=').unwrap_or(false) {
            self.advance();
            self.advance();
            return TokenType::Equal;
        }

        if self.peek_char() == '!' && self.peek_char_at(1).map(|c| c == '=').unwrap_or(false) {
            self.advance();
            self.advance();
            return TokenType::NotEqual;
        }

        if self.peek_char() == '<' && self.peek_char_at(1).map(|c| c == '=').unwrap_or(false) {
            self.advance();
            self.advance();
            return TokenType::LessEqual;
        }

        if self.peek_char() == '>' && self.peek_char_at(1).map(|c| c == '=').unwrap_or(false) {
            self.advance();
            self.advance();
            return TokenType::GreaterEqual;
        }

        match ch {
            '(' => {
                self.advance();
                TokenType::LParen
            }
            ')' => {
                self.advance();
                TokenType::RParen
            }
            ',' => {
                self.advance();
                TokenType::Comma
            }
            '.' => {
                self.advance();
                TokenType::Dot
            }
            ';' => {
                self.advance();
                TokenType::Semicolon
            }
            ':' => {
                self.advance();
                TokenType::Colon
            }
            '+' => {
                self.advance();
                TokenType::Plus
            }
            '-' => {
                self.advance();
                TokenType::Minus
            }
            '*' => {
                self.advance();
                TokenType::Star
            }
            '/' => {
                self.advance();
                TokenType::Divide
            }
            '<' => {
                self.advance();
                TokenType::Less
            }
            '>' => {
                self.advance();
                TokenType::Greater
            }
            '"' | '\'' => self.read_string_literal(ch),
            c if c.is_ascii_digit() || (c == '-' && self.peek_char_at(1).map(|n| n.is_ascii_digit()).unwrap_or(false)) => self.read_number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.read_identifier(),
            _ => TokenType::Error(format!("Illegal character: '{}'", ch)),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek_char().map(|c| c.is_whitespace()).unwrap_or(false) {
            self.advance();
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn peek_char_at(&self, offset: usize) -> Option<char> {
        self.input.chars().nth(self.position + offset)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn read_identifier(&mut self) -> TokenType {
        let start = self.position;
        while self.peek_char().map(|c| c.is_ascii_alphanumeric() || c == '_').unwrap_or(false) {
            self.advance();
        }
        let ident = self.input[start..self.position].to_string();
        
        // Check for keywords
        match ident.to_uppercase().as_str() {
            "SELECT" => TokenType::SELECT,
            "INSERT" => TokenType::INSERT,
            "UPDATE" => TokenType::UPDATE,
            "DELETE" => TokenType::DELETE,
            "CREATE" => TokenType::CREATE,
            "TABLE" => TokenType::TABLE,
            "DROP" => TokenType::DROP,
            "ALTER" => TokenType::ALTER,
            "WHERE" => TokenType::WHERE,
            "AND" => TokenType::AND,
            "OR" => TokenType::OR,
            "NOT" => TokenType::NOT,
            "FROM" => TokenType::FROM,
            "INTO" => TokenType::INTO,
            "VALUES" => TokenType::VALUES,
            "SET" => TokenType::SET,
            "AS" => TokenType::AS,
            "ON" => TokenType::ON,
            "JOIN" => TokenType::JOIN,
            "INNER" => TokenType::INNER,
            "LEFT" => TokenType::LEFT,
            "RIGHT" => TokenType::RIGHT,
            "FULL" => TokenType::FULL,
            "GROUP" => TokenType::GROUP,
            "BY" => TokenType::BY,
            "HAVING" => TokenType::HAVING,
            "LIMIT" => TokenType::LIMIT,
            "OFFSET" => TokenType::OFFSET,
            "PRIMARY" => TokenType::PRIMARY,
            "FOREIGN" => TokenType::FOREIGN,
            "KEY" => TokenType::KEY,
            "REFERENCES" => TokenType::REFERENCES,
            "NULL" => TokenType::NULL,
            "NOT" => TokenType::NOT,
            "DEFAULT" => TokenType::DEFAULT,
            "UNIQUE" => TokenType::UNIQUE,
            "INDEX" => TokenType::INDEX,
            "CHECK" => TokenType::CHECK,
            _ => TokenType::Identifier(ident),
        }
    }

    fn read_string_literal(&mut self, quote: char) -> TokenType {
        self.advance(); // Skip opening quote
        let start = self.position;
        
        while let Some(c) = self.peek_char() {
            if c == quote {
                break;
            }
            self.advance();
        }
        
        let literal = self.input[start..self.position].to_string();
        self.advance(); // Skip closing quote
        
        TokenType::StringLiteral(literal)
    }

    fn read_number(&mut self) -> TokenType {
        let start = self.position;
        
        // Handle negative numbers
        if self.peek_char() == Some('-') {
            self.advance();
        }
        
        while self.peek_char().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            self.advance();
        }
        
        // Check for decimal point
        if self.peek_char() == Some('.') {
            self.advance();
            while self.peek_char().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                self.advance();
            }
        }
        
        let num_str = self.input[start..self.position].parse::<f64>()
            .unwrap_or(0.0);
        
        TokenType::NumberLiteral(num_str)
    }
}

// ============================================================================
// AST Nodes
// ============================================================================

#[derive(Debug, Clone)]
pub enum Query {
    Select(Box<SelectQuery>),
    Insert(Box<InsertQuery>),
    Update(Box<UpdateQuery>),
    Delete(Box<DeleteQuery>),
    CreateTable(Box<CreateTableQuery>),
    DropTable(Box<DropTableQuery>),
}

#[derive(Debug, Clone)]
pub struct SelectQuery {
    pub columns: Vec<Column>,
    pub from: Vec<FromClause>,
    pub where_clause: Option<Expr>,
    pub group_by: Vec<Expr>,
    pub having: Option<Expr>,
    pub order_by: Vec<OrderByClause>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct InsertQuery {
    pub table: String,
    pub columns: Vec<String>,
    pub values: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct UpdateQuery {
    pub table: String,
    pub assignments: Vec<Assignment>,
    pub where_clause: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct DeleteQuery {
    pub table: String,
    pub where_clause: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct CreateTableQuery {
    pub table: String,
    pub columns: Vec<ColumnDef>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone)]
pub struct DropTableQuery {
    pub table: String,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub constraints: Vec<ColumnConstraint>,
}

#[derive(Debug, Clone)]
pub enum DataType {
    Integer,
    Float,
    Text,
    Boolean,
    Date,
    Timestamp,
    Binary,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum ColumnConstraint {
    NotNull,
    PrimaryKey,
    Unique,
    Default(Expr),
    Check(Expr),
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub name: Option<String>,
    pub kind: ConstraintKind,
}

#[derive(Debug, Clone)]
pub enum ConstraintKind {
    PrimaryKey(Vec<String>),
    ForeignKey {
        columns: Vec<String>,
        references_table: String,
        references_columns: Vec<String>,
    },
    Unique(Vec<String>),
    Check(Expr),
}

#[derive(Debug, Clone)]
pub struct FromClause {
    pub table: String,
    pub alias: Option<String>,
    pub join_type: Option<JoinType>,
    pub join_condition: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Debug, Clone)]
pub struct OrderByClause {
    pub expr: Expr,
    pub direction: OrderDirection,
}

#[derive(Debug, Clone)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub column: String,
    pub value: Expr,
}

// ============================================================================
// Expression AST
// ============================================================================

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Literal(Literal),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    InList {
        expr: Box<Expr>,
        list: Vec<Expr>,
        negated: bool,
    },
    Between {
        expr: Box<Expr>,
        low: Box<Expr>,
        high: Box<Expr>,
        negated: bool,
    },
    IsNull {
        expr: Box<Expr>,
        negated: bool,
    },
    Subquery {
        query: Box<Query>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
    Negative,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Like,
    ILike,
    In,
    NotIn,
    Between,
    NotBetween,
    Is,
    IsNot,
}

// ============================================================================
// Parser
// ============================================================================

pub struct Parser {
    tokens: VecDeque<TokenType>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Self {
            tokens: VecDeque::from(tokens),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Query, String> {
        match self.peek_token()? {
            TokenType::SELECT => self.parse_select(),
            TokenType::INSERT => self.parse_insert(),
            TokenType::UPDATE => self.parse_update(),
            TokenType::DELETE => self.parse_delete(),
            TokenType::CREATE => self.parse_create(),
            TokenType::DROP => self.parse_drop(),
            TokenType::EOF => Err("Empty query".to_string()),
            other => Err(format!("Unexpected token: {:?}", other)),
        }
    }

    // SELECT parsing
    fn parse_select(&mut self) -> Result<Query, String> {
        self.expect(TokenType::SELECT)?;
        
        let columns = self.parse_column_list()?;
        
        let from = if self.peek_is(TokenType::FROM)? {
            self.expect(TokenType::FROM)?;
            self.parse_from_clause()?
        } else {
            Vec::new()
        };
        
        let where_clause = if self.peek_is(TokenType::WHERE)? {
            self.expect(TokenType::WHERE)?;
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        let group_by = if self.peek_is(TokenType::GROUP)? {
            self.expect(TokenType::GROUP)?;
            self.expect(TokenType::BY)?;
            self.parse_expr_list()?
        } else {
            Vec::new()
        };
        
        let having = if self.peek_is(TokenType::HAVING)? {
            self.expect(TokenType::HAVING)?;
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        let order_by = if self.peek_is(TokenType::ORDER)? {
            self.expect(TokenType::ORDER)?;
            self.expect(TokenType::BY)?;
            self.parse_order_by_list()?
        } else {
            Vec::new()
        };
        
        let limit = if self.peek_is(TokenType::LIMIT)? {
            self.expect(TokenType::LIMIT)?;
            Some(self.parse_number()?)
        } else {
            None
        };
        
        let offset = if self.peek_is(TokenType::OFFSET)? {
            self.expect(TokenType::OFFSET)?;
            Some(self.parse_number()?)
        } else {
            None
        };
        
        Ok(Query::Select(Box::new(SelectQuery {
            columns,
            from,
            where_clause,
            group_by,
            having,
            order_by,
            limit,
            offset,
        })))
    }

    fn parse_column_list(&mut self) -> Result<Vec<Column>, String> {
        let mut columns = Vec::new();
        
        loop {
            let col = self.parse_column()?;
            columns.push(col);
            
            if !self.peek_is(TokenType::Comma)? {
                break;
            }
            self.expect(TokenType::Comma)?;
        }
        
        Ok(columns)
    }

    fn parse_column(&mut self) -> Result<Column, String> {
        let name = match self.peek_token()? {
            TokenType::Star => {
                self.advance();
                return Ok(Column {
                    name: "*".to_string(),
                    alias: None,
                });
            }
            TokenType::Identifier(ref name) => name.clone(),
            _ => return Err("Expected column name or *".to_string()),
        };
        
        self.advance();
        
        let alias = if self.peek_is(TokenType::AS)? {
            self.expect(TokenType::AS)?;
            match self.peek_token()? {
                TokenType::Identifier(ref s) => Some(s.clone()),
                _ => return Err("Expected alias name".to_string()),
            }
        } else if self.peek_is(TokenType::Identifier) {
            Some(self.parse_identifier()?)
        } else {
            None
        };
        
        Ok(Column { name, alias })
    }

    fn parse_from_clause(&mut self) -> Result<Vec<FromClause>, String> {
        let mut clauses = Vec::new();
        
        loop {
            let table = self.parse_identifier()?;
            
            let alias = if self.peek_is(TokenType::AS)? {
                self.expect(TokenType::AS)?;
                Some(self.parse_identifier()?)
            } else if self.peek_is(TokenType::Identifier) {
                Some(self.parse_identifier()?)
            } else {
                None
            };
            
            let mut join_type = None;
            let mut join_condition = None;
            
            if self.peek_is(TokenType::JOIN)? {
                if self.peek_is(TokenType::INNER) {
                    self.advance();
                    join_type = Some(JoinType::Inner);
                } else if self.peek_is(TokenType::LEFT) {
                    self.advance();
                    join_type = Some(JoinType::Left);
                } else if self.peek_is(TokenType::RIGHT) {
                    self.advance();
                    join_type = Some(JoinType::Right);
                } else if self.peek_is(TokenType::FULL) {
                    self.advance();
                    join_type = Some(JoinType::Full);
                } else {
                    join_type = Some(JoinType::Inner);
                }
                
                self.expect(TokenType::JOIN)?;
                
                let right_table = self.parse_identifier()?;
                let right_alias = if self.peek_is(TokenType::AS)? {
                    self.expect(TokenType::AS)?;
                    Some(self.parse_identifier()?)
                } else if self.peek_is(TokenType::Identifier) {
                    Some(self.parse_identifier()?)
                } else {
                    None
                };
                
                join_condition = Some(self.parse_join_condition(right_table, right_alias)?);
            }
            
            clauses.push(FromClause {
                table,
                alias,
                join_type,
                join_condition,
            });
            
            if self.peek_is(TokenType::Comma)? {
                self.expect(TokenType::Comma)?;
                continue;
            }
            
            if join_type.is_some() && self.peek_is(TokenType::JOIN)? {
                continue;
            }
            
            break;
        }
        
        Ok(clauses)
    }

    fn parse_join_condition(&mut self, right_table: String, right_alias: Option<String>) -> Result<Expr, String> {
        self.expect(TokenType::ON)?;
        
        let condition = self.parse_expr()?;
        
        Ok(condition)
    }

    fn parse_insert(&mut self) -> Result<Query, String> {
        self.expect(TokenType::INSERT)?;
        self.expect(TokenType::INTO)?;
        
        let table = self.parse_identifier()?;
        
        let columns = if self.peek_is(TokenType::LParen)? {
            self.expect(TokenType::LParen)?;
            let cols = self.parse_identifier_list()?;
            self.expect(TokenType::RParen)?;
            cols
        } else {
            Vec::new()
        };
        
        self.expect(TokenType::VALUES)?;
        
        self.expect(TokenType::LParen)?;
        let values = self.parse_expr_list()?;
        self.expect(TokenType::RParen)?;
        
        Ok(Query::Insert(Box::new(InsertQuery {
            table,
            columns,
            values,
        })))
    }

    fn parse_update(&mut self) -> Result<Query, String> {
        self.expect(TokenType::UPDATE)?;
        
        let table = self.parse_identifier()?;
        
        self.expect(TokenType::SET)?;
        
        let assignments = self.parse_assignment_list()?;
        
        let where_clause = if self.peek_is(TokenType::WHERE)? {
            self.expect(TokenType::WHERE)?;
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        Ok(Query::Update(Box::new(UpdateQuery {
            table,
            assignments,
            where_clause,
        })))
    }

    fn parse_delete(&mut self) -> Result<Query, String> {
        self.expect(TokenType::DELETE)?;
        self.expect(TokenType::FROM)?;
        
        let table = self.parse_identifier()?;
        
        let where_clause = if self.peek_is(TokenType::WHERE)? {
            self.expect(TokenType::WHERE)?;
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        Ok(Query::Delete(Box::new(DeleteQuery {
            table,
            where_clause,
        })))
    }

    fn parse_create(&mut self) -> Result<Query, String> {
        self.expect(TokenType::CREATE)?;
        
        if self.peek_is(TokenType::TABLE)? {
            self.expect(TokenType::TABLE)?;
            let table = self.parse_identifier()?;
            
            self.expect(TokenType::LParen)?;
            let columns = self.parse_column_defs()?;
            self.expect(TokenType::RParen)?;
            
            // Parse constraints (simplified)
            let mut constraints = Vec::new();
            while self.peek_is(TokenType::Comma)? {
                self.advance();
                if self.peek_is(TokenType::PRIMARY)? {
                    self.advance();
                    self.expect(TokenType::KEY)?;
                    self.expect(TokenType::LParen)?;
                    let cols = self.parse_identifier_list()?;
                    self.expect(TokenType::RParen)?;
                    constraints.push(Constraint {
                        name: None,
                        kind: ConstraintKind::PrimaryKey(cols),
                    });
                } else if self.peek_is(TokenType::FOREIGN)? {
                    self.advance();
                    self.expect(TokenType::KEY)?;
                    self.expect(TokenType::LParen)?;
                    let cols = self.parse_identifier_list()?;
                    self.expect(TokenType::RParen)?;
                    self.expect(TokenType::REFERENCES)?;
                    let ref_table = self.parse_identifier()?;
                    self.expect(TokenType::LParen)?;
                    let ref_cols = self.parse_identifier_list()?;
                    self.expect(TokenType::RParen)?;
                    constraints.push(Constraint {
                        name: None,
                        kind: ConstraintKind::ForeignKey {
                            columns: cols,
                            references_table: ref_table,
                            references_columns: ref_cols,
                        },
                    });
                }
            }
            
            Ok(Query::CreateTable(Box::new(CreateTableQuery {
                table,
                columns,
                constraints,
            })))
        } else {
            Err("Expected TABLE after CREATE".to_string())
        }
    }

    fn parse_drop(&mut self) -> Result<Query, String> {
        self.expect(TokenType::DROP)?;
        
        if self.peek_is(TokenType::TABLE)? {
            self.expect(TokenType::TABLE)?;
            let table = self.parse_identifier()?;
            Ok(Query::DropTable(Box::new(DropTableQuery { table })))
        } else {
            Err("Expected TABLE after DROP".to_string())
        }
    }

    // Helper parsing methods
    fn parse_column_defs(&mut self) -> Result<Vec<ColumnDef>, String> {
        let mut defs = Vec::new();
        
        loop {
            let def = self.parse_column_def()?;
            defs.push(def);
            
            if !self.peek_is(TokenType::Comma)? {
                break;
            }
            self.expect(TokenType::Comma)?;
        }
        
        Ok(defs)
    }

    fn parse_column_def(&mut self) -> Result<ColumnDef, String> {
        let name = self.parse_identifier()?;
        
        let data_type = self.parse_data_type()?;
        
        let mut constraints = Vec::new();
        while self.peek_is_keyword() {
            match self.peek_token()? {
                TokenType::NOT => {
                    self.advance();
                    self.expect(TokenType::NULL)?;
                    constraints.push(ColumnConstraint::NotNull);
                }
                TokenType::PRIMARY => {
                    self.advance();
                    self.expect(TokenType::KEY)?;
                    constraints.push(ColumnConstraint::PrimaryKey);
                }
                TokenType::UNIQUE => {
                    self.advance();
                    constraints.push(ColumnConstraint::Unique);
                }
                TokenType::DEFAULT => {
                    self.advance();
                    let default_val = self.parse_expr()?;
                    constraints.push(ColumnConstraint::Default(default_val));
                }
                TokenType::CHECK => {
                    self.advance();
                    self.expect(TokenType::LParen)?;
                    let check_expr = self.parse_expr()?;
                    self.expect(TokenType::RParen)?;
                    constraints.push(ColumnConstraint::Check(check_expr));
                }
                _ => break,
            }
        }
        
        Ok(ColumnDef {
            name,
            data_type,
            constraints,
        })
    }

    fn parse_data_type(&mut self) -> Result<DataType, String> {
        let dtype = match self.peek_token()? {
            TokenType::Identifier(ref s) => s.clone(),
            other => return Err(format!("Expected data type, got {:?}", other)),
        };
        
        self.advance();
        
        match dtype.to_uppercase().as_str() {
            "INT" | "INTEGER" | "BIGINT" | "SMALLINT" | "TINYINT" => Ok(DataType::Integer),
            "FLOAT" | "DOUBLE" | "REAL" | "DECIMAL" | "NUMERIC" => Ok(DataType::Float),
            "VARCHAR" | "CHAR" | "TEXT" | "STRING" => Ok(DataType::Text),
            "BOOLEAN" | "BOOL" => Ok(DataType::Boolean),
            "DATE" => Ok(DataType::Date),
            "TIMESTAMP" => Ok(DataType::Timestamp),
            "BINARY" | "VARBINARY" => Ok(DataType::Binary),
            _ => Ok(DataType::Custom(dtype)),
        }
    }

    fn parse_assignment_list(&mut self) -> Result<Vec<Assignment>, String> {
        let mut assignments = Vec::new();
        
        loop {
            let column = self.parse_identifier()?;
            self.expect(TokenType::Equal)?;
            let value = self.parse_expr()?;
            assignments.push(Assignment { column, value });
            
            if !self.peek_is(TokenType::Comma)? {
                break;
            }
            self.expect(TokenType::Comma)?;
        }
        
        Ok(assignments)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_and_expr()?;
        
        while self.peek_is(TokenType::OR)? {
            self.advance();
            let right = self.parse_and_expr()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn parse_and_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison_expr()?;
        
        while self.peek_is(TokenType::AND)? {
            self.advance();
            let right = self.parse_comparison_expr()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn parse_comparison_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary_expr()?;
        
        loop {
            match self.peek_token()? {
                TokenType::Equal => {
                    self.advance();
                    let right = self.parse_unary_expr()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::Equal,
                        right: Box::new(right),
                    };
                }
                TokenType::NotEqual => {
                    self.advance();
                    let right = self.parse_unary_expr()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::NotEqual,
                        right: Box::new(right),
                    };
                }
                TokenType::Less => {
                    self.advance();
                    let right = self.parse_unary_expr()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::Less,
                        right: Box::new(right),
                    };
                }
                TokenType::LessEqual => {
                    self.advance();
                    let right = self.parse_unary_expr()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::LessEqual,
                        right: Box::new(right),
                    };
                }
                TokenType::Greater => {
                    self.advance();
                    let right = self.parse_unary_expr()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::Greater,
                        right: Box::new(right),
                    };
                }
                TokenType::GreaterEqual => {
                    self.advance();
                    let right = self.parse_unary_expr()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::GreaterEqual,
                        right: Box::new(right),
                    };
                }
                TokenType::IS => {
                    self.advance();
                    let negated = self.peek_is(TokenType::NOT)?;
                    if negated {
                        self.advance();
                    }
                    if self.peek_is(TokenType::NULL)? {
                        self.advance();
                        expr = Expr::IsNull {
                            expr: Box::new(expr),
                            negated,
                        };
                    } else {
                        return Err("Expected NULL after IS".to_string());
                    }
                }
                TokenType::NOT => {
                    self.advance();
                    if self.peek_is(TokenType::IN)? {
                        self.advance();
                        self.expect(TokenType::LParen)?;
                        let list = self.parse_expr_list()?;
                        self.expect(TokenType::RParen)?;
                        expr = Expr::InList {
                            expr: Box::new(expr),
                            list,
                            negated: true,
                        };
                    } else if self.peek_is(TokenType::BETWEEN)? {
                        self.advance();
                        let low = self.parse_unary_expr()?;
                        self.expect(TokenType::AND)?;
                        let high = self.parse_unary_expr()?;
                        expr = Expr::Between {
                            expr: Box::new(expr),
                            low: Box::new(low),
                            high: Box::new(high),
                            negated: true,
                        };
                    } else {
                        let right = self.parse_unary_expr()?;
                        expr = Expr::Binary {
                            left: Box::new(expr),
                            op: BinaryOp::Equal,
                            right: Box::new(Expr::Unary {
                                op: UnaryOp::Not,
                                expr: Box::new(right),
                            }),
                        };
                    }
                }
                TokenType::IN => {
                    self.advance();
                    self.expect(TokenType::LParen)?;
                    let list = self.parse_expr_list()?;
                    self.expect(TokenType::RParen)?;
                    expr = Expr::InList {
                        expr: Box::new(expr),
                        list,
                        negated: false,
                    };
                }
                TokenType::BETWEEN => {
                    self.advance();
                    let low = self.parse_unary_expr()?;
                    self.expect(TokenType::AND)?;
                    let high = self.parse_unary_expr()?;
                    expr = Expr::Between {
                        expr: Box::new(expr),
                        low: Box::new(low),
                        high: Box::new(high),
                        negated: false,
                    };
                }
                TokenType::LIKE => {
                    self.advance();
                    let pattern = self.parse_unary_expr()?;
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinaryOp::Like,
                        right: Box::new(pattern),
                    };
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }

    fn parse_unary_expr(&mut self) -> Result<Expr, String> {
        match self.peek_token()? {
            TokenType::Not => {
                self.advance();
                let expr = self.parse_unary_expr()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            TokenType::Minus => {
                self.advance();
                let expr = self.parse_unary_expr()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Negative,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_primary_expr(),
        }
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, String> {
        match self.peek_token()? {
            TokenType::NumberLiteral(n) => {
                self.advance();
                Ok(Expr::Literal(Literal::Number(n)))
            }
            TokenType::StringLiteral(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            TokenType::Star => {
                self.advance();
                Ok(Expr::Identifier("*".to_string()))
            }
            TokenType::Identifier(ref name) => {
                self.advance();
                
                // Check for function call
                if self.peek_is(TokenType::LParen)? {
                    let args = self.parse_expr_list()?;
                    Ok(Expr::FunctionCall {
                        name: name.clone(),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier(name.clone()))
                }
            }
            TokenType::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenType::RParen)?;
                Ok(expr)
            }
            other => Err(format!("Unexpected token in expression: {:?}", other)),
        }
    }

    fn parse_expr_list(&mut self) -> Result<Vec<Expr>, String> {
        let mut list = Vec::new();
        
        loop {
            let expr = self.parse_expr()?;
            list.push(expr);
            
            if !self.peek_is(TokenType::Comma)? {
                break;
            }
            self.expect(TokenType::Comma)?;
        }
        
        Ok(list)
    }

    fn parse_order_by_list(&mut self) -> Result<Vec<OrderByClause>, String> {
        let mut clauses = Vec::new();
        
        loop {
            let expr = self.parse_expr()?;
            let direction = if self.peek_is(TokenType::ASC)? {
                self.advance();
                OrderDirection::Asc
            } else if self.peek_is(TokenType::DESC)? {
                self.advance();
                OrderDirection::Desc
            } else {
                OrderDirection::Asc
            };
            
            clauses.push(OrderByClause { expr, direction });
            
            if !self.peek_is(TokenType::Comma)? {
                break;
            }
            self.expect(TokenType::Comma)?;
        }
        
        Ok(clauses)
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        match self.peek_token()? {
            TokenType::Identifier(ref s) => {
                self.advance();
                Ok(s.clone())
            }
            _ => Err("Expected identifier".to_string()),
        }
    }

    fn parse_identifier_list(&mut self) -> Result<Vec<String>, String> {
        let mut list = Vec::new();
        
        loop {
            list.push(self.parse_identifier()?);
            
            if !self.peek_is(TokenType::Comma)? {
                break;
            }
            self.expect(TokenType::Comma)?;
        }
        
        Ok(list)
    }

    fn parse_number(&mut self) -> Result<usize, String> {
        match self.peek_token()? {
            TokenType::NumberLiteral(n) => {
                self.advance();
                Ok(n as usize)
            }
            _ => Err("Expected number".to_string()),
        }
    }

    // Helper methods
    fn peek_token(&self) -> Result<TokenType, String> {
        self.tokens.get(self.current).cloned().ok_or_else(|| "Unexpected EOF".to_string())
    }

    fn peek_is(&self, token: TokenType) -> Result<bool, String> {
        Ok(self.peek_token()? == token)
    }

    fn peek_is_keyword(&self) -> bool {
        self.peek_token().map(|t| t.is_keyword()).unwrap_or(false)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn expect(&mut self, expected: TokenType) -> Result<(), String> {
        let actual = self.peek_token()?;
        if actual == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, actual))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_basic() {
        let input = "SELECT * FROM users WHERE id = 1";
        let mut tokenizer = Tokenizer::new(input);
        
        assert_eq!(tokenizer.next_token(), TokenType::SELECT);
        assert_eq!(tokenizer.next_token(), TokenType::Star);
        assert_eq!(tokenizer.next_token(), TokenType::FROM);
        assert_eq!(tokenizer.next_token(), TokenType::Identifier("users".to_string()));
        assert_eq!(tokenizer.next_token(), TokenType::WHERE);
        assert_eq!(tokenizer.next_token(), TokenType::Identifier("id".to_string()));
        assert_eq!(tokenizer.next_token(), TokenType::Equal);
        assert_eq!(tokenizer.next_token(), TokenType::NumberLiteral(1.0));
        assert_eq!(tokenizer.next_token(), TokenType::EOF);
    }

    #[test]
    fn test_parser_select() {
        let input = "SELECT id, name FROM users WHERE id = 1";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = std::iter::from_fn(|| Some(tokenizer.next_token())).collect::<Vec<_>>();
        
        let mut parser = Parser::new(tokens);
        let query = parser.parse();
        
        assert!(query.is_ok());
    }

    #[test]
    fn test_parser_insert() {
        let input = "INSERT INTO users (id, name) VALUES (1, 'Alice')";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = std::iter::from_fn(|| Some(tokenizer.next_token())).collect::<Vec<_>>();
        
        let mut parser = Parser::new(tokens);
        let query = parser.parse();
        
        assert!(query.is_ok());
    }

    #[test]
    fn test_parser_create_table() {
        let input = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = std::iter::from_fn(|| Some(tokenizer.next_token())).collect::<Vec<_>>();
        
        let mut parser = Parser::new(tokens);
        let query = parser.parse();
        
        assert!(query.is_ok());
    }
}
