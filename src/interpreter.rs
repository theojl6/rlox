use crate::ast::{Expr, Visitor};
use crate::class::Class;
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::function::{Function, NativeFunction};
use crate::instance::Instance;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub enum Object {
    Class(Class),
    Instance(Instance),
    String(String),
    Number(f32),
    Bool(bool),
    Nil,
    Function(Box<Function>),
    NativeFunction(NativeFunction),
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::String(s) => s.hash(state),
            Object::Number(n) => n.to_bits().hash(state),
            Object::Bool(b) => b.hash(state),
            Object::Class(_c) => self.hash(state),
            Object::Instance(_i) => self.hash(state),
            Object::Nil => self.hash(state),
            Object::Function(f) => f.hash(state),
            Object::NativeFunction(f) => f.hash(state),
        }
    }
}

pub trait Callable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Rc<RefCell<Object>>>,
    ) -> Result<Rc<RefCell<Object>>, RuntimeError>
    where
        Self: Sized;

    fn arity(&self) -> usize;
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Bool(b) => {
                write!(f, "{:}", b)
            }
            Object::Class(c) => {
                write!(f, "{:}", c)
            }
            Object::Instance(i) => {
                write!(f, "{:}", i)
            }
            Object::String(s) => {
                write!(f, "{:}", s)
            }
            Object::Number(n) => {
                write!(f, "{:}", n)
            }
            Object::Nil => {
                write!(f, "{:}", "nil")
            }
            Object::Function(func) => {
                if let Stmt::Function {
                    name,
                    params: _,
                    body: _,
                } = &func.declaration
                {
                    return write!(f, "{:}", "Function<".to_owned() + &name.lexeme + ">");
                }
                write!(f, "{:}", "Anonymous Function")
            }
            Object::NativeFunction(_) => {
                write!(f, "{:}", "Native Function")
            }
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        match (self, other) {
            (&Object::Number(l), &Object::Number(r)) => l == r,
            (Object::String(l), Object::String(r)) => l == r,
            (Object::Bool(l), Object::Bool(r)) => l == r,
            (Object::Nil, Object::Nil) => true,
            (_, _) => false,
        }
    }
}

impl Eq for Object {}

pub struct Interpreter<'a> {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<Expr, usize>,
    writer: &'a mut (dyn Write + 'a),
}

impl<'a> Interpreter<'a> {
    pub fn new(writer: &'a mut (dyn Write + 'a)) -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        globals.borrow_mut().define(
            String::from("clock"),
            Rc::new(RefCell::new(Object::NativeFunction(NativeFunction::new(
                0,
                || {
                    let start = SystemTime::now();
                    let since_the_epoch = start
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_millis();
                    println!("{:?}", since_the_epoch);
                },
            )))),
        );
        Interpreter {
            globals: Rc::clone(&globals),
            environment: Rc::clone(&globals),
            locals: HashMap::new(),
            writer,
        }
    }
    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> () {
        for stmt in stmts {
            let _ = self.visit_stmt(stmt);
        }
    }
    pub fn interpret_block(
        &mut self,
        stmts: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        let previous = Rc::clone(&self.environment);
        self.environment = environment;
        for stmt in stmts {
            let s = self.visit_stmt(&stmt);
            if let Err(e) = s {
                self.environment = previous;
                return Err(e);
            }
        }
        self.environment = previous;
        Ok(())
    }
    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }
    fn look_up_variable(
        &mut self,
        name: &Token,
        expr: &Expr,
    ) -> Result<Rc<RefCell<Object>>, RuntimeError> {
        let distance = self.locals.get(expr);
        if let Some(d) = distance {
            return self.environment.borrow().get_at(*d, name.lexeme.clone());
        } else {
            self.globals.borrow().get(name.clone())
        }
    }
}

