use crate::token::{Token, TokenKind, KEYWORDS};

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

        self.tokens
            .push(Token::new(TokenKind::EOF, "".into(), None, self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenKind::LeftParanthesis, None),
            ')' => self.add_token(TokenKind::RightParanthesis, None),
            '{' => self.add_token(TokenKind::LeftBrace, None),
            '}' => self.add_token(TokenKind::RightBrace, None),
            ',' => self.add_token(TokenKind::Comma, None),
            '.' => self.add_token(TokenKind::Dot, None),
            '-' => self.add_token(TokenKind::Minus, None),
            '+' => self.add_token(TokenKind::Plus, None),
            ';' => self.add_token(TokenKind::Semicolon, None),
            '*' => self.add_token(TokenKind::Star, None),
            '!' => {
                let kind = match self.try_match_next('=') {
                    true => TokenKind::BangEqual,
                    false => TokenKind::Bang,
                };
                self.add_token(kind, None);
            }
            '=' => {
                let kind = match self.try_match_next('=') {
                    true => TokenKind::EqualEqual,
                    false => TokenKind::Equal,
                };
                self.add_token(kind, None);
            }
            '<' => {
                let kind = match self.try_match_next('=') {
                    true => TokenKind::LessEqual,
                    false => TokenKind::Less,
                };
                self.add_token(kind, None);
            }
            '>' => {
                let kind = match self.try_match_next('=') {
                    true => TokenKind::GreaterEqual,
                    false => TokenKind::Greater,
                };
                self.add_token(kind, None);
            }
            '/' => match self.try_match_next('/') {
                true => {
                    //comments
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenKind::Slash, None),
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
                self.add_token(TokenKind::String, Some(literal));
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

                let mut literal: String = self.source[self.start..self.current]
                    .iter()
                    .collect::<String>();
                if !literal.contains(".") {
                    literal.push_str(".0");
                } else {
                    let mut split = literal.split(".").collect::<Vec<&str>>();
                    if split[1].chars().all(|c| c == '0') {
                        split.pop();
                        split.push("0");
                        literal = split.join(".");
                    }
                }

                self.add_token(TokenKind::Number, Some(literal));
            }
            c if c.is_alphabetic() || c == '_' => {
                while self.peek().is_alphanumeric() || self.peek() == '_' {
                    self.advance();
                }

                let lexume: String = self.source[self.start..self.current].iter().collect();
                match KEYWORDS.get(&lexume.as_str()) {
                    Some(kind) => self.add_token(*kind, None),
                    None => self.add_token(TokenKind::Identifier, None),
                }
            }
            _ => {
                self.has_errors = true;
                eprintln!("[line {}] Error: Unexpected character: {}", self.line, c)
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, kind: TokenKind, literal: Option<String>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(kind, lexeme, literal, self.line));
    }

    fn try_match_next(&mut self, expected: char) -> bool {
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
