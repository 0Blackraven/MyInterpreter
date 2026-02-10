use std::fmt;

use crate::callable::Callable;

#[derive(Debug,Clone,PartialEq)]
pub enum TokenType {
    // single char tokens
    LEFTPAREN, RIGHTPAREN, LEFTBRACE, RIGHTBRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens. 
    // BANG IS !
    BANG, BANGEQUAL,
    EQUAL, EQUALEQUAL,
    GREATER, GREATEREQUAL,
    LESS, LESSEQUAL,MODULO,DECREMENTOR,INCREMENTOR,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUNCTION, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, LET, WHILE,

    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenType::PLUS => "+",
            TokenType::MINUS => "-",
            TokenType::STAR => "*",
            TokenType::SLASH => "/",
            TokenType::MODULO => "%",
            TokenType::BANG => "!",
            TokenType::BANGEQUAL => "!=",
            TokenType::EQUAL => "=",
            TokenType::EQUALEQUAL => "==",
            TokenType::GREATER => ">",
            TokenType::GREATEREQUAL => ">=",
            TokenType::LESS => "<",
            TokenType::LESSEQUAL => "<=",
            TokenType::DECREMENTOR => "--",
            TokenType::INCREMENTOR => "++",
            _ => return write!(f, "{:?}", self), // fallback for non-operators
        };

        write!(f, "{s}")
    }
}

#[derive(Clone,PartialEq)]
#[allow(dead_code)]

pub enum AtomicLiteral {
    String(String),
    Number(f32),
    Bool(bool),
    Nil,
}
pub enum Literal {
    Basic(AtomicLiteral),
    LoxCallable(Box<dyn Callable>)
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Basic(atom) => match atom {
                AtomicLiteral::String(s) => write!(f, "{}", s),
                AtomicLiteral::Number(n) => write!(f, "{}", n),
                AtomicLiteral::Bool(b) => write!(f, "{}", b),
                AtomicLiteral::Nil => write!(f, "nil"),
            },
            Literal::LoxCallable(_) => write!(f, "<fn>"),
        }
    }
}


#[derive(Clone)]
pub struct Token {
    pub tokentype: TokenType,
    pub lexeme: String,
    pub literal: Option<AtomicLiteral>,
    pub line: u32,
}
impl Token {
    pub fn literal (&self) -> String {
        let literal_result = self.literal.clone();
        match literal_result {
            Some(value) => {
                match value {
                    AtomicLiteral::String(value ) => return value,
                    AtomicLiteral::Number(value) => return value.to_string(),
                    AtomicLiteral::Bool(value) => return value.to_string(),
                    _ => return String::from("not a good thing")
                }
            }
            None => {
                panic!("Not a literal");
            }
        }
    }
    pub fn new(tokentype: TokenType, lexeme: String, line: u32, literal:AtomicLiteral) -> Self {
        match literal {
            AtomicLiteral::String(_) | AtomicLiteral::Number(_) | AtomicLiteral::Bool(_) => Token {
                tokentype,
                lexeme,
                literal: Some(literal),
                line,
            },
            AtomicLiteral::Nil => Token {
                tokentype,
                lexeme,
                literal: None,
                line,
            },
        }
    }
}
 
 