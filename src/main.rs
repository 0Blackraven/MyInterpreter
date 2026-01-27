mod terminal_reader;
mod scanner;
mod token;

use terminal_reader::terminal_reader;


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
                    for token in tokens.iter() {
                        token.print_token();
                    }
                }
                Err(e) => execute_error(0, &format!("Scanner error: {}", e)),
            }
        }
        Err(e) => execute_error(0, &format!("Scanner error: {}", e)),
    }
    // loop {
    // }
}
