use core::f64;
use std::fmt::Display;

use anyhow::bail;

#[derive(Debug)]
pub enum Object {
    Str(String),
    Boolean(bool),
    Number(f64),
    Null,
}

impl Object {
    pub fn expect_number(self) -> anyhow::Result<f64> {
        if let Object::Number(n) = self {
            return Ok(n);
        }

        bail!("Expected number")
    }

    pub fn expect_string(self) -> anyhow::Result<String> {
        if let Object::Str(s) = self {
            return Ok(s);
        }

        bail!("Expected string")
    }

    pub fn expect_boolean(self) -> anyhow::Result<bool> {
        if let Object::Boolean(b) = self {
            return Ok(b);
        }

        bail!("Expected boolean")
    }

    pub fn thrutiness(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Boolean(b) => *b,
            _ => true,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Object::Str(s) => s.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Number(n) => n.to_string(),
            Object::Null => "null".to_string(),
        };

        write!(f, "{}", msg)
    }
}
