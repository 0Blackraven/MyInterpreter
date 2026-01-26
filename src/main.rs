mod token_type;
mod terminal_reader;

use terminal_reader::terminal_reader;
use token_type::TokenType;
#[allow(dead_code)]
struct Token {
    tokentype: TokenType,
    lexeme: String,
    line: u32, 
}
impl Token {
    #[allow(dead_code)]
    fn new(tokentype: TokenType, lexeme: String, line: u32) -> Self {
        Self { tokentype, lexeme, line }
    }
}

// improve error reporting to accommodate different error showing mechanisms like simple print to console, logging to file, etc.

fn execute_error (line: u32, message: &str) {
    eprintln!("Error at line {}: {}", line, message);
}

fn main() {
    let input = terminal_reader();
    match input {
        Ok(result) => {println!("{}",result.trim())},
        Err(e) => execute_error(0, &format!("Scanner error: {}",e)),
    } 
    // loop {
    // } 
}