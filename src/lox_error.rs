use crate::token::{Token,Literal};
use std::rc::Rc;

pub enum LoxError {
    ScanError { token:Token, message: String },
    ParseError { token: Token, message: String },
    RuntimeError { token: Option<Token>, message: String },
    ReturnValue(Rc<Literal>)
}

impl std::fmt::Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::ScanError { token, message } | LoxError::ParseError { token, message }  => {
                write!(f, "[Line {}] [at {}] {}", token.line, token.lexeme, message)
            }
            LoxError::RuntimeError { token, message } => {
                if let Some(token) = token {
                    write!(f, "[Line {}] [at {}] {}", token.line, token.lexeme, message)
                } else {
                    write!(f, "{}", message)
                }
            }
            LoxError::ReturnValue(_) => write!(f, "ReturnValue error" ) // should not reach this 
        }
    }
}

pub type LoxResult<T> = Result<T, LoxError>;