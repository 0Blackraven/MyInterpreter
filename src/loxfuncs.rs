use crate::parser::FunctionProps;
use crate::{callable::Callable, interpreter::Interpreter, token::Literal, environment::Environment};
use std::cell::RefCell;
use std::rc::Rc;

pub struct LoxFunction {
    declaration: FunctionProps,
    closure: Rc<RefCell<Environment>>
}

impl LoxFunction {
    fn new ( declaration : FunctionProps, interpreter: &mut Interpreter) -> Self {
        return LoxFunction {
            declaration: declaration ,
            closure: interpreter.storage.clone()
        };
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Rc<Literal>>,
    ) -> Rc<Literal> {

        let environment = Rc::new(RefCell::new(Environment::new(Some(self.closure.clone()))));



        for (param, arg) in self.declaration.params.iter().zip(arguments){
            environment.borrow_mut().define(param.lexeme.clone(), arg);
        }
        
        interpreter.evaluate_func_block(&*self.declaration.body, environment);

        // return null;
        Rc::new(Literal::Basic(crate::token::AtomicLiteral::Nil))
    }
}


