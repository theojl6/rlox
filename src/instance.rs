use crate::{class::Class, error::RuntimeError, interpreter::Object, token::Token};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug)]
pub struct Instance {
    klass: Class,
    fields: HashMap<String, Object>,
}

impl Instance {
    pub fn new(klass: Class) -> Self {
        Instance {
            klass,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        if self.fields.contains_key(&name.lexeme) {
            return Ok(self.fields.get(&name.lexeme).unwrap().clone());
        }
        Err(RuntimeError::new(
            name.clone(),
            &format!("Undefined property '{}'.", &name.lexeme),
            None,
        ))
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", format!("{} instance", self.klass.name.clone()))
    }
}
