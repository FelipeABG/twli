use std::collections::HashMap;

use anyhow::anyhow;

use crate::runtime::Object;

#[derive(Debug)]
pub struct Environment {
    bindings: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn define(&mut self, key: String, value: Object) {
        self.bindings.insert(key, value);
    }

    pub fn get(&self, key: &str) -> anyhow::Result<&Object> {
        self.bindings
            .get(key)
            .ok_or(anyhow!(format!("Undefined variable '{}'", key)))
    }
}
