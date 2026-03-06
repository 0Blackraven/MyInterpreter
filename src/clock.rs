use std::{time::{SystemTime, UNIX_EPOCH}, rc::Rc};
use crate::lox_error::{LoxError, LoxResult};

use crate::{callable::Callable, interpreter::Interpreter, token::{AtomicLiteral, Literal}};


pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, _: Vec<Rc<Literal>>) -> LoxResult<Rc<Literal>> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                let now = duration.as_secs_f64();
                Ok(Rc::new(Literal::Basic(AtomicLiteral::Number(now as i32))))
            }
            Err(_) => Err(LoxError::RuntimeError {
                token: None,
                message: "System time error".to_string(),
            })
        }
    }
}