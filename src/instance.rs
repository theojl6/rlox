use crate::{class::Class, error::RuntimeError, interpreter::Object, token::Token};
use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Clone, Debug)]
pub struct Instance {
    klass: Class,
    fields: HashMap<String, Rc<RefCell<Object>>>,
}

impl Instance {
    pub fn new(klass: Class) -> Self {
        Instance {
            klass,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Rc<RefCell<Object>>, RuntimeError> {
        if self.fields.contains_key(&name.lexeme) {
            return Ok(self.fields.get(&name.lexeme).unwrap().clone());
        }
        let method = self.klass.find_method(name.lexeme.clone());
        if let Some(mut m) = method {
            return Ok(Rc::new(RefCell::new(Object::Function(Box::new(m)))));
        }
        Err(RuntimeError::new(
            name.clone(),
            &format!("Undefined property '{}'.", &name.lexeme),
            None,
        ))
    }

    pub fn set(&mut self, name: &Token, value: Rc<RefCell<Object>>) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", format!("{} instance", self.klass.name.clone()))
    }
}
