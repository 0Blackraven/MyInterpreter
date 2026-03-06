use crate::statement::{StatementType, IfProps, WhileProps};
use crate::expression::{ExpressionType};
use crate::token::{AtomicLiteral, Literal};
use crate::{clock::Clock, environment::Environment};
use std::cell::RefCell;
use std::rc::Rc;
use crate::lox_error::{LoxError, LoxResult};

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

    pub fn evaluate_blocks(&mut self, statements: &mut Vec<StatementType>) -> LoxResult<()> {
        let previous = Rc::clone(&self.storage);
    
        self.storage = Rc::new(RefCell::new(Environment::new(Some(previous.clone()))));
    
        for statement in statements {
            statement.evaluate(self)?;
        }
    
        self.storage = previous;
        Ok(())
    }

    pub fn evaluate_func_block(
        &mut self,
        statement: &mut StatementType,
        closure: Rc<RefCell<Environment>>,
    ) -> LoxResult<()> {
        let previous = Rc::clone(&self.storage);
    
        self.storage = closure;
    
        let result = statement.evaluate(self);
    
        self.storage = previous;
        result  
    }

    pub fn evaluate_if(&mut self, ifinput: &mut IfProps) -> LoxResult<()> {
        let comparison = ifinput.comparison.evaluate(self)?;
    
        if self.is_truthy(&comparison) {
            ifinput.ifcase.evaluate(self)?;
        } else if let Some(elsecase) = &mut ifinput.elsecase {
            elsecase.evaluate(self)?;
        }
        Ok(())
    }

    pub fn evaluate_while(&mut self, wild: &mut WhileProps) -> LoxResult<()> {
        let condition = &mut wild.condition;
        let statement = &mut *wild.statement;
    
        while {
            let cond = condition.evaluate(self)?;
            self.is_truthy(&cond)
        } {
            statement.evaluate(self)?;
        }
        Ok(())
    }

    
    pub fn is_truthy(&self, value: &Literal) -> bool {
        match value {
            Literal::Basic(AtomicLiteral::Nil) => false,
            Literal::Basic(AtomicLiteral::Bool(false)) => false,
            _ => true,
        }
    }

    pub fn is_equal(&self, a: &Literal, b: &Literal) -> LoxResult<bool> {
        match (a, b) {
            (Literal::Basic(a), Literal::Basic(b)) => Ok(a == b),
            _ => Err(LoxError::RuntimeError {
                token: None,
                message: "Cannot compare callable objects".to_string(),
            })
        }
    }

    pub fn interpreter(&mut self, mut statements: Vec<StatementType>) -> LoxResult<()> {
        for statement in &mut statements {
            statement.evaluate(self)?;
        }
        Ok(())
    }
}
