use crate::{interpreter::Interpreter, token::Literal};
use crate::lox_error::LoxResult;


pub trait Callable {
    fn arity(&self) ->usize;
    fn call(&self, env:&mut Interpreter ,args:Vec<Literal>) -> LoxResult<Literal>;
}

