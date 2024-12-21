use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::instance::Instance;
use crate::interpreter::{Callable, Interpreter, Object};
use crate::stmt::Stmt;

#[derive(Debug, Clone)]
pub struct Function {
    is_initializer: bool,
    pub declaration: Stmt,
    closure: Rc<RefCell<Environment>>,
}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.declaration.hash(state);
    }
}

impl Function {
    pub fn new(
        declaration: Stmt,
        environment: Rc<RefCell<Environment>>,
        is_initializer: bool,
    ) -> Function {
        if let Stmt::Function { .. } = declaration {
            return Function {
                is_initializer,
                declaration,
                closure: environment,
            };
        }
        panic!("Function implemented without declaration")
    }

    pub fn bind(&self, instance: Instance) -> Function {
        let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.closure,
        )))));
        environment.borrow_mut().define(
            "this".into(),
            Rc::new(RefCell::new(Object::Instance(instance))),
        );
        return Function::new(self.declaration.clone(), environment, self.is_initializer);
    }
}

impl Callable for Function {
    fn call<W: Write + 'static>(
        &self,
        interpreter: &mut Interpreter<W>,
        arguments: Vec<Rc<RefCell<Object>>>,
    ) -> Result<Rc<RefCell<Object>>, RuntimeError>
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
                    .expect("Error mapping arguments to parameters");
                environment
                    .borrow_mut()
                    .define(p.lexeme.clone(), Rc::clone(arg))
            }
            let result = interpreter.interpret_block(&body, environment);

            if let Err(e) = result {
                match e.value {
                    Some(v) => {
                        if self.is_initializer {
                            return self.closure.borrow().get_at(0, String::from("this"));
                        }

                        return Ok(v);
                    }
                    None => return Err(e),
                }
            }
            if self.is_initializer {
                return self.closure.borrow().get_at(0, String::from("this"));
            }
        }
        Ok(Rc::new(RefCell::new(Object::Nil)))
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
    native_function: fn() -> Object,
}

impl NativeFunction {
    pub fn new(arity: usize, native_function: fn() -> Object) -> NativeFunction {
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

    fn call<W: Write + 'static>(
        &self,
        _interpreter: &mut Interpreter<W>,
        _arguments: Vec<Rc<RefCell<Object>>>,
    ) -> Result<Rc<RefCell<Object>>, RuntimeError>
    where
        Self: Sized,
    {
        let result = (self.native_function)();

        Ok(Rc::new(RefCell::new(result)))
    }
}
