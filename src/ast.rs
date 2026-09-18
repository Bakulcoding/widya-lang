use crate::error::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate, // -
    Not,    // bukan / !
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignOp {
    Assign,       // =
    PlusAssign,   // +=
    MinusAssign,  // -=
    StarAssign,   // *=
    SlashAssign,  // /=
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignTarget {
    Variable(String),
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64, Span),
    String(String, Span),
    Boolean(bool, Span),
    Nil(Span),
    Identifier(String, Span),
    This(Span),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
        span: Span,
    },
    Grouping(Box<Expr>, Span),
    Array(Vec<Expr>, Span),
    Map(Vec<(Expr, Expr)>, Span),
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
        span: Span,
    },
    FunctionExpr {
        params: Vec<String>,
        body: Vec<Stmt>,
        span: Span,
    },
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        span: Span,
    },
    Match {
        target: Box<Expr>,
        arms: Vec<MatchArm>,
        span: Span,
    },
    Try(Box<Expr>, Span), // Rust-like '?' operator
    Await(Box<Expr>, Span), // Rust-like await (tunggu_hasil)
    VolatileRead {
        address: Box<Expr>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternalFunctionDecl {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub body: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPattern {
    Wildcard(Span),                     // _
    Literal(Expr),                      // 10, "halo", benar
    Identifier(String, Span),           // x
    EnumVariant {                       // Status::Aktif atau Status::Galat(pesan)
        enum_name: Option<String>,
        variant_name: String,
        bindings: Vec<String>,
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> &Span {
        match self {
            Expr::Number(_, span)
            | Expr::String(_, span)
            | Expr::Boolean(_, span)
            | Expr::Nil(span)
            | Expr::Identifier(_, span)
            | Expr::This(span)
            | Expr::Binary { span, .. }
            | Expr::Unary { span, .. }
            | Expr::Grouping(_, span)
            | Expr::Array(_, span)
            | Expr::Map(_, span)
            | Expr::Index { span, .. }
            | Expr::Call { span, .. }
            | Expr::FunctionExpr { span, .. }
            | Expr::Range { span, .. }
            | Expr::Match { span, .. }
            | Expr::Try(_, span)
            | Expr::Await(_, span)
            | Expr::VolatileRead { span, .. } => span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariantDecl {
    pub name: String,
    pub fields_count: usize,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructMethod {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitMethodSignature {
    pub name: String,
    pub params: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub arguments: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    VarDecl {
        name: String,
        is_const: bool,
        initializer: Option<Expr>,
        span: Span,
    },
    Assignment {
        target: AssignTarget,
        op: AssignOp,
        value: Expr,
        span: Span,
    },
    Block(Vec<Stmt>, Span),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        elif_branches: Vec<(Expr, Stmt)>,
        else_branch: Option<Box<Stmt>>,
        span: Span,
    },
    IfLet {
        pattern: MatchPattern,
        value: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
        span: Span,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
        span: Span,
    },
    ForIn {
        var_name: String,
        iterable: Expr,
        body: Box<Stmt>,
        span: Span,
    },
    Loop {
        body: Box<Stmt>,
        span: Span,
    },
    DestructureDecl {
        names: Vec<String>,
        initializer: Expr,
        is_const: bool,
        span: Span,
    },
    FunctionDecl {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        attributes: Vec<Attribute>,
        span: Span,
    },
    StructDecl {
        name: String,
        fields: Vec<String>,
        methods: Vec<StructMethod>,
        attributes: Vec<Attribute>,
        span: Span,
    },
    TraitDecl {
        name: String,
        methods: Vec<TraitMethodSignature>,
        span: Span,
    },
    ImplDecl {
        trait_name: Option<String>,
        target_name: String,
        methods: Vec<StructMethod>,
        span: Span,
    },
    EnumDecl {
        name: String,
        variants: Vec<EnumVariantDecl>,
        attributes: Vec<Attribute>,
        span: Span,
    },
    Import {
        path: String,
        alias: Option<String>,
        span: Span,
    },
    TryCatch {
        try_block: Box<Stmt>,
        error_var: String,
        catch_block: Box<Stmt>,
        span: Span,
    },
    Throw {
        expr: Expr,
        span: Span,
    },
    Return {
        value: Option<Expr>,
        span: Span,
    },
    ExternalBlock {
        abi: String,
        functions: Vec<ExternalFunctionDecl>,
        span: Span,
    },
    VolatileWrite {
        address: Expr,
        value: Expr,
        span: Span,
    },
    Break(Span),
    Continue(Span),
}

impl Stmt {
    pub fn span(&self) -> &Span {
        match self {
            Stmt::Expression(e) => e.span(),
            Stmt::VarDecl { span, .. }
            | Stmt::Assignment { span, .. }
            | Stmt::Block(_, span)
            | Stmt::If { span, .. }
            | Stmt::IfLet { span, .. }
            | Stmt::While { span, .. }
            | Stmt::ForIn { span, .. }
            | Stmt::Loop { span, .. }
            | Stmt::DestructureDecl { span, .. }
            | Stmt::FunctionDecl { span, .. }
            | Stmt::StructDecl { span, .. }
            | Stmt::TraitDecl { span, .. }
            | Stmt::ImplDecl { span, .. }
            | Stmt::EnumDecl { span, .. }
            | Stmt::Import { span, .. }
            | Stmt::TryCatch { span, .. }
            | Stmt::Throw { span, .. }
            | Stmt::Return { span, .. }
            | Stmt::ExternalBlock { span, .. }
            | Stmt::VolatileWrite { span, .. }
            | Stmt::Break(span)
            | Stmt::Continue(span) => span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Subtract => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
            BinaryOp::Modulo => write!(f, "%"),
            BinaryOp::Power => write!(f, "^"),
            BinaryOp::Equal => write!(f, "=="),
            BinaryOp::NotEqual => write!(f, "!="),
            BinaryOp::Less => write!(f, "<"),
            BinaryOp::LessEqual => write!(f, "<="),
            BinaryOp::Greater => write!(f, ">"),
            BinaryOp::GreaterEqual => write!(f, ">="),
            BinaryOp::And => write!(f, "dan"),
            BinaryOp::Or => write!(f, "atau"),
        }
    }
}
