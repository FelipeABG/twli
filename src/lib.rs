pub mod env;
pub mod grammar;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod std;
pub mod token;
use colored::Colorize;

pub fn syntax_error(line: &usize, msg: &str) -> String {
    format!("\n{} [line {}]: {}.", "SyntaxError".bold().red(), line, msg)
}

pub fn runtime_error(line: &usize, msg: &str) -> String {
    format!(
        "\n{} [line {}]: {}.",
        "RuntimeError".bold().red(),
        line,
        msg
    )
}
