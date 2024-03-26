use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::interpreter::{Callable, Interpreter, Object};
use crate::stmt::Stmt;

#[derive(Debug, Clone)]
pub struct Function {
    pub declaration: Stmt,
    closure: Rc<RefCell<Environment>>,
}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.declaration.hash(state);
    }
}

impl Function {
    pub fn new(declaration: Stmt, environment: Rc<RefCell<Environment>>) -> Function {
        if let Stmt::Function { .. } = declaration {
            return Function {
                declaration,
                closure: environment,
            };
        }
        panic!("Function implemented without declaration")
    }
}

impl Callable for Function {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>
    where
        Self: Sized,
    {
        let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.closure,
        )))));
        if let Stmt::Function {
            name: _,
            params,
            body,
        } = &self.declaration
        {
            let mut arguments_iter = arguments.iter();
            for p in params {
                let arg = arguments_iter
                    .next()
                    .expect("Error mapping arguments to parameters")
                    .clone();
                environment.borrow_mut().define(p.lexeme.clone(), arg)
            }
            let result = interpreter.interpret_block(&body, environment);

            if let Err(e) = result {
                match e.value {
                    Some(v) => return Ok(v),
                    None => return Err(e),
                }
            }
        }
        Ok(Object::Nil)
    }

    fn arity(&self) -> usize {
        if let Stmt::Function {
            name: _,
            params,
            body: _,
        } = &self.declaration
        {
            return params.len();
        }
        0
    }
}

#[derive(Debug, Clone, Hash)]
pub struct NativeFunction {
    arity: usize,
    native_function: fn(),
}

impl NativeFunction {
    pub fn new(arity: usize, native_function: fn()) -> NativeFunction {
        NativeFunction {
            arity,
            native_function,
        }
    }
}

impl Callable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>
    where
        Self: Sized,
    {
        (self.native_function)();

        Ok(Object::Nil)
    }
}
