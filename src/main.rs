mod terminal_reader;
mod scanner;
mod token;
mod parser;
mod interpreter;
use terminal_reader::terminal_reader;
use parser::print_expr;


// improve error reporting to accommodate different error showing mechanisms like simple print to console, logging to file, etc.

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
                    let output = interpreter::interpreter(&expression);
                    println!("Output: {}", output);
                }
                Err(e) => execute_error(0, &format!("Scanner error: {}", e)),
            }
        }
        Err(e) => execute_error(0, &format!("Scanner error: {}", e)),
    }
    
    // loop {
    // }
}
