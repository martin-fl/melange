use crate::{
    ast::{Ident, Record},
    lexer::TokenStream,
    token::{Kind, Token},
};

#[derive(Clone, Copy, Debug)]
enum Error {
    UnexpectedToken,
}

type Result<T> = std::result::Result<T, Error>;

trait Parse: Sized {
    fn parse(input: &mut TokenStream) -> Result<Self>;
}

trait ParserExtension {
    fn parse(&mut self, kind: Kind) -> Result<Token>;
}

impl ParserExtension for TokenStream<'_> {
    fn parse(&mut self, kind: Kind) -> Result<Token> {
        self.next_if_is(Kind::Ident).ok_or(Error::UnexpectedToken)
    }
}

impl Parse for Ident {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        input.parse(Kind::Ident).map(|t| Ident {
            name: t.lexeme().unwrap(),
            span: t.span(),
        })
    }
}

impl Parse for Record {
    fn parse(input: &mut TokenStream) -> Result<Self> {
        input.parse(Kind::Record)?;
        Err(Error::UnexpectedToken)
    }
}
