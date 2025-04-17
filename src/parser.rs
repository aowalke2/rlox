use crate::{
    expr::{Assignment, Binary, Expr, Grouping, Literal, Logical, Super, This, Unary, Variable},
    stmt::{Block, Expression, If, Print, Stmt, Var, While},
    token::{LiteralKind, Token, TokenKind},
};

#[derive(Debug)]
pub struct ParserError;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = Vec::new();
        let mut has_error = false;
        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(_) => has_error = true,
            }
        }

        match has_error {
            false => Ok(statements),
            true => Err(ParserError),
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expr, ParserError> {
        match self.assignment() {
            Ok(expr) => Ok(expr),
            Err(_) => Err(ParserError),
        }
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        let statement = if self.token_match(&[TokenKind::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match &statement {
            Ok(_) => statement,
            Err(_) => {
                self.synchronize();
                Err(ParserError)
            }
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.token_match(&[TokenKind::For]) {
            return self.for_statement();
        }
        if self.token_match(&[TokenKind::If]) {
            return self.if_statement();
        }
        if self.token_match(&[TokenKind::Print]) {
            return self.print_statement();
        }
        if self.token_match(&[TokenKind::While]) {
            return self.while_statement();
        }
        if self.token_match(&[TokenKind::LeftBrace]) {
            return Ok(Stmt::Block(Block {
                statements: self.block()?,
            }));
        }
        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenKind::LeftParenthesis, "Expect '(' after 'for'.")?;
        let initializer = if self.token_match(&[TokenKind::Semicolon]) {
            None
        } else if self.token_match(&[TokenKind::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(&TokenKind::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal(Literal {
                value: LiteralKind::Bool(true),
            })
        };
        self.consume(TokenKind::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(&TokenKind::RightParenthesis) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenKind::RightParenthesis, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = Stmt::Block(Block {
                statements: Vec::from([
                    body,
                    Stmt::Expression(Expression {
                        expression: Box::new(increment),
                    }),
                ]),
            });
        };

        body = Stmt::While(While {
            condition: Box::new(condition),
            body: Box::new(body),
        });

        if let Some(initializer) = initializer {
            body = Stmt::Block(Block {
                statements: Vec::from([initializer, body]),
            })
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenKind::LeftParenthesis, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(
            TokenKind::RightParenthesis,
            "Expect ')' after if condition.",
        )?;

        let then_branch = self.statement()?;
        let else_branch = if self.token_match(&[TokenKind::Else]) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Stmt::If(If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Print {
            expression: Box::new(value),
        }))
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenKind::Identifier, "Expect variable name.")?;
        let mut initializer = Expr::Literal(Literal {
            value: LiteralKind::Nil,
        });
        if self.token_match(&[TokenKind::Equal]) {
            initializer = self.expression()?;
        }
        self.consume(
            TokenKind::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var(Var {
            name,
            initializer: Box::new(initializer),
        }))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenKind::LeftParenthesis, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenKind::RightParenthesis, "Expect ')' after condition.")?;
        let body = self.statement()?;
        Ok(Stmt::While(While {
            condition: Box::new(condition),
            body: Box::new(body),
        }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(Expression {
            expression: Box::new(expr),
        }))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenKind::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.or()?;

        if self.token_match(&[TokenKind::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable(variable) = expr {
                return Ok(Expr::Assignment(Assignment {
                    name: variable.name,
                    value: Box::new(value),
                }));
            } else {
                self.error(&equals, "Invalid assignment target.");
                return Err(ParserError);
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.and()?;
        while self.token_match(&[TokenKind::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.equality()?;
        while self.token_match(&[TokenKind::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison();
        while self.token_match(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            }))
        }

        expr
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term();
        while self.token_match(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            }))
        }

        expr
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor();
        while self.token_match(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            }))
        }

        expr
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary();
        while self.token_match(&[TokenKind::Slash, TokenKind::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Ok(Expr::Binary(Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            }))
        }

        expr
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.token_match(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        match self.peek().kind {
            TokenKind::False => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value: LiteralKind::Bool(false),
                }))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value: LiteralKind::Bool(true),
                }))
            }
            TokenKind::Nil => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value: LiteralKind::Nil,
                }))
            }
            TokenKind::String | TokenKind::Number => {
                self.advance();
                Ok(Expr::Literal(Literal {
                    value: self.previous().literal,
                }))
            }
            TokenKind::Super => {
                self.advance();
                let keyword = self.previous();
                self.consume(TokenKind::Dot, "Expect '.' after 'super'.")?;
                let method =
                    self.consume(TokenKind::Identifier, "Expect superclass method name.")?;
                Ok(Expr::Super(Super { keyword, method }))
            }
            TokenKind::This => {
                self.advance();
                Ok(Expr::This(This {
                    keyword: self.previous(),
                }))
            }
            TokenKind::Identifier => {
                self.advance();
                Ok(Expr::Variable(Variable {
                    name: self.previous(),
                }))
            }
            TokenKind::LeftParenthesis => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenKind::RightParenthesis, "Expect ')' after expression.")?;
                Ok(Expr::Grouping(Grouping {
                    expr: Box::new(expr),
                }))
            }
            _ => {
                self.error(self.peek(), "Expect expression.");
                self.advance();
                Err(ParserError {})
            }
        }
    }

    fn token_match(&mut self, tokens: &[TokenKind]) -> bool {
        for token in tokens.iter() {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == *token
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, ParserError> {
        if !self.check(&kind) {
            self.error(&self.previous(), message);
            return Err(ParserError);
        }

        self.advance();
        Ok(self.previous())
    }

    fn error(&self, token: &Token, message: &str) {
        crate::error(token.clone(), message);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }

            match self.peek().kind {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return,
                _ => self.advance(),
            }
        }
    }
}
