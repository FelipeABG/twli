use anyhow::bail;

use crate::runtime_error;

#[derive(PartialEq, PartialOrd)]
pub enum Object {
    Number(f64),
    Str(String),
    Boolean(bool),
    Null,
}

impl Object {
    pub fn expect_number(&self, line: usize) -> anyhow::Result<f64> {
        if let Object::Number(n) = self {
            return Ok(*n);
        }

        bail!(runtime_error(&line, "Expected number"))
    }

    pub fn expect_string(&self, line: usize) -> anyhow::Result<String> {
        if let Object::Str(s) = self {
            return Ok(s.clone());
        }

        bail!(runtime_error(&line, "Expected number"))
    }

    pub fn truthiness(&self) -> bool {
        match self {
            Object::Boolean(b) => *b,
            Object::Null => false,
            _ => true,
        }
    }
}
