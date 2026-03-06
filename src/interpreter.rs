use crate::statement::{StatementType};
use crate::expression::{ExpressionType};
use crate::token::{Literal};
use crate::{clock::Clock, environment::Environment};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::lox_error::{LoxResult};

pub struct Interpreter {
    pub global: Rc<RefCell<Environment>>,
    pub env: Rc<RefCell<Environment>>,
    pub local: HashMap<ExpressionType, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        globals.borrow_mut().define(
            "clock".to_string(),
            Rc::new(Literal::LoxCallable(Box::new(Clock))),
        );
        
        Interpreter {
            global: globals.clone(),
            env: globals,
            local: HashMap::new(),
        }
    }
 
    pub fn resolve(&mut self, expr: &ExpressionType, depth: usize) {
        self.local.insert(expr.clone(), depth);
    }

    pub fn interpreter(&mut self, mut statements: Vec<StatementType>) -> LoxResult<()> {
        for statement in &mut statements {
            statement.evaluate(self)?;
        }
        Ok(())
    }
}
