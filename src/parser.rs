use crate::ast::Expr;
use crate::error::SyntaxError;
use crate::interpreter::Object;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
pub struct Parser<'a> {
    pub tokens: &'a Vec<Token>,
    pub current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, SyntaxError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            let declaration = self.declaration();
            match declaration {
                Some(d) => {
                    statements.push(d);
                }
                None => {}
            }
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, SyntaxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, SyntaxError> {
        let expr = self.or()?;
        if self.matches(&vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            let v = value.clone();

            match expr {
                Expr::Variable(t) => {
                    return Ok(Expr::Assign(t.clone(), Box::new(v)));
                }
                _ => {
                    return Err(SyntaxError::new(
                        equals.clone(),
                        "Invalid assignment target.",
                    ));
                }
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.and()?;

        while self.matches(&vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }
        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.equality()?;

        while self.matches(&vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }
        return Ok(expr);
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.matches(&vec![TokenType::Var]) {
            let declared_var = self.var_declaration();
            match declared_var {
                Ok(s) => return Some(s),
                Err(e) => {
                    self.synchronize();
                    return None;
                }
            }
        }
        let stmt = self.statement();
        match stmt {
            Ok(s) => return Some(s),
            Err(e) => {
                self.synchronize();
                return None;
            }
        };
    }

    fn statement(&mut self) -> Result<Stmt, SyntaxError> {
        if self.matches(&vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.matches(&vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.matches(&vec![TokenType::LeftBrace]) {
            return Ok(Stmt::Block {
                statements: self.block()?,
            });
        }
        return self.expression_statement();
    }

    fn if_statement(&mut self) -> Result<Stmt, SyntaxError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;

        if self.matches(&vec![TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }
        return Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        });
    }

    fn print_statement(&mut self) -> Result<Stmt, SyntaxError> {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    fn var_declaration(&mut self) -> Result<Stmt, SyntaxError> {
        let name = self.consume(&TokenType::Identifier, "Expect variable name.")?;
        let mut initializer = None;
        if self.matches(&vec![TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }
        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after variable declaration",
        )?;
        return Ok(Stmt::Var { name, initializer });
    }

    fn expression_statement(&mut self) -> Result<Stmt, SyntaxError> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expr(expr))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, SyntaxError> {
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Some(d) => statements.push(d),
                None => {}
            }
        }
        self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;
        return Ok(statements);
    }

    fn equality(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.comparison()?;

        while self.matches(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }
        Ok(expr)
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

    fn advance(&mut self) -> Token {
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

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }
    }

    fn comparison(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.term()?;
        while self.matches(&vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.factor()?;
        while self.matches(&vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.unary()?;
        while self.matches(&vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, SyntaxError> {
        if self.matches(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator.clone(), Box::new(right)));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, SyntaxError> {
        if self.matches(&vec![TokenType::False]) {
            return Ok(Expr::Literal(Object::Bool(false)));
        }
        if self.matches(&vec![TokenType::True]) {
            return Ok(Expr::Literal(Object::Bool(true)));
        }
        if self.matches(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Object::Nil));
        }
        if self.matches(&vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(
                self.previous().literal.expect("No literal found in token"),
            ));
        }
        if self.matches(&vec![TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous()));
        }

        if self.matches(&vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, &"Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        return Err(SyntaxError::new(
            self.tokens[self.current].clone(),
            &"Expected expression.",
        ));
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<Token, SyntaxError> {
        if self.check(token_type) == true {
            return Ok(self.advance());
        }

        return Err(SyntaxError::new(self.tokens[self.current].clone(), message));
    }
}
