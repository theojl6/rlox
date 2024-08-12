use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io::Write;
use std::rc::Rc;

use crate::error::RuntimeError;
use crate::function::Function;
use crate::instance::Instance;
use crate::interpreter::{Callable, Interpreter, Object};

#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
    superclass: Option<Rc<RefCell<Object>>>,
    methods: HashMap<String, Function>,
}

impl Class {
    pub fn new(
        name: String,
        superclass: Option<Rc<RefCell<Object>>>,
        methods: HashMap<String, Function>,
    ) -> Self {
        Class {
            name,
            superclass,
            methods,
        }
    }

    pub fn find_method(&self, name: String) -> Option<Function> {
        if self.methods.contains_key(&name) {
            return self.methods.get(&name).cloned();
        }

        if let Some(sc) = &self.superclass {
            if let Object::Class(c) = &*sc.borrow_mut() {
                return c.find_method(name);
            }
        }

        return None;
    }
}

impl Callable for Class {
    fn call<W: Write + 'static>(
        &self,
        interpreter: &mut Interpreter<W>,
        arguments: Vec<Rc<RefCell<Object>>>,
    ) -> Result<Rc<RefCell<Object>>, RuntimeError>
    where
        Self: Sized,
    {
        let instance = Instance::new(self.clone());
        if let Some(initializer) = self.find_method(String::from("init")) {
            initializer
                .bind(instance.clone())
                .call(interpreter, arguments)?;
        }
        Ok(Rc::new(RefCell::new(Object::Instance(instance))))
    }

    fn arity(&self) -> usize {
        match self.find_method(String::from("init")) {
            Some(initializer) => initializer.arity(),
            None => 0,
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", self.name.clone())
    }
}
