#[cfg(test)]
use crate::{error, extra::source, extra::source::Parsable as _, parser, value};
#[cfg(test)]
use logos::Logos as _;

#[cfg(not(test))]
use crate::extra::never;

#[cfg(test)]
#[derive(Debug)]
pub enum TestError<'a> {
    InvalidSourceParsedOk(value::Value<'a>, &'a source::Source<'a>),
}

#[cfg(not(test))]
pub type TestError<'a> = never::Never;

// TODO look at how much boiler plate is actually saved by doing this in one type...
// The ErrorHandling implementation at least would be a lot clearer if these were
// different types.
#[cfg(test)]
pub enum Test<'a> {
    TestValid(source::Source<'a>),
    TestInvalid(source::Source<'a>),
}

#[cfg(test)]
impl<'a> Test<'a> {
    pub fn should_fail(&self) -> bool {
        match self {
            Self::TestValid(_) => false,
            Self::TestInvalid(_) => true,
        }
    }
}

#[cfg(test)]
impl<'a> source::Parsable<'a> for Test<'a> {
    type SourceContext = Test<'a>;
    fn parse(&'a self) -> source::Parsed<'a, Self> {
        match self {
            Test::TestValid(src) | Test::TestInvalid(src) => {
                let lexer = crate::lex::Token::lexer(src.as_ref())
                    .spanned()
                    .map(crate::lex::Token::to_lalr_triple);
                source::Parsed {
                    source_ctxt: &self,
                    parse_result: parser::jsonParser::new().parse(lexer),
                }
            }
        }
    }

    fn source(&'a self) -> &'a source::Source<'a> {
        match self {
            Self::TestValid(source) | Self::TestInvalid(source) => source,
        }
    }
}

#[cfg(test)]
impl<'a> source::ErrorHandling<'a> for source::Parsed<'a, Test<'a>> {
    #[cfg(not(feature = "pretty_errors"))]
    fn handle_errors(self) -> Result<value::Value<'a>, error::JsonPopError<'a>> {
        match (self.parse_result.is_err(), self.source_ctxt.should_fail()) {
            (true, true) => {
                eprint!("{:#?}", self.parse_result);
                return Ok(value::Value::Null);
            }
            (false, true) => Err(error::JsonPopError::TestError(
                TestError::InvalidSourceParsedOk(
                    self.parse_result.unwrap(),
                    self.source_ctxt.source(),
                ),
            )),
            (_, _) => Ok(self.parse_result.map_err(error::JsonPopError::Parse)?),
        }
    }

    #[cfg(feature = "pretty_errors")]
    fn handle_errors(self) -> Result<value::Value<'a>, error::JsonPopError<'a>> {
        if let Err(error) = self.parse_result {
            let mut writer = codespan_reporting::term::termcolor::Buffer::no_color();
            let config = codespan_reporting::term::Config::default();
            let (files, diagnostic) = crate::extra::codespan::from_parse_error(
                "stdin",
                self.source_ctxt.source(),
                &error,
            );

            let () = codespan_reporting::term::emit(&mut writer, &config, &files, &diagnostic)?;
            eprint!("{}", std::str::from_utf8(writer.as_slice()).unwrap());
            if self.source_ctxt.should_fail() {
                Ok(value::Value::Null)
            } else {
                Err(crate::error::JsonPopError::Parse(error))
            }
        } else {
            if self.source_ctxt.should_fail() {
                return Err(error::JsonPopError::TestError(
                    TestError::InvalidSourceParsedOk(
                        self.parse_result.unwrap(),
                        self.source_ctxt.source(),
                    ),
                ));
            }
            self.parse_result.map_err(crate::error::JsonPopError::Parse)
        }
    }
}
