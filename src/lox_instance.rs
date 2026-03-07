use crate::lox_class::LoxClass;
use std::{collections::HashMap, rc::Rc};
use crate::token::Literal;
use crate::lox_error::{LoxResult,LoxError};
use crate::token::Token;

pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Rc<Literal>>
}

impl LoxInstance {
    pub fn new (class: LoxClass) -> Self {
        LoxInstance { 
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&mut self, name:Token) -> LoxResult<Rc<Literal>> {
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

        if let Some(method) = self.class.find_method(&name) {
            return Ok(Rc::new(Literal::LoxCallable(Box::new(method))));
        }

        Err(LoxError::RuntimeError {
            token: Some(name),
            message: "Undefined property".to_string(),
        })
    }

    pub fn set(&mut self, name:Token, value:Rc<Literal>) {
        self.fields.insert(name.lexeme, value);
    }
}