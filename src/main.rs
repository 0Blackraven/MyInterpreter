mod terminal_reader;
mod scanner;
mod token;
mod parser;
mod environment;
mod interpreter;
mod callable;
mod clock;
mod lox_error;
mod loxfuncs;
use terminal_reader::terminal_reader;
use lox_error::{LoxResult};


// improve error reporting to accommodate different error showing mechanisms like simple print to console, logging to file, etc.
// replace all panic! calls with proper error handling mechanisms.

fn execute_error(line: u32, message: &str) {
    eprintln!("Error at line {}: {}", line, message);
}


fn main() {
    let input = terminal_reader();
    match input {
        Ok(result) => {
            if let Err(e) = run(&result) {
                eprintln!("{}", e);
            }
        }
        Err(e) => execute_error(0, &format!("Scanner error: {}", e)),
    }
    
    // loop {
    // }
}

fn run(source: &str) -> LoxResult<()> {
    let tokens = scanner::scanner(source)?;
    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse()?;
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpreter(statements)?;
    Ok(())
}
