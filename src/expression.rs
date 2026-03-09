use crate::token::{Token, TokenType, AtomicLiteral};
use crate::resolver::{Resolver, Resolvable};
use crate::lox_error::LoxError;
use crate::lox_error::LoxResult;
use crate::token::Literal;
use crate::interpreter::{Interpreter};
use std::rc::Rc;

#[derive(Clone,PartialEq, Eq, Hash)]
pub enum ExpressionType {
    This(Token),
    Binary(BinaryExpression),
    Logical(BinaryExpression),
    Unary(UnaryExpression),
    Literal(AtomicLiteral),
    Grouping(Box<ExpressionType>),
    Call(CallArgs),
    Get(GetArgs),
    Set(SetArgs),
    Super(SuperArgs),
    Variable(Token),
    Assignment(AssignExpression),
    Postfix(PostfixExpression),
}
#[derive(Clone,PartialEq, Eq, Hash)]
pub enum FunctionType {
    Function,
    None,
    Method,
    Initializer
}
#[derive(Clone,PartialEq, Eq, Hash)]
pub enum ClassType {
    None,
    Class,
    SubClass
}

#[derive(Clone,PartialEq, Eq, Hash)]
pub struct SuperArgs {
    pub keyword: Token,
    pub method: Token
}
#[derive(Clone,PartialEq, Eq, Hash)]
pub struct GetArgs {
    pub name: Token,
    pub object: Box<ExpressionType>
}
#[derive(Clone,PartialEq, Eq, Hash)]
pub struct SetArgs {
    pub name: Token,
    pub object: Box<ExpressionType>,
    pub value: Box<ExpressionType>
}
#[derive(Clone,PartialEq, Eq, Hash)]
pub struct CallArgs {
    pub callee: Box<ExpressionType>,
    pub paren: Token,
    pub args: Vec<Box<ExpressionType>>
}
#[derive(Clone,PartialEq, Eq, Hash)]
pub struct AssignExpression {
    pub name: Token,
    pub value: Box<ExpressionType>,
}
#[derive(Clone,PartialEq, Eq, Hash)]
pub struct BinaryExpression {
    pub left: Box<ExpressionType>,
    pub operator: TokenType,
    pub right: Box<ExpressionType>,
}
#[derive(Clone,PartialEq, Eq, Hash)]
pub struct UnaryExpression {
    pub operator: TokenType,
    pub right: Box<ExpressionType>,
}
#[derive(Clone,PartialEq, Eq, Hash)]
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
            },
            ExpressionType::Binary(binary) => {
                resolver.resolve(&*binary.left)?;
                resolver.resolve(&*binary.right)?;
            },
            ExpressionType::Call(call) => {
                resolver.resolve(&*call.callee)?;
                for arg in &call.args{
                    resolver.resolve(&**arg)?;
                }            
            },
            ExpressionType::Grouping(group) => {
                resolver.resolve(&**group)?;
            },
            ExpressionType::Literal(_) => {},
            ExpressionType::Logical(logic) => {
                resolver.resolve(&*logic.left)?;
                resolver.resolve(&*logic.right)?;
            },
            ExpressionType::Unary(unary) => {
                resolver.resolve(&*unary.right)?;
            },
            ExpressionType::Get(get ) => resolver.resolve(get.object.as_ref())?,
            ExpressionType::Set(set) => {
                resolver.resolve(set.object.as_ref())?;
                resolver.resolve(set.value.as_ref())?;
            },
            ExpressionType::This(this) => {
                if resolver.current_class != ClassType::Class {
                    return Err(LoxError::ParseError {
                        token: this.clone(), 
                        message: "Cannot use this outside of methods or class".to_string() })
                }
                resolver.resolve_local(self, this)?
            },
            ExpressionType::Super(superb) => {
                match resolver.current_class {
                    ClassType::None => {
                        Err(LoxError::RuntimeError { token: Some(superb.keyword.clone()), message: String::from("Cannot use super outside of a class") })?
                    }
                    ClassType::Class => {
                        Err(LoxError::RuntimeError { token: Some(superb.keyword.clone()), message: String::from("Cannot use super in a class with no superclass") })?
                    }
                    ClassType::SubClass => resolver.resolve_local(self, &superb.keyword)?
                }
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

    pub fn evaluate(&self, interpreter: &mut Interpreter) -> LoxResult<Literal> {

        let varibale_lookup = |name: &Token, expr: &ExpressionType| {
            let distance = interpreter.local.get(expr);
            match distance {
                Some(d) => {
                    match interpreter.env.borrow().get_at(*d, &name.lexeme) {
                        Ok(value) => Ok(value),
                        Err(_) => interpreter.env.borrow().get(&name)
                    }
                },
                None => interpreter.global.borrow().get(&name),
            }
        };

        match self {
            ExpressionType::This(this) => varibale_lookup(this,self),

            ExpressionType::Super(sup) =>{
                let distance = interpreter.local.get(self);
                match distance {
                    Some(distance) => {
                        let superclass = interpreter.env.borrow().get_at(*distance, "super")?.as_class()?;
                        let object = interpreter.env.borrow().get_at(*distance-1, "this")?.as_instance()?;
                        let method = superclass.find_method(&sup.method.lexeme);
                        match method {
                            None => Err(LoxError::RuntimeError { token: Some(sup.method.clone()), message: "method not found".to_string()}),
                            Some(method) => Ok(Literal::LoxCallable(Rc::new(method.bind(&object))))
                        }
                    }
                    None => {
                        unreachable!()
                    }
                }
            }

            ExpressionType::Literal(value) => Ok(Literal::Basic(value.clone())),

            ExpressionType::Grouping(expr) => expr.evaluate(interpreter),

            ExpressionType::Variable(name) => varibale_lookup(name, self),

            ExpressionType::Assignment(assignment) => {
                let value = assignment.value.evaluate(interpreter)?;
                let distance = interpreter.local.get(self);
                match distance {
                    Some(d) => {
                        interpreter.env.borrow_mut().assign_at(*d, assignment.name.clone(), value.clone())?;
                    }
                    None => {
                        interpreter.global.borrow_mut().assign(assignment.name.clone(), value.clone())?;
                    }
                }
                Ok(value)
            }

            ExpressionType::Call(called) => {
                let callee = called.callee.evaluate(interpreter)?;
                let mut args: Vec<Literal> = Vec::new();
                for arg in &called.args {
                    args.push((**arg).evaluate(interpreter)?);
                }
                
                match callee {
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
                    let current = match interpreter.env.borrow().get(&name) {
                        Ok(val) => val,
                        Err(e) => return Err(e),
                    };
                    
                    match (&post.operator, &current) {
                        (TokenType::INCREMENTOR, Literal::Basic(AtomicLiteral::Number(n))) => {
                            match interpreter.env.borrow_mut().assign(
                                name.clone(),
                                Literal::Basic(AtomicLiteral::Number(n + 1)),
                            ) {
                                Ok(()) => Ok(Literal::Basic(AtomicLiteral::Number(*n))),
                                Err(e) => Err(e),
                            }
                        }
                        (TokenType::DECREMENTOR, Literal::Basic(AtomicLiteral::Number(n))) => {
                            match interpreter.env.borrow_mut().assign(
                                name.clone(),
                                Literal::Basic(AtomicLiteral::Number(n - 1)),
                            ) {
                                Ok(()) => Ok(Literal::Basic(AtomicLiteral::Number(*n))),
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

            ExpressionType::Get(get) => {
                let object = get.object.evaluate(interpreter)?;
                match object {
                    Literal::Instance(i) => {
                        let result = i.get(get.name.clone())?;
                        Ok(result)
                    }
                    _ => {
                        return Err(LoxError::RuntimeError {
                            token: Some(get.name.clone()),
                            message: "Only instances have properties".to_string(),
                        })
                    }
                }
            }

            ExpressionType::Set(set) => {
                let object = set.object.evaluate(interpreter)?;
                let value = set.value.evaluate(interpreter)?;
                match object {
                    Literal::Instance(mut i) => {
                        i.set(set.name.clone(), value.clone());
                        Ok(value)
                    }
                    _ => {
                        Err(LoxError::RuntimeError {
                            token: Some(set.name.clone()),
                            message: "Only instances have fields".to_string(),
                        })
                    }
                }
            }

            ExpressionType::Unary(expr) => {
                let right = &expr.right.evaluate(interpreter)?;
                
                match expr.operator {
                    TokenType::MINUS => {
                        if let Literal::Basic(AtomicLiteral::Number(n)) = right {
                            Ok(Literal::Basic(AtomicLiteral::Number(-n)))
                        } else {
                            Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operand must be a number".to_string(),
                            })
                        }
                    }
                    TokenType::BANG => {
                        Ok(Literal::Basic(AtomicLiteral::Bool(!is_truthy(&right))))
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
                    TokenType::PLUS => match (left, right) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Literal::Basic(AtomicLiteral::Number(a + b))),
                        (
                            Literal::Basic(AtomicLiteral::String(a)),
                            Literal::Basic(AtomicLiteral::String(b)),
                        ) => Ok(Literal::Basic(AtomicLiteral::String(a.to_string() + &b))),
                        (
                            Literal::Basic(AtomicLiteral::String(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Literal::Basic(AtomicLiteral::String(a.to_owned() + &b.to_string()))),
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::String(b)),
                        ) => Ok(Literal::Basic(AtomicLiteral::String(a.to_string() + &b))),
                        _ => return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be two numbers or two strings".to_string(),
                            }),
                    },

                    TokenType::MODULO => match (left, right) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => {
                            if b == 0 {
                                return Err(LoxError::RuntimeError {
                                    token: None,
                                    message: "Modulo by zero".to_string(),
                                });
                            }
                            Ok(Literal::Basic(AtomicLiteral::Number(a % b)))
                        },
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        }
                    },

                    TokenType::MINUS => match (left, right) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Literal::Basic(AtomicLiteral::Number(a - b))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },

                    TokenType::STAR => match (left, right) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Literal::Basic(AtomicLiteral::Number(a * b))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },

                    TokenType::SLASH => match (left, right) {
                        (
                            Literal::Basic(AtomicLiteral::Number(_)),
                            Literal::Basic(AtomicLiteral::Number(0)),
                        ) => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Division by zero".to_string(),
                            });
                        }
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Literal::Basic(AtomicLiteral::Number(a / b))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },
                    TokenType::GREATER => match (left, right) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Literal::Basic(AtomicLiteral::Bool(a > b))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },
                    TokenType::GREATEREQUAL => match (left, right) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Literal::Basic(AtomicLiteral::Bool(a >= b))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },
                    TokenType::LESS => match (left, right) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Literal::Basic(AtomicLiteral::Bool(a < b))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },
                    TokenType::LESSEQUAL => match (left, right) {
                        (
                            Literal::Basic(AtomicLiteral::Number(a)),
                            Literal::Basic(AtomicLiteral::Number(b)),
                        ) => Ok(Literal::Basic(AtomicLiteral::Bool(a <= b))),
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operands must be numbers".to_string(),
                            });
                        },
                    },
                    TokenType::EQUALEQUAL => Ok(Literal::Basic(AtomicLiteral::Bool(
                        is_equal(&left, &right)?,
                    ))),
                    TokenType::BANGEQUAL => Ok(Literal::Basic(AtomicLiteral::Bool(
                        !is_equal(&left, &right)?,
                    ))),
                    _ => Ok(Literal::Basic(AtomicLiteral::Nil)), // should not reach here
                }
            }
        }
    }
}