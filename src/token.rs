use std::{collections::HashMap, fmt::Display};

use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    //Single character tokens
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // Or or more character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    //Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    //
    EOF,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        match self {
            LeftParenthesis => write!(f, "LEFT_PAREN"),
            RightParenthesis => write!(f, "RIGHT_PAREN"),
            LeftBrace => write!(f, "LEFT_BRACE"),
            RightBrace => write!(f, "RIGHT_BRACE"),
            Comma => write!(f, "COMMA"),
            Dot => write!(f, "DOT"),
            Minus => write!(f, "MINUS"),
            Plus => write!(f, "PLUS"),
            Semicolon => write!(f, "SEMICOLON"),
            Slash => write!(f, "SLASH"),
            Star => write!(f, "STAR"),
            Bang => write!(f, "BANG"),
            BangEqual => write!(f, "BANG_EQUAL"),
            Equal => write!(f, "EQUAL"),
            EqualEqual => write!(f, "EQUAL_EQUAL"),
            Greater => write!(f, "GREATER"),
            GreaterEqual => write!(f, "GREATER_EQUAL"),
            Less => write!(f, "LESS"),
            LessEqual => write!(f, "LESS_EQUAL"),
            Identifier => write!(f, "IDENTIFIER"),
            String => write!(f, "STRING"),
            Number => write!(f, "NUMBER"),
            And => write!(f, "AND"),
            Class => write!(f, "CLASS"),
            Else => write!(f, "ELSE"),
            False => write!(f, "FALSE"),
            Fun => write!(f, "FUN"),
            For => write!(f, "FOR"),
            If => write!(f, "IF"),
            Nil => write!(f, "NIL"),
            Or => write!(f, "OR"),
            Print => write!(f, "PRINT"),
            Return => write!(f, "RETURN"),
            Super => write!(f, "SUPER"),
            This => write!(f, "THIS"),
            True => write!(f, "TRUE"),
            Var => write!(f, "VAR"),
            While => write!(f, "WHILE"),
            EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl From<LiteralKind> for String {
    fn from(literal: LiteralKind) -> Self {
        match literal {
            LiteralKind::String(string) => string,
            LiteralKind::Number(number) => {
                let mut number = number.to_string();
                if !number.contains(".") {
                    number.push_str(".0");
                } else {
                    let mut split = number.split(".").collect::<Vec<&str>>();
                    if split[1].chars().all(|c| c == '0') {
                        split.pop();
                        split.push("0");
                        number = split.join(".");
                    }
                }
                number.to_string()
            }
            LiteralKind::Bool(bool) => bool.to_string(),
            LiteralKind::Nil => "null".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: LiteralKind,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, literal: LiteralKind, line: usize) -> Self {
        Token {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.kind,
            self.lexeme,
            String::from(self.literal.clone())
        )
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenKind> = {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenKind::And);
        keywords.insert("class", TokenKind::Class);
        keywords.insert("else", TokenKind::Else);
        keywords.insert("false", TokenKind::False);
        keywords.insert("for", TokenKind::For);
        keywords.insert("fun", TokenKind::Fun);
        keywords.insert("if", TokenKind::If);
        keywords.insert("nil", TokenKind::Nil);
        keywords.insert("or", TokenKind::Or);
        keywords.insert("print", TokenKind::Print);
        keywords.insert("return", TokenKind::Return);
        keywords.insert("super", TokenKind::Super);
        keywords.insert("this", TokenKind::This);
        keywords.insert("true", TokenKind::True);
        keywords.insert("var", TokenKind::Var);
        keywords.insert("while", TokenKind::While);
        keywords
    };
}
