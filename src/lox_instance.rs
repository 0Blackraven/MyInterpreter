use crate::lox_class::LoxClass;
use std::rc::Rc;
use std::{collections::HashMap};
use crate::token::Literal;
use crate::lox_error::{LoxResult,LoxError};
use crate::token::Token;

#[derive(Clone)]
pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Literal>
}

impl LoxInstance {
    pub fn new (class: LoxClass) -> Self {
        LoxInstance { 
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name:Token) -> LoxResult<Literal> {
        if self.fields.contains_key(&name.lexeme) {
            match self.fields.get(&name.lexeme) {
                Some(value) => return Ok(value.clone()),
                None => return Err(LoxError::RuntimeError {
                    token: Some(name),
                    message: "Undefined property".to_string(),
                    //should never reach this case because of the contains_key
                })
            }   
        } 

        if let Some(method) = self.class.find_method(&name.lexeme) {
            return Ok(Literal::LoxCallable(Rc::new(method.bind(self))));
        }

        Err(LoxError::RuntimeError {
            token: Some(name),
            message: "Undefined property".to_string(),
        })
    }

    pub fn set(&mut self, name:Token, value:Literal) {
        self.fields.insert(name.lexeme, value);
    }
}