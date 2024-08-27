use crate::token::Token;
use crate::token_type::TokenType;
use crate::literal::Literal;
use crate::expr::Expr;
use crate::error::Error;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0usize,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.trial( vec![TokenType::BangEqual, TokenType::EqualEqual] ) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.trial( vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual] ) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.trial( vec![TokenType::Plus, TokenType::Minus] ) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.trial( vec![TokenType::Slash, TokenType::Star] ) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        while self.trial( vec![TokenType::Bang, TokenType::Minus] ) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            return Expr::Unary { operator, right: Box::new(right) }
        }

        self.primary().expect("Unknown Token while parsing")
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.trial( vec![TokenType::False] ) { return Ok(Expr::Literal { literal: Literal::Boolean(false) }); }
        if self.trial( vec![TokenType::True] ) { return Ok(Expr::Literal { literal: Literal::Boolean(true) }); }
        if self.trial( vec![TokenType::Nil] ) { return Ok(Expr::Literal { literal: Literal::Empty }); }
        if self.trial( vec![TokenType::Number, TokenType::String] ) { 
            return Ok(Expr::Literal { literal: self.previous().literal });
        }
        if self.trial( vec![TokenType::LeftParen] ) { 
            let expr: Expr = self.expression();
            self.consume(TokenType::RightParen, String::from("Expect ')' after expression."));
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }

        Err(Error::Parser)

    }

    fn consume(&self, token_type: TokenType, exception: String) {
        todo!();
    } 

    fn trial(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() { return false; }
        self.peek().token_type == token_type 
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1 }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