impl<'a> Visitor<Rc<RefCell<Object>>, ()> for Interpreter<'a> {
    fn visit_expr(&mut self, e: &Expr) -> Result<Rc<RefCell<Object>>, RuntimeError> {
        match e {
            Expr::Assign { name, value } => {
                let object = self.visit_expr(value)?;

                let distance = self.locals.get(e);
                match distance {
                    Some(d) => {
                        self.environment.borrow_mut().assign_at(
                            *d,
                            name.clone(),
                            Rc::clone(&object),
                        );
                    }
                    None => {
                        self.globals
                            .borrow_mut()
                            .assign(name.clone(), Rc::clone(&object))?;
                    }
                };

                Ok(object)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_obj = self.visit_expr(left)?;
                let right_obj = self.visit_expr(right)?;

                match operator.token_type {
                    TokenType::BangEqual => Ok(Rc::new(RefCell::new(Object::Bool(!is_equal(
                        left_obj, right_obj,
                    ))))),
                    TokenType::EqualEqual => Ok(Rc::new(RefCell::new(Object::Bool(is_equal(
                        left_obj, right_obj,
                    ))))),
                    TokenType::Greater => match (&*left_obj.borrow(), &*right_obj.borrow()) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Rc::new(RefCell::new(Object::Bool(l > r))))
                        }
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::GreaterEqual => match (&*left_obj.borrow(), &*right_obj.borrow()) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Rc::new(RefCell::new(Object::Bool(l >= r))))
                        }
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::Less => match (&*left_obj.borrow(), &*right_obj.borrow()) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Rc::new(RefCell::new(Object::Bool(l < r))))
                        }
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::LessEqual => match (&*left_obj.borrow(), &*right_obj.borrow()) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Rc::new(RefCell::new(Object::Bool(l <= r))))
                        }
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::Minus => match (&*left_obj.borrow(), &*right_obj.borrow()) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Rc::new(RefCell::new(Object::Number(l - r))))
                        }
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::Plus => match (&*left_obj.borrow(), &*right_obj.borrow()) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Rc::new(RefCell::new(Object::Number(l + r))))
                        }
                        (Object::String(l), Object::String(r)) => {
                            Ok(Rc::new(RefCell::new(Object::String(l.to_owned() + r))))
                        }
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be two numbers or two strings.",
                            None,
                        )),
                    },
                    TokenType::Slash => match (&*left_obj.borrow(), &*right_obj.borrow()) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Rc::new(RefCell::new(Object::Number(l / r))))
                        }
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::Star => match (&*left_obj.borrow(), &*right_obj.borrow()) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Rc::new(RefCell::new(Object::Number(l * r))))
                        }
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    _ => Err(RuntimeError::new(
                        operator.clone(),
                        "Invalid use of operator.",
                        None,
                    )),
                }
            }
            Expr::Call {
                callee: c,
                paren: p,
                arguments: a,
            } => {
                let callee = self.visit_expr(c)?;

                let mut arguments = vec![];
                for argument in a {
                    arguments.push(self.visit_expr(argument)?)
                }

                let x = match &*callee.borrow() {
                    Object::Function(func) => {
                        if arguments.len() != func.arity() {
                            return Err(RuntimeError::new(
                                p.clone(),
                                &("Expected ".to_owned()
                                    + &func.arity().to_string()
                                    + " arguments but got "
                                    + &arguments.len().to_string()
                                    + "."),
                                None,
                            ));
                        }
                        func.call(self, arguments)
                    }
                    Object::NativeFunction(func) => {
                        if arguments.len() != func.arity() {
                            return Err(RuntimeError::new(
                                p.clone(),
                                &("Expected ".to_owned()
                                    + &func.arity().to_string()
                                    + " arguments but got "
                                    + &arguments.len().to_string()
                                    + "."),
                                None,
                            ));
                        }
                        func.call(self, arguments)
                    }
                    Object::Class(class) => {
                        if arguments.len() != class.arity() {
                            return Err(RuntimeError::new(
                                p.clone(),
                                &("Expected ".to_owned()
                                    + &class.arity().to_string()
                                    + " arguments but got "
                                    + &arguments.len().to_string()
                                    + "."),
                                None,
                            ));
                        }
                        class.call(self, arguments)
                    }
                    _ => Err(RuntimeError::new(
                        p.clone(),
                        "Can only call functions and classes",
                        None,
                    )),
                };
                return x;
            }
            Expr::Get { object, name } => {
                let object = self.visit_expr(&object)?;
                if let Object::Instance(i) = &*object.borrow() {
                    return Ok(i.get(name)?);
                }
                Err(RuntimeError::new(
                    name.clone(),
                    "Only instances have properties.",
                    None,
                ))
            }
            Expr::Grouping { expression } => self.visit_expr(expression),
            Expr::Literal { value } => Ok(Rc::new(RefCell::new(value.clone()))),
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expr(left)?;

                if operator.token_type == TokenType::Or {
                    if is_truthy(&*left.borrow()) {
                        return Ok(left);
                    }
                } else {
                    if !is_truthy(&*left.borrow()) {
                        return Ok(left);
                    }
                }
                self.visit_expr(right)
            }
            Expr::Set {
                object,
                name,
                value,
            } => {
                let object = self.visit_expr(&object)?;
                if let Object::Instance(i) = &mut *object.borrow_mut() {
                    let value = self.visit_expr(value)?;
                    i.set(name, Rc::clone(&value));
                    return Ok(value);
                }
                Err(RuntimeError::new(
                    name.clone(),
                    "Only instances have fields.",
                    None,
                ))
            }
            Expr::This { keyword } => self.look_up_variable(keyword, e),

            Expr::Unary { operator, right } => {
                let obj = self.visit_expr(right)?;
                match operator.token_type {
                    TokenType::Bang => Ok(Rc::new(RefCell::new(Object::Bool(is_truthy(
                        &*obj.borrow(),
                    ))))),
                    TokenType::Minus => match &*obj.borrow() {
                        Object::Number(n) => Ok(Rc::new(RefCell::new(Object::Number(-n)))),
                        _ => Err(RuntimeError::new(
                            operator.clone(),
                            "Operand must be a number",
                            None,
                        )),
                    },
                    _ => Ok(Rc::new(RefCell::new(Object::Nil))),
                }
            }
            Expr::Variable { name } => self.look_up_variable(name, e),
        }
    }
    fn visit_stmt(&mut self, s: &Stmt) -> Result<(), RuntimeError> {
        match s {
            Stmt::Expr(e) => {
                self.visit_expr(e)?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&*self.visit_expr(condition)?.borrow()) {
                    self.visit_stmt(&then_branch)?;
                } else {
                    match else_branch {
                        Some(s) => {
                            self.visit_stmt(s)?;
                        }
                        None => {}
                    }
                }
            }
            Stmt::Print(e) => {
                let obj = self.visit_expr(e)?;
                let buffer = format!("{}\n", obj.borrow());
                self.writer
                    .write_all(buffer.as_bytes())
                    .expect("Error writing to writer");
            }
            Stmt::Return { keyword, value } => {
                let ret = self.visit_expr(value);
                match ret {
                    Ok(o) => {
                        return Err(RuntimeError::new(keyword.clone(), "", Some(Rc::clone(&o))));
                    }
                    Err(e) if e.value.is_some() => {
                        return Err(e);
                    }
                    Err(e) => return Err(e),
                }
            }
            Stmt::Var { name, initializer } => {
                let mut value = Rc::new(RefCell::new(Object::Nil));
                match initializer {
                    Some(i) => {
                        value = self.visit_expr(i)?;
                    }
                    None => {}
                }
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value);
            }
            Stmt::Block { statements } => {
                self.interpret_block(
                    statements,
                    Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                        &(self.environment),
                    ))))),
                )?;
            }
            Stmt::Class {
                name,
                methods: stmt_methods,
            } => {
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), Rc::new(RefCell::new(Object::Nil)));
                let mut methods = HashMap::new();
                for method in stmt_methods {
                    let function = Function::new(method.clone(), Rc::clone(&self.environment));
                    if let Stmt::Function {
                        name,
                        params: _,
                        body: _,
                    } = method
                    {
                        methods.insert(name.lexeme.clone(), function);
                    }
                }

                let klass = Rc::new(RefCell::new(Object::Class(Class::new(
                    name.lexeme.clone(),
                    methods,
                ))));
                self.environment.borrow_mut().assign(name.clone(), klass)?;
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.visit_expr(condition)?.borrow()) {
                    self.visit_stmt(body)?;
                }
            }
            Stmt::Function {
                name,
                params: _,
                body: _,
            } => {
                let function = Function::new(s.clone(), Rc::clone(&self.environment));
                self.environment.borrow_mut().define(
                    name.lexeme.clone(),
                    Rc::new(RefCell::new(Object::Function(Box::new(function)))),
                );
            }
        };
        Ok(())
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Nil => false,
        Object::Bool(b) => *b,
        _ => true,
    }
}

