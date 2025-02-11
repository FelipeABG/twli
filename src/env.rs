use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{anyhow, bail};

use crate::runtime::Object;

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

    pub fn get(&self, key: &str) -> anyhow::Result<Object> {
        match self.bindings.get(key) {
            Some(obj) => Ok(obj.clone()),
            None => match &self.enclosing {
                Some(enclosing) => RefCell::borrow(enclosing).get(key),
                None => Err(anyhow!(format!("Undefined variable '{}'", key))),
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
