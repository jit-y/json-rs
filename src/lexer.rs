use crate::token::{Token, TokenType};
use std::fmt::Debug;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: Vec<char>,
    current_position: usize,
    next_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.chars().collect();
        let current_position = 0;
        let next_position = 0;
        let ch = '\u{0000}';

        let mut lex = Self {
            input,
            current_position,
            next_position,
            ch,
        };

        lex.read_char();

        lex
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            ch @ '{' => self.build_token(TokenType::LBrace, to_literal(ch)),
            ch @ '}' => self.build_token(TokenType::RBrace, to_literal(ch)),
            ch @ '[' => self.build_token(TokenType::LBracket, to_literal(ch)),
            ch @ ']' => self.build_token(TokenType::RBracket, to_literal(ch)),
            ch @ ':' => self.build_token(TokenType::Colon, to_literal(ch)),
            ch @ '+' => self.build_token(TokenType::Plus, to_literal(ch)),
            ch @ '-' => self.build_token(TokenType::Minus, to_literal(ch)),
            '\u{0000}' => self.build_token(TokenType::EOF, to_literal("")),
            _ => return Err(anyhow!("error")),
        };

        self.read_char();

        Ok(token)
    }

    fn read_char(&mut self) {
        self.ch = {
            if self.next_position >= self.input_len() {
                '\u{0000}'
            } else {
                self.input[self.next_position]
            }
        };

        self.current_position = self.next_position;
        self.next_position += 1;
    }

    fn build_token(&self, token_type: TokenType, literal: String) -> Token {
        Token::new(token_type, literal)
    }

    fn input_len(&self) -> usize {
        self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() {
            match self.ch {
                ' ' | '\r' | '\n' | '\t' => self.read_char(),
                _ => break,
            }
        }
    }

    fn is_eof(&self) -> bool {
        self.ch == '\u{0000}'
    }
}

fn to_literal<T>(s: T) -> String
where
    T: ToString,
{
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        check_tokenize(
            "{}[]:+-".into(),
            vec![
                Token::new(TokenType::LBrace, "{".to_string()),
                Token::new(TokenType::RBrace, "}".to_string()),
                Token::new(TokenType::LBracket, "[".to_string()),
                Token::new(TokenType::RBracket, "]".to_string()),
                Token::new(TokenType::Colon, ":".to_string()),
                Token::new(TokenType::Plus, "+".to_string()),
                Token::new(TokenType::Minus, "-".to_string()),
                Token::new(TokenType::EOF, "".to_string()),
            ],
        );
    }

    fn check_tokenize(input: String, expected: Vec<Token>) {
        let mut lex = Lexer::new(input);
        let mut tokens = vec![];
        while !lex.is_eof() {
            let tok = lex.next_token();
            assert!(tok.is_ok());

            tokens.push(tok.unwrap());
        }
        tokens.push(lex.next_token().unwrap());

        assert_eq!(tokens.len(), expected.len());

        for i in 0..expected.len() {
            assert_eq!(tokens[i], expected[i]);
        }
    }
}
