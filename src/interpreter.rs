use crate::{
    environment::Environment,
    parser::{ExpressionType, IfType, StatementType, WhileType},
    token::{Literal, TokenType},
};
use core::panic;
use std::mem::replace;

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

    fn evaluate(&mut self, expr: &ExpressionType) -> Literal {
        match expr {
            ExpressionType::Literal(value) => value.clone(),

            ExpressionType::Grouping(expr) => self.evaluate(expr),

            ExpressionType::Variable(name) => self.storage.get(&name.lexeme),

            ExpressionType::Assignment(pookie) => {
                let value: Literal = self.evaluate(&pookie.value);
                self.storage.assign(pookie.name.lexeme.clone(), &value);
                return value;
            }

            ExpressionType::Postfix(post) => {
                match &*post.expr {
                    ExpressionType::Variable(name) => {
                        let current = self.storage.get(&name.lexeme);

                        match (&post.operator, &current) {
                            (TokenType::INCREMENTOR, Literal::Number(n)) => {
                                self.storage
                                    .assign(name.lexeme.clone(), &Literal::Number(n + 1.0));
                                return Literal::Number(*n);
                            }

                            (TokenType::DECREMENTOR, Literal::Number(n)) => {
                                self.storage
                                    .assign(name.lexeme.clone(), &Literal::Number(n - 1.0));
                                return Literal::Number(*n);
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
                        if let Literal::Number(n) = right {
                            Literal::Number(-n)
                        } else {
                            panic!("Operand must be a number.");
                        }
                    }
                    TokenType::BANG => Literal::Bool(!self.is_truthy(&right)),
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
                    TokenType::PLUS => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a + b),
                        (Literal::String(a), Literal::String(b)) => Literal::String(a + &b),
                        (Literal::String(a), Literal::Number(b)) => {
                            Literal::String(a + &b.to_string())
                        }
                        (Literal::Number(a), Literal::String(b)) => {
                            Literal::String(a.to_string() + &b)
                        }
                        _ => panic!("Operands must be two numbers or two strings."),
                    },

                    TokenType::MODULO => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a % b),
                        _ => panic!("Operands must be numbers."),
                    },

                    TokenType::MINUS => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a - b),
                        _ => panic!("Operands must be numbers."),
                    },

                    TokenType::STAR => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a * b),
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::SLASH => match (left, right) {
                        (Literal::Number(0.0), Literal::Number(0.0)) => panic!("Division by zero."),
                        (Literal::Number(a), Literal::Number(b)) => Literal::Number(a / b),
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::GREATER => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Bool(a > b),
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::GREATEREQUAL => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Bool(a >= b),
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::LESS => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Bool(a < b),
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::LESSEQUAL => match (left, right) {
                        (Literal::Number(a), Literal::Number(b)) => Literal::Bool(a <= b),
                        _ => panic!("Operands must be numbers."),
                    },
                    TokenType::EQUALEQUAL => Literal::Bool(self.is_equal(&left, &right)),
                    TokenType::BANGEQUAL => Literal::Bool(!self.is_equal(&left, &right)),
                    _ => Literal::Nil, // should not reach here
                }
            }
        }
    }

    fn is_truthy(&self, value: &Literal) -> bool {
        match value {
            Literal::Nil => false,
            Literal::Bool(false) => false,
            _ => true,
        }
    }

    fn is_equal(&self, a: &Literal, b: &Literal) -> bool {
        a == b
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
                ExpressionType::Literal(Literal::Nil) => {
                    self.storage.define(expr.name.lexeme.clone(), Literal::Nil)
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
