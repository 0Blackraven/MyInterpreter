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
mod output;
use terminal_reader::terminal_reader;
use lox_error::LoxResult;
use serde::Serialize;

#[derive(Serialize)]
struct RunResponse {
    success: bool,
    output: String,
    error: Option<String>,
}

fn execute_error(line: u32, message: &str) {
    eprintln!("Error at line {}: {}", line, message);
}

fn main() {
    let input = match terminal_reader() {
        Ok(result) => result,
        Err(e) => {
            let resp = RunResponse {
                success: false,
                output: String::new(),
                error: Some(format!("Scanner error: {}", e)),
            };
            println!("{}", serde_json::to_string(&resp).unwrap());
            return;
        }
    };

    output::clear_output();

    match run(&input) {
        Ok(()) => {
            let resp = RunResponse {
                success: true,
                output: output::get_output(),
                error: None,
            };
            println!("{}", serde_json::to_string(&resp).unwrap());
        }
        Err(e) => {
            let resp = RunResponse {
                success: false,
                output: output::get_output(),
                error: Some(e.to_string()),
            };
            println!("{}", serde_json::to_string(&resp).unwrap());
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
    interpreter.interpreter(statements)?;
    Ok(())
}