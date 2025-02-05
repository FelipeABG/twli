use interp::{lexer::Lexer, parser::Parser};
use std::fs::read_to_string;

fn main() {
    let source = read_to_string("test.lox").unwrap();
    let mut lexer = Lexer::new(source.trim().to_string());
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    println!("{:#?}", expr)
}
