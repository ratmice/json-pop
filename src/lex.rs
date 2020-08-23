use crate::error::CompilationError;
use logos::Logos;
use std::fmt;
use std::ops::Range;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum Token<'a> {
    #[regex("[ \r\n\t]+", logos::skip)]
    #[error]
    Error,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("null")]
    Null,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBrack,

    #[token("]")]
    RBrack,

    /* The last 2 could use some cleaning up,
       In particular matching Number as a regex... turning it into a string,
       and then doing an entire pass over the string, rather than converting
       numbers as digits match is a likely improvement.
    */
    #[regex(r#"-?([0-9]|([1-9][0-9]*))((\.[0-9]+)?)([eE][+-]?[0-9]+)?"#, |lex| lex.slice())]
    Number(&'a str),

    #[regex(r#""([ -!#-\[\]-\x{10ffff}]|([\\](["\\/bfnrt]|[u][[:xdigit:]][[:xdigit:]][[:xdigit:]][[:xdigit:]])))*""#)]
    String(&'a str),
    #[regex(r#""([ -!#-\[\]-\x{10ffff}]|([\\](["\\/bfnrt]|[u][[:xdigit:]][[:xdigit:]][[:xdigit:]][[:xdigit:]])))*"#)]
    MissingEndQuote(&'a str),
}

impl<'a> Token<'a> {
    pub fn to_lalr_triple(
        (t, r): (Token<'a>, Range<usize>),
    ) -> Result<(usize, Token, usize), CompilationError> {
        if t == Token::Error {
            Err(CompilationError::LexicalError { range: r })
        } else {
            Ok((r.start, t, r.end))
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
