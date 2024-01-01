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
                Expr::Variable { name } => {
                    return Ok(Expr::Assign {
                        name: name.clone(),
                        value: Box::new(v),
                    });
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
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.equality()?;

        while self.matches(&vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.matches(&vec![TokenType::Var]) {
            let declared_var = self.var_declaration();
            match declared_var {
                Ok(s) => return Some(s),
                Err(_) => {
                    self.synchronize();
                    return None;
                }
            }
        }
        let stmt = self.statement();
        match stmt {
            Ok(s) => return Some(s),
            Err(_) => {
                self.synchronize();
                return None;
            }
        };
    }

    fn statement(&mut self) -> Result<Stmt, SyntaxError> {
        if self.matches(&vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.matches(&vec![TokenType::For]) {
            return self.for_statement();
        }
        if self.matches(&vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.matches(&vec![TokenType::While]) {
            return self.while_statement();
        }
        if self.matches(&vec![TokenType::LeftBrace]) {
            return Ok(Stmt::Block {
                statements: self.block()?,
            });
        }
        return self.expression_statement();
    }

    fn for_statement(&mut self) -> Result<Stmt, SyntaxError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer;
        if self.matches(&vec![TokenType::Semicolon]) {
            initializer = None;
        } else if self.matches(&vec![TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition = None;
        if !self.check(&TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment = None;
        if !self.check(&TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(&TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        match increment {
            Some(i) => {
                body = Stmt::Block {
                    statements: vec![body, Stmt::Expr(i)],
                }
            }
            None => {}
        }

        if condition.is_none() {
            condition = Some(Expr::Literal {
                value: Object::Bool(true),
            });
        }

        match condition {
            Some(c) => {
                body = Stmt::While {
                    condition: c,
                    body: Box::new(body),
                }
            }
            None => {}
        }

        match initializer {
            Some(i) => {
                body = Stmt::Block {
                    statements: vec![i, body],
                }
            }
            None => {}
        }

        return Ok(body);
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

    fn while_statement(&mut self) -> Result<Stmt, SyntaxError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;
        return Ok(Stmt::While {
            condition,
            body: Box::new(body),
        });
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
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
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
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.factor()?;
        while self.matches(&vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.unary()?;
        while self.matches(&vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, SyntaxError> {
        if self.matches(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator: operator.clone(),
                right: Box::new(right),
            });
        }
        return self.call();
    }

    fn finish_call(&mut self, callee: &Expr) -> Result<Expr, SyntaxError> {
        let mut arguments = vec![];
        if !self.check(&TokenType::RightParen) {
            while self.matches(&vec![TokenType::Comma]) {
                let expression = Box::new(self.expression()?);
                arguments.push(expression);
            }
        }
        let paren = self.consume(&TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call {
            callee: Box::new(callee.clone()),
            paren,
            arguments,
        })
    }

    fn call(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.primary()?;
        loop {
            if self.matches(&vec![TokenType::LeftParen]) {
                expr = self.finish_call(&expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, SyntaxError> {
        if self.matches(&vec![TokenType::False]) {
            return Ok(Expr::Literal {
                value: Object::Bool(false),
            });
        }
        if self.matches(&vec![TokenType::True]) {
            return Ok(Expr::Literal {
                value: Object::Bool(true),
            });
        }
        if self.matches(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal { value: Object::Nil });
        }
        if self.matches(&vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal {
                value: self.previous().literal.expect("No literal found in token"),
            });
        }
        if self.matches(&vec![TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous(),
            });
        }

        if self.matches(&vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, &"Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
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
