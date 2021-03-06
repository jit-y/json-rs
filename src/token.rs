use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
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
    Colon,
    Comma,
    Period,

    Plus,
    Minus,

    Integer,
    String,

    Null,
    True,
    False,

    EOF,
}

pub fn build_token<T>(token_type: TokenType, literal: T) -> Token
where
    T: ToString,
{
    Token::new(token_type, literal.to_string())
}

pub fn lookup_keyword(literal: &str) -> Option<TokenType> {
    match literal {
        "null" => Some(TokenType::Null),
        "true" => Some(TokenType::True),
        "false" => Some(TokenType::False),
        _ => None,
    }
}
