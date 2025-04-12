use token::{Token, TokenKind};

pub mod ast_printer;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;

pub fn report(line: usize, message: &str) {
    let err = format!("[line {}] Error: {}", line, message);
    eprintln!("{}", err);
}

pub fn error(token: Token, message: &str) {
    if token.kind == TokenKind::EOF {
        report(token.line, &format!(" at end {}", message));
    } else {
        report(token.line, &format!("at '{}': {}", &token.lexeme, message));
    }
}
