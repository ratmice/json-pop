use logos_derive::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(trivia = "[ \r\n\t]+")]
pub enum Token<'a> {
    // Logos requires that we define two default variants,
    // one for end of input source,
    #[end]
    End,

    #[error]
    Error,

    #[token = "\""]
    Quote,

    #[token = "true"]
    True,

    #[token = "false"]
    False,

    #[token = "null"]
    Null,

    #[token = ":"]
    Colon,

    #[token = ","]
    Comma,

    #[token = "{"]
    LBrace,

    #[token = "}"]
    RBrace,

    #[token = "["]
    LBrack,

    #[token = "]"]
    RBrack,

    /* The last 2 could use some cleaning up,
       In particular matching Number as a regex... turning it into a string,
       and then doing an entire pass over the string, rather than converting
       numbers as digits match is a likely improvement.
    */
    #[regex(r#"-?([0-9]|([1-9][0-9]*))((\.[0-9]+)?)([eE][+-]?[0-9]+)?"#, |lex| lex.slice())]
    Number(&'a str),

    #[regex(r#""([ -!#-\[\]-\x{10ffff}]|([\\](["\\/bfnrt]|[u][[:xdigit:]][[:xdigit:]][[:xdigit:]][[:xdigit:]])))*""#, |lex| lex.slice())]
    String(&'a str),
}

pub mod wrap {
    use super::Token;
    use crate::lex;
    use logos::Logos;
    use std::fmt;
    use std::ops::Range;

    pub struct Tokens<'source>{lex: logos::Lexer<'source, lex::Token<'source>>}
    pub type Spanned<Tok, Loc, E> = Result<(Loc, Tok, Loc), E>;

    #[derive(Debug)]
    pub enum Error {
        LexicalError { range: Range<usize> },
        NumericalError { range: Range<usize> },
    }

    impl<'source> fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Error::LexicalError { range: _ } => write!(f, "Lexical error"),
                Error::NumericalError { range: _ } => write!(f, "Numerical conversion error"),
            }
        }
    }

    impl<'source> Tokens<'source> {
        pub fn new(source: &'source str) -> Tokens {
            let lex = lex::Token::lexer(source);
            Tokens { lex }
        }
    }

    impl<'source> Iterator for Tokens<'source> {
        type Item = Spanned<Token<'source>, usize, Error>;
        fn next(&mut self) -> Option<Self::Item> {
             self.lex.next().map(|tok| {
                let range = self.lex.range();
                if tok == Token::Error {
                    Err(Error::LexicalError {range: range })
                } else {
                    Ok((range.start, tok, range.end))
                }
             })
        }
    }
}
