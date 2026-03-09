use std::{time::{SystemTime, UNIX_EPOCH}};
use crate::lox_error::{LoxError, LoxResult};

use crate::{callable::Callable, interpreter::Interpreter, token::{AtomicLiteral, Literal}};


pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, _: Vec<Literal>) -> LoxResult<Literal> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                let now = duration.as_secs_f64();
                Ok(Literal::Basic(AtomicLiteral::Number(now as i32)))
            }
            Err(_) => Err(LoxError::RuntimeError {
                token: None,
                message: "System time error".to_string(),
            })
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}