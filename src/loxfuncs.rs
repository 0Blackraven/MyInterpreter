use crate::statement::{FunctionProps,StatementType};
use crate::{callable::Callable, interpreter::Interpreter, token::Literal, environment::Environment};
use std::cell::RefCell;
use std::rc::Rc;
use crate::token::Token;
use crate::lox_error::{LoxError, LoxResult};

pub struct LoxFunction {
    _name: Token,                   
    params: Vec<Token>,             
    body: Rc<StatementType>,        
    closure: Rc<RefCell<Environment>>
}

impl LoxFunction {
    pub fn new ( func_props : Rc<&FunctionProps>, interpreter: &mut Interpreter) -> Self {
        return LoxFunction {
            _name: func_props.name.clone(),      
            params: func_props.params.clone(),   
            body: func_props.body.clone(),       
            closure: interpreter.env.clone() 
        };
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Rc<Literal>>,
    ) -> LoxResult<Rc<Literal>> {
        let environment = Rc::new(RefCell::new(Environment::new(Some(self.closure.clone()))));

        for (param, arg) in self.params.iter().zip(arguments) {
            environment.borrow_mut().define(param.clone(), arg)?;
        }

        let mut body_clone = (*self.body).clone();
        match StatementType::evaluate_func_block(&mut body_clone, environment, interpreter){
            Ok(()) => Ok(Rc::new(Literal::Basic(crate::token::AtomicLiteral::Nil))),
            Err(e) => {
                match e {
                    LoxError::ReturnValue(v) => Ok(v),
                    _ => Err(e),
                }
            }
        }
    }
}


