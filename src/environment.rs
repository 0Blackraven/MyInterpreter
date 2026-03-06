use crate::lox_error::{LoxError, LoxResult};
use crate::{
    token::{Literal, Token},
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    variables: HashMap<String, Rc<Literal>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing: enclosing,
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Rc<Literal>) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, token: &Token) -> LoxResult<Rc<Literal>> {
        let value_option: Option<&Rc<Literal>> = self.variables.get(token.lexeme.as_str());
        match value_option {
            Some(value) => return Ok(value.to_owned()),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.borrow().get(token);
                } else {
                    return Err(LoxError::RuntimeError {
                        token: Some(token.clone()),
                        message: format!("cannot find the variable {}", token.lexeme),
                    });
                }
            }
        }
    }

    pub fn assign(&mut self, token: Token, value: Rc<Literal>) -> LoxResult<()> {
        let x_result = self.variables.get_mut(&token.lexeme);
        match x_result {
            Some(x) => {
                *x = value;
                Ok(())
            }
            None => {
                if let Some(enclosing) = &mut self.enclosing {
                    return enclosing.borrow_mut().assign(token, value);
                } else {
                    return Err(LoxError::RuntimeError {
                        token: Some(token.clone()),
                        message: format!("Undefined variable {}", token.lexeme),
                    });
                }
            }
        }
    }

    pub fn get_at(&self, dist: usize, name: &str) -> LoxResult<Rc<Literal>> {
        if dist == 0 {
            return self
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| LoxError::RuntimeError {
                    token: None,
                    message: format!("Resolved variable '{}' not found in current scope ", name),
                });
        }
        let value = self.ancestor(dist)?;
        return value
            .borrow()
            .variables
            .get(name)
            .cloned()
            .ok_or_else(|| LoxError::RuntimeError {
                token: None,
                message: format!("Resolved variable '{}' not found in scope ", name),
            });
    }

    pub fn ancestor(&self, dist: usize) -> LoxResult<Rc<RefCell<Environment>>>{
        let mut current = self
            .enclosing
            .clone()
            .ok_or_else(|| LoxError::RuntimeError {
                token: None,
                message: "Tried to find ancestor of global scope.".into(),
            })?;
        for _ in 1..dist {
            let next =current
                .borrow()
                .enclosing
                .clone()
                .ok_or_else(|| LoxError::RuntimeError {
                    token: None,
                    message: "Environment hop distance exceeded.".into(),
                })?;
            current = next;
        }
        Ok(current)
    }

    pub fn assign_at(&mut self, dist: usize, token: Token, value: Rc<Literal>) -> LoxResult<()> {
        if dist == 0 {
            return self.assign(token, value);
        }
        let ancestor = self.ancestor(dist)?;
        ancestor.borrow_mut().assign(token, value)
    }
}
