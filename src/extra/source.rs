pub use self::source::*;

mod source {
    #[cfg(feature = "pretty_errors")]
    use crate::extra::codespan;
    use crate::{error, lex, parser, value};

    use logos::Logos as _;

    #[derive(Debug)]
    pub struct Source<'a>(&'a str);

    pub trait Parsable<'a> {
        fn parse(&'a self) -> parser::Parsed<'a>;
        fn source(&'a self) -> &Source<'a>;
    }

    impl<'a> Parsable<'a> for Source<'a> {
        fn parse(&self) -> parser::Parsed {
            let lexer = lex::Token::lexer(self.0)
                .spanned()
                .map(lex::Token::to_lalr_triple);
            parser::Parsed(parser::jsonParser::new().parse(lexer))
        }

        fn source(&self) -> &Source<'a> {
            self
        }
    }

    pub trait ErrorHandling<'a> {
        fn handle_errors(
            &'a self,
            parsed: parser::Parsed<'a>,
        ) -> Result<value::Value<'a>, error::JsonPopError<'a>>;
    }

    impl<'a> ErrorHandling<'a> for Source<'a> {
        fn handle_errors(
            &'a self,
            parsed: parser::Parsed<'a>,
        ) -> Result<value::Value<'a>, error::JsonPopError<'a>> {
            use cfg_if::cfg_if;
            cfg_if! {
                if #[cfg(feature = "pretty_errors")] {
                    codespan::maybe_show_error(self.0, parsed.0)
                } else {
                  use std::io::Write;
                  if parsed.0.is_err() == false {
                      write!(std::io::stderr(), "{:#?}", self.0)?;
                  }
                  Ok(parsed.0?)
               }
            }
        }
    }

    impl<'a, T: AsRef<str> + 'a> From<&'a T> for Source<'a> {
        fn from(it: &'a T) -> Source<'a> {
            Source(it.as_ref())
        }
    }

    impl<'a> AsRef<str> for Source<'a> {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }
}
