use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

use codecrafters_interpreter::ast_printer::AstPrinter;
use codecrafters_interpreter::interpreter::Interpreter;
use codecrafters_interpreter::parser::Parser;
use codecrafters_interpreter::scanner::Scanner;

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
        let mut ast_printer = AstPrinter {};
        let mut interpreter = Interpreter::new();
        let mut parser = Parser::new(tokens.clone());

        match command.as_str() {
            "tokenize" => {
                for token in tokens {
                    println!("{}", token)
                }

                if scanner.errors() {
                    process::exit(65);
                }
            }
            "parse" => match parser.parse_expression() {
                Ok(expr) => println!("{}", ast_printer.print(expr)),
                Err(_) => process::exit(65),
            },
            "evaluate" => {
                let expression = match parser.parse_expression() {
                    Ok(expr) => expr,
                    Err(_) => process::exit(65),
                };
                match interpreter.interpret_expression(&expression) {
                    Ok(result) => println!("{}", result),
                    Err(_) => process::exit(70),
                }
            }
            "run" => {
                let statements = match parser.parse() {
                    Ok(stmt) => stmt,
                    Err(_) => process::exit(65),
                };

                if let Err(_) = interpreter.interpret(&statements) {
                    process::exit(70);
                };
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
