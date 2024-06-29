use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::error::RuntimeError;
use crate::function::Function;
use crate::instance::Instance;
use crate::interpreter::{Callable, Interpreter, Object};

#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
    methods: HashMap<String, Function>,
}

impl Class {
    pub fn new(name: String, methods: HashMap<String, Function>) -> Self {
        Class { name, methods }
    }

    pub fn find_method(&self, name: String) -> Option<Function> {
        return self.methods.get(&name).cloned();
    }
}

impl Callable for Class {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Rc<RefCell<Object>>>,
    ) -> Result<Rc<RefCell<Object>>, RuntimeError>
    where
        Self: Sized,
    {
        let instance = Instance::new(self.clone());
        Ok(Rc::new(RefCell::new(Object::Instance(instance))))
    }

    fn arity(&self) -> usize {
        0
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", self.name.clone())
    }
}
