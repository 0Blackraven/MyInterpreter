use crate::token::{Token,TokenType,Literal};

// problem : line 76 , 176
// disclaimer: i will not use the generic way of rust here 
pub enum ExpressionType {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Literal(Literal),
    Grouping(Box<ExpressionType>),
    Variable(Token),
    Assignment(AssignExpression)
}

pub enum StatementType {
    ExpressionStatement(ExpressionType),
    PrintStatement(ExpressionType),
    LetStatement(LetExpressionType),
    BlockStatement(Vec<StatementType>),
}
pub struct AssignExpression {
    pub name: Token,
    pub value : Box<ExpressionType>
}

pub struct LetExpressionType {
    pub name : Token,
    pub initializer : Box<ExpressionType> 
}

pub struct BinaryExpression {
    pub left: Box<ExpressionType>,
    pub operator: TokenType,
    pub right: Box<ExpressionType>,
}

pub struct UnaryExpression {
    pub operator: TokenType,
    pub right: Box<ExpressionType>,
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

    fn advance (&mut self) -> Token {
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

    fn previous (&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn consume (&mut self, tokentype: TokenType, message: &str) -> Token {
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
    
    pub fn parse (&mut self) -> Vec<StatementType> {
        let mut statements:Vec<StatementType> = Vec::new();
        while !self.is_at_end() {
            let statement = self.declaration();
            statements.push(statement);
        }
        return statements;
    }

    fn declaration (&mut self) -> StatementType {
        if self.match_token(&[TokenType::LET]) {
            return self.var_declaration();
        }
        return self.statement();
    }

    fn var_declaration(&mut self) -> StatementType {
        let name = self.consume(TokenType::IDENTIFIER, "Expected a identifier here");
        let mut initializer:ExpressionType = ExpressionType::Literal(Literal::Nil);
        if self.match_token(&[TokenType::EQUAL]) {
            initializer = self.expression();
        }
        self.consume(TokenType::SEMICOLON, "Expected ; at the end");
        return StatementType::LetStatement(LetExpressionType { name: name, initializer: Box::new(initializer) })
    }

    fn statement (&mut self) -> StatementType {
        if self.match_token(&[TokenType::PRINT]) {
            return self.print_statement();
        } else if self.match_token(&[TokenType::LEFTBRACE]) {
            return self.block_statement();
        } else {
            return self.expression_statement();
        }
    }

    fn block_statement (&mut self) -> StatementType {
        let mut statements:Vec<StatementType> = Vec::new();

        while !self.check_token(&TokenType::RIGHTBRACE) && !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume(TokenType::RIGHTBRACE, "Expected a }");
        return StatementType::BlockStatement(statements);
    }

    fn print_statement (&mut self) -> StatementType {
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");
        return StatementType::PrintStatement(expr);
    }

    fn expression_statement (&mut self) -> StatementType {
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.");
        return StatementType::ExpressionStatement(expr);
    }

    fn expression (&mut self) -> ExpressionType {
        return self.assignment(); 
    }

    fn assignment(&mut self) -> ExpressionType {
        let expr = self.equality();

        if self.match_token(&[TokenType::EQUAL]) {
            let _equals = self.previous().tokentype.clone();
            let value = self.assignment();

            match expr {
                ExpressionType::Variable(name ) => {
                    let name = name;
                    return ExpressionType::Assignment(AssignExpression { name: name, value: Box::new(value) })
                }
                _ => panic!("Wrong assignment")
            }
        }
        return expr;
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
            expr = ExpressionType::Literal(Literal::Bool(false));
        }else if self.match_token(&[TokenType::TRUE]) {
            expr = ExpressionType::Literal(Literal::Bool(true));
        }else if self.match_token(&[TokenType::NIL]) {
            expr = ExpressionType::Literal(Literal::Nil);
        }else if self.match_token(&[TokenType::NUMBER]) {
            expr = ExpressionType::Literal(Literal::Number(self.previous().literal().parse().unwrap()));
        }else if self.match_token(&[TokenType::STRING]) {
            expr = ExpressionType::Literal(Literal::String(self.previous().literal().clone()));
        }else if self.match_token(&[TokenType::LEFTPAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression.");
            return ExpressionType::Grouping(Box::new(expr));
        } else if self.match_token(&[TokenType::IDENTIFIER]) {
            return ExpressionType::Variable(self.previous())
        }
        else {
            // handle this gracefully later;
            self.error(self.peek(), "Expect expression.");
        }
        return expr;
    }
}

#[allow(dead_code)]
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
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.clone(),
            Literal::Bool(b) => b.to_string(),
            Literal::Nil => "nil".to_string(),
        },
        ExpressionType::Variable(token) => format! (
            "{} {} {} {} ",
            token.lexeme, token.line, token.literal(), token.tokentype
        ),
        ExpressionType::Assignment(v) => format!(
            "{} {}",
            v.name.lexeme,
            print_expr(&v.value)
        )
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

