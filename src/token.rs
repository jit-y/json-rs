use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
}
