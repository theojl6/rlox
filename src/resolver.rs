use std::collections::HashMap;

use crate::ast::Expr;
use crate::ast::Visitor;
use crate::error::RuntimeError;
use crate::interpreter::Interpreter;
use crate::interpreter::Object;
use crate::stmt::Stmt;
use crate::token::Token;

pub struct Resolver<'a> {
    pub interpreter: Interpreter<'a>,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl Resolver<'_> {
    pub fn new(interpreter: Interpreter) -> Resolver {
        Resolver {
            interpreter,
            // this only tracks local block scopes, variables declared at the top level in the global scope
            // are NOT tracked
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
        }
    }

    pub fn resolve_stmts(&mut self, statements: &Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.visit_stmt(statement)?;
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> Result<(), RuntimeError> {
        if self.scopes.is_empty() {
            return Ok(());
        }
        let scope = self.scopes.last_mut().unwrap();
        if scope.contains_key(&name.lexeme) {
            return Err(RuntimeError::new(
                name.clone(),
                &"Already a variable with this name in this scope.",
                None,
            ));
        }
        scope.insert(name.lexeme.clone(), false);
        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name.lexeme.clone(), true);
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
                return;
            }
        }
    }
    fn resolve_function(
        &mut self,
        stmt: &Stmt,
        function_type: FunctionType,
    ) -> Result<(), RuntimeError> {
        if let Stmt::Function {
            name: _,
            params,
            body,
        } = stmt
        {
            let enclosing_function = self.current_function.clone();
            self.current_function = function_type;
            self.begin_scope();
            for param in params {
                self.declare(param)?;
                self.define(param);
            }
            self.resolve_stmts(body)?;
            self.end_scope();
            self.current_function = enclosing_function;
        }
        Ok(())
    }
}

impl<'a> Visitor<(), ()> for Resolver<'_> {
    fn visit_expr(&mut self, e: &Expr) -> Result<(), RuntimeError> {
        match e {
            Expr::Assign { name, value } => {
                self.visit_expr(value)?;
                self.resolve_local(e, name);
                Ok(())
            }
            Expr::Binary {
                left,
                operator: _,
                right,
            } => {
                self.visit_expr(left)?;
                self.visit_expr(right)?;
                Ok(())
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                self.visit_expr(callee)?;

                for argument in arguments {
                    self.visit_expr(argument)?;
                }
                Ok(())
            }
            Expr::Get { object, name: _ } => {
                self.visit_expr(&object)?;
                Ok(())
            }
            Expr::Grouping { expression } => self.visit_expr(expression),
            Expr::Literal { value: _ } => Ok(()),
            Expr::Logical {
                left,
                operator: _,
                right,
            } => {
                self.visit_expr(left)?;
                self.visit_expr(right)?;
                Ok(())
            }
            Expr::Set {
                object,
                name: _,
                value,
            } => {
                self.visit_expr(value)?;
                self.visit_expr(object)?;
                Ok(())
            }
            Expr::This { keyword } => {
                if self.current_class == ClassType::None {
                    return Err(RuntimeError::new(
                        keyword.clone(),
                        "Can't use 'this' keyword outside of a class.",
                        None,
                    ));
                }
                self.resolve_local(e, keyword);
                Ok(())
            }
            Expr::Unary { operator: _, right } => self.visit_expr(right),
            Expr::Variable { name } => {
                if !self.scopes.is_empty()
                    && self
                        .scopes
                        .last()
                        .unwrap()
                        .get(&name.lexeme)
                        .is_some_and(|b| *b == false)
                {
                    return Err(RuntimeError::new(
                        name.clone(),
                        "Can't read local variable in its own initializer.",
                        None,
                    ));
                }
                self.resolve_local(e, name);
                Ok(())
            }
        }
    }

    fn visit_stmt(&mut self, s: &crate::stmt::Stmt) -> Result<(), RuntimeError> {
        match s {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_stmts(statements)?;
                self.end_scope();
                Ok(())
            }
            Stmt::Class {
                name,
                methods,
                superclass,
            } => {
                let enclosing_class = self.current_class.clone();
                self.current_class = ClassType::Class;
                self.declare(name)?;
                self.define(name);

                if let Some(sc) = superclass {
                    if let Expr::Variable {
                        name: variable_name,
                    } = sc
                    {
                        if name.lexeme == variable_name.lexeme {
                            return Err(RuntimeError::new(
                                variable_name.clone(),
                                "A class can't inherit from itself",
                                None,
                            ));
                        }
                    }
                }

                if let Some(sc) = superclass {
                    self.visit_expr(sc)?;
                }

                self.begin_scope();
                let scope = self.scopes.last_mut().unwrap();
                scope.insert("this".into(), true);

                for method in methods {
                    let mut declaration = FunctionType::Method;
                    if let Stmt::Function { name, .. } = method {
                        if name.lexeme == "init" {
                            declaration = FunctionType::Initializer;
                        }
                    }
                    self.resolve_function(method, declaration)?;
                }

                self.end_scope();
                self.current_class = enclosing_class;
                Ok(())
            }
            Stmt::Expr(e) => self.visit_expr(e),
            Stmt::Function {
                name,
                params: _,
                body: _,
            } => {
                self.declare(name)?;
                self.define(name);
                self.resolve_function(s, FunctionType::Function)?;
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expr(condition)?;
                self.visit_stmt(&then_branch)?;
                if let Some(s) = else_branch {
                    self.visit_stmt(s)?;
                }
                Ok(())
            }
            Stmt::Print(e) => self.visit_expr(e),
            Stmt::Return { keyword, value } => {
                if self.current_function == FunctionType::None {
                    return Err(RuntimeError::new(
                        keyword.clone(),
                        "Can't return from top-level code.",
                        None,
                    ));
                }
                if let Expr::Literal {
                    value: literal_value,
                } = value
                {
                    if *literal_value != Object::Nil {
                        if self.current_function == FunctionType::Initializer {
                            return Err(RuntimeError::new(
                                keyword.clone(),
                                "Can't return a value from an initializer.",
                                None,
                            ));
                        }
                        self.visit_expr(value)?;
                    }
                } else {
                    self.visit_expr(value)?;
                }
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                self.declare(name)?;
                if let Some(i) = initializer {
                    self.visit_expr(&i)?;
                }
                self.define(name);
                Ok(())
            }
            Stmt::While { condition, body } => {
                self.visit_expr(condition)?;
                self.visit_stmt(body)?;
                Ok(())
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Clone, PartialEq)]
enum ClassType {
    None,
    Class,
}
