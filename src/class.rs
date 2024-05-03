use std::fmt;

use crate::error::RuntimeError;
use crate::instance::Instance;
use crate::interpreter::{Callable, Interpreter, Object};

#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class { name }
    }

    pub fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Callable for Class {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>
    where
        Self: Sized,
    {
        let instance = Instance::new(self.clone());
        Ok(Object::Instance(instance))
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
