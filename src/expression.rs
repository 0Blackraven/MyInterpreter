// REFACTORING SUMMARY:
// This file has been updated with the following key changes:
// 1. Added evaluate method to ExpressionType enum for better code organization
// 2. The evaluate method takes a mutable interpreter reference for environment access
// 3. Returns Rc<Literal> to enable shared ownership of values across the interpreter
// 4. All expression evaluation logic is now centralized in this file

use crate::token::{Token, TokenType, AtomicLiteral};
use crate::resolver::{Resolver, Resolvable};
use crate::lox_error::LoxError;
use crate::lox_error::LoxResult;
use crate::token::Literal;
use crate::interpreter::{Interpreter};
use std::rc::Rc;

#[derive(Clone)]
pub enum ExpressionType {
    Binary(BinaryExpression),
    Logical(BinaryExpression),
    Unary(UnaryExpression),
    Literal(AtomicLiteral),
    Grouping(Box<ExpressionType>),
    Call(CallArgs),
    Variable(Token),
    Assignment(AssignExpression),
    Postfix(PostfixExpression),
}
#[derive(Clone)]
pub enum FunctionType {
    Function,
    // Method
}
#[derive(Clone)]
pub struct CallArgs {
    pub callee: Box<ExpressionType>,
    pub paren: Token,
    pub args: Vec<Box<ExpressionType>>
}
#[derive(Clone)]
pub struct AssignExpression {
    pub name: Token,
    pub value: Box<ExpressionType>,
}
#[derive(Clone)]
pub struct BinaryExpression {
    pub left: Box<ExpressionType>,
    pub operator: TokenType,
    pub right: Box<ExpressionType>,
}
#[derive(Clone)]
pub struct UnaryExpression {
    pub operator: TokenType,
    pub right: Box<ExpressionType>,
}
#[derive(Clone)]
pub struct PostfixExpression {
    pub operator: TokenType,
    pub expr: Box<ExpressionType>,
}

impl Resolvable for ExpressionType {
    fn resolve(&self, resolver: & mut Resolver) -> LoxResult<()> {
        match &self {
            ExpressionType::Variable(token) => {
                if resolver.get(token) == false {
                    return Err(LoxError::RuntimeError { token: Some(token.clone()), message: "Cannot read local variable in its own initializer".to_string() });
                }
                resolver.resolve_local(self, token)?;
            },
            ExpressionType::Assignment(assignment) => {
                resolver.resolve(&*assignment.value)?;
                resolver.resolve_local(self, &assignment.name)?;
            }
            ExpressionType::Binary(binary) => {
                resolver.resolve(&*binary.left)?;
                resolver.resolve(&*binary.right)?;
            }
            ExpressionType::Call(call) => {
                resolver.resolve(&*call.callee)?;
                for arg in &call.args{
                    resolver.resolve(&**arg)?;
                }            
            }
            ExpressionType::Grouping(group) => {
                resolver.resolve(&**group)?;
            }
            ExpressionType::Literal(_) => {},
            ExpressionType::Logical(logic) => {
                resolver.resolve(&*logic.left)?;
                resolver.resolve(&*logic.right)?;
            }
            ExpressionType::Unary(unary) => {
                resolver.resolve(&*unary.right)?;
            }
            _ => {}
        }
        Ok(())
    }
}


pub fn is_truthy(value: &Literal) -> bool {
    match value {
        Literal::Basic(AtomicLiteral::Nil) => false,
        Literal::Basic(AtomicLiteral::Bool(false)) => false,
        _ => true,
    }
}

pub fn is_equal(a: &Literal, b: &Literal) -> LoxResult<bool> {
    match (a, b) {
        (Literal::Basic(a), Literal::Basic(b)) => Ok(a == b),
        _ => Err(LoxError::RuntimeError {
            token: None,
            message: "Cannot compare callable objects".to_string(),
        })
    }
}

impl ExpressionType {

