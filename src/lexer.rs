use crate::token::{build_token, lookup_keyword, Token, TokenType};
use std::fmt::Debug;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: Vec<u8>,
    current_position: usize,
    next_position: usize,
    current_char: Option<u8>,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Self {
        let input = input.to_owned();
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

        if self.is_eof() {
            return Ok(build_token(TokenType::EOF, ""));
        }

        let ch = self.current_char.ok_or(anyhow!("error"))?;

        if is_digit(ch) {
            let literal = self.read_integer_literal();

            return Ok(build_token(TokenType::Integer, literal));
        }

        if is_lowercase(ch) {
            let literal = self.read_keyword_literal();

            let token_type =
                lookup_keyword(literal.as_str()).ok_or(anyhow!("unknown keyword {}", literal))?;

            return Ok(build_token(token_type, literal));
        }

        let token = match ch {
            ch @ b'{' => build_token(TokenType::LBrace, ch as char),
            ch @ b'}' => build_token(TokenType::RBrace, ch as char),
            ch @ b'[' => build_token(TokenType::LBracket, ch as char),
            ch @ b']' => build_token(TokenType::RBracket, ch as char),
            ch @ b':' => build_token(TokenType::Colon, ch as char),
            ch @ b',' => build_token(TokenType::Comma, ch as char),
            ch @ b'.' => build_token(TokenType::Period, ch as char),
            ch @ b'+' => build_token(TokenType::Plus, ch as char),
            ch @ b'-' => build_token(TokenType::Minus, ch as char),
            b'"' => {
                let literal = self.take_string_literal()?;
                build_token(TokenType::String, literal)
            }
            _ => return Err(anyhow!("error {}", ch)),
        };

        self.read_char();

        Ok(token)
    }

    pub fn is_eof(&self) -> bool {
        self.current_char.is_none()
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

    fn peek_char(&self) -> Option<u8> {
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

        unsafe {
            String::from_utf8_unchecked((&self.input[current..self.current_position]).to_vec())
        }
    }

    fn read_keyword_literal(&mut self) -> String {
        let current = self.current_position;

        while let Some(ch) = self.current_char {
            if is_lowercase(ch) {
                self.read_char();
            } else {
                break;
            }
        }

        unsafe {
            String::from_utf8_unchecked((&self.input[current..self.current_position]).to_vec())
        }
    }

    // TODO:escape
    fn take_string_literal(&mut self) -> Result<String> {
        let position = self.current_position + 1;

        loop {
            self.read_char();

            match self.current_char {
                Some(ch) => {
                    if ch == b'"' {
                        break;
                    }
                }
                // TODO:error
                None => break,
            }
        }

        String::from_utf8((&self.input[position..self.current_position]).to_vec())
            .map_err(|e| anyhow!("{}", e))
    }

    fn input_len(&self) -> usize {
        self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            match ch {
                b' ' | b'\r' | b'\n' | b'\t' => self.read_char(),
                _ => break,
            }
        }
    }
}

fn is_digit(b: u8) -> bool {
    b.is_ascii_digit()
}

fn is_lowercase(b: u8) -> bool {
    b.is_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::build_token;

    #[test]
    fn test_tokenize() {
        check_tokenize(
            r#"{"abcde": "fghij"}[12345, 67890, null, true, false]:.+-"#.into(),
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
                build_token(TokenType::Comma, ","),
                build_token(TokenType::Null, "null"),
                build_token(TokenType::Comma, ","),
                build_token(TokenType::True, "true"),
                build_token(TokenType::Comma, ","),
                build_token(TokenType::False, "false"),
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
        let mut lex = Lexer::new(input.as_bytes().to_owned());
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
