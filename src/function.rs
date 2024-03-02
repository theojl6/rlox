use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::interpreter::{Callable, Interpretor, Object};
use crate::stmt::Stmt;

#[derive(Debug, Clone)]
pub struct Function {
    pub declaration: Stmt,
}

impl Function {
    pub fn new(declaration: Stmt) -> Function {
        if let Stmt::Function { .. } = declaration {
            return Function { declaration };
        }
        panic!("Function implemented without declaration")
    }
}

impl Callable for Function {
    fn call(
        &self,
        interpretor: &mut Interpretor,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>
    where
        Self: Sized,
    {
        let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &interpretor.globals,
        )))));
        if let Stmt::Function {
            name: _,
            params,
            body,
        } = &self.declaration
        {
            let mut arguments_iter = arguments.iter();
            for p in params {
                environment.borrow_mut().define(
                    p.lexeme.clone(),
                    arguments_iter
                        .next()
                        .expect("Error mapping arguments to parameters")
                        .clone(),
                )
            }
            let result = interpretor.interpret_block(&body, environment);

            if let Err(e) = result {
                let value = e.value.unwrap();
                println!("returned value: {:?}", value);
                return Ok(value);
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

#[derive(Debug, Clone)]
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
        _interpretor: &mut Interpretor,
        _arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>
    where
        Self: Sized,
    {
        (self.native_function)();

        Ok(Object::Nil)
    }
}