    pub fn evaluate(&mut self, interpreter: &mut Interpreter) -> LoxResult<Rc<Literal>> {
        match self {
            ExpressionType::Literal(value) => Ok(Rc::new(Literal::Basic(value.clone()))),

            ExpressionType::Grouping(expr) => expr.evaluate(interpreter),

            ExpressionType::Variable(name) => match interpreter.storage.borrow().get(&name) {
                    Ok(value) => Ok(value),
                    Err(e) => return Err(e),
                }

            ExpressionType::Assignment(pookie) => {
                let value: Rc<Literal> = pookie.value.evaluate(interpreter)?;
                match interpreter.storage.borrow_mut().assign(pookie.name.clone(), value.clone()) {
                    Ok(()) => Ok(value),
                    Err(e) => return Err(e),
                }
            }

            ExpressionType::Call(called) => {
                let callee = called.callee.evaluate(interpreter)?;
                let mut args: Vec<Rc<Literal>> = Vec::new();
                for arg in &mut called.args {
                    args.push((**arg).evaluate(interpreter)?);
                }
                
                match callee.as_ref() {
                    Literal::LoxCallable(function) => {
                        if args.len() != function.arity() {
                            return Err(LoxError::RuntimeError {
                                token: Some(called.paren.clone()),
                                message: format!("Expected {} arguments but got {}", function.arity(), args.len()),
                            });
                        }
                        function.call(interpreter, args)
                    }
                    _ => {
                        Err(LoxError::RuntimeError {
                            token: Some(called.paren.clone()),
                            message: "Can only call functions and classes".to_string(),
                        })
                    }
                }
            }

            ExpressionType::Postfix(post) => match &*post.expr {
                ExpressionType::Variable(name) => {
                    let current = match interpreter.storage.borrow().get(&name) {
                        Ok(val) => val,
                        Err(e) => return Err(e),
                    };
                    
                    match (&post.operator, &*current) {
                        (TokenType::INCREMENTOR, Literal::Basic(AtomicLiteral::Number(n))) => {
                            match interpreter.storage.borrow_mut().assign(
                                name.clone(),
                                Rc::new(Literal::Basic(AtomicLiteral::Number(n + 1.0))),
                            ) {
                                Ok(()) => Ok(Rc::new(Literal::Basic(AtomicLiteral::Number(*n)))),
                                Err(e) => Err(e),
                            }
                        }
                        (TokenType::DECREMENTOR, Literal::Basic(AtomicLiteral::Number(n))) => {
                            match interpreter.storage.borrow_mut().assign(
                                name.clone(),
                                Rc::new(Literal::Basic(AtomicLiteral::Number(n - 1.0))),
                            ) {
                                Ok(()) => Ok(Rc::new(Literal::Basic(AtomicLiteral::Number(*n)))),
                                Err(e) => Err(e),
                            }
                        }
                        _ => Err(LoxError::RuntimeError {
                            token: Some(name.clone()),
                            message: "Postfix operators can only be applied to numbers".to_string(),
                        })
                    }
                }
                _ => Err(LoxError::RuntimeError {
                        token: None,
                        message: "Postfix operators can only be applied to variables".to_string(),
                    }),
            },

            ExpressionType::Unary(expr) => {
                let right = &expr.right.evaluate(interpreter)?;
                
                match expr.operator {
                    TokenType::MINUS => {
                        if let Literal::Basic(AtomicLiteral::Number(n)) = **right {
                            Ok(Rc::new(Literal::Basic(AtomicLiteral::Number(-n))))
                        } else {
                            Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operand must be a number".to_string(),
                            })
                        }
                    }
                    TokenType::BANG => {
                        Ok(Rc::new(Literal::Basic(AtomicLiteral::Bool(!is_truthy(&right)))))
                    }
                    _ => unreachable!(),
                }
            }

            ExpressionType::Logical(expr) => {
                let left = &expr.left.evaluate(interpreter)?;
                if expr.operator == TokenType::OR {
                    if is_truthy(&left) {
                        Ok(left.clone())
                    } else {
                        expr.right.evaluate(interpreter)
                    }
                } else {
                    if !is_truthy(&left) {
                        Ok(left.clone())
                    } else {
                        expr.right.evaluate(interpreter)
                    }
                }
            }

            ExpressionType::Binary(expr) => {
                let left = expr.left.evaluate(interpreter)?;
                let right = expr.right.evaluate(interpreter)?;

                match expr.operator {
                    TokenType::PLUS => match (left.as_ref(), right.as_ref()) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Rc::new(Literal::Basic(AtomicLiteral::Number(a + b)))),
                        (
                            Literal::Basic(AtomicLiteral::String(a)),
                            Literal::Basic(AtomicLiteral::String(b)),
                        ) => Ok(Rc::new(Literal::Basic(AtomicLiteral::String(a.to_string() + b)))),
                        (
                            Literal::Basic(AtomicLiteral::String(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Rc::new(Literal::Basic(AtomicLiteral::String(a.to_owned() + &b.to_string())))),
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::String(b)),
                        ) => Ok(Rc::new(Literal::Basic(AtomicLiteral::String(a.to_string() + &b)))),
                        _ => return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be two numbers or two strings".to_string(),
                            }),
                    },

                    TokenType::MODULO => match (left.as_ref(), right.as_ref()) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => {
                            if *b == 0.0 {
                                return Err(LoxError::RuntimeError {
                                    token: None,
                                    message: "Modulo by zero".to_string(),
                                });
                            }
                            Ok(Rc::new(Literal::Basic(AtomicLiteral::Number(a % b))))
                        },
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        }
                    },

                    TokenType::MINUS => match (left.as_ref(), right.as_ref()) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Rc::new(Literal::Basic(AtomicLiteral::Number(a - b)))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },

                    TokenType::STAR => match (left.as_ref(), right.as_ref()) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Rc::new(Literal::Basic(AtomicLiteral::Number(a * b)))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },

                    TokenType::SLASH => match (left.as_ref(), right.as_ref()) {
                        (
                            Literal::Basic(AtomicLiteral::Number(_)),
                            Literal::Basic(AtomicLiteral::Number(0.0)),
                        ) => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Division by zero".to_string(),
                            });
                        }
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Rc::new(Literal::Basic(AtomicLiteral::Number(a / b)))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },
                    TokenType::GREATER => match (left.as_ref(), right.as_ref()) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Rc::new(Literal::Basic(AtomicLiteral::Bool(a > b)))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },
                    TokenType::GREATEREQUAL => match (left.as_ref(), right.as_ref()) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Rc::new(Literal::Basic(AtomicLiteral::Bool(a >= b)))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },
                    TokenType::LESS => match (left.as_ref(), right.as_ref()) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Rc::new(Literal::Basic(AtomicLiteral::Bool(a < b)))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },
                    TokenType::LESSEQUAL => match (left.as_ref(), right.as_ref()) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Rc::new(Literal::Basic(AtomicLiteral::Bool(a <= b)))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },
                    TokenType::EQUALEQUAL => Ok(Rc::new(Literal::Basic(AtomicLiteral::Bool(
                        is_equal(&left, &right)?,
                    )))),
                    TokenType::BANGEQUAL => Ok(Rc::new(Literal::Basic(AtomicLiteral::Bool(
                        !is_equal(&left, &right)?,
                    )))),
                    _ => Ok(Rc::new(Literal::Basic(AtomicLiteral::Nil))), // should not reach here
                }
            }
        }
    }
}