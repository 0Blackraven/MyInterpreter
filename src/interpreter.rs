use crate::{
    environment::Environment,
    parser::{ExpressionType, IfType, StatementType, WhileType},
    token::{Literal, TokenType, AtomicLiteral},
};
use core::panic;
use std::mem::replace;
use std::rc::Rc;
// use crate::callable::Callable;

pub struct Interpreter {
    storage: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            storage: Environment::new(None),
        }
    }

    fn evaluate_blocks(&mut self, statements: &Vec<StatementType>) {
        let previous = replace(&mut self.storage, Environment::new(None));
        self.storage = Environment::new(Some(Box::new(previous)));
        for statement in statements {
            self.evaluate_statement(&statement)
        }
        let enclosing = self.storage.enclosing.take().unwrap();
        self.storage = *enclosing;
    }

    fn evaluate_if(&mut self, ifinput: &IfType) {
        let comparison = self.evaluate(&ifinput.comparison);

        if self.is_truthy(&comparison) {
            let ifcase: &StatementType = &*ifinput.ifcase;
            self.evaluate_statement(ifcase);
        } else if let Some(elsecase) = &ifinput.elsecase {
            self.evaluate_statement(&*elsecase);
        }
    }

    fn evaluate_while(&mut self, wild: &WhileType) {
        let condition = &wild.condition;
        let statement = &*wild.statement;

        while {
            let cond = self.evaluate(condition);
            self.is_truthy(&cond)
        } {
            self.evaluate_statement(statement);
        }
    }

    fn evaluate(&mut self, expr: &ExpressionType) -> Rc<Literal> {
        match expr {
            ExpressionType::Literal(value) => Rc::new(Literal::Basic(value.clone())),

            ExpressionType::Grouping(expr) => self.evaluate(expr),

            ExpressionType::Variable(name) => self.storage.get(&name.lexeme),

            ExpressionType::Assignment(pookie) => {
                let value:Rc<Literal>  = self.evaluate(&pookie.value);
                self.storage.assign(pookie.name.lexeme.clone(), value.clone());
                return value;
            }

            ExpressionType::Call(called) => {
                let callee = self.evaluate(&called.callee);
                let mut args: Vec<Rc<Literal>> = Vec::new();
                for arg in &called.args {
                    args.push(self.evaluate(&*arg));
                }

                match callee.as_ref() {
                    Literal::LoxCallable(function ) => {
                        return function.call(self,args);
                    },
                    _ => {
                        panic!("should not reach here")
                    }
                }
            }
            ExpressionType::Postfix(post) => {
                match &*post.expr {
                    ExpressionType::Variable(name) => {
                        let current = self.storage.get(&name.lexeme);

                        match (&post.operator, &*current) {
                            (TokenType::INCREMENTOR, Literal::Basic(AtomicLiteral::Number(n))) => {
                                self.storage
                                    .assign(name.lexeme.clone(), Rc::new(Literal::Basic(AtomicLiteral::Number(n + 1.0))));
                                return Rc::new(Literal::Basic(AtomicLiteral::Number(*n)));
                            }

                            (TokenType::DECREMENTOR, Literal::Basic(AtomicLiteral::Number(n))) => {
                                self.storage
                                    .assign(name.lexeme.clone(), Rc::new(Literal::Basic(AtomicLiteral::Number(n - 1.0))));
                                return Rc::new(Literal::Basic(AtomicLiteral::Number(*n)));
                            }

                            _ => panic!("++ / -- only allowed on numbers"),
                        }
                    }
                    _ => panic!("Parser should not ever reach this"),
                }
            }

            ExpressionType::Unary(expr) => {
                let right = self.evaluate(&expr.right);

                match expr.operator {
                    TokenType::MINUS => {
                        if let Literal::Basic(AtomicLiteral::Number(n)) = *right {
                            Rc::new(Literal::Basic(AtomicLiteral::Number(-n)))
                        } else {
                            panic!("Operand must be a number.");
                        }
                    }
                    TokenType::BANG => Rc::new(Literal::Basic(AtomicLiteral::Bool(!self.is_truthy(&right)))),
                    _ => unreachable!(),
                }
            }

            ExpressionType::Logical(expr) => {
                let left = self.evaluate(&expr.left);
                if expr.operator == TokenType::OR {
                    if self.is_truthy(&left) {
                        return left;
                    }
                } else {
                    if !self.is_truthy(&left) {
                        return left;
                    }
                }
                return self.evaluate(&expr.right);
            }

            ExpressionType::Binary(expr) => {
                let left = self.evaluate(&expr.left);
                let right = self.evaluate(&expr.right);

                match expr.operator {
                    TokenType::PLUS => match (left.as_ref(), right.as_ref()) {
                        (Literal::Basic(AtomicLiteral::Number(a)), Literal::Basic(AtomicLiteral::Number(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::Number(a + b)))
                        },
                        (Literal::Basic(AtomicLiteral::String(a)), Literal::Basic(AtomicLiteral::String(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::String(a.to_string() + b)))
                        },
                        (Literal::Basic(AtomicLiteral::String(a)), Literal::Basic(AtomicLiteral::Number(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::String(b.to_string() + a)))
                        }
                        (Literal::Basic(AtomicLiteral::Number(a)), Literal::Basic(AtomicLiteral::String(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::String(a.to_string() + &b)))
                        }
                        _ => panic!("Operands must be two numbers or two strings."),
                    },

                    TokenType::MODULO => match (left.as_ref(), right.as_ref()) {
                        (Literal::Basic(AtomicLiteral::Number(a)), Literal::Basic(AtomicLiteral::Number(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::Number(a % b)))
                        },
                        _ => panic!("Operands must be numbers."),
                    },

                    TokenType::MINUS => match (left.as_ref(), right.as_ref()) {
                        (Literal::Basic(AtomicLiteral::Number(a)), Literal::Basic(AtomicLiteral::Number(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::Number(a - b)))
                        },
                        _ => panic!("Operands must be numbers."),
                    },

                    TokenType::STAR => match (left.as_ref(), right.as_ref()) {
                        (Literal::Basic(AtomicLiteral::Number(a)), Literal::Basic(AtomicLiteral::Number(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::Number(a * b)))
                        },
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::SLASH => match (left.as_ref(), right.as_ref()) {
                        (Literal::Basic(AtomicLiteral::Number(_)), Literal::Basic(AtomicLiteral::Number(0.0))) => {
                            panic!("Division by zero.")
                        },
                        (Literal::Basic(AtomicLiteral::Number(a)), Literal::Basic(AtomicLiteral::Number(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::Number(a / b)))
                        },
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::GREATER => match (left.as_ref(), right.as_ref()) {
                        (Literal::Basic(AtomicLiteral::Number(a)), Literal::Basic(AtomicLiteral::Number(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::Bool(a > b)))
                        },
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::GREATEREQUAL => match (left.as_ref(), right.as_ref()) {
                        (Literal::Basic(AtomicLiteral::Number(a)), Literal::Basic(AtomicLiteral::Number(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::Bool(a >= b)))
                        },
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::LESS => match (left.as_ref(), right.as_ref()) {
                        (Literal::Basic(AtomicLiteral::Number(a)), Literal::Basic(AtomicLiteral::Number(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::Bool(a < b)))
                        },
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::LESSEQUAL => match (left.as_ref(), right.as_ref()) {
                        (Literal::Basic(AtomicLiteral::Number(a)), Literal::Basic(AtomicLiteral::Number(b))) => {
                            Rc::new(Literal::Basic(AtomicLiteral::Bool(a <= b)))
                        },
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::EQUALEQUAL => Rc::new(Literal::Basic(AtomicLiteral::Bool(self.is_equal(&left, &right)))),
                    TokenType::BANGEQUAL => Rc::new(Literal::Basic(AtomicLiteral::Bool(!self.is_equal(&left, &right)))),
                    _ => Rc::new(Literal::Basic(AtomicLiteral::Nil)), // should not reach here
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

    fn is_equal(&self, a: &Literal, b: &Literal) -> bool {
        match (a,b) {
            (Literal::Basic(a), Literal::Basic(b)) => return a == b,
            _ => panic!("not possible to compare callables")
        }
    }

    fn evaluate_statement(&mut self, statement: &StatementType) {
        match statement {
            StatementType::ExpressionStatement(value) => {
                self.evaluate(&value);
                println!("expr type so no output but worked ! ");
            }
            StatementType::PrintStatement(expr) => {
                let output = self.evaluate(&expr);
                println!("{}", output);
            }
            StatementType::LetStatement(expr) => match *expr.initializer {
                ExpressionType::Literal(AtomicLiteral::Nil) => {
                    self.storage.define(expr.name.lexeme.clone(), Rc::new(Literal::Basic(AtomicLiteral::Nil)))
                }
                _ => {
                    let result = self.evaluate(&expr.initializer);
                    self.storage.define(expr.name.lexeme.clone(), result)
                }
            },
            StatementType::BlockStatement(statements) => self.evaluate_blocks(statements),
            StatementType::IfStatement(iftype) => self.evaluate_if(iftype),
            StatementType::WhileStatement(wild) => self.evaluate_while(wild),
        }
    }
    pub fn interpreter(&mut self, statements: Vec<StatementType>) {
        for statement in statements {
            self.evaluate_statement(&statement);
        }
    }
}
