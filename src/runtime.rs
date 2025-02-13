use crate::{
    env::Environment,
    grammar::{FnDecl, Statement},
    interpreter::Interpreter,
    runtime_error,
};
use anyhow::bail;
use core::f64;
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops,
    rc::Rc,
};

pub trait Callable {
    fn call(&mut self, interp: &mut Interpreter, args: Vec<Object>) -> anyhow::Result<Object>;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
    fn clone_box(&self) -> Box<dyn Callable>;
}

pub enum Object {
    Str(String),
    Boolean(bool),
    Number(f64),
    Callable(Box<dyn Callable>),
    Null,
}

//user function definition
pub struct Function {
    pub declaration: FnDecl,
}

impl Callable for Function {
    fn call(&mut self, interp: &mut Interpreter, args: Vec<Object>) -> anyhow::Result<Object> {
        let env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &interp.global,
        )))));

        for idx in 0..self.declaration.params.len() {
            let param = self.declaration.params[idx].lexeme.clone();
            let value = args[idx].clone();
            RefCell::borrow_mut(&env).define(param, value);
        }

        if let Statement::BlockStmt(b) = &self.declaration.body {
            interp.exec_block_statement(&b, env)?;
        }

        Ok(Object::Null)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn to_string(&self) -> String {
        format!("<user fn {}>", self.declaration.ident.lexeme)
    }

    fn clone_box(&self) -> Box<dyn Callable> {
        Box::new(Function {
            declaration: self.declaration.clone(),
        })
    }
}

impl Object {
    pub fn expect_number(self, line: &usize) -> anyhow::Result<f64> {
        if let Object::Number(n) = self {
            return Ok(n);
        }

        bail!(runtime_error(line, "Expected number"))
    }

    pub fn expect_string(self, line: &usize) -> anyhow::Result<String> {
        if let Object::Str(s) = self {
            return Ok(s);
        }

        bail!(runtime_error(line, "Expected string"))
    }

    pub fn expect_boolean(self, line: &usize) -> anyhow::Result<bool> {
        if let Object::Boolean(b) = self {
            return Ok(b);
        }

        bail!(runtime_error(line, "Expected boolean"))
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
            Object::Callable(callable) => callable.to_string(),
        };

        write!(f, "{}", msg)
    }
}

impl ops::Add for Object {
    type Output = anyhow::Result<Object>;

    fn add(self, other: Object) -> Self::Output {
        match (self, other) {
            (Object::Str(s1), Object::Str(s2)) => Ok(Object::Str(s1 + &s2)),
            (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 + n2)),
            (Object::Str(_), Object::Number(_)) | (Object::Number(_), Object::Str(_)) => {
                bail!("Expected both operands to be of the same type")
            }
            _ => bail!(
                "Unsuported operands types for addition. Supported ones are 'string' and 'number'"
            ),
        }
    }
}

impl ops::Div for Object {
    type Output = anyhow::Result<Object>;

    fn div(self, other: Object) -> Self::Output {
        match (self, other) {
            (Object::Number(n1), Object::Number(n2)) => {
                if n2 == 0.0 {
                    bail!("Division by zero is not allowed")
                }
                Ok(Object::Number(n1 / n2))
            }
            _ => bail!("Expected both operands to be numbers in division operation"),
        }
    }
}

impl ops::Mul for Object {
    type Output = anyhow::Result<Object>;

    fn mul(self, other: Object) -> Self::Output {
        match (self, other) {
            (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 * n2)),
            _ => bail!("Expected both operands to be numbers in multiplication operation"),
        }
    }
}

impl ops::Sub for Object {
    type Output = anyhow::Result<Object>;

    fn sub(self, other: Object) -> Self::Output {
        match (self, other) {
            (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 - n2)),
            _ => bail!("Expected both operands to be numbers in subtraction operation"),
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Str(a), Object::Str(b)) => a.partial_cmp(b),
            (Object::Number(a), Object::Number(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::Str(a), Object::Str(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Null, Object::Null) => true,
            (Object::Callable(_), Object::Callable(_)) => false,
            _ => false,
        }
    }
}

impl Clone for Box<dyn Callable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        match self {
            Object::Number(n) => Object::Number(*n),
            Object::Str(s) => Object::Str(s.clone()),
            Object::Boolean(b) => Object::Boolean(*b),
            Object::Null => Object::Null,
            Object::Callable(c) => Object::Callable(c.clone()),
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Object::Str(s) => format!("{s}"),
            Object::Number(n) => format!("{n}"),
            Object::Null => format!("null"),
            Object::Boolean(b) => format!("{b}"),
            Object::Callable(c) => format!("{}", c.to_string()),
        };
        write!(f, "{msg}")
    }
}
