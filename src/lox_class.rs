use crate::loxfuncs::LoxFunction;
use crate::{interpreter::Interpreter, token::Token};
use crate::token::Literal;
use crate::callable::Callable;
use crate::lox_error::LoxResult;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Clone)]
pub struct LoxClass {
    name: Token,
    methods: HashMap<String, LoxFunction>
}

impl LoxClass {
    pub fn new(name: Token, methods: HashMap<String, LoxFunction>) -> Self {
        LoxClass { name , methods}
    }

    pub fn find_method (&mut self, token: &Token) -> Option<LoxFunction> {
        match self.methods.get(&token.lexeme) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }
}

impl Callable for LoxClass {
    fn arity (&self) -> usize {
        0
    }

    fn call (&self, _: &mut Interpreter, _:Vec<Rc<Literal>>) -> LoxResult<Rc<Literal>> {
        let instance = crate::lox_instance::LoxInstance::new(self.clone());
        Ok(Rc::new(Literal::Instance(Rc::new(RefCell::new(instance)))))
    }

}