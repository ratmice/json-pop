// In the style of: https://github.com/nst/JSONTestSuite

use json_pop::lex::wrap::Tokens as lex;
use json_pop::parser::jsonParser as parser;
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
    let lexer = lex::new(&buffer);
    let parsed = parser::new().parse(lexer);
    match parsed {
        Err(_) => std::process::exit(1),
        _ => std::process::exit(0),
    }
}
