mod terminal_reader;
mod scanner;
mod token;
mod parser;
mod environment;
mod interpreter;
mod expression;
mod statement;
mod callable;
mod clock;
mod lox_error;
mod lox_class;
mod resolver;
mod loxfuncs;
mod lox_instance;
use terminal_reader::terminal_reader;
use lox_error::{LoxResult};
use std::env;

use crate::lox_error::LoxError;

fn execute_error(line: u32, message: &str) {
    eprintln!("Error at line {}: {}", line, message);
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if let Err(e) = file_reader(&args[1]) {
            eprintln!("{}", e);
        }
    } else if args.len() > 2 {
        eprintln!("Usage: {} [script]", args[0]);
        std::process::exit(64);
    } else {
        let input = terminal_reader();
        match input {
            Ok(result) => {
                if let Err(e) = run(&result) {
                    eprintln!("{}", e)
                }
            }
            Err(e) => execute_error(0, &format!("Scanner error: {}", e)),
        }
    }
}

fn file_reader(path: &str)-> LoxResult<()> {
    let source = std::fs::read_to_string(path);
    match source {
        Ok(result) => run(&result),
        Err(e) => {
            return Err(LoxError::GeneralError { message: format!("Failed to read file: {}", e) })
        }
    }
}

fn run(source: &str) -> LoxResult<()> {
    let tokens = scanner::scanner(source)?;
    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse()?;
    let mut interpreter = interpreter::Interpreter::new();
    let mut resolver = resolver::Resolver::new(&mut interpreter);
    resolver.resolve(&statements)?;
    interpreter.interpreter(&statements)?;
    Ok(())
}
