use crate::token::{LiteralKind, Token, TokenKind, KEYWORDS};

//lexer
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_errors: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            has_errors: false,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenKind::EOF,
            "".into(),
            LiteralKind::Nil,
            self.line,
        ));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenKind::LeftParenthesis, LiteralKind::Nil),
            ')' => self.add_token(TokenKind::RightParenthesis, LiteralKind::Nil),
            '{' => self.add_token(TokenKind::LeftBrace, LiteralKind::Nil),
            '}' => self.add_token(TokenKind::RightBrace, LiteralKind::Nil),
            ',' => self.add_token(TokenKind::Comma, LiteralKind::Nil),
            '.' => self.add_token(TokenKind::Dot, LiteralKind::Nil),
            '-' => self.add_token(TokenKind::Minus, LiteralKind::Nil),
            '+' => self.add_token(TokenKind::Plus, LiteralKind::Nil),
            ';' => self.add_token(TokenKind::Semicolon, LiteralKind::Nil),
            '*' => self.add_token(TokenKind::Star, LiteralKind::Nil),
            '!' => {
                let kind = match self.is_next_expected('=') {
                    true => TokenKind::BangEqual,
                    false => TokenKind::Bang,
                };
                self.add_token(kind, LiteralKind::Nil);
            }
            '=' => {
                let kind = match self.is_next_expected('=') {
                    true => TokenKind::EqualEqual,
                    false => TokenKind::Equal,
                };
                self.add_token(kind, LiteralKind::Nil);
            }
            '<' => {
                let kind = match self.is_next_expected('=') {
                    true => TokenKind::LessEqual,
                    false => TokenKind::Less,
                };
                self.add_token(kind, LiteralKind::Nil);
            }
            '>' => {
                let kind = match self.is_next_expected('=') {
                    true => TokenKind::GreaterEqual,
                    false => TokenKind::Greater,
                };
                self.add_token(kind, LiteralKind::Nil);
            }
            '/' => match self.is_next_expected('/') {
                true => {
                    //comments
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenKind::Slash, LiteralKind::Nil),
            },
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }

                if self.is_at_end() {
                    self.has_errors = true;
                    eprintln!("[line {}] Error: Unterminated string.", self.line);
                    return;
                }

                self.advance();
                let literal: String = self.source[self.start + 1..self.current - 1]
                    .iter()
                    .collect();
                self.add_token(TokenKind::String, LiteralKind::String(literal));
            }
            c if c.is_digit(10) => {
                while self.peek().is_digit(10) {
                    self.advance();
                }

                if self.peek() == '.' && self.peek_next().is_digit(10) {
                    self.advance();
                    while self.peek().is_digit(10) {
                        self.advance();
                    }
                }

                let literal: f64 = self.source[self.start..self.current]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();

                self.add_token(TokenKind::Number, LiteralKind::Number(literal));
            }
            c if c.is_alphabetic() || c == '_' => {
                while self.peek().is_alphanumeric() || self.peek() == '_' {
                    self.advance();
                }

                let lexume: String = self.source[self.start..self.current].iter().collect();
                match KEYWORDS.get(&lexume.as_str()) {
                    Some(kind) => self.add_token(*kind, LiteralKind::Nil),
                    None => self.add_token(TokenKind::Identifier, LiteralKind::Nil),
                }
            }
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
                self.has_errors = true;
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, kind: TokenKind, literal: LiteralKind) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(kind, lexeme, literal, self.line));
    }

    fn is_next_expected(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    pub fn errors(&self) -> bool {
        self.has_errors
    }
}
