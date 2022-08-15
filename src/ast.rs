mod pretty;

use crate::{interner::Symbol, span::Span, token::Token};

#[derive(Clone, Debug)]
pub struct Ident {
    pub span: Span,
    pub name: Symbol,
}

impl Ident {
    pub fn new(span: Span, name: Symbol) -> Self {
        Self { span, name }
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    pub span: Span,
    pub segments: Vec<Ident>,
}

impl Path {
    pub fn new(span: Span, segments: Vec<Ident>) -> Self {
        Self { span, segments }
    }
}

#[derive(Clone, Debug)]
pub struct Tuple {
    pub span: Span,
    pub types: Vec<Ty>,
}

impl Tuple {
    pub fn new(span: Span, types: Vec<Ty>) -> Self {
        Self { span, types }
    }
}

#[derive(Clone, Debug)]
pub struct Array {
    pub span: Span,
    pub ty: Box<Ty>,
    pub len: Token,
}

impl Array {
    pub fn new(span: Span, ty: Box<Ty>, len: Token) -> Self {
        Self { span, ty, len }
    }
}

#[derive(Clone, Debug)]
pub enum TyKind {
    Path(Path),
    Tuple(Tuple),
    Array(Array),
}

#[derive(Clone, Debug)]
pub struct Ty {
    pub span: Span,
    pub kind: TyKind,
}

impl Ty {
    pub fn new(span: Span, kind: TyKind) -> Self {
        Self { span, kind }
    }
}

#[derive(Clone, Debug)]
pub struct Variant {
    pub span: Span,
    pub name: Ident,
    pub data: Vec<Ty>,
}

impl Variant {
    pub fn new(span: Span, name: Ident, data: Vec<Ty>) -> Self {
        Self { span, name, data }
    }
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub span: Span,
    pub variants: Vec<Variant>,
}

impl Enum {
    pub fn new(span: Span, variants: Vec<Variant>) -> Self {
        Self { span, variants }
    }
}

#[derive(Clone, Debug)]
pub struct Param {
    pub span: Span,
    pub name: Ident,
    pub ty: Ty,
}

impl Param {
    pub fn new(span: Span, name: Ident, ty: Ty) -> Self {
        Self { span, name, ty }
    }
}

#[derive(Clone, Debug)]
pub struct Record {
    pub span: Span,
    pub fields: Vec<Param>,
}

impl Record {
    pub fn new(span: Span, fields: Vec<Param>) -> Self {
        Self { span, fields }
    }
}

#[derive(Clone, Debug)]
pub enum TyDefKind {
    Alias(Ty),
    Record(Record),
    Enum(Enum),
}

#[derive(Clone, Debug)]
pub struct TyDef {
    pub span: Span,
    pub name: Ident,
    pub ty: TyDefKind,
}

impl TyDef {
    pub fn new(span: Span, name: Ident, ty: TyDefKind) -> Self {
        Self { span, name, ty }
    }
}
