use crate::statement::{StatementType, IfProps, WhileProps};
use crate::expression::{ExpressionType, is_truthy};
use crate::token::{Literal};
use crate::{clock::Clock, environment::Environment};
use std::cell::RefCell;
use std::rc::Rc;
use crate::lox_error::{LoxResult};

pub struct Interpreter {
    pub storage: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let interpreter = Interpreter {
            storage: Rc::new(RefCell::new(Environment::new(None))),
        };
        interpreter.storage.borrow_mut().define(
            "clock".to_string(),
            Rc::new(Literal::LoxCallable(Box::new(Clock))),
        );
        interpreter
    }
 
    pub fn resolve(&self, expr: &ExpressionType, depth: usize) -> LoxResult<()> {
        Ok(())
    }

    pub fn interpreter(&mut self, mut statements: Vec<StatementType>) -> LoxResult<()> {
        for statement in &mut statements {
            statement.evaluate(self)?;
        }
        Ok(())
    }
}
