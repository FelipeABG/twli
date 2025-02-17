use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{anyhow, bail};

use crate::{
    error::syntax_error,
    runtime::{Callable, Object},
    token::Token,
};

#[derive(Debug)]
pub struct Environment {
    bindings: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            bindings: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, key: String, value: Object) {
        self.bindings.insert(key, value);
    }

    pub fn define_callable(&mut self, key: String, value: impl Callable + Send + Sync + 'static) {
        self.bindings.insert(key, Object::Callable(Box::new(value)));
    }

    pub fn get(&self, key: &Token) -> anyhow::Result<Object> {
        match self.bindings.get(&key.lexeme) {
            Some(obj) => Ok(obj.clone()),
            None => match &self.enclosing {
                Some(enclosing) => RefCell::borrow(enclosing).get(key),
                None => Err(anyhow!(syntax_error(
                    &key.line,
                    &format!("Undefined variable '{}'", key.lexeme)
                ))),
            },
        }
    }

    pub fn assign(&mut self, key: &str, value: Object) -> anyhow::Result<()> {
        match self.bindings.get(key) {
            Some(_) => {
                self.bindings.insert(key.to_string(), value).unwrap();
                Ok(())
            }
            None => match &self.enclosing {
                Some(enclosing) => RefCell::borrow_mut(enclosing).assign(key, value),
                None => bail!(format!("Tried to assign to non-existent binding '{}'", key)),
            },
        }
    }
}
