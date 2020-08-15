use crate::token::{build_token, Token, TokenType};
use std::fmt::Debug;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: Vec<char>,
    current_position: usize,
    next_position: usize,
    current_char: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.chars().collect();
        let current_position = 0;
        let next_position = 0;
        let current_char = '\u{0000}';

        let mut lex = Self {
            input,
            current_position,
            next_position,
            current_char,
        };

        lex.read_char();

        lex
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let token = match self.current_char {
            ch @ '{' => build_token(TokenType::LBrace, ch),
            ch @ '}' => build_token(TokenType::RBrace, ch),
            ch @ '[' => build_token(TokenType::LBracket, ch),
            ch @ ']' => build_token(TokenType::RBracket, ch),
            ch @ ':' => build_token(TokenType::Colon, ch),
            ch @ '+' => build_token(TokenType::Plus, ch),
            ch @ '-' => build_token(TokenType::Minus, ch),
            '\u{0000}' => build_token(TokenType::EOF, ""),
            _ => return Err(anyhow!("error")),
        };

        self.read_char();

        Ok(token)
    }

    fn read_char(&mut self) {
        self.current_char = {
            if self.next_position >= self.input_len() {
                '\u{0000}'
            } else {
                self.input[self.next_position]
            }
        };

        self.current_position = self.next_position;
        self.next_position += 1;
    }

    fn input_len(&self) -> usize {
        self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() {
            match self.current_char {
                ' ' | '\r' | '\n' | '\t' => self.read_char(),
                _ => break,
            }
        }
    }

    fn is_eof(&self) -> bool {
        self.current_char == '\u{0000}'
    }
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
