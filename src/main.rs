use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use ast_printer::AstPrinter;
use parser::Parser;
use scanner::Scanner;

mod ast_printer;
mod expr;
mod parser;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    if !file_contents.is_empty() {
        let mut scanner = Scanner::new(file_contents);
        let tokens = scanner.scan_tokens();

        match command.as_str() {
            "tokenize" => {
                for token in tokens {
                    println!("{}", token)
                }

                if scanner.errors() {
                    process::exit(65);
                }
            }
            "parse" => {
                let mut ast_printer = AstPrinter {};
                let mut parser = Parser::new(tokens.clone());
                match parser.parse() {
                    Ok(expr) => println!("{}", ast_printer.print(expr)),
                    Err(_) => return,
                }
            }
            _ => {
                writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
                return;
            }
        }
    } else {
        println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
    }
}
