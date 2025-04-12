use crate::{
    expr::{Binary, Expr, Grouping, Literal, Super, This, Unary, Variable},
    report,
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

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        match self.expression() {
            Ok(expr) => Ok(expr),
            Err(_) => Err(ParserError),
        }
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
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
