use std::error::Error;

use crate::ast::Expr;
use crate::token::{Literal, Token, TokenType};
pub struct Parser<'a> {
    pub tokens: &'a Vec<Token>,
    pub current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.matches(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }
        expr
    }

    fn matches(&mut self, token_types: &Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.matches(&vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }
        expr
    }

    fn term(&self) -> Expr {
        let mut expr = self.factor();
        while self.matches(&vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }
        expr
    }

    fn factor(&self) -> Expr {
        let mut expr = self.unary();
        while self.matches(&vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }
        expr
    }

    fn unary(&self) -> Expr {
        if self.matches(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(operator.clone(), Box::new(right));
        }
        return self.primary();
    }

    fn primary(&self) -> Result<Expr, Box<dyn Error>> {
        if self.matches(&vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal::False));
        }
        if self.matches(&vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal::True));
        }
        if self.matches(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if self.matches(&vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal.expect("BOOM")));
        }
        if self.matches(&vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, &"Expect ')' after expression.");
            return Ok(Expr::Grouping(expr));
        }
        return Err("Expected expression, found BOOM".into());
    }
}
