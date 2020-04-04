use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum Quoted {
    #[end]
    End,

    #[error]
    Error,

    #[token = "\""]
    QuoteEnd,

    #[regex = r#"([ -!#-\[\]-\x{10ffff}]|([\\](["\\/bfnrt]|[u][[:xdigit:]][[:xdigit:]][[:xdigit:]][[:xdigit:]])))+"#]
    String,
}

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(trivia = r"[\p{Whitespace}]+")]
pub enum Token {
    // Logos requires that we define two default variants,
    // one for end of input source,
    #[end]
    End,

    #[error]
    Error,

    #[token = "\""]
    Quote,

    String,

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
}

impl Into<Token> for Quoted {
    fn into(self) -> Token {
        match self {
            Self::End => Token::End,
            Self::QuoteEnd => Token::Quote,
            Self::String => Token::String,
            Self::Error => Token::Error,
        }
    }
}

pub mod wrap {
    use super::Quoted;
    use super::Token;
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

    enum Mode<'source> {
        Quote(logos::Lexer<lex::Quoted, &'source str>),
        Top(logos::Lexer<lex::Token, &'source str>),
    }

    pub struct Tokens<'source> {
        mode: Mode<'source>,
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
            let mode = Mode::Top(lex::Token::lexer(source));
            Tokens { mode }
        }
    }

    impl<'source> Iterator for Tokens<'source> {
        type Item = Spanned<Wrap<'source>, usize, Error>;

        // The general reason we need a lexer mode is that:
        // a) we want to tell the parser about the start and end quote tokens, so that it can point to them
        // in error messages should one be missing,
        // to do that we must leave the quote symbols separate from the String regex.
        // b) once we've taken them out of the String regex, they inner string part
        // will clash with all the other Token regexes.
        //
        // A possibly better way to do this is have a stack of tokens in wrap::Tokens,
        // which next drains first before lex.advance().
        //
        // we can do this in constant space since we just need 2 extra calls to next to drain it.
        // that would reduce this to a single branch, and token type,
        // as well as get rid of having to move the lex mode, since there will only be one.
        //
        // Anyhow that is worth a try.

        fn next(&mut self) -> Option<Self::Item> {
            match &mut self.mode {
                Mode::Top(lex) => {
                    let range = lex.range();
                    let wrapped = Wrap::Token {
                        tok: lex.token,
                        string: lex.slice(),
                    };
                    let ret: Option<Self::Item> = Some(Ok((range.start, wrapped, range.end)));

                    match lex.token {
                        lex::Token::End => return None,
                        Token::Quote => {
                            self.mode = Mode::Quote(lex.to_owned().advance_as::<Quoted>())
                        }
                        _ => lex.advance(),
                    }
                    ret
                }
                Mode::Quote(lex) => {
                    let range = lex.range();
                    let result = Some(Ok((
                        range.start,
                        Wrap::Token {
                            tok: lex.token.into(),
                            string: lex.slice(),
                        },
                        range.end,
                    )));
                    match lex.token {
                        lex::Quoted::End => return None,
                        lex::Quoted::QuoteEnd => self.mode = Mode::Top(lex.to_owned().advance_as()),
                        _ => lex.advance(),
                    }
                    result
                }
            }
        }
    }
}
