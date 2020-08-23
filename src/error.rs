use crate::parser;
use std::fmt;
use std::ops::Range;

#[derive(Debug)]
pub enum JsonPopError<'a> {
    Parse(crate::parser::ParseError<'a>),
    Io(std::io::Error),
    TestError(crate::test_utils::TestError<'a>),
}

#[derive(Debug)]
pub enum TopLevelError {
    TestingError,
    ParseError,
    Io(std::io::Error),
}

impl<'a> From<JsonPopError<'a>> for TopLevelError {
    fn from(it: JsonPopError<'a>) -> TopLevelError {
        match it {
            JsonPopError::Parse(_) => {
                // Convert to an error without the associated lifetimes.
                TopLevelError::ParseError
            }
            JsonPopError::Io(err) => TopLevelError::Io(err),
            JsonPopError::TestError(_) => TopLevelError::TestingError,
        }
    }
}

#[derive(Debug)]
pub enum CompilationError {
    LexicalError { range: Range<usize> },
    NumericalError { range: Range<usize> },
    UnterminatedStringLiteral { range: Range<usize> },
}

impl fmt::Display for CompilationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl<'a> fmt::Display for JsonPopError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl<'a> From<std::io::Error> for JsonPopError<'a> {
    fn from(err: std::io::Error) -> Self {
        JsonPopError::Io(err)
    }
}

impl<'a> From<parser::ParseError<'a>> for JsonPopError<'a> {
    fn from(err: parser::ParseError<'a>) -> Self {
        JsonPopError::Parse(err)
    }
}
