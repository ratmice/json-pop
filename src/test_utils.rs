#[cfg(test)]
use crate::{error, parser, source, source::Parsable as _, value};

#[cfg(not(test))]
use crate::never;

#[derive(Debug)]
#[cfg(test)]
pub enum TestError<'a> {
    InvalidSourceParsedOk(value::Value<'a>, &'a source::Source<'a>),
}
#[cfg(not(test))]
pub type TestError<'a> = never::Never<'a>;

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
    fn parse(&'a self) -> parser::Parsed<'a> {
        match self {
            Test::TestValid(src) | Test::TestInvalid(src) => src.parse(),
        }
    }

    fn source(&'a self) -> &'a source::Source<'a> {
        match self {
            Self::TestValid(source) | Self::TestInvalid(source) => source,
        }
    }
}

#[cfg(test)]
impl<'a> source::ErrorHandling<'a> for Test<'a> {
    #[cfg(not(feature = "pretty_errors"))]
    fn handle_errors(
        &'a self,
        parsed: parser::Parsed<'a>,
    ) -> Result<value::Value<'a>, error::JsonPopError<'a>> {
        match (parsed.0.is_err(), self.should_fail()) {
            (true, true) => {
                eprint!("{:#?}", parsed);
                return Ok(value::Value::Null);
            }
            (false, true) => Err(error::JsonPopError::TestError(
                TestError::InvalidSourceParsedOk(parsed.0.unwrap(), self.source()),
            )),
            (_, _) => Ok(parsed.0.map_err(error::JsonPopError::Parse)?),
        }
    }

    #[cfg(feature = "pretty_errors")]
    fn handle_errors(
        &'a self,
        parsed: parser::Parsed<'a>,
    ) -> Result<value::Value<'a>, error::JsonPopError<'a>> {
        if let Err(error) = parsed.0 {
            let mut writer = codespan_reporting::term::termcolor::Buffer::no_color();
            let config = codespan_reporting::term::Config::default();
            let (files, diagnostic) =
                crate::codespan::from_parse_error("stdin", self.source(), &error);

            let () = codespan_reporting::term::emit(&mut writer, &config, &files, &diagnostic)?;
            eprint!("{}", std::str::from_utf8(writer.as_slice()).unwrap());
            if self.should_fail() {
                Ok(value::Value::Null)
            } else {
                Err(crate::error::JsonPopError::Parse(error))
            }
        } else {
            if self.should_fail() {
                return Err(error::JsonPopError::TestError(
                    TestError::InvalidSourceParsedOk(parsed.0.unwrap(), self.source()),
                ));
            }
            parsed.0.map_err(crate::error::JsonPopError::Parse)
        }
    }
}
