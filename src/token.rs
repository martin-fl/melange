use crate::interner::{get, Symbol};
use crate::span::Span;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    // 1 symbol tokens
    LBrack,
    RBrack,
    LParen,
    RParen,
    Dot,
    Comma,
    Semi,
    Vert,
    Tilde,
    Star,
    Amp,
    Plus,
    Hash,

    // Ambiguous tokens
    Colon,
    Underscore,
    Minus,

    Eq,
    Lt,
    Gt,
    Slash,

    // 2 symbols tokens
    LtEq,
    GtEq,
    Neq,
    ColonEq,
    LArrow,
    RArrow,
    RFatArrow,

    // Literals
    IntLit,
    FloatLit,
    ComplexLit,
    CharLit,
    StrLit,
    Ident,

    // Keywords
    Char,
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    C32,
    C64,
    Raw,
    Type,
    Pub,
    Let,
    Mut,
    Fun,
    Begin,
    End,
    Impl,
    If,
    Is,
    Then,
    Elif,
    Else,
    Match,
    With,
    Loop,
    For,
    In,
    Do,
}

impl Kind {
    pub fn repr(self) -> Option<&'static str> {
        match self {
            Kind::IntLit
            | Kind::FloatLit
            | Kind::ComplexLit
            | Kind::CharLit
            | Kind::StrLit
            | Kind::Ident => None,
            Kind::LBrack => Some("["),
            Kind::RBrack => Some("]"),
            Kind::LParen => Some("("),
            Kind::RParen => Some(")"),
            Kind::Dot => Some("."),
            Kind::Comma => Some(","),
            Kind::Semi => Some(";"),
            Kind::Vert => Some("|"),
            Kind::Tilde => Some("~"),
            Kind::Star => Some("*"),
            Kind::Amp => Some("&"),
            Kind::Plus => Some("+"),
            Kind::Hash => Some("#"),
            Kind::Colon => Some(":"),
            Kind::Underscore => Some("_"),
            Kind::Minus => Some("-"),
            Kind::Eq => Some("="),
            Kind::Lt => Some("<"),
            Kind::Gt => Some(">"),
            Kind::Slash => Some("/"),
            Kind::LtEq => Some("<="),
            Kind::GtEq => Some(">="),
            Kind::Neq => Some("/="),
            Kind::ColonEq => Some(":="),
            Kind::LArrow => Some("->"),
            Kind::RArrow => Some("<-"),
            Kind::RFatArrow => Some("=>"),
            Kind::Char => Some("char"),
            Kind::Bool => Some("bool"),
            Kind::U8 => Some("u8"),
            Kind::U16 => Some("u16"),
            Kind::U32 => Some("u32"),
            Kind::U64 => Some("u64"),
            Kind::I8 => Some("i8"),
            Kind::I16 => Some("i16"),
            Kind::I32 => Some("i32"),
            Kind::I64 => Some("i64"),
            Kind::F32 => Some("f32"),
            Kind::F64 => Some("f64"),
            Kind::C32 => Some("c32"),
            Kind::C64 => Some("c64"),
            Kind::Raw => Some("raw"),
            Kind::Type => Some("type"),
            Kind::Pub => Some("pub"),
            Kind::Let => Some("let"),
            Kind::Mut => Some("mut"),
            Kind::Fun => Some("fun"),
            Kind::Begin => Some("begin"),
            Kind::End => Some("end"),
            Kind::Impl => Some("impl"),
            Kind::If => Some("if"),
            Kind::Is => Some("is"),
            Kind::Then => Some("then"),
            Kind::Elif => Some("elif"),
            Kind::Else => Some("else"),
            Kind::Match => Some("match"),
            Kind::With => Some("with"),
            Kind::Loop => Some("loop"),
            Kind::For => Some("for"),
            Kind::In => Some("in"),
            Kind::Do => Some("do"),
        }
    }

    pub fn ident_or_kw(s: &str) -> Self {
        use Kind::*;
        match s {
            "char" => Char,
            "bool" => Bool,
            "u8" => U8,
            "u16" => U16,
            "u32" => U32,
            "u64" => U64,
            "i8" => I8,
            "i16" => I16,
            "i32" => I32,
            "i64" => I64,
            "f32" => F32,
            "f64" => F64,
            "c32" => C32,
            "c64" => C64,
            "raw" => Raw,
            "type" => Type,
            "pub" => Pub,
            "let" => Let,
            "mut" => Mut,
            "fun" => Fun,
            "begin" => Begin,
            "end" => End,
            "impl" => Impl,
            "if" => If,
            "is" => Is,
            "then" => Then,
            "elif" => Elif,
            "else" => Else,
            "match" => Match,
            "with" => With,
            "loop" => Loop,
            "for" => For,
            "in" => In,
            "do" => Do,
            _ => Ident,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token {
    kind: Kind,
    span: Span,
    lexeme: Option<Symbol>,
}

impl Token {
    pub fn new(kind: Kind, span: Span, lexeme: Option<Symbol>) -> Self {
        Self { kind, span, lexeme }
    }

    pub fn kind(self) -> Kind {
        self.kind
    }

    pub fn span(self) -> Span {
        self.span
    }

    pub fn lexeme(self) -> Option<Symbol> {
        self.lexeme
    }

    pub fn repr(self) -> &'static str {
        self.kind().repr().unwrap_or_else(|| {
            get(self.lexeme().expect(&format!(
                "token '{:?}' should have a lexeme but doesn't",
                self.kind(),
            )))
        })
    }
}
