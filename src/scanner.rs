use std::collections::HashMap;

use crate::token::Token;
use crate::token_type::TokenType;
use crate::literal::Literal;
use crate::Lox;

pub struct Scanner<'a> {
    lox: &'a mut Lox,
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, lox: &'a mut Lox) -> Self {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);

        Self {
            lox,
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            //starting next lexme
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::from(TokenType::Eof, String::new(), Literal::Empty, self.line.clone()));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token_helper(TokenType::LeftParen),
            ')' => self.add_token_helper(TokenType::RightParen),
            '{' => self.add_token_helper(TokenType::LeftBrace),
            '}' => self.add_token_helper(TokenType::RightBrace),
            ',' => self.add_token_helper(TokenType::Comma),
            '.' => self.add_token_helper(TokenType::Dot),
            '-' => self.add_token_helper(TokenType::Minus),
            '+' => self.add_token_helper(TokenType::Plus),
            ';' => self.add_token_helper(TokenType::Semicolon),
            '*' => self.add_token_helper(TokenType::Star),
            '!' => {
                match self.check_next_char('=') {
                    true => self.add_token_helper(TokenType::BangEqual),
                    false => self.add_token_helper(TokenType::Bang),
                }
            },
            '=' => {
                match self.check_next_char('=') {
                    true => self.add_token_helper(TokenType::EqualEqual),
                    false => self.add_token_helper(TokenType::Equal),
                }
            },
            '<' => {
                match self.check_next_char('=') {
                    true => self.add_token_helper(TokenType::LessEqual),
                    false => self.add_token_helper(TokenType::Less),
                }
            },
            '>' => {
                match self.check_next_char('=') {
                    true => self.add_token_helper(TokenType::GreaterEqual),
                    false => self.add_token_helper(TokenType::Greater),
                }
            },
            '/' =>  {
                match self.check_next_char('/') {
                    true => {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }
                    false => self.add_token_helper(TokenType::Slash),
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.lox.error(self.line, String::from("Unexpected character"));
                }
            },
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) { self.advance(); };
        
        let text: String = String::from(&self.source[self.start..self.current]);

        match self.keywords.get(&text) {
            Some(token_type) => self.add_token_helper(token_type.clone()),
            None => self.add_token_helper(TokenType::Identifier),
        }
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) { self.advance(); }

        // Look for a fractional part.
        match self.peek() {
            '.' if self.is_digit(self.peek_next()) => {
                self.advance();

                while self.is_digit(self.peek()) { self.advance(); }
            }
            _ => ()
        }
        
        match self.source[self.start..self.current].parse::<f64>() {
            Ok(n) => self.add_token(TokenType::Number, Literal::Number(n)),
            Err(_) => self.lox.error(self.line, String::from("DEBUG: Error parsing a string to a f32 in scanner.number function")),
        }
        
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            match self.peek() {
                '\n' => self.line += 1,
                _ => {
                    self.advance();
                },
            }
        }

        if self.is_at_end() {
            self.lox.error(self.line, String::from("Unterminated string."));
            return;
        }

        // handling the closing '"'.
        self.advance();

        // trimming the surrounding quotes.
        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Literal::String(value));
    }

    fn check_next_char(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.as_bytes()[self.current] as char != expected { return false; }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        match self.is_at_end() {
            true => '\0',
            false => self.source.as_bytes()[self.current] as char,
        }
    }

    fn peek_next(&self) -> char {
        match self.current + 1 >= self.source.len() {
            true => '\0',
            false => self.source.as_bytes()[self.current + 1] as char,
        } 
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let tmp: usize = self.current;
        self.current += 1;
        self.source.as_bytes()[tmp] as char
    }

    fn add_token_helper(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::Empty);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text: &str = &self.source[self.start..self.current];
        self.tokens.push(Token::from(token_type, String::from(text), literal, self.line));
    }
}
