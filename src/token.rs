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
    Module,
    Import,
    Root,
    When,
}

impl Kind {
    pub fn repr(self) -> Option<&'static str> {
        use Kind::*;
        match self {
            IntLit | FloatLit | ComplexLit | CharLit | StrLit | Ident => None,
            LBrack => Some("["),
            RBrack => Some("]"),
            LParen => Some("("),
            RParen => Some(")"),
            Dot => Some("."),
            Comma => Some(","),
            Semi => Some(";"),
            Vert => Some("|"),
            Tilde => Some("~"),
            Star => Some("*"),
            Amp => Some("&"),
            Plus => Some("+"),
            Hash => Some("#"),
            Colon => Some(":"),
            Underscore => Some("_"),
            Minus => Some("-"),
            Eq => Some("="),
            Lt => Some("<"),
            Gt => Some(">"),
            Slash => Some("/"),
            LtEq => Some("<="),
            GtEq => Some(">="),
            Neq => Some("/="),
            ColonEq => Some(":="),
            LArrow => Some("->"),
            RArrow => Some("<-"),
            RFatArrow => Some("=>"),
            Char => Some("char"),
            Bool => Some("bool"),
            U8 => Some("u8"),
            U16 => Some("u16"),
            U32 => Some("u32"),
            U64 => Some("u64"),
            I8 => Some("i8"),
            I16 => Some("i16"),
            I32 => Some("i32"),
            I64 => Some("i64"),
            F32 => Some("f32"),
            F64 => Some("f64"),
            C32 => Some("c32"),
            C64 => Some("c64"),
            Raw => Some("raw"),
            Type => Some("type"),
            Pub => Some("pub"),
            Let => Some("let"),
            Mut => Some("mut"),
            Fun => Some("fun"),
            Begin => Some("begin"),
            End => Some("end"),
            Impl => Some("impl"),
            If => Some("if"),
            Is => Some("is"),
            Then => Some("then"),
            Elif => Some("elif"),
            Else => Some("else"),
            Match => Some("match"),
            With => Some("with"),
            Loop => Some("loop"),
            For => Some("for"),
            In => Some("in"),
            Do => Some("do"),
            Module => Some("module"),
            Import => Some("import"),
            Root => Some("root"),
            When => Some("when"),
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
            "module" => Module,
            "import" => Import,
            "root" => Root,
            "when" => When,
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
