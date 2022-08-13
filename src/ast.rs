use crate::{interner::Symbol, span::Span};

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: Symbol,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub name: Ident,
    pub fields: Vec<(Ident, Type)>,
}

#[derive(Debug, Clone)]
pub enum NumericType {
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
}

#[derive(Debug, Clone)]
pub enum Type {
    Numeric(NumericType),
    Char,
    Bool,
    Record(Record),
}
