use crate::ast::{Expr, Visitor};
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::function::{Function, NativeFunction};
use crate::stmt::Stmt;
use crate::token::TokenType;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f32),
    Bool(bool),
    Nil,
    Function(Box<Function>),
    NativeFunction(NativeFunction),
}

pub trait Callable {
    fn call(
        &self,
        interpretor: &mut Interpretor,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>
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

pub struct Interpretor {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpretor {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        globals.borrow_mut().define(
            String::from("clock"),
            Object::NativeFunction(NativeFunction::new(0, || {
                let start = SystemTime::now();
                let since_the_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis();
                println!("{:?}", since_the_epoch);
            })),
        );
        Interpretor {
            globals: Rc::clone(&globals),
            environment: Rc::clone(&globals),
        }
    }
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> () {
        for stmt in stmts {
            let _ = self.visit_stmt(&stmt);
        }
    }
    pub fn interpret_block(
        &mut self,
        stmts: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        let previous = self.environment.clone();
        self.environment = environment;
        for stmt in stmts {
            self.visit_stmt(&stmt)?;
        }
        self.environment = previous;
        Ok(())
    }
}

impl Visitor<Object> for Interpretor {
    fn visit_expr(&mut self, e: &Expr) -> Result<Object, RuntimeError> {
        match e {
            Expr::Assign { name, value } => {
                let value = self.visit_expr(value)?;
                let v = value.clone();
                self.environment.borrow_mut().assign(name.clone(), v)?;
                Ok(value)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_obj = self.visit_expr(left)?;
                let right_obj = self.visit_expr(right)?;

                match operator.token_type {
                    TokenType::BangEqual => Ok(Object::Bool(!is_equal(&left_obj, &right_obj))),
                    TokenType::EqualEqual => Ok(Object::Bool(is_equal(&left_obj, &right_obj))),
                    TokenType::Greater => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l > r)),
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::GreaterEqual => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l >= r)),
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::Less => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l < r)),
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::LessEqual => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l <= r)),
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::Minus => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l - r)),
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::Plus => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::String(l + &r)),
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be two numbers or two strings.",
                            None,
                        )),
                    },
                    TokenType::Slash => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l / r)),
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    TokenType::Star => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l * r)),
                        (_, _) => Err(RuntimeError::new(
                            operator.clone(),
                            "Operands must be numbers.",
                            None,
                        )),
                    },
                    _ => Ok(Object::Nil),
                }
            }
            Expr::Call {
                callee: c,
                paren: p,
                arguments: a,
            } => {
                let callee = self.visit_expr(&c)?;

                let mut arguments = vec![];
                for argument in a {
                    arguments.push(self.visit_expr(argument)?)
                }

                match callee {
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
                    _ => Err(RuntimeError::new(
                        p.clone(),
                        "Can only call functions and classes",
                        None,
                    )),
                }
            }

            Expr::Grouping { expression } => self.visit_expr(expression),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expr(left)?;

                if operator.token_type == TokenType::Or {
                    if is_truthy(&left) {
                        return Ok(left);
                    }
                } else {
                    if !is_truthy(&left) {
                        return Ok(left);
                    }
                }
                self.visit_expr(right)
            }

            Expr::Unary { operator, right } => {
                let obj: Object = self.visit_expr(right)?;
                match operator.token_type {
                    TokenType::Bang => Ok(Object::Bool(is_truthy(&obj))),
                    TokenType::Minus => match obj {
                        Object::Number(n) => Ok(Object::Number(-n)),
                        _ => Err(RuntimeError::new(
                            operator.clone(),
                            "Operand must be a number",
                            None,
                        )),
                    },
                    _ => Ok(Object::Nil),
                }
            }
            Expr::Variable { name } => {
                let value = self.environment.borrow().get(name.clone())?;
                Ok(value.clone())
            }
        }
    }
    fn visit_stmt(&mut self, s: &Stmt) -> Result<(), RuntimeError> {
        match s {
            Stmt::Expr(e) => {
                let _ = self.visit_expr(e)?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&self.visit_expr(condition)?) {
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
                println!("{obj}");
            }
            Stmt::Return { keyword, value } => {
                let mut return_value = Object::Nil;
                if let Expr::Literal { .. } = value {
                } else {
                    println!("returned expression: {:?}", value);
                    return_value = self.visit_expr(value)?;
                }
                return Err(RuntimeError::new(keyword.clone(), "", Some(return_value)));
            }
            Stmt::Var { name, initializer } => {
                let mut value = Object::Nil;
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
                let _ = self.interpret_block(
                    statements,
                    Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                        &(self.environment),
                    ))))),
                );
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.visit_expr(condition)?) {
                    self.visit_stmt(body)?;
                }
            }
            Stmt::Function {
                name,
                params: _,
                body: _,
            } => {
                let function = Function::new(s.clone());
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), Object::Function(Box::new(function)));
            }

            _ => {}
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

fn is_equal(l_obj: &Object, r_obj: &Object) -> bool {
    match (l_obj, r_obj) {
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
        let mut interpretor = Interpretor::new();
        let unary_expression = Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: String::from("-"),
                literal: None,
                line: 0,
            },
            right: Box::new(Expr::Literal {
                value: Object::Number(1.0),
            }),
        };
        match interpretor.visit_expr(&unary_expression) {
            Ok(r) => assert_eq!(r, Object::Number(-1.0)),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn assignment() {
        let mut _interpretor = Interpretor::new();
        let _assignment_expression = Expr::Assign {
            name: Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("a"),
                literal: None,
                line: 0,
            },
            value: Box::new(Expr::Literal {
                value: Object::Number(1.0),
            }),
        };
    }
}
