use crate::token::{Token, TokenType, AtomicLiteral};
use std::rc::Rc;
use crate::lox_error::{LoxError, LoxResult};
use crate::statement::*;
use crate::expression::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // helper functions
    fn match_token(&mut self, types: &[TokenType]) -> bool {
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

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().tokentype == TokenType::EOF;
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn consume(&mut self, tokentype: TokenType, message: &str) -> LoxResult<Token> {
        if self.check_token(&tokentype) {
            return Ok(self.advance());
        }else {
            return Err(self.error(self.peek(), message));
        }     
    }

    fn error(&self, token: &Token, message: &str) -> LoxError {
        return LoxError::ParseError { 
                token: token.clone(),
                message: message.to_string(),
            }

    }

    fn synchronize(&mut self) {
        self.advance();
    
        while !self.is_at_end() {
            if self.previous().tokentype == TokenType::SEMICOLON {
                return;
            }
    
            match self.peek().tokentype {
                TokenType::CLASS | TokenType::FUNCTION | TokenType::LET | 
                TokenType::FOR | TokenType::IF | TokenType::WHILE | 
                TokenType::PRINT | TokenType::RETURN => return,
                _ => {}
            }
    
            self.advance();
        }
    }

    // main parsing functions

    pub fn parse(&mut self) -> LoxResult<Vec<StatementType>> {
        let mut statements: Vec<StatementType> = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(e) => {
                    eprintln!("{}", e);
                    self.synchronize();
                }
            }
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> LoxResult<StatementType> {
        let result = if self.match_token(&[TokenType::LET]) {
            self.var_declaration()
        } else if self.match_token(&[TokenType::FUNCTION]) {
            self.function_declaration(FunctionType::Function)
        }else if self.match_token(&[TokenType::CLASS]){
            self.class_declaration()
        } else {
            self.statement()
        };
        result
    }

    fn class_declaration(&mut self) -> LoxResult<StatementType> {
        let name = self.consume(TokenType::IDENTIFIER, "Expected class name")?;
        let mut super_class = None;
        if self.match_token(&[TokenType::COLON]) {
            self.consume(TokenType::IDENTIFIER, "Expected a superclass name")?;
            super_class = Some(ExpressionType::Variable(self.previous()));
        }
        self.consume(TokenType::LEFTBRACE, "Expected '{' before class body")?;
        let mut methods: Vec<StatementType> = Vec::new();
        while !self.check_token(&TokenType::RIGHTBRACE) && !self.is_at_end() {
            methods.push(self.function_declaration(FunctionType::Method)?);
        }
        self.consume(TokenType::RIGHTBRACE, "Expected '}' after class body")?;
        Ok(StatementType::ClassStatement(ClassProps{
            name,
            methods,
            superclass:super_class
        }))
    }

    fn function_declaration(&mut self, _func_type: FunctionType) -> LoxResult<StatementType> {
        let name = self.consume(TokenType::IDENTIFIER, "exprected identifier")?;
        self.consume(TokenType::LEFTPAREN, "expected a (")?;
        let mut tokens : Vec<Token> = Vec::new();
        if !self.check_token(&TokenType::RIGHTPAREN) {
            tokens.push(self.consume(TokenType::IDENTIFIER, "expected a identifier for arguments")?);
            while self.match_token(&[TokenType::COMMA]) {
                tokens.push(self.consume(TokenType::IDENTIFIER, "expected a identifier for arguments")?);
            }
        }
        self.consume(TokenType::RIGHTPAREN, "expected a ) at the end of arguments")?;
        self.consume(TokenType::LEFTBRACE, "expected { at the start of body")?;
        let body = self.block_statement()?;

        Ok(StatementType::Function(FunctionProps { 
            name, 
            params: tokens, 
            body: Rc::new(body) 
        }))
    }

    fn var_declaration(&mut self) -> LoxResult<StatementType> {
        let name = self.consume(TokenType::IDENTIFIER, "Expected a identifier here")?;
        let mut initializer: ExpressionType = ExpressionType::Literal(AtomicLiteral::Nil);
        if self.match_token(&[TokenType::EQUAL]) {
            initializer = self.expression()?;
        }
        self.consume(TokenType::SEMICOLON, "Expected ; at the end")?;
        Ok(StatementType::LetStatement(LetExpressionProps {
            name: name,
            initializer: Box::new(initializer),
        }))
    }

    fn return_statement(&mut self) -> LoxResult<StatementType> {
        let keyword = self.previous();
        let mut value = None;
        if !self.check_token(&TokenType::SEMICOLON) {
            value = Some(self.expression()?);
        }
        self.consume(TokenType::SEMICOLON, "Expected ; after return value")?;
        Ok(StatementType::ReturnStatement(ReturnProps {
            _keyword: keyword,
            value
        }))
    }

    fn statement(&mut self) -> LoxResult<StatementType> {
        if self.match_token(&[TokenType::PRINT]) {
            return self.print_statement();
        } else if self.match_token(&[TokenType::LEFTBRACE]) {
            return self.block_statement();
        } else if self.match_token(&[TokenType::IF]) {
            return self.if_statement();
        } else if self.match_token(&[TokenType::WHILE]) {
            return self.while_statement();
        } else if self.match_token(&[TokenType::FOR]) {
            return self.for_statement();
        }else if self.match_token(&[TokenType::RETURN]){
            return self.return_statement();
        } else {
            return self.expression_statement();
        }
    }

    fn for_statement(&mut self) -> LoxResult<StatementType> {
        self.consume(TokenType::LEFTPAREN, "expected a (")?;
        let mut _initializer: Option<StatementType> = None;
        if self.match_token(&[TokenType::SEMICOLON]) {
            _initializer = None;
        } else if self.check_token(&TokenType::LET) {
            _initializer = Some(self.declaration()?);
        } else {
            _initializer = Some(self.expression_statement()?);
        }
    
        let mut condition: Option<ExpressionType> = None;
        if !self.check_token(&TokenType::SEMICOLON) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::SEMICOLON, "expected ; after expression")?;
    
        let mut increment: Option<ExpressionType> = None;
        if !self.check_token(&TokenType::RIGHTPAREN) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RIGHTPAREN, "expected ) after end of For")?;
    
        let mut body: StatementType = self.statement()?;
    
        if let Some(increment_result) = increment {
            let args: Vec<StatementType> =
                vec![body, StatementType::ExpressionStatement(increment_result)];
            body = StatementType::BlockStatement(args);
        }
    
        if condition.is_none() {
            condition = Some(ExpressionType::Literal(AtomicLiteral::Bool(true)));
        }
        body = StatementType::WhileStatement(WhileProps {
            condition: condition.unwrap(),
            statement: Box::new(body),
        });
    
        if let Some(initializer_result) = _initializer {
            let args: Vec<StatementType> = vec![initializer_result, body];
            body = StatementType::BlockStatement(args);
        }
    
        Ok(body)
    }
    
    fn while_statement(&mut self) -> LoxResult<StatementType> {
        self.consume(TokenType::LEFTPAREN, "expected (")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHTPAREN, "expected )")?;
        let statement = self.statement()?;
        Ok(StatementType::WhileStatement(WhileProps {
            statement: Box::new(statement),
            condition: condition,
        }))
    }
    
    fn if_statement(&mut self) -> LoxResult<StatementType> {
        self.consume(TokenType::LEFTPAREN, "expected ( after if")?;
        let comparison = self.expression()?;
        self.consume(TokenType::RIGHTPAREN, "expected a ) at end of condition")?;
    
        let ifcase = self.statement()?;
        let mut elsecase = None;
        if self.match_token(&[TokenType::ELSE]) {
            let temp = self.statement()?;
            elsecase = Some(Box::new(temp));
        }
        Ok(StatementType::IfStatement(IfProps {
            comparison,
            ifcase: Box::new(ifcase),
            elsecase: elsecase,
        }))
    }
    
    fn block_statement(&mut self) -> LoxResult<StatementType> {
        let mut statements: Vec<StatementType> = Vec::new();
    
        while !self.check_token(&TokenType::RIGHTBRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RIGHTBRACE, "Expected a }")?;
        Ok(StatementType::BlockStatement(statements))
    }
    
    fn print_statement(&mut self) -> LoxResult<StatementType> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(StatementType::PrintStatement(expr))
    }
    
    fn expression_statement(&mut self) -> LoxResult<StatementType> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;
        Ok(StatementType::ExpressionStatement(expr))
    }

    fn finish_call (&mut self, callee:ExpressionType) -> LoxResult<ExpressionType> {
        let mut arguments: Vec<Box<ExpressionType>> = Vec::new();
        if !self.check_token(&TokenType::RIGHTPAREN) {
            arguments.push(Box::new(self.expression()?));
            while self.match_token(&[TokenType::COMMA]) {
                if arguments.len() >= 255 {
                    return Err(LoxError::ParseError {
                        token: self.peek().clone(),
                        message: "Can't have more than 255 arguments".to_string(),
                    });
                } 
                arguments.push(Box::new(self.expression()?));
            }
        }
        let paren = self.consume(TokenType::RIGHTPAREN, "Expect ')' after arguments")?;
        Ok(ExpressionType::Call(CallArgs { 
            callee: Box::new(callee), 
            paren, 
            args: arguments
        }))
    }

    fn expression(&mut self) -> LoxResult<ExpressionType> {
        let expr = self.assignment();
        return expr;
    }

    fn assignment(&mut self) -> LoxResult<ExpressionType> {
        let expr = self.or()?;
    
        if self.match_token(&[TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;
    
            match expr {
                ExpressionType::Variable(name) => {
                    Ok(ExpressionType::Assignment(AssignExpression {
                        name: name,
                        value: Box::new(value),
                    }))
                }
                ExpressionType::Get(get ) => {
                    Ok(ExpressionType::Set(SetArgs {
                        name: get.name,
                        object: get.object,
                        value: Box::new(value),
                    }))
                }
                _ => Err(LoxError::ParseError {
                    token: equals,
                    message: "Invalid assignment target".to_string(),
                })
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> LoxResult<ExpressionType> {
        let mut expr = self.and()?;
    
        while self.match_token(&[TokenType::OR]) {
            let operator = self.previous().tokentype;
            let right = self.and()?;
            expr = ExpressionType::Logical(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn and(&mut self) -> LoxResult<ExpressionType> {
        let mut expr = self.equality()?;
    
        while self.match_token(&[TokenType::AND]) {
            let operator = self.previous().tokentype;
            let right = self.equality()?;
            expr = ExpressionType::Logical(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn equality(&mut self) -> LoxResult<ExpressionType> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BANGEQUAL, TokenType::EQUALEQUAL]) {
            let operator = self.previous().tokentype;
            let right: ExpressionType = self.comparison()?;
            expr = ExpressionType::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> LoxResult<ExpressionType> {
        let mut expr = self.term()?;
        let types = [
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ];
        while self.match_token(&types) {
            let operator = self.previous().tokentype;
            let right: ExpressionType = self.term()?;
            expr = ExpressionType::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        return Ok(expr);
    }

    fn term(&mut self) -> LoxResult<ExpressionType> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous().tokentype;
            let right: ExpressionType = self.factor()?;
            expr = ExpressionType::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        return Ok(expr);
    }

    fn factor(&mut self) -> LoxResult<ExpressionType> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::STAR, TokenType::SLASH, TokenType::MODULO]) {
            let operator = self.previous().tokentype;
            let right: ExpressionType = self.unary()?;
            expr = ExpressionType::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        return Ok(expr);
    }

    fn unary(&mut self) -> LoxResult<ExpressionType> {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().tokentype;
            let right = self.unary()?;
            Ok(ExpressionType::Unary(UnaryExpression {
                operator: operator,
                right: Box::new(right),
            }))
        } else {
            self.postfix()
        }
    }

    fn postfix(&mut self) -> LoxResult<ExpressionType> {
        let mut expr = self.primary()?;
    
        if self.check_token(&TokenType::LEFTPAREN) {
            loop {
                if self.match_token(&[TokenType::LEFTPAREN]) {
                    expr = self.finish_call(expr)?;
                } else if self.match_token(&[TokenType::DOT]) {
                    let name = self.consume(TokenType::IDENTIFIER, "Expected property name after '.'")?;
                    expr = ExpressionType::Get(GetArgs {
                        name,
                        object: Box::new(expr),
                    });
                } else {
                    break;
                }
            }
            return Ok(expr);
        }
        if self.match_token(&[TokenType::INCREMENTOR, TokenType::DECREMENTOR]) {
            match expr {
                ExpressionType::Variable(_) => {
                    let operator = self.previous().tokentype;
                    Ok(ExpressionType::Postfix(PostfixExpression {
                        expr: Box::new(expr),
                        operator,
                    }))
                }
                _ => Err(LoxError::ParseError {
                    token: self.previous(),
                    message: "Invalid target for postfix operator".to_string(),
                })
            }
        } else {
            Ok(expr)
        }
    }

    fn primary(&mut self) -> LoxResult<ExpressionType> {
        if self.match_token(&[TokenType::FALSE]) {
            Ok(ExpressionType::Literal(AtomicLiteral::Bool(false)))
        } else if self.match_token(&[TokenType::TRUE]) {
            Ok(ExpressionType::Literal(AtomicLiteral::Bool(true)))
        } else if self.match_token(&[TokenType::NIL]) {
            Ok(ExpressionType::Literal(AtomicLiteral::Nil))
        } else if self.match_token(&[TokenType::NUMBER]) {
            Ok(ExpressionType::Literal(AtomicLiteral::Number(
                self.previous().literal().parse().unwrap(),
            )))
        } else if self.match_token(&[TokenType::STRING]) {
            Ok(ExpressionType::Literal(AtomicLiteral::String(self.previous().literal().clone())))
        } else if self.match_token(&[TokenType::LEFTPAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression.")?;
            Ok(ExpressionType::Grouping(Box::new(expr)))
        } else if self.match_token(&[TokenType::IDENTIFIER]) {
            Ok(ExpressionType::Variable(self.previous()))
        } else if self.match_token(&[TokenType::THIS]) {
            Ok(ExpressionType::This(self.previous()))
        } else if self.match_token(&[TokenType::SUPER]) {
            let keyword = self.previous();
            self.consume(TokenType::DOT, "Expected a '.' after super")?;
            let method = self.consume(TokenType::IDENTIFIER, "Method name missing")?;
            Ok(ExpressionType::Super(SuperArgs{keyword,method}))
        } else {
            Err(self.error(self.peek(), "Expect expression."))
        }
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

        ExpressionType::Unary(u) => format!("({} {})", u.operator, print_expr(&u.right),),

        ExpressionType::Grouping(expr) => format!("(group {})", print_expr(expr),),

        ExpressionType::Literal(lit) => match lit {
                AtomicLiteral::Number(n) => n.to_string(),
                AtomicLiteral::String(s) => s.clone(),
                AtomicLiteral::Bool(b) => b.to_string(),
                AtomicLiteral::Nil => "nil".to_string(),
        },
        ExpressionType::Variable(token) => format!(
            "{} {} {} {} ",
            token.lexeme,
            token.line,
            token.literal(),
            token.tokentype
        ),
        ExpressionType::Assignment(v) => format!("{} {}", v.name.lexeme, print_expr(&v.value)),
        ExpressionType::Logical(v) => format!(
            "{} {} {}",
            print_expr(&v.left),
            v.operator,
            print_expr(&v.right)
        ),
        ExpressionType::Postfix(post) => format!("{} {}", print_expr(&post.expr), post.operator),
        ExpressionType::Call(called) => format!(
            "{} {}",
            print_expr(&called.callee),
            called.paren.lexeme,
        ),
        _ => format!("really tired of writing these print functions"),

    }
}
