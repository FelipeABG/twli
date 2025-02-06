use interp::{lexer::Lexer, parser::Parser};
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let source = read_to_string("test.lox").unwrap();
    let mut lexer = Lexer::new(source.trim().to_string());
    match lexer.tokenize() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            let expr = parser.parse()?;
            println!("{:#?}", expr);
        }
        Err(e) => {
            println!("{e}");
            std::process::exit(65);
        }
    }

    Ok(())
}
