use crate::expression::{ClassType, ExpressionType, FunctionType};
use crate::interpreter::Interpreter;
use crate::lox_error::{LoxError, LoxResult};
use crate::statement::FunctionProps;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    pub scopes: RefCell<Vec<Scope>>,
    pub current_function: FunctionType,
    pub current_class: ClassType
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
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Default::default(),
            current_function: FunctionType::None,
            current_class: ClassType::None
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

    pub fn declare (&self, token: &Token) -> LoxResult<()> {
        if self.scopes.borrow().is_empty() {
            return Ok(());
        }
        let mut scopes = self.scopes.borrow_mut();
        let scope_result = scopes.last_mut();
        if let Some(scope) = scope_result {
            if scope.contains_key(&token.lexeme) {
                return Err(
                    LoxError::RuntimeError { token: Some(token.clone()), message: format!("Variable with name {} already declared in this scope", token.lexeme) }
                );}
            scope.insert(token.lexeme.clone(), false);
        }    
        Ok(())
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

    pub fn resolve_local (&mut self, expr: &ExpressionType, token: &Token) -> LoxResult<()>{
        let scopes = self.scopes.borrow();
        for (idx, scope) in scopes.iter().enumerate().rev() {
            if scope.contains_key(&token.lexeme) {
                self.interpreter.resolve(expr, scopes.len() - idx - 1);
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn resolve_function (&mut self, func: &FunctionProps, func_type: FunctionType) -> LoxResult<()> {
        let enclosing_function = self.current_function.clone();
        self.current_function = func_type;
        self.begin_scope();
        for param in &func.params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve(&*func.body)?;
        self.end_scope();
        self.current_function = enclosing_function;
        Ok(())
    }
}
