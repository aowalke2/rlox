use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    //Single character tokens
    LeftParanthesis,
    RightParanthesis,
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
            LeftParanthesis => write!(f, "LEFT_PAREN"),
            RightParanthesis => write!(f, "RIGHT_PAREN"),
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

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, literal: Option<String>, line: usize) -> Self {
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
        let literal = match &self.literal {
            Some(s) => s.clone(),
            None => "null".to_string(),
        };
        write!(f, "{} {} {}", self.kind, self.lexeme, literal)
    }
}
