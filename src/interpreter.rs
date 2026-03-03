use crate::loxfuncs::LoxFunction;
use crate::parser::{ExpressionType, IfProps, StatementType, WhileProps};
use crate::token::{AtomicLiteral, Literal, TokenType};
use crate::{clock::Clock, environment::Environment};
use std::cell::RefCell;
use std::rc::Rc;
use crate::lox_error::{LoxError, LoxResult};

pub struct Interpreter {
    pub storage: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let interpreter = Interpreter {
            storage: Rc::new(RefCell::new(Environment::new(None))),
        };
        interpreter.storage.borrow_mut().define(
            "clock".to_string(),
            Rc::new(Literal::LoxCallable(Box::new(Clock))),
        );
        interpreter
    }

    fn evaluate_blocks(&mut self, statements: &Vec<StatementType>) -> LoxResult<()> {
        let previous = Rc::clone(&self.storage);
    
        self.storage = Rc::new(RefCell::new(Environment::new(Some(previous.clone()))));
    
        for statement in statements {
            self.evaluate_statement(statement)?;
        }
    
        self.storage = previous;
        Ok(())
    }

    pub fn evaluate_func_block(
        &mut self,
        statement: &StatementType,
        closure: Rc<RefCell<Environment>>,
    ) -> LoxResult<()> {
        let previous = Rc::clone(&self.storage);
    
        self.storage = closure;
    
        self.evaluate_statement(statement)?;
    
        self.storage = previous;
        Ok(())
    }

    fn evaluate_if(&mut self, ifinput: &IfProps) -> LoxResult<()> {
        let comparison = self.evaluate(&ifinput.comparison)?;
    
        if self.is_truthy(&comparison) {
            let ifcase: &StatementType = &*ifinput.ifcase;
            self.evaluate_statement(ifcase)?;
        } else if let Some(elsecase) = &ifinput.elsecase {
            self.evaluate_statement(&*elsecase)?;
        }
        Ok(())
    }

    fn evaluate_while(&mut self, wild: &WhileProps) -> LoxResult<()> {
        let condition = &wild.condition;
        let statement = &*wild.statement;
    
        while {
            let cond = self.evaluate(condition)?;
            self.is_truthy(&cond)
        } {
            self.evaluate_statement(statement)?;
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &ExpressionType) -> LoxResult<Rc<Literal>> {
        match expr {
            ExpressionType::Literal(value) => Ok(Rc::new(Literal::Basic(value.clone()))),

            ExpressionType::Grouping(expr) => self.evaluate(expr),

            ExpressionType::Variable(name) => match self.storage.borrow().get(&name) {
                    Ok(value) => Ok(value),
                    Err(e) => return Err(e),
                }

            ExpressionType::Assignment(pookie) => {
                let value: Rc<Literal> = self.evaluate(&pookie.value)?;
                match self.storage.borrow_mut().assign(pookie.name.clone(), value.clone()) {
                    Ok(()) => Ok(value),
                    Err(e) => return Err(e),
                }
            }

            ExpressionType::Call(called) => {
                let callee = self.evaluate(&called.callee)?;
                let mut args: Vec<Rc<Literal>> = Vec::new();
                for arg in &called.args {
                    args.push(self.evaluate(&*arg)?);
                }
                
                match callee.as_ref() {
                    Literal::LoxCallable(function) => {
                        if args.len() != function.arity() {
                            return Err(LoxError::RuntimeError {
                                token: Some(called.paren.clone()),
                                message: format!("Expected {} arguments but got {}", function.arity(), args.len()),
                            });
                        }
                        function.call(self, args)
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
                    let current = match self.storage.borrow().get(&name) {
                        Ok(val) => val,
                        Err(e) => return Err(e),
                    };
                    
                    match (&post.operator, &*current) {
                        (TokenType::INCREMENTOR, Literal::Basic(AtomicLiteral::Number(n))) => {
                            match self.storage.borrow_mut().assign(
                                name.clone(),
                                Rc::new(Literal::Basic(AtomicLiteral::Number(n + 1.0))),
                            ) {
                                Ok(()) => Ok(Rc::new(Literal::Basic(AtomicLiteral::Number(*n)))),
                                Err(e) => Err(e),
                            }
                        }
                        (TokenType::DECREMENTOR, Literal::Basic(AtomicLiteral::Number(n))) => {
                            match self.storage.borrow_mut().assign(
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
                let right = self.evaluate(&expr.right)?;
                
                match expr.operator {
                    TokenType::MINUS => {
                        if let Literal::Basic(AtomicLiteral::Number(n)) = *right {
                            Ok(Rc::new(Literal::Basic(AtomicLiteral::Number(-n))))
                        } else {
                            Err(LoxError::RuntimeError {
                                token: None,
                                message: "Operand must be a number".to_string(),
                            })
                        }
                    }
                    TokenType::BANG => {
                        Ok(Rc::new(Literal::Basic(AtomicLiteral::Bool(!self.is_truthy(&right)))))
                    }
                    _ => unreachable!(),
                }
            }

            ExpressionType::Logical(expr) => {
                let left = self.evaluate(&expr.left)?;
                if expr.operator == TokenType::OR {
                    if self.is_truthy(&left) {
                        Ok(left)
                    } else {
                        self.evaluate(&expr.right)
                    }
                } else {
                    if !self.is_truthy(&left) {
                        Ok(left)
                    } else {
                        self.evaluate(&expr.right)
                    }
                }
            }

            ExpressionType::Binary(expr) => {
                let left = self.evaluate(&expr.left)?;
                let right = self.evaluate(&expr.right)?;

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
                        self.is_equal(&left, &right)?,
                    )))),
                    TokenType::BANGEQUAL => Ok(Rc::new(Literal::Basic(AtomicLiteral::Bool(
                        !self.is_equal(&left, &right)?,
                    )))),
                    _ => Ok(Rc::new(Literal::Basic(AtomicLiteral::Nil))), // should not reach here
                }
            }
        }
    }

    fn is_truthy(&self, value: &Literal) -> bool {
        match value {
            Literal::Basic(AtomicLiteral::Nil) => false,
            Literal::Basic(AtomicLiteral::Bool(false)) => false,
            _ => true,
        }
    }

    fn is_equal(&self, a: &Literal, b: &Literal) -> LoxResult<bool> {
        match (a, b) {
            (Literal::Basic(a), Literal::Basic(b)) => Ok(a == b),
            _ => Err(LoxError::RuntimeError {
                token: None,
                message: "Cannot compare callable objects".to_string(),
            })
        }
    }

    fn evaluate_statement(&mut self, statement: &StatementType) -> LoxResult<()> {
        match statement {
            StatementType::ExpressionStatement(value) => {
                self.evaluate(&value)?;
                Ok(())
            }
            StatementType::PrintStatement(expr) => {
                let output = self.evaluate(&expr)?;
                println!("{}", output);
                Ok(())
            }
            StatementType::LetStatement(expr) => match *expr.initializer {
                ExpressionType::Literal(AtomicLiteral::Nil) => {
                    self.storage.borrow_mut().define(
                        expr.name.lexeme.clone(),
                        Rc::new(Literal::Basic(AtomicLiteral::Nil)),
                    );
                    Ok(())
                }
                _ => {
                    let result = self.evaluate(&expr.initializer)?;
                    self.storage
                        .borrow_mut()
                        .define(expr.name.lexeme.clone(), result);
                    Ok(())
                }
            },
            StatementType::BlockStatement(statements) => self.evaluate_blocks(statements),
            StatementType::IfStatement(iftype) => self.evaluate_if(iftype),
            StatementType::WhileStatement(wild) => self.evaluate_while(wild),
            StatementType::Function(func_props) => {
                let function = LoxFunction::new(Rc::new(func_props), self);
                self.storage.borrow_mut().define(
                    func_props.name.lexeme.clone(),
                    Rc::new(Literal::LoxCallable(Box::new(function))),
                );
                Ok(())
            }
            _ => Ok(())
        }
    }

    pub fn interpreter(&mut self, statements: Vec<StatementType>) -> LoxResult<()> {
        for statement in statements {
            self.evaluate_statement(&statement)?;
        }
        Ok(())
    }
}
