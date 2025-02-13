use std::{error::Error, fmt::Display};

use colored::Colorize;

use crate::runtime::Object;

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

#[derive(Debug)]
pub struct Return {
    pub value: Option<Object>,
}

impl Return {
    pub fn new(value: Option<Object>) -> Self {
        Self { value }
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl Error for Return {}
