use crate::expression::ExpressionType;
use crate::interpreter::Interpreter;
use crate::lox_error::LoxResult;
use crate::statement::FunctionProps;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Resolver<'a> {
    interpreter: &'a Interpreter,
    scopes: RefCell<Vec<Scope>>,
}

type Scope = HashMap<String, bool>;

pub trait Resolvable {
    fn resolve(&self, resolver: &mut Resolver) -> LoxResult<()>;
}

impl<T: Resolvable> Resolvable for Vec<T> {
    fn resolve(&self, resolver: &mut Resolver) -> LoxResult<()> {
        for i in self {
            i.resolve(resolver)?;
        }
        Ok(())
    }
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Default::default(),
        }
    }

    pub fn begin_scope(&mut self) {
        self.scopes.borrow_mut().push(HashMap::new());
    }

    pub fn end_scope(&mut self) {
        self.scopes.borrow_mut().pop();
    }

    pub fn resolve<T: Resolvable>(&mut self, node: &T) -> LoxResult<()> {
        node.resolve(self)
    }

    pub fn declare (&self, token: &Token) {
        if self.scopes.borrow().is_empty() {
            return;
        }
        let mut scopes = self.scopes.borrow_mut();
        let scope_result = scopes.last_mut();
        if let Some(scope) = scope_result {
            scope.insert(token.lexeme.clone(), false);
        }
    }

    pub fn define (&mut self, token: &Token) {
        if self.scopes.borrow().is_empty() {
            return;
        }
        let mut scopes = self.scopes.borrow_mut();
        let scope_result = scopes.last_mut();
        if let Some(scope) = scope_result {
            scope.insert(token.lexeme.clone(), true);
        }
    }

    pub fn get(&mut self, token: &Token) -> bool{
        let mut result = true;
        if self.scopes.borrow().is_empty() {
            return true;
        }
        let mut scopes = self.scopes.borrow_mut();
        let scope_result = scopes.last_mut();
        if let Some(scope) = scope_result {
            if scope.get(&token.lexeme) == Some(&false) {
                result = false;
            }
        }
        return result;
    }

    pub fn resolve_local (&self, expr: &ExpressionType, token: &Token) -> LoxResult<()>{
        let scopes = self.scopes.borrow();
        for (idx, scope) in scopes.iter().enumerate().rev() {
            if scope.contains_key(&token.lexeme) {
                self.interpreter.resolve(expr, scopes.len() - idx - 1)?;
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn resolve_function (&mut self, func: &FunctionProps) -> LoxResult<()> {
        self.begin_scope();
        for param in &func.params {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&*func.body)?;
        self.end_scope();
        Ok(())
    }
}
