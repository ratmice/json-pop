// In the style of: https://github.com/nst/JSONTestSuite

use json_pop::lex::Token;
use json_pop::parser::jsonParser as parser;
use logos::Logos;
use std::io::Read;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} file.json", args[0]);
        std::process::exit(1);
    }

    let ref path = args[1];
    let mut buffer = String::new();
    let mut f = std::fs::File::open(path).expect("Unable to open file");
    f.read_to_string(&mut buffer)?;
    let tokens = Token::lexer(&buffer).spanned().map(Token::to_lalr_triple);
    let parsed = parser::new().parse(tokens);
    match parsed {
        Err(_) => std::process::exit(1),
        _ => std::process::exit(0),
    }
}
