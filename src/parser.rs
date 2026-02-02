use crate::token::{Token,TokenType};

// problem : line 76 , 176
// disclaimer: i will not use the generic way of rust here 
pub enum ExpressionType {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Literal(LiteralValue),
    Grouping(Box<ExpressionType>),
}
pub enum LiteralValue {
    String(String),
    Number(f32),
    Bool(bool),
    Nil,
}

pub struct BinaryExpression {
    left: Box<ExpressionType>,
    operator: TokenType,
    right: Box<ExpressionType>,
}

pub struct UnaryExpression {
    operator: TokenType,
    right: Box<ExpressionType>,
}
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new (tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse (&mut self) -> ExpressionType {
        return self.expression();
    }
    // helper functions 
    fn match_token (&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check_token(t) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check_token(&self, tokentype: &TokenType) -> bool {
        !self.is_at_end() && &self.peek().tokentype == tokentype
    }

    fn advance (&mut self) -> &Token {
            if !self.is_at_end() {
                self.current += 1;
            }
            return self.previous();
    }

    fn is_at_end (&self) -> bool {
        return self.peek().tokentype == TokenType::EOF;
    }

    fn peek (&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn previous (&self) -> &Token {
        return &self.tokens[self.current - 1];
    }

    fn consume (&mut self, tokentype: TokenType, message: &str) -> &Token {
        if self.check_token(&tokentype) {
            return self.advance();
        }
        // this too handle gracefully later
        self.error(self.peek(), message);
    }

    fn error (&self, token: &Token, message: &str) -> ! {
        panic!("Error at line {}: {}", token.line, message);
    }

    // main parsing functions

    fn expression (&mut self) -> ExpressionType {
        self.equality() 
    }

    fn equality (&mut self) -> ExpressionType {
        let mut expr = self.comparison();
    
        while self.match_token(&[TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator = self.previous().tokentype.clone();
            let right: ExpressionType = self.comparison();
            expr = ExpressionType::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn comparison (&mut self) -> ExpressionType {
        let mut expr = self.term();
        let types = [
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ];
        while self.match_token(&types) {
            let operator = self.previous().tokentype.clone();
            let right: ExpressionType = self.term();
            expr = ExpressionType::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn term (&mut self) -> ExpressionType {
        let mut expr = self.factor();
    
        while self.match_token(&[TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous().tokentype.clone();
            let right: ExpressionType = self.factor();
            expr = ExpressionType::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn factor (&mut self) -> ExpressionType {
        let mut expr = self.unary();
    
        while self.match_token(&[TokenType::STAR, TokenType::SLASH]) {
            let operator = self.previous().tokentype.clone();
            let right: ExpressionType = self.unary();
            expr = ExpressionType::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn unary (&mut self) -> ExpressionType {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().tokentype.clone();
            let right: ExpressionType = self.unary();
            return ExpressionType::Unary(UnaryExpression {
                operator: operator,
                right: Box::new(right),
            });
        }
        return self.primary();
    }

    fn primary (&mut self) -> ExpressionType {
        let expr : ExpressionType;
        if self.match_token(&[TokenType::FALSE]) {
            expr = ExpressionType::Literal(LiteralValue::Bool(false));
        }else if self.match_token(&[TokenType::TRUE]) {
            expr = ExpressionType::Literal(LiteralValue::Bool(true));
        }else if self.match_token(&[TokenType::NIL]) {
            expr = ExpressionType::Literal(LiteralValue::Nil);
        }else if self.match_token(&[TokenType::NUMBER]) {
            expr = ExpressionType::Literal(LiteralValue::Number(self.previous().lexeme.parse().unwrap()));
        }else if self.match_token(&[TokenType::STRING]) {
            expr = ExpressionType::Literal(LiteralValue::String(self.previous().lexeme.clone()));
        }else if self.match_token(&[TokenType::LEFTPAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression.");
            return ExpressionType::Grouping(Box::new(expr));
        } else {
            // handle this gracefully later;
            self.error(self.peek(), "Expect expression.");
        }
        return expr;
    }
}

pub fn print_expr(expr: &ExpressionType) -> String {
    match expr {
        ExpressionType::Binary(b) => format!(
            "({} {} {})",
            b.operator,
            print_expr(&b.left),
            print_expr(&b.right),
        ),

        ExpressionType::Unary(u) => format!(
            "({} {})",
            u.operator,
            print_expr(&u.right),
        ),

        ExpressionType::Grouping(expr) => format!(
            "(group {})",
            print_expr(expr),
        ),

        ExpressionType::Literal(lit) => match lit {
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Bool(b) => b.to_string(),
            LiteralValue::Nil => "nil".to_string(),
        },
    }
}

//   private void synchronize() {
//     advance();

//     while (!isAtEnd()) {
//       if (previous().type == SEMICOLON) return;

//       switch (peek().type) {
//         case CLASS:
//         case FUN:
//         case VAR:
//         case FOR:
//         case IF:
//         case WHILE:
//         case PRINT:
//         case RETURN:
//           return;
//       }

//       advance();
//     }
//   }

