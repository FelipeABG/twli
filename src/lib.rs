pub mod grammar;
pub mod lexer;
pub mod parser;
pub mod token;
use colored::Colorize;

pub fn syntax_error(line: &usize, msg: &str) -> String {
    format!("\n{} [line {}]: {}.", "SyntaxError".bold().red(), line, msg)
}
