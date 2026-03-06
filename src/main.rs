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
mod resolver;
mod loxfuncs;
use terminal_reader::terminal_reader;
use lox_error::{LoxResult};

fn execute_error(line: u32, message: &str) {
    eprintln!("Error at line {}: {}", line, message);
}


fn main() {
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
