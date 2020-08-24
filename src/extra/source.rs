pub use self::source::*;

mod source {
    #[cfg(feature = "pretty_errors")]
    use crate::extra::codespan;
    use crate::{error, lex, parser, value};

    use logos::Logos as _;

    #[derive(Debug)]
    pub struct Source<'a>(&'a str);

    pub trait Parsable<'a> {
        type SourceContext;
        fn parse(&'a self) -> parser::Parsed<'a, Self::SourceContext>;
        fn source(&'a self) -> &Source<'a>;
    }

    impl<'a> Parsable<'a> for Source<'a> {
        type SourceContext = Self;
        fn parse(&'a self) -> parser::Parsed<'a, Self> {
            let lexer = lex::Token::lexer(self.0)
                .spanned()
                .map(lex::Token::to_lalr_triple);
            parser::Parsed {
                source_ctxt: &self,
                parse_result: parser::jsonParser::new().parse(lexer),
            }
        }

        fn source(&self) -> &Source<'a> {
            self
        }
    }

    pub trait ErrorHandling<'a> {
        fn handle_errors(self) -> Result<value::Value<'a>, error::JsonPopError<'a>>;
    }

    impl<'a> ErrorHandling<'a> for parser::Parsed<'a, Source<'a>> {
        fn handle_errors(self) -> Result<value::Value<'a>, error::JsonPopError<'a>> {
            use cfg_if::cfg_if;
            cfg_if! {
                if #[cfg(feature = "pretty_errors")] {
                    codespan::maybe_show_error(self.source_ctxt.as_ref(), self.parse_result)
                } else {
                  use std::io::Write;
                  if self.parse_result.is_err() == false {
                      write!(std::io::stderr(), "{:#?}", self.source_ctxt)?;
                  }
                  Ok(self.parse_result?)
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
