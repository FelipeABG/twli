use interp::lexer::Lexer;
use std::fs::read_to_string;

fn main() {
    let source = read_to_string("test.lox").unwrap();
    let mut lexer = Lexer::new(source.trim().to_string());
    match lexer.tokenize() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token)
            }
        }
        Err(err) => println!("{err}"),
    }
}
