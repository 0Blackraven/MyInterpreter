#[derive(Debug,Clone)]
pub enum TokenType {
    // single char tokens
    LEFTPAREN, RIGHTPAREN, LEFTBRACE, RIGHTBRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens. 
    // BANG IS !
    BANG, BANGEQUAL,
    EQUAL, EQUALEQUAL,
    GREATER, GREATEREQUAL,
    LESS, LESSEQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUNCTION, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    // EOF
}
#[derive(Debug,Clone)]
#[allow(dead_code)]
pub enum Literal {
    String(String),
    Number(f32),
    Bool(bool),
    Nil,
}
pub struct Token {
    tokentype: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}
impl Token {
    #[allow(dead_code)]
    pub fn new(tokentype: TokenType, lexeme: String, line: u32, literal:Literal) -> Self {
        match literal {
            Literal::String(_) | Literal::Number(_) | Literal::Bool(_) => Token {
                tokentype,
                lexeme,
                literal: Some(literal),
                line,
            },
            Literal::Nil => Token {
                tokentype,
                lexeme,
                literal: None,
                line,
            },
        }
    }

    //just testing purpose
    #[allow(dead_code)]
    pub fn print_token(&self) {
        println!("{:?} {} {} {:?}", self.tokentype, self.lexeme, self.line, self.literal);
    }
}
 