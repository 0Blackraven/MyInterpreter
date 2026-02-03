use crate::{
    environment::Environment,
    parser::{ExpressionType, StatementType},
    token::{Literal, TokenType},
};

pub struct Interpreter {
    storage: Environment,
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            storage: Environment::new(None)
        }
    }

    fn evaluate(&mut self, expr: &ExpressionType) -> Literal {
        match expr {
            ExpressionType::Literal(value) => value.clone(),

            ExpressionType::Grouping(expr) => self.evaluate(expr),

            ExpressionType::Variable(name) => self.storage.get(&name.lexeme),

            ExpressionType::Assignment(pookie) => {
                let value:Literal = self.evaluate(&pookie.value);
                self.storage.assign(pookie.name.lexeme.clone(), &value);
                return value;
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

    pub fn interpreter(&mut self, statements: Vec<StatementType>) {
        for statement in statements {
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
                        self.storage.define(expr.name.lexeme, Literal::Nil)
                    }
                    _ => {
                        let result = self.evaluate(&expr.initializer);
                        self.storage.define(expr.name.lexeme, result)
                    }
                },
            }
        }
    }
}
