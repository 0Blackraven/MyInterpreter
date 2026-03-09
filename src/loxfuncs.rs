use crate::lox_instance::LoxInstance;
use crate::statement::{FunctionProps,StatementType};
use crate::{callable::Callable, interpreter::Interpreter, token::Literal, environment::Environment};
use std::cell::RefCell;
use std::rc::Rc;
use crate::token::Token;
use crate::lox_error::{LoxError, LoxResult};

#[derive(Clone)]
pub struct LoxFunction {
    _name: Token,                   
    params: Vec<Token>,             
    body: Rc<StatementType>,        
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool
}
impl LoxFunction {
    pub fn new ( func_props : Rc<&FunctionProps>, interpreter: &mut Interpreter, is_initializer: bool) -> Self {
        return LoxFunction {
            _name: func_props.name.clone(),      
            params: func_props.params.clone(),   
            body: func_props.body.clone(),       
            closure: interpreter.env.clone(),
            is_initializer
        };
    }

    pub fn bind(&self, instance: &LoxInstance) -> LoxFunction {
        let mut env = Environment::new(Some(self.closure.clone()));
        let _ = env.define(
            Token::new(crate::token::TokenType::THIS, "this".to_string(), 0, crate::token::AtomicLiteral::Nil),
            Literal::Instance(instance.to_owned()));
        LoxFunction { 
            _name: self._name.clone(), 
            params: self.params.clone(), 
            body: self.body.clone(), 
            closure: Rc::new(RefCell::new(env)),
            is_initializer: self.is_initializer 
        }    
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> LoxResult<Literal> {
        let previous = Rc::clone(&interpreter.env);
        
        let closure= Rc::new(RefCell::new(Environment::new(Some(self.closure.clone()))));

        for (param, arg) in self.params.iter().zip(arguments) {
            interpreter.env.borrow_mut().define(param.clone(), arg)?;
        }

        let mut body_clone = (*self.body).clone();
        let result = StatementType::evaluate_func_block(&mut body_clone, closure, interpreter);

        interpreter.env = previous;
        
        match result {
            Ok(()) => {
                if self.is_initializer {
                    self.closure.borrow_mut().get_at(0, "this")?;
                }
                Ok(Literal::Basic(crate::token::AtomicLiteral::Nil))
            },
            Err(e) => {
                match e {
                    LoxError::ReturnValue(v) => {
                        if self.is_initializer {
                            self.closure.borrow_mut().get_at(0, "this")?;
                        }
                        Ok(v)
                    },
                    _ => return Err(e),
                }
            }
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}


