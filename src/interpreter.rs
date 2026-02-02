use crate::{
    parser::{ExpressionType, LiteralValue},
    token::{TokenType},
};

pub fn interpreter(expr: &ExpressionType) -> LiteralValue {
    match expr {
        ExpressionType::Literal(value) => value.clone(),

        ExpressionType::Grouping(expr) => interpreter(expr),

        ExpressionType::Unary(expr) => {
            let right = interpreter(&expr.right);

            match expr.operator {
                TokenType::MINUS => {
                    if let LiteralValue::Number(n) = right {
                        LiteralValue::Number(-n)
                    } else {
                        panic!("Operand must be a number.");
                    }
                }
                TokenType::BANG => {
                    LiteralValue::Bool(!is_truthy(&right))
                }
                _ => unreachable!(),
            }
        }

        ExpressionType::Binary(expr) => {
            let left = interpreter(&expr.left);
            let right = interpreter(&expr.right);

            match expr.operator {
                TokenType::PLUS =>
                     match (left, right) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) =>
                            LiteralValue::Number(a + b),
                        (LiteralValue::String(a), LiteralValue::String(b)) =>
                            LiteralValue::String(a + &b),
                        (LiteralValue::String(a), LiteralValue::Number(b)) =>
                            LiteralValue::String(a + &b.to_string()),
                        (LiteralValue::Number(a),LiteralValue::String(b)) => 
                            LiteralValue::String(a.to_string() + &b),
                        _ => panic!("Operands must be two numbers or two strings."),
                    },

                TokenType::MINUS =>
                    match (left, right) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) =>
                            LiteralValue::Number(a - b),
                        _ => panic!("Operands must be numbers."),
                    }   

                TokenType::STAR  => 
                    match (left, right) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) =>
                            LiteralValue::Number(a * b),
                        _ => panic!("Operands must be numbers."),
                    }
                TokenType::SLASH => 
                    match (left, right) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) =>
                            LiteralValue::Number(a / b),
                        _ => panic!("Operands must be numbers."),
                    }
                TokenType::GREATER =>
                    match (left, right) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) =>
                            LiteralValue::Bool(a > b),
                        _ => panic!("Operands must be numbers."),
                    }
                TokenType::GREATEREQUAL =>
                    match (left, right) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) =>
                            LiteralValue::Bool(a >= b),
                        _ => panic!("Operands must be numbers."),
                    }
                TokenType::LESS =>
                    match (left, right) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) =>
                            LiteralValue::Bool(a < b),
                        _ => panic!("Operands must be numbers."),
                    }
                TokenType::LESSEQUAL =>
                    match (left, right) {
                        (LiteralValue::Number(a), LiteralValue::Number(b)) =>
                            LiteralValue::Bool(a <= b),
                        _ => panic!("Operands must be numbers."),
                    }
                TokenType::EQUALEQUAL =>
                    LiteralValue::Bool(is_equal(&left, &right)),
                TokenType::BANGEQUAL =>
                    LiteralValue::Bool(!is_equal(&left, &right)),
                _ => LiteralValue::Nil, // should not reach here
            }
        }
    }
}

fn is_truthy(value: &LiteralValue) -> bool {
    match value {
        LiteralValue::Nil => false,
        LiteralValue::Bool(false) => false,
        _ => true,
    }
}

fn is_equal(a: &LiteralValue, b: &LiteralValue) -> bool {
    a == b
}