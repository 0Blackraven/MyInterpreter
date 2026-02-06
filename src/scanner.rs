use crate::token::{Literal, Token, TokenType};
use std::io::Result;

const _KEYWORDS: [&str; 16] = [
    "and", "class", "else", "false", "func", "for", "if", "null", "or", "print", "return", "super",
    "this", "true", "let", "while",
];

fn _is_keyword(input: &str) -> bool {
    for keyword in _KEYWORDS.iter() {
        if input == *keyword {
            return true;
        }
    }
    return false;
}

fn _match_keyword(input: &str) -> TokenType {
    match input {
        "class" => TokenType::CLASS,
        "else" => TokenType::ELSE,
        "false" => TokenType::FALSE,
        "func" => TokenType::FUNCTION,
        "for" => TokenType::FOR,
        "if" => TokenType::IF,
        "null" => TokenType::NIL,
        "print" => TokenType::PRINT,
        "return" => TokenType::RETURN,
        "super" => TokenType::SUPER,
        "this" => TokenType::THIS,
        "true" => TokenType::TRUE,
        "let" => TokenType::LET,
        "while" => TokenType::WHILE,
        _ => TokenType::IDENTIFIER, // should not reach here
    }
}

fn _is_number(c: char) -> bool {
    if c >= '0' && c <= '9' {
        return true;
    }
    return false;
}

fn _is_alphanumeric(c: char) -> bool {
    if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_' {
        return true;
    }
    return false;
}

