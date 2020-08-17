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
            let literal = self.read_integer_literal();

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
            '"' => {
                let literal = self.take_string_literal()?;
                build_token(TokenType::String, literal)
            }
            ch @ _ => return Err(anyhow!("error {}", ch)),
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

    fn peek_char(&self) -> Option<char> {
        if self.next_position >= self.input_len() {
            None
        } else {
            Some(self.input[self.next_position])
        }
    }

    fn read_integer_literal(&mut self) -> String {
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

    // TODO:escape
    fn take_string_literal(&mut self) -> Result<String> {
        let position = self.current_position + 1;

        loop {
            self.read_char();

            match self.current_char {
                Some(ch) => {
                    if ch == '"' {
                        break;
                    }
                }
                // TODO:error
                None => break,
            }
        }

        let literal = (&self.input[position..self.current_position])
            .into_iter()
            .collect();

        Ok(literal)
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
    use crate::token::build_token;

    #[test]
    fn test_tokenize() {
        check_tokenize(
            "{\"abcde\": \"fghij\"}[12345, 67890]:.+-".into(),
            vec![
                build_token(TokenType::LBrace, "{"),
                build_token(TokenType::String, "abcde"),
                build_token(TokenType::Colon, ":"),
                build_token(TokenType::String, "fghij"),
                build_token(TokenType::RBrace, "}"),
                build_token(TokenType::LBracket, "["),
                build_token(TokenType::Integer, "12345"),
                build_token(TokenType::Comma, ","),
                build_token(TokenType::Integer, "67890"),
                build_token(TokenType::RBracket, "]"),
                build_token(TokenType::Colon, ":"),
                build_token(TokenType::Period, "."),
                build_token(TokenType::Plus, "+"),
                build_token(TokenType::Minus, "-"),
                build_token(TokenType::EOF, ""),
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