fn is_equal(l_obj: Rc<RefCell<Object>>, r_obj: Rc<RefCell<Object>>) -> bool {
    match (&*l_obj.borrow(), &*r_obj.borrow()) {
        (Object::Number(l), Object::Number(r)) => l == r,
        (Object::String(l), Object::String(r)) => l == r,
        (Object::Bool(l), Object::Bool(r)) => l == r,
        (Object::Nil, Object::Nil) => true,
        (_, _) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn unary() {
        let mut writer: Box<dyn std::io::Write + 'static> = Box::new(std::io::stdout());
        let mut interpreter = Interpreter::new(&mut writer);
        let unary_expression = Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: String::from("-"),
                literal: None,
                line: 0,
                position: 0,
            },
            right: Box::new(Expr::Literal {
                value: Object::Number(1.0),
            }),
        };
        match interpreter.visit_expr(&unary_expression) {
            Ok(r) => assert_eq!(*r.borrow(), Object::Number(-1.0)),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn assignment() {
        let mut writer: Box<dyn std::io::Write + 'static> = Box::new(std::io::stdout());
        let mut _interpreter = Interpreter::new(&mut writer);
        let _assignment_expression = Expr::Assign {
            name: Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("a"),
                literal: None,
                line: 0,
                position: 0,
            },
            value: Box::new(Expr::Literal {
                value: Object::Number(1.0),
            }),
        };
    }
}
