use crate::parser;
use std::fmt;
use std::ops::Range;

/// This is the error type returned by the parser.
/// It's bounded by the lifetime of the source string
/// which was parsed
#[derive(Debug)]
pub enum JsonPopError<'a> {
    Parse(crate::parser::ParseError<'a>),
    Io(std::io::Error),
    /// This type is a never variation unless the testsuite is being run.
    TestError(crate::test_utils::TestError<'a>),
}

/// A top level error returned by processes or tests..
/// It's not bounded by the lifetime of the program
/// we should add error codes to these.
///
/// That will be a breaking change.
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

// This error lives inside the the parsers Error type.
// So it's a sub-error of a parse error.
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
