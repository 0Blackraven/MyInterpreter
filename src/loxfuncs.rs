use crate::parser::FunctionProps;
use crate::{callable::Callable, interpreter::Interpreter, token::Literal, environment::Environment};
use std::cell::RefCell;
use std::rc::Rc;
use crate::token::Token;
use crate::parser::StatementType;

pub struct LoxFunction {
    name: Token,                   
    params: Vec<Token>,              
    body: Rc<StatementType>,
    closure: Rc<RefCell<Environment>>
}

impl LoxFunction {
    pub fn new ( func_props : Rc<&FunctionProps>, interpreter: &mut Interpreter) -> Self {
        return LoxFunction {
            name: func_props.name.clone(),
            params: func_props.params.clone(),
            body: func_props.body.clone(),
            closure: interpreter.storage.clone()
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
    ) -> Rc<Literal> {

        let environment = Rc::new(RefCell::new(Environment::new(Some(self.closure.clone()))));



        for (param, arg) in self.params.iter().zip(arguments){
            environment.borrow_mut().define(param.lexeme.clone(), arg);
        }
        
        interpreter.evaluate_func_block(&*self.body, environment);

        // return null;
        Rc::new(Literal::Basic(crate::token::AtomicLiteral::Nil))
    }
}


