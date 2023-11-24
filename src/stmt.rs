use crate::{
    ast::{Expr, Visitor},
    token::Token,
};

pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Class {
        name: Token,
        superclass: Expr,
        methods: Vec<Stmt>,
    },
    Expression(Expr),
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        thenBranch: Box<Stmt>,
        elseBranch: Box<Stmt>,
    },
    Print(Expr),
    Return {
        keyword: Token,
        value: Expr,
    },
    Var {
        name: Token,
        intializer: Expr,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

pub struct Statement;

impl Visitor<Stmt, Stmt> for Statement {
    fn visit_expr(&mut self, stmt: &Stmt) -> Stmt {
        todo!();
    }
}
