use std::{time::{SystemTime, UNIX_EPOCH}, rc::Rc};

use crate::{callable::Callable, interpreter::Interpreter, token::{AtomicLiteral, Literal}};


pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self ,_: &mut Interpreter, _: Vec<Rc<Literal>>) -> Rc<Literal> {
        let now: f64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        Rc::new(Literal::Basic(AtomicLiteral::Number(now as f32)))
    }
}