use std::collections::HashMap;

use crate::ast::Expr;
use crate::ast::Visitor;
use crate::error::RuntimeError;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token::Token;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
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
        let mut scope = self.scopes.pop().unwrap();
        if scope.contains_key(&name.lexeme) {
            return Err(RuntimeError::new(
                name.clone(),
                &"Already a variable with this name in this scope.",
                None,
            ));
        }
        scope.insert(name.lexeme.clone(), false);
        println!("declare scope: {:?}", scope);
        self.scopes.push(scope);
        println!("declare self.scopes: {:?}", self.scopes);
        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let mut scope = self.scopes.pop().unwrap();
        scope.insert(name.lexeme.clone(), true);
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        println!("resolve_local: {:?}", self.scopes);
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
        if let Stmt::Function { name, params, body } = stmt {
            println!("resolve_function stmt {:?}", stmt);
            let enclosing_function = self.current_function.clone();
            self.current_function = function_type;
            self.begin_scope();
            println!("resolve_function params {:?}", params);
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

impl<'a> Visitor<(), ()> for Resolver<'a> {
    fn visit_expr(&mut self, e: &Expr) -> Result<(), RuntimeError> {
        match e {
            Expr::Assign { name, value } => {
                self.visit_expr(value)?;
                self.resolve_local(value, name);
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
                self.resolve_local(e, keyword);
                Ok(())
            }
            Expr::Unary { operator: _, right } => self.visit_expr(right),
            Expr::Variable { name } => {
                println!("resolve: visit variable stmt {:?}", name);
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
            Stmt::Class { name, methods } => {
                self.declare(name)?;
                self.define(name);

                self.begin_scope();
                let mut scope = self.scopes.pop().unwrap();
                scope.insert("this".into(), true);
                self.scopes.push(scope);
                println!("visit class stmt {:?}", self.scopes);

                for method in methods {
                    self.resolve_function(method, FunctionType::Method)?;
                }

                self.end_scope();
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
                self.visit_expr(value)?;
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
    Method,
}
