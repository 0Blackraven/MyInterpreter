use crate::loxfuncs::LoxFunction;
use crate::{interpreter::Interpreter, token::Token};
use crate::token::Literal;
use crate::callable::Callable;
use crate::lox_error::LoxResult;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Clone)]
pub struct LoxClass {
    name: Token,
    methods: HashMap<String, LoxFunction>,
    superclass: Option<Rc<LoxClass>>
}

impl LoxClass {
    pub fn new(name: Token, methods: HashMap<String, LoxFunction>, superclass: Option<Rc<LoxClass>>) -> Self {
        LoxClass { name , methods, superclass}
    }

    pub fn find_method (&self, token: &str) -> Option<LoxFunction> {
        match self.methods.get(token) {
            Some(v) => Some(v.clone()),
            None => {
                if let Some(superclass) = &self.superclass {
                    superclass.find_method(token)
                } else {
                    None
                }
            }
        }
    }
}

impl Callable for LoxClass {
    fn arity (&self) -> usize {
        let initializer = self.find_method("init");
        match initializer {
            Some(result ) => result.arity(),
            None => 0
        }
    }

    fn call (&self, interpreter: &mut Interpreter, v:Vec<Literal>) -> LoxResult<Literal> {
        let instance = crate::lox_instance::LoxInstance::new(self.clone());
        if let Some(initializer) = self.find_method("init") {
            let _ = initializer.bind(&instance).call(interpreter, v);
        }
        Ok(Literal::Instance(instance))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}