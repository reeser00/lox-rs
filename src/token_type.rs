#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual, 
    Equal, EqualEqual, 
    Greater, GreaterEqual, 
    Less, LessEqual,

    // Literals
    Identifier, STRING, Number,

    // KEYWORDS
    And, Class, Else, False, Fun, For, If, Nil, Or, 
    Print, Return, Super, This, True, Var, While,

    EOF
}
