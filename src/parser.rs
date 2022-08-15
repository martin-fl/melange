use crate::{
    ast::{Array, Enum, Ident, Param, Path, Record, Tuple, Ty, TyDef, TyDefKind, TyKind, Variant},
    lexer::TokenStream,
    span::Span,
    token::{Kind, Token},
};

#[derive(Clone, Copy, Debug)]
pub enum Error {
    UnexpectedToken,
    MissingToken,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Parse: Sized {
    fn parse(input: &mut TokenStream) -> Result<Self>;

    fn parse_separated(sep: Kind, input: &mut TokenStream) -> Result<Vec<Self>> {
        let mut list = Vec::new();

        loop {
            list.push(Self::parse(input)?);

            if input.eat(sep).is_err() {
                break;
            }
        }

        Ok(list)
    }
}

trait ParserExtension {
    fn eat(&mut self, kind: Kind) -> Result<Token>;
    fn eat_if(&mut self, f: impl Fn(Kind) -> bool) -> Result<Token>;
    fn eat_from(&mut self, kinds: &[Kind]) -> Result<Token>;
}

impl ParserExtension for TokenStream<'_> {
    fn eat(&mut self, kind: Kind) -> Result<Token> {
        self.next_if_is(kind).ok_or(Error::UnexpectedToken)
    }

    fn eat_if(&mut self, f: impl Fn(Kind) -> bool) -> Result<Token> {
        self.next_if(|t| f(t.kind())).ok_or(Error::UnexpectedToken)
    }

    fn eat_from(&mut self, kinds: &[Kind]) -> Result<Token> {
        self.eat_if(|tk| kinds.iter().any(|&k| k == tk))
    }
}

impl Parse for Ident {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        input
            .eat_if(|k| k == Kind::Ident || k.is_kw())
            .map(|t| Ident::new(t.span(), t.symbol().unwrap()))
    }
}

impl Parse for Path {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        Ident::parse_separated(Kind::Tilde, input).map(|segments| {
            Path::new(
                Span::combine(
                    segments.first().unwrap().span,
                    segments.last().unwrap().span,
                ),
                segments,
            )
        })
    }
}

impl Parse for Tuple {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        let s1 = input.eat(Kind::LParen)?.span();
        let res = Ty::parse_separated(Kind::Comma, input)?;
        let s2 = input.eat(Kind::RParen)?.span();
        Ok(Tuple::new(Span::combine(s1, s2), res))
    }
}

impl Parse for Array {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        let s1 = input.eat(Kind::LBrack)?.span();
        let ty = Ty::parse(input).map(Box::new)?;
        input.eat(Kind::Semi)?;
        let len = input.eat(Kind::IntLit)?;
        let s2 = input.eat(Kind::RBrack)?.span();
        Ok(Array::new(Span::combine(s1, s2), ty, len))
    }
}

impl Parse for TyKind {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        match input.peek().ok_or(Error::MissingToken)?.kind() {
            Kind::LParen => Tuple::parse(input).map(TyKind::Tuple),
            Kind::LBrack => Array::parse(input).map(TyKind::Array),
            _ => Path::parse(input).map(TyKind::Path),
        }
    }
}

impl Parse for Ty {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        let kind = TyKind::parse(input)?;
        let span = match &kind {
            TyKind::Path(p) => p.span,
            TyKind::Tuple(t) => t.span,
            TyKind::Array(a) => a.span,
        };
        Ok(Ty::new(span, kind))
    }
}

impl Parse for Variant {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        let name = Ident::parse(input)?;
        let (data, span) = if input.peek().map(|t| t.kind()) == Some(Kind::LParen) {
            let tup = Tuple::parse(input)?;
            (tup.types, tup.span)
        } else {
            (Vec::new(), name.span)
        };
        let span = Span::combine(name.span, span);
        Ok(Variant::new(span, name, data))
    }
}

impl Parse for Enum {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        let lvert = input.eat(Kind::Vert)?.span();
        let variants = Variant::parse_separated(Kind::Vert, input)?;
        let span = if !variants.is_empty() {
            Span::combine(lvert, variants.last().unwrap().span)
        } else {
            lvert
        };
        Ok(Enum::new(span, variants))
    }
}

impl Parse for Vec<Param> {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        let lparen = input.eat(Kind::LParen)?.span();
        let mut idents = Vec::new();
        while let Ok(ident) = Ident::parse(input) {
            idents.push(ident);
        }

        input.eat(Kind::Colon)?;

        let ty = Ty::parse(input)?;
        let rparen = input.eat(Kind::RParen)?.span();

        let span = Span::combine(lparen, rparen);

        Ok(idents
            .into_iter()
            .map(|i| Param::new(span, i, ty.clone()))
            .collect())
    }
}

impl Parse for Record {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        let rec = input.eat(Kind::Record)?.span();
        let mut fields = Vec::new();

        while matches!(input.peek().map(|t| t.kind()), Some(Kind::LParen)) {
            fields.append(&mut <Vec<Param>>::parse(input)?);
        }

        let span = if !fields.is_empty() {
            Span::combine(rec, fields.last().unwrap().span)
        } else {
            rec
        };

        Ok(Record::new(span, fields))
    }
}

impl Parse for TyDefKind {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        match input.peek().ok_or(Error::MissingToken)?.kind() {
            Kind::Record => Record::parse(input).map(TyDefKind::Record),
            Kind::Vert => Enum::parse(input).map(TyDefKind::Enum),
            _ => Ty::parse(input).map(TyDefKind::Alias),
        }
    }
}

impl Parse for TyDef {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        let ty_kw = input.eat(Kind::Type)?.span();
        let name = Ident::parse(input)?;
        input.eat(Kind::ColonEq)?;
        let ty = TyDefKind::parse(input)?;
        let dot = input.eat(Kind::Dot)?.span();
        let span = Span::combine(ty_kw, dot);
        Ok(TyDef::new(span, name, ty))
    }
}
