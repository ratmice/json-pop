#[cfg(feature = "pretty_errors")]
pub mod codespan;
pub mod lex;

pub mod parser {
    #![allow(clippy::all)]

    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub json);
    pub use json::*;
    pub type ParseError<'a> =
        lalrpop_util::ParseError<usize, crate::lex::Token::<'a>, crate::lex::wrap::Error>;
}
pub use lalrpop_util;

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
    lalrpop_util::ParseError<usize, lex::Token<'a>, lex::wrap::Error>,
> {
    let lexer = lex::wrap::Tokens::new(bytes);
    parser::jsonParser::new().parse(lexer)
}

pub fn stringify<'a, W: std::io::Write>(w: &mut W, v: &'a value::Value<'a>) -> std::io::Result<()> {
    write!(w, "{}", *v)
}

#[cfg(test)]
mod test {
    #[test]
    fn test() -> std::result::Result<(), anyhow::Error> {
        let _ = crate::parse_str("�")?;
        Ok(())
    }
}
