#[cfg(feature = "pretty_errors")]
pub mod codespan;
pub mod lex;
use crate::lex::Token;
use logos::Logos;
use std::fmt;

pub mod parser {
    #![allow(clippy::all)]
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub json);
    use super::*;
    pub use json::*;

    pub type ParseError<'a> = lalrpop_util::ParseError<usize, Token<'a>, super::CompilationError>;
}
pub use lalrpop_util;

#[derive(Debug)]
pub enum CompilationError {
    LexicalError { pos: usize },
    NumericalError { pos: usize },
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

pub mod value {
    use lexical;
    use std::fmt;

    #[derive(Debug, Clone)]
    pub enum Value<'a> {
        Number(f64),
        String(&'a str),
        Object(Vec<(&'a str, Value<'a>)>),
        Bool(bool),
        Null,
        Array(Vec<Value<'a>>),
    }

    impl<'a> fmt::Display for Value<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Value::Number(float) => write!(f, "{}", lexical::to_string(*float)),
                Value::String(string) => write!(f, "\"{}\"", string),
                Value::Object(obj) => {
                    write!(f, "{{")?;
                    if let Some(((key, value), rest)) = obj.split_first() {
                        write!(f, "\"{}\": {}", key, value)?;
                        for (key, value) in rest.iter() {
                            write!(f, ", \"{}\": {}", key, value)?
                        }
                    }
                    write!(f, "}}")
                }
                Value::Bool(flag) => write!(f, "{}", flag),
                Value::Null => write!(f, "null"),
                Value::Array(array) => {
                    write!(f, "[")?;
                    if let Some((value, rest)) = array.split_first() {
                        write!(f, "{}", value)?;
                        for value in rest.iter() {
                            write!(f, ", {}", value)?
                        }
                    }
                    write!(f, "]")
                }
            }
        }
    }
}

pub fn parse_str<'a>(
    bytes: &'a str,
) -> std::result::Result<
    value::Value<'a>,
    lalrpop_util::ParseError<usize, Token<'a>, CompilationError>,
> {
    let lexer = Token::lexer(bytes).spanned().map(Token::to_lalr_triple);
    parser::jsonParser::new().parse(lexer)
}

pub fn stringify<'a, W: std::io::Write>(w: &mut W, v: &'a value::Value<'a>) -> std::io::Result<()> {
    write!(w, "{}", *v)
}

pub fn maybe_show_error<'a>(
    _source: &str,
    parsed: Result<value::Value<'a>, crate::parser::ParseError<'a>>,
) -> Result<value::Value<'a>, crate::parser::ParseError<'a>> {
    use cfg_if::cfg_if;
    cfg_if! {
        if #[cfg(feature = "pretty_errors")] {
          codespan::maybe_show_error(_source, parsed)
        } else {
          if parsed.is_err() == false {
              eprintln!("{:#?}", parsed);
          }
              parsed
        }
    }
}

pub fn show_error_test<'a>(
    _source: &str,
    parsed: Result<value::Value<'a>, crate::parser::ParseError<'a>>,
) -> Result<value::Value<'a>, crate::parser::ParseError<'a>> {
    use cfg_if::cfg_if;
    cfg_if! {
        if #[cfg(feature = "pretty_errors")] {
          codespan::show_error_test(_source, parsed)
        } else {
          if parsed.is_err() == false {
              eprintln!("{:#?}", parsed);
          }
              parsed
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let source = "�";
        let parsed = show_error_test(source, parse_str(source));
        assert_eq!(parsed.is_err(), true);
    }
}
