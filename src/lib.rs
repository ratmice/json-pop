pub mod error;
pub mod extra;
pub mod lex;

use crate::error::CompilationError;
use crate::lex::Token;

pub use lalrpop_util;
use logos::Logos;

pub mod parser {
    #![allow(clippy::all)]
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub json);
    use super::*;
    pub use json::*;

    pub type ParseError<'a> = lalrpop_util::ParseError<usize, Token<'a>, CompilationError>;
    pub type ParseResult<'a> = Result<value::Value<'a>, ParseError<'a>>;
}

pub mod value {
    use lexical;
    use std::fmt;

    #[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::extra::source;
    use crate::extra::test_utils::Test;
    use source::ErrorHandling as _;
    use source::Parsable as _;

    #[test]
    fn test_invalid() -> Result<(), error::TopLevelError> {
        let sources = ["�", r#""string with missing end quote"#]
            .iter()
            .map(|src| Test::TestInvalid(src.into()));
        Ok(for test in sources {
            assert_eq!(
                test.parse()
                    .handle_errors()
                    .map_err(|e| error::TopLevelError::from(e))?,
                crate::value::Value::Null
            );
        })
    }

    #[test]
    fn test_valid() -> Result<(), error::TopLevelError> {
        use crate::value::Value;
        let sources = [
            ("[]", Value::Array([].to_vec())),
            (r#""foo bar""#, Value::String("foo bar")),
            (r#""""#, Value::String("")),
        ];
        let tests = sources
            .iter()
            .map(|(src, result)| (Test::TestValid(src.into()), result));
        Ok(for (test, result) in tests {
            assert_eq!(
                test.parse()
                    .handle_errors()
                    .map_err(|e| error::TopLevelError::from(e))?,
                *result
            );
        })
    }
}
