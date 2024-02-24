use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::interpreter::{Callable, Interpretor, Object};
use crate::stmt::Stmt;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Function {
    declaration: Stmt,
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
        token: &Token,
        interpretor: &mut Interpretor,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>
    where
        Self: Sized,
    {
        let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &interpretor.globals,
        )))));
        if let Stmt::Function { name, params, body } = &self.declaration {
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
            interpretor.interpret_block(&body, environment);
        }
        Ok(Object::Nil)
    }

    fn arity(&self) -> usize {
        if let Stmt::Function { name, params, body } = &self.declaration {
            return params.len();
        }
        0
    }
}
