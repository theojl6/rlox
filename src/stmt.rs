use crate::{
    ast::{Expr, Visitor},
    token::{Token, TokenType},
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

impl Visitor<Stmt, ()> for Statement {
    fn visit_expr(&mut self, stmt: &Stmt) {
        match stmt {
            (_) => {}
        }
    }
}

impl Statement {
    fn parse(&self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while (!self.is_at_end()) {
            statements.push(self.statement());
        }

        return statements;
    }

    fn statement(&self) -> Stmt {
        if (self.matches(TokenType::Print)) {
            return self.print_statement();
        }
        return self.expression_statement();
    }

    fn print_statement(&self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        return Stmt::Print(value);
    }

    fn expression_statement(&self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        return Stmt::Expression(expr);
    }

    fn is_at_end(&self) -> bool {
        todo!();
    }

    fn expression(&self) -> Expr {
        todo!();
    }
}
