pub mod lexer;
pub mod token;
use colored::Colorize;

pub fn syntax_error(line: &usize, msg: &str) -> String {
    format!("{} [line {}]: {}", "SyntaxError".bold().red(), line, msg)
}
