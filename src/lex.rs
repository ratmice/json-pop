use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(trivia = r"[\p{Whitespace}]+")]
pub enum Token {
    // Logos requires that we define two default variants,
    // one for end of input source,
    #[end]
    End,

    #[error]
    Error,

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
    #[regex = r"-?([0-9]|([1-9][0-9]*))((\.[0-9]+)?)([eE][+-]?[0-9]+)?"]
    Number,

    #[regex = r#""([ -!#-\[\]-\x{10ffff}]|([\\](["\\/bfnrt]|[u][[:xdigit:]][[:xdigit:]][[:xdigit:]][[:xdigit:]])))*""#]
    String,
}

pub mod wrap {
    use crate::lex;
    use logos::Logos;
    use std::fmt;
    use std::ops::Range;

    #[derive(Debug, Clone)]
    pub enum Wrap<'source> {
        Token {
            tok: lex::Token,
            string: &'source str,
        },
    }

    impl<'source> fmt::Display for Wrap<'source> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:#?}", self)
        }
    }

    pub struct Tokens<'source> {
        logos: logos::Lexer<lex::Token, &'source str>,
    }

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
            let logos = lex::Token::lexer(source);
            Tokens { logos }
        }
    }

    impl<'source> Iterator for Tokens<'source> {
        type Item = Spanned<Wrap<'source>, usize, Error>;

        fn next(&mut self) -> Option<Spanned<self::Wrap<'source>, usize, Error>> {
            let range = self.logos.range();
            let tok = self.logos.token;
            let string = self.logos.slice();
            if tok == lex::Token::End {
                return None;
            }
            self.logos.advance();
            Some(Ok((range.start, Wrap::Token { tok, string }, range.end)))
        }
    }
}
