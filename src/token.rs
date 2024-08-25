use crate::token_type::TokenType;
use crate::literal::Literal;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    pub fn from(token_type: TokenType, lexme: String, literal: Literal, line: usize) -> Self {
        Self {
            token_type,
            lexme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}, {}, {:?}", self.token_type, self.lexme, self.literal)
    }
}
