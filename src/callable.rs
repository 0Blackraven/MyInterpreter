use crate::{interpreter::Interpreter, token::Literal};
use std::rc::Rc;
pub trait Callable {
    fn arity(&self) ->usize;
    fn call(&self, env:&mut Interpreter ,args:Vec<Rc<Literal>>) -> Rc<Literal>;
}

