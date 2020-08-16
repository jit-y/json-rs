use crate::token::{build_token, Token, TokenType};
use std::fmt::Debug;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: Vec<char>,
    current_position: usize,
    next_position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.chars().collect();
        let current_position = 0;
        let next_position = 0;
        let current_char = None;

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

        let ch = match self.current_char {
            None => return Ok(build_token(TokenType::EOF, "")),
            Some(c) => c,
        };

        if is_digit(ch) {
            let literal = self.take_integer_literal();

            return Ok(build_token(TokenType::Integer, literal));
        }

        let token = match ch {
            ch @ '{' => build_token(TokenType::LBrace, ch),
            ch @ '}' => build_token(TokenType::RBrace, ch),
            ch @ '[' => build_token(TokenType::LBracket, ch),
            ch @ ']' => build_token(TokenType::RBracket, ch),
            ch @ ':' => build_token(TokenType::Colon, ch),
            ch @ ',' => build_token(TokenType::Comma, ch),
            ch @ '.' => build_token(TokenType::Period, ch),
            ch @ '+' => build_token(TokenType::Plus, ch),
            ch @ '-' => build_token(TokenType::Minus, ch),
            _ => return Err(anyhow!("error")),
        };

        self.read_char();

        Ok(token)
    }

    fn read_char(&mut self) {
        self.current_char = {
            if self.next_position >= self.input_len() {
                None
            } else {
                Some(self.input[self.next_position])
            }
        };

        self.current_position = self.next_position;
        self.next_position += 1;
    }

    fn take_integer_literal(&mut self) -> String {
        let current = self.current_position;

        while let Some(ch) = self.current_char {
            if is_digit(ch) {
                self.read_char();
            } else {
                break;
            }
        }

        (&self.input[current..self.current_position])
            .to_owned()
            .into_iter()
            .collect()
    }

    fn input_len(&self) -> usize {
        self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            match ch {
                ' ' | '\r' | '\n' | '\t' => self.read_char(),
                _ => break,
            }
        }
    }
}

fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        check_tokenize(
            "{}[12345, 67890]:.+-".into(),
            vec![
                Token::new(TokenType::LBrace, "{".to_string()),
                Token::new(TokenType::RBrace, "}".to_string()),
                Token::new(TokenType::LBracket, "[".to_string()),
                Token::new(TokenType::Integer, "12345".to_string()),
                Token::new(TokenType::Comma, ",".to_string()),
                Token::new(TokenType::Integer, "67890".to_string()),
                Token::new(TokenType::RBracket, "]".to_string()),
                Token::new(TokenType::Colon, ":".to_string()),
                Token::new(TokenType::Period, ".".to_string()),
                Token::new(TokenType::Plus, "+".to_string()),
                Token::new(TokenType::Minus, "-".to_string()),
                Token::new(TokenType::EOF, "".to_string()),
            ],
        );
    }

    fn check_tokenize(input: String, expected: Vec<Token>) {
        let mut lex = Lexer::new(input);
        let mut tokens = vec![];
        while let Some(_) = lex.current_char {
            let tok = lex.next_token();
            assert!(tok.is_ok());

            tokens.push(tok.unwrap());
        }
        tokens.push(lex.next_token().unwrap());

        dbg!(&tokens);

        assert_eq!(tokens.len(), expected.len());

        for i in 0..expected.len() {
            assert_eq!(tokens[i], expected[i]);
        }
    }
}
