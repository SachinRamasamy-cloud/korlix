use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    // ── Keywords ──────────────────────────────────────────────────────
    Import,
    From,
    App,
    Page,
    Layout,
    Component,
    Prop,
    State,
    Let,
    Derived,
    Store,
    Data,
    Action,
    If,
    Else,
    For,
    In,
    Meta,
    Theme,
    Mount,
    To,
    Route,
    Slot,
    Providers,
    Routes,
    HttpMethod(String),

    // ── Literals ──────────────────────────────────────────────────────
    Ident(String),
    Class(String), // .flex .bg-blue-500 .hover:bg-red
    StringLit(String),
    Number(f64),
    Bool(bool),
    Null,

    // ── Punctuation ───────────────────────────────────────────────────
    Colon,     // :
    Dot,       // .
    Comma,     // ,
    Equals,    // =
    Arrow,     // =>
    Bang,      // !
    Question,  // ?
    Semicolon, // ;

    // ── Operators ─────────────────────────────────────────────────────
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    AmpAmp,
    PipePipe,

    // ── Brackets ──────────────────────────────────────────────────────
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    // ── Events ────────────────────────────────────────────────────────
    OnEvent(String), // on:click, on:input, on:submit …

    // ── Indentation ───────────────────────────────────────────────────
    Indent,
    Dedent,
    Newline,
    Eof,


    // ── Api ───────────────────────────────────────────────────────
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
    Reload,
}

impl TokenKind {
    pub fn is_ident_like(&self) -> bool {
        matches!(
            self,
            TokenKind::Ident(_)
                | TokenKind::App
                | TokenKind::Page
                | TokenKind::Layout
                | TokenKind::Component
                | TokenKind::Prop
                | TokenKind::State
                | TokenKind::Let
                | TokenKind::Derived
                | TokenKind::Store
                | TokenKind::Data
                | TokenKind::Action
                | TokenKind::If
                | TokenKind::Else
                | TokenKind::For
                | TokenKind::In
                | TokenKind::Meta
                | TokenKind::Theme
                | TokenKind::Mount
                | TokenKind::To
                | TokenKind::Route
                | TokenKind::Slot
                | TokenKind::Providers
                | TokenKind::Routes
        )
    }

    pub fn as_ident_str(&self) -> Option<&str> {
        match self {
            TokenKind::Ident(s) => Some(s),
            TokenKind::App => Some("app"),
            TokenKind::Page => Some("page"),
            TokenKind::Layout => Some("layout"),
            TokenKind::Component => Some("component"),
            TokenKind::Prop => Some("prop"),
            TokenKind::State => Some("state"),
            TokenKind::Let => Some("let"),
            TokenKind::Derived => Some("derived"),
            TokenKind::Store => Some("store"),
            TokenKind::Data => Some("data"),
            TokenKind::Action => Some("action"),
            TokenKind::If => Some("if"),
            TokenKind::Else => Some("else"),
            TokenKind::For => Some("for"),
            TokenKind::In => Some("in"),
            TokenKind::Meta => Some("meta"),
            TokenKind::Theme => Some("theme"),
            TokenKind::Mount => Some("mount"),
            TokenKind::To => Some("to"),
            TokenKind::Route => Some("route"),
            TokenKind::Slot => Some("slot"),
            TokenKind::Providers => Some("providers"),
            TokenKind::Routes => Some("routes"),
            TokenKind::HttpMethod(s) => Some(s),
            _ => None,
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Ident(s) => write!(f, "{}", s),
            TokenKind::Class(s) => write!(f, ".{}", s),
            TokenKind::StringLit(s) => write!(f, "\"{}\"", s),
            TokenKind::Number(n) => write!(f, "{}", n),
            TokenKind::Bool(b) => write!(f, "{}", b),
            TokenKind::Null => write!(f, "null"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Equals => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::EqEq => write!(f, "=="),
            TokenKind::BangEq => write!(f, "!="),
            TokenKind::OnEvent(e) => write!(f, "on:{}", e),
            TokenKind::Indent => write!(f, "INDENT"),
            TokenKind::Dedent => write!(f, "DEDENT"),
            TokenKind::Newline => write!(f, "NEWLINE"),
            TokenKind::Eof => write!(f, "EOF"),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: korlix_core::Span,
    pub raw: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: korlix_core::Span, raw: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            raw: raw.into(),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
