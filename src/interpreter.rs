use crate::statement::{StatementType};
use crate::expression::{ExpressionType};
use crate::token::{Literal,Token};
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
        let _ = globals.borrow_mut().define(
            Token::new(crate::token::TokenType::IDENTIFIER, "clock".to_string(),0,crate::token::AtomicLiteral::Nil),
            Literal::LoxCallable(Rc::new(Clock)),
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

    pub fn interpreter(&mut self, statements: &[StatementType]) -> LoxResult<()> {
        for statement in statements {
            statement.evaluate(self)?;
        }
        Ok(())
    }
}
