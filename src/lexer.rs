use crate::interner::intern;
use crate::span::Span;
use crate::token::{Kind, Token};
use core::str::Chars;
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct Scanner<'src> {
    stream: Peekable<Chars<'src>>,
    start: usize,
    pos: usize,
}

#[derive(Debug)]
enum ScanError {
    NotCharLiteral,
    UnterminatedCharLiteral,
    UnterminatedStrLiteral,
}

impl<'src> Scanner<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            stream: src.chars().peekable(),
            start: 0,
            pos: 0,
        }
    }

    fn make_span(&mut self) -> Span {
        Span::new(self.start, self.pos - self.start)
    }

    fn nextc(&mut self) -> Option<char> {
        self.stream.next().map(|c| {
            self.pos += c.len_utf8();
            c
        })
    }

    fn nextc_if(&mut self, f: impl Fn(char) -> bool) -> Option<char> {
        self.stream.next_if(|&c| f(c)).map(|c| {
            self.pos += c.len_utf8();
            c
        })
    }

    fn nextc_while(&mut self, f: impl Fn(char) -> bool) -> String {
        let mut buf = String::new();

        while let Some(c) = self.nextc_if(&f) {
            buf.push(c)
        }

        buf
    }

    fn skip_while(&mut self, f: impl Fn(char) -> bool) {
        while let Some(_) = self.nextc_if(&f) {}
    }

    fn skip_whitespace(&mut self) {
        self.skip_while(|c| c.is_whitespace())
    }

    fn peekc(&mut self) -> Option<char> {
        self.stream.peek().copied()
    }

    fn scan_number(&mut self, n: char) -> Token {
        let mut number = self.nextc_while(char::is_numeric);
        number.insert(0, n);

        let mut kind = Kind::IntLit;

        match self.peekc() {
            Some('.') => {
                let mut stream = self.clone();
                stream.nextc();
                let decimal = stream.nextc_while(char::is_numeric);
                if decimal.len() > 0 {
                    kind = Kind::FloatLit;
                    number.push_str(&decimal);
                    *self = stream;
                }
            }
            _ => {}
        }

        return Token::new(kind, self.make_span(), Some(intern(number)));
    }

    fn scan_char(&mut self) -> Result<Token, ScanError> {
        let c = self.nextc_while(|c| c != '\'');
        if (c.len() == 2 && !c.starts_with('\\')) || c.len() >= 2 {
            Err(ScanError::NotCharLiteral)
        } else if self.peekc().is_none() {
            Err(ScanError::UnterminatedCharLiteral)
        } else {
            Ok(Token::new(Kind::CharLit, self.make_span(), Some(intern(c))))
        }
    }

    fn scan_str(&mut self) -> Result<Token, ScanError> {
        let s = self.nextc_while(|c| c != '\"');
        if self.peekc().is_none() {
            Err(ScanError::UnterminatedStrLiteral)
        } else {
            Ok(Token::new(Kind::StrLit, self.make_span(), Some(intern(s))))
        }
    }

    fn scan_ident(&mut self, c: char) -> Token {
        let mut ident = self.nextc_while(is_ident);
        ident.insert(0, c);

        Token::new(
            Kind::ident_or_kw(&ident),
            self.make_span(),
            Some(intern(ident)),
        )
    }

    pub fn scan(&mut self) -> Option<Token> {
        self.skip_whitespace();

        macro_rules! tok {
            ($kind:expr) => {
                Token::new($kind, self.make_span(), None)
            };
        }

        self.start = self.pos;
        self.nextc().map(|c| match c {
            '[' => tok!(Kind::LBrack),
            ']' => tok!(Kind::RBrack),
            '(' => tok!(Kind::LParen),
            ')' => tok!(Kind::RParen),
            '.' => tok!(Kind::Dot),
            ',' => tok!(Kind::Comma),
            ';' => tok!(Kind::Semi),
            '|' => tok!(Kind::Vert),
            '~' => tok!(Kind::Tilde),
            '*' => tok!(Kind::Star),
            '&' => tok!(Kind::Amp),
            '+' => tok!(Kind::Plus),
            '#' => tok!(Kind::Hash),

            ':' => match self.peekc() {
                Some('=') => {
                    self.nextc();
                    tok!(Kind::ColonEq)
                }
                _ => tok!(Kind::Colon),
            },

            '-' => match self.peekc() {
                Some('>') => {
                    self.nextc();
                    tok!(Kind::RArrow)
                }
                _ => tok!(Kind::Minus),
            },

            '=' => match self.peekc() {
                Some('>') => {
                    self.nextc();
                    tok!(Kind::RFatArrow)
                }
                _ => tok!(Kind::Eq),
            },

            '<' => match self.peekc() {
                Some('-') => {
                    self.nextc();
                    tok!(Kind::LArrow)
                }
                Some('=') => {
                    self.nextc();
                    tok!(Kind::LtEq)
                }
                _ => tok!(Kind::Lt),
            },

            '>' => match self.peekc() {
                Some('=') => {
                    self.nextc();
                    tok!(Kind::GtEq)
                }
                _ => tok!(Kind::Gt),
            },

            '/' => match self.peekc() {
                Some('=') => {
                    self.nextc();
                    tok!(Kind::Neq)
                }
                _ => tok!(Kind::Slash),
            },

            '_' => match self.peekc() {
                Some(c) if is_ident(c) => self.scan_ident('_'),
                _ => tok!(Kind::Underscore),
            },

            '"' => self.scan_str().expect("couldn't scan string literal"),
            '\'' => self.scan_char().expect("couldn't scan char literal"),
            c if c.is_numeric() => self.scan_number(c),
            c => self.scan_ident(c),
        })
    }

    pub fn stream(&'src mut self) -> impl Iterator<Item = Token> + 'src {
        core::iter::from_fn(|| self.scan())
    }
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
