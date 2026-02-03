mod terminal_reader;
mod scanner;
mod token;
mod parser;
mod environment;
mod interpreter;
use terminal_reader::terminal_reader;


// improve error reporting to accommodate different error showing mechanisms like simple print to console, logging to file, etc.
// replace all panic! calls with proper error handling mechanisms.

fn execute_error(line: u32, message: &str) {
    eprintln!("Error at line {}: {}", line, message);
}


fn main() {
    let input = terminal_reader();
    match input {
        Ok(result) => {
            match scanner::scanner(&result) {
                Ok(tokens) => {
                    let mut parser = parser::Parser::new(tokens);
                    let expression = parser.parse();
                    let mut interpreter = interpreter::Interpreter::new();
                    let _output = interpreter.interpreter(expression);
                }
                Err(e) => execute_error(0, &format!("Scanner error: {}", e)),
            }
        }
        Err(e) => execute_error(0, &format!("Scanner error: {}", e)),
    }
    
    // loop {
    // }
}
