#![cfg(feature = "pretty_errors")]

use crate::parser::ParseError;
use crate::value;
use crate::CompilationError;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

pub fn from_parse_error<'a>(
    filename: &'a str,
    data: &'a str,
    error: &ParseError<'a>,
) -> (SimpleFiles<&'a str, &'a str>, Diagnostic<usize>) {
    use lalrpop_util::ParseError::*;

    let mut files = SimpleFiles::new();
    let file_id = files.add(filename, data);
    let join_expected = |expected: &Vec<String>| -> String {
        if let Some((caboose, rest)) = expected.split_last() {
            if rest.is_empty() {
                format!("Expected: {}", &caboose)
            } else {
                format!("Expected: {} or {}", rest.join(", "), &caboose)
            }
        } else {
            // If this error occurs we need to test for it at the caller.
            "Had great expectations?".to_string()
        }
    };

    let diag = match error {
        InvalidToken { location } => Diagnostic::error()
            .with_message("Invalid token")
            .with_labels(vec![Label::primary(file_id, *location..*location)]),
        UnrecognizedEOF { location, expected } => Diagnostic::error()
            .with_message("Unexpected EOF")
            .with_labels(vec![
                Label::primary(file_id, *location..*location).with_message(join_expected(expected))
            ]),
        UnrecognizedToken {
            token: (start, _tok, end),
            expected,
        } => Diagnostic::error()
            .with_message("Unrecognized token")
            .with_labels(vec![
                Label::primary(file_id, *start..*end).with_message(join_expected(expected))
            ]),
        ExtraToken {
            token: (start, _tok, end),
        } => Diagnostic::error()
            .with_message("Extra token")
            .with_labels(vec![Label::primary(file_id, *start..*end)])
            .with_message("Extra token"),
        User { error } => {
            let pos = match error {
                CompilationError::LexicalError { pos } => pos,
                CompilationError::NumericalError { pos } => pos,
            };

            Diagnostic::error()
                .with_message(format!("{:?}", error))
                .with_labels(vec![Label::primary(file_id, *pos..*pos)])
        }
    };
    (files, diag)
}

pub fn maybe_show_error<'a>(
    _source: &str,
    parsed: Result<value::Value<'a>, crate::parser::ParseError<'a>>,
) -> Result<value::Value<'a>, crate::parser::ParseError<'a>> {
    if let Err(error) = parsed {
        let writer = StandardStream::stderr(ColorChoice::Auto);
        let config = codespan_reporting::term::Config::default();
        let (files, diagnostic) = crate::codespan::from_parse_error("stdin", &_source, &error);
        // Swalling this error is fairly awkward.
        // We should really wrap the parse error in an IO error with source.
        // and return an IO error.
        //
        // however in that case, what do we return if IO succeeds? sigh
        assert_eq!(
            term::emit(&mut writer.lock(), &config, &files, &diagnostic).is_err(),
            false
        );
        Err(error)
    } else {
        parsed
    }
}
pub fn show_error_test<'a>(
    _source: &str,
    parsed: Result<value::Value<'a>, crate::parser::ParseError<'a>>,
) -> Result<value::Value<'a>, crate::parser::ParseError<'a>> {
    if let Err(error) = parsed {
        let mut writer = codespan_reporting::term::termcolor::Buffer::no_color();
        let config = codespan_reporting::term::Config::default();
        let (files, diagnostic) = crate::codespan::from_parse_error("stdin", &_source, &error);
        assert_eq!(
            term::emit(&mut writer, &config, &files, &diagnostic).is_err(),
            false,
        );
        // We want the output to be captured by cargo test.
        // This one doesn't seem to be captured.
        // let _ = std::io::stderr().write_all(writer.as_slice());
        // Nor does
        // let writer = StandardStream::stderr(ColorChoice::Auto);
        // The following works.
        eprintln!("{}", std::str::from_utf8(writer.as_slice()).unwrap());
        Err(error)
    } else {
        parsed
    }
}