pub fn scanner(input: &str) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_line = 1;
    let mut token_lexeme = String::new();

    let push_token = |tokens: &mut Vec<Token>, token: Token, token_lexeme: &mut String| {
        tokens.push(token);
        token_lexeme.clear();
    };

    let mut char_iter = input.chars().peekable();
    while let Some(current_char) = char_iter.next() {
        match current_char {
            // single character tokens
            ' ' | '\t' => {
                continue;
            }
            '\n' => {
                current_line += 1;
            }
            '\0' => {
                break;
            }
            '.' => {
                push_token(
                    &mut tokens,
                    Token::new(TokenType::DOT, ".".to_string(), current_line, Literal::Nil),
                    &mut token_lexeme,
                );
            }
            '{' => {
                push_token(
                    &mut tokens,
                    Token::new(
                        TokenType::LEFTBRACE,
                        "{".to_string(),
                        current_line,
                        Literal::Nil,
                    ),
                    &mut token_lexeme,
                );
            }
            '%' => {
                push_token(
                    &mut tokens,
                    Token::new(
                        TokenType::MODULO,
                        "%".to_string(),
                        current_line,
                        Literal::Nil,
                    ),
                    &mut token_lexeme,
                );
            }
            '}' => {
                push_token(
                    &mut tokens,
                    Token::new(
                        TokenType::RIGHTBRACE,
                        "}".to_string(),
                        current_line,
                        Literal::Nil,
                    ),
                    &mut token_lexeme,
                );
            }
            '(' => {
                push_token(
                    &mut tokens,
                    Token::new(
                        TokenType::LEFTPAREN,
                        "(".to_string(),
                        current_line,
                        Literal::Nil,
                    ),
                    &mut token_lexeme,
                );
            }
            ')' => {
                push_token(
                    &mut tokens,
                    Token::new(
                        TokenType::RIGHTPAREN,
                        ")".to_string(),
                        current_line,
                        Literal::Nil,
                    ),
                    &mut token_lexeme,
                );
            }
            ',' => {
                push_token(
                    &mut tokens,
                    Token::new(
                        TokenType::COMMA,
                        ",".to_string(),
                        current_line,
                        Literal::Nil,
                    ),
                    &mut token_lexeme,
                );
            }
            '-' => {
                push_token(
                    &mut tokens,
                    Token::new(
                        TokenType::MINUS,
                        "-".to_string(),
                        current_line,
                        Literal::Nil,
                    ),
                    &mut token_lexeme,
                );
            }
            '+' => {
                push_token(
                    &mut tokens,
                    Token::new(TokenType::PLUS, "+".to_string(), current_line, Literal::Nil),
                    &mut token_lexeme,
                );
            }
            ';' => {
                push_token(
                    &mut tokens,
                    Token::new(
                        TokenType::SEMICOLON,
                        ";".to_string(),
                        current_line,
                        Literal::Nil,
                    ),
                    &mut token_lexeme,
                );
                current_line += 1;
            }
            '*' => {
                push_token(
                    &mut tokens,
                    Token::new(TokenType::STAR, "*".to_string(), current_line, Literal::Nil),
                    &mut token_lexeme,
                );
            }

            // one or two character tokens
            '=' => {
                if let Some(next_char) = char_iter.peek() {
                    if next_char == &'\0' {
                        // give out error that unexpected end of file after = Even better try saying out that the assignment or whatever the user wanted to do is incomplete
                    } else if next_char == &'=' {
                        char_iter.next();
                        push_token(
                            &mut tokens,
                            Token::new(
                                TokenType::EQUALEQUAL,
                                "==".to_string(),
                                current_line,
                                Literal::Nil,
                            ),
                            &mut token_lexeme,
                        );
                    } else {
                        push_token(
                            &mut tokens,
                            Token::new(
                                TokenType::EQUAL,
                                "=".to_string(),
                                current_line,
                                Literal::Nil,
                            ),
                            &mut token_lexeme,
                        );
                    }
                }
            }
            '&' => {
                if let Some(next_char) = char_iter.peek(){
                    if next_char == &'\0' {
                        // give out error
                    } else if next_char == &'&' {
                        char_iter.next();
                        push_token(
                            &mut tokens,
                            Token::new(
                                TokenType::AND,
                                "&&".to_string(),
                                current_line,
                                Literal::Nil,
                            ),
                            &mut token_lexeme,
                        );
                    }
                }
            }
            '|' => {
                if let Some(next_char) = char_iter.peek(){
                    if next_char == &'\0' {
                        // give out error
                    } else if next_char == &'|' {
                        char_iter.next();
                        push_token(
                            &mut tokens,
                            Token::new(
                                TokenType::OR,
                                "||".to_string(),
                                current_line,
                                Literal::Nil,
                            ),
                            &mut token_lexeme,
                        );
                    }
                }
            }
            '>' => {
                if let Some(next_char) = char_iter.peek() {
                    if next_char == &'\0' {
                        // give out error that unexpected end of file after >
                    } else if next_char == &'=' {
                        char_iter.next();
                        push_token(
                            &mut tokens,
                            Token::new(
                                TokenType::GREATEREQUAL,
                                ">=".to_string(),
                                current_line,
                                Literal::Nil,
                            ),
                            &mut token_lexeme,
                        );
                    } else {
                        push_token(
                            &mut tokens,
                            Token::new(
                                TokenType::GREATER,
                                ">".to_string(),
                                current_line,
                                Literal::Nil,
                            ),
                            &mut token_lexeme,
                        );
                    }
                }
            }
            '<' => {
                if let Some(next_char) = char_iter.peek() {
                    if next_char == &'\0' {
                        // give out error that unexpected end of file after <
                    } else if next_char == &'=' {
                        char_iter.next();
                        push_token(
                            &mut tokens,
                            Token::new(
                                TokenType::LESSEQUAL,
                                "<=".to_string(),
                                current_line,
                                Literal::Nil,
                            ),
                            &mut token_lexeme,
                        );
                    } else {
                        push_token(
                            &mut tokens,
                            Token::new(
                                TokenType::LESS,
                                "<".to_string(),
                                current_line,
                                Literal::Nil,
                            ),
                            &mut token_lexeme,
                        );
                    }
                }
            }
            '!' => {
                if let Some(next_char) = char_iter.peek() {
                    if next_char == &'\0' {
                        // give out error that unexpected end of file after !
                    } else if next_char == &'=' {
                        char_iter.next();
                        push_token(
                            &mut tokens,
                            Token::new(
                                TokenType::BANGEQUAL,
                                "!=".to_string(),
                                current_line,
                                Literal::Nil,
                            ),
                            &mut token_lexeme,
                        );
                    } else {
                        push_token(
                            &mut tokens,
                            Token::new(
                                TokenType::BANG,
                                "!".to_string(),
                                current_line,
                                Literal::Nil,
                            ),
                            &mut token_lexeme,
                        );
                    }
                }
            }
            '/' => {
                if let Some('/') = char_iter.peek() {
                    char_iter.next();
                    while let Some(c) = char_iter.next() {
                        if c == '\n' {
                            current_line += 1;
                            break;
                        }
                    }
                } else {
                    push_token(
                        &mut tokens,
                        Token::new(
                            TokenType::SLASH,
                            "/".to_string(),
                            current_line,
                            Literal::Nil,
                        ),
                        &mut token_lexeme,
                    );
                }
            }
            '"' => {
                while let Some(next_char) = char_iter.next() {
                    if next_char == '"' {
                        break;
                    }
                    if next_char == '\0' {
                        // give out error that unexpected end of file in string literal
                    }
                    if next_char == '\n' {
                        current_line += 1;
                    }
                    token_lexeme.push(next_char);
                }
                push_token(
                    &mut tokens,
                    Token::new(
                        TokenType::STRING,
                        token_lexeme.clone(),
                        current_line,
                        Literal::String(token_lexeme.clone()),
                    ),
                    &mut token_lexeme,
                );
            }
            c if _is_number(c) => {
                token_lexeme.push(c);
                while let Some(&next_char) = char_iter.peek() {
                    if _is_number(next_char) {
                        token_lexeme.push(next_char);
                        char_iter.next();
                    } else {
                        break;
                    }
                }
                let num_result = token_lexeme.parse::<f32>();
                match num_result {
                    Err(_) => {
                        // give out error that number literal could not be parsed
                    }
                    Ok(num) => {
                        push_token(
                            &mut tokens,
                            Token::new(
                                TokenType::NUMBER,
                                token_lexeme.clone(),
                                current_line,
                                Literal::Number(num),
                            ),
                            &mut token_lexeme,
                        );
                    }
                }
            }

            c if _is_alphanumeric(c) => {
                token_lexeme.push(c);
                while let Some(&next_char) = char_iter.peek() {
                    if _is_alphanumeric(next_char) {
                        token_lexeme.push(next_char);
                        char_iter.next();
                    } else {
                        break;
                    }
                }
                if _is_keyword(&token_lexeme) {
                    if token_lexeme == "true" {
                        push_token(
                            &mut tokens,
                            Token::new(
                                _match_keyword(&token_lexeme),
                                token_lexeme.clone(),
                                current_line,
                                Literal::Bool(true),
                            ),
                            &mut token_lexeme,
                        )
                    } else if token_lexeme == "false" {
                        push_token(
                            &mut tokens,
                            Token::new(
                                _match_keyword(&token_lexeme),
                                token_lexeme.clone(),
                                current_line,
                                Literal::Bool(false),
                            ),
                            &mut token_lexeme,
                        )
                    } else {
                        push_token(
                            &mut tokens,
                            Token::new(
                                _match_keyword(&token_lexeme),
                                token_lexeme.clone(),
                                current_line,
                                Literal::Nil,
                            ),
                            &mut token_lexeme,
                        );
                    }
                } else {
                    push_token(
                        &mut tokens,
                        Token::new(
                            TokenType::IDENTIFIER,
                            token_lexeme.clone(),
                            current_line,
                            Literal::Nil,
                        ),
                        &mut token_lexeme,
                    );
                }
            }

            _ => {
                continue;
                // handle the unused characters like ^ etc
            }
        }
    }

    tokens.push(Token::new(
        TokenType::EOF,
        "".to_string(),
        current_line,
        Literal::Nil,
    ));
    Ok(tokens)
}
