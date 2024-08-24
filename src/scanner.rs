use std::collections::HashMap;

use crate::token::{Token, Literal};
use crate::token_type::TokenType;
use crate::Lox;

pub struct Scanner<'a> {
    lox: &'a mut Lox,
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, lox: &'a mut Lox) -> Self {
        Self {
            lox,
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            //starting next lexme
            self.start = self.current;
            self.scan_tokens();
        }

        self.tokens.push(Token::from(TokenType::EOF, String::new(), Literal::new(), self.line.clone()));
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
            _ => self.lox.error(self.line, String::from("Unexpected character.")),
        }
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
        self.add_token(token_type, Literal::new());
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text: &str = &self.source[self.start..self.current];
        self.tokens.push(Token::from(token_type, String::from(text), literal, self.line));
    }
}
