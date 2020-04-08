use clap::arg_enum;

cfg_if::cfg_if! {
  if #[cfg(feature = "pretty_errors")] {
        use codespan_reporting::term::termcolor::StandardStream;
        use codespan_reporting::term::{self, ColorArg};
  }
}

use json_pop::lex::Token;
use json_pop::parser::jsonParser as parser;
use json_pop::value;
use logos::Logos;

use std::io;
use std::io::BufRead;
use std::io::Read;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum Mode {
      lex,
      parse,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "options", about = "json-pop options.")]
struct Opts {
    /// whether to lex or parse
    #[structopt(possible_values = &Mode::variants(), case_insensitive = true, default_value = "parse")]
    mode: Mode,
    /// parse each line as a separate json file.
    #[structopt(short, long)]
    line: bool,

    #[cfg(feature = "pretty_errors")]
    #[structopt(
        long = "color",
        default_value = "auto",
        possible_values = ColorArg::VARIANTS,
        case_insensitive = true,
    )]
    pub color: ColorArg,
}

fn main() -> anyhow::Result<()> {
    let opt = Opts::from_args();
    match opt.mode {
        Mode::parse => {
            if opt.line {
                parse_stdin_line()
            } else {
                parse_stdin()
            }
        }
        Mode::lex => lex_stdin_lalr(),
    }
}

fn parse_stdin() -> anyhow::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    let tokens = Token::lexer(&buffer).spanned().map(Token::to_lalr_triple);
    let parsed = parser::new().parse(tokens);
    display_value_or_error(&buffer, parsed)
}

// each line is parsed as though it were a valid json object
// It fails to parse things like: "{ \n "foo" : "bar" }"
// since "{\n" isn't a valid json object.
fn parse_stdin_line() -> anyhow::Result<()> {
    let reader = io::BufReader::new(io::stdin());
    for input_line in reader.lines() {
        let input_line = input_line?;
        let tokens = Token::lexer(&input_line.as_str())
            .spanned()
            .map(Token::to_lalr_triple);
        let parsed = parser::new().parse(tokens);
        if let Some(_) = display_value_or_error(&input_line, parsed).ok() {
            continue;
        }
    }
    Ok(())
}

/// Dumps lexer tokens...
fn lex_stdin_lalr() -> anyhow::Result<()> {
    let reader = io::BufReader::new(io::stdin());
    for line in reader.lines() {
        let line = line?;
        let tokens = Token::lexer(line.as_str());
        for tok in tokens {
            println!("{:?}", tok);
        }
    }
    Ok(())
}

fn display_value_or_error(
    _source: &str,
    parsed: Result<value::Value, json_pop::parser::ParseError>,
) -> anyhow::Result<()> {
    match parsed {
        Ok(value) => println!("{}", value),
        Err(error) => {
            cfg_if::cfg_if! {
              if #[cfg(feature = "pretty_errors")] {
                   let opts = Opts::from_args();
                   let writer = StandardStream::stderr(opts.color.into());
                   let config = codespan_reporting::term::Config::default();
                   let (files, diagnostic) = json_pop::codespan::from_parse_error("stdin", &_source, &error);
                  term::emit(&mut writer.lock(), &config, &files, &diagnostic)?;
              } else {
                  use std::io::Write;
                  write!(io::stderr().lock(), "{:#?}", error)?
              }
            }
            anyhow::bail!("Parse error");
        }
    }
    Ok(())
}
