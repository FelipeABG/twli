use std::fs::read_to_string;

use test_interp::lexer::Lexer;

fn main() {
    let source = read_to_string("test.lox").unwrap();
    let mut lexer = Lexer::new(source.trim().to_string());
    let tokens = lexer.tokenize().unwrap();
    for token in tokens {
        println!("{:?}", token)
    }
}
