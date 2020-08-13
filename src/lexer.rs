use crate::token::{Token, TokenType};
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: Vec<char>,
    current_position: usize,
    next_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.chars().collect();
        let current_position = 0;
        let next_position = 0;
        let ch = None;

        let mut lex = Self {
            input,
            current_position,
            next_position,
            ch,
        };

        lex.read_char();

        lex
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token = match self.ch? {
            ch @ '{' => self.build_token(TokenType::LBrace, ch.to_string()),
            ch @ '}' => self.build_token(TokenType::RBrace, ch.to_string()),
            ch @ '[' => self.build_token(TokenType::LBracket, ch.to_string()),
            ch @ ']' => self.build_token(TokenType::RBracket, ch.to_string()),
            _ => return None,
        };

        self.read_char();

        Some(token)
    }

    fn read_char(&mut self) {
        self.ch = {
            if self.next_position >= self.input_len() {
                None
            } else {
                Some(self.input[self.next_position])
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
        while let Some(ch) = self.ch {
            match ch {
                ' ' | '\r' | '\n' | '\t' => self.read_char(),
                _ => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        check_lexer(
            "{}".into(),
            vec![
                Token::new(TokenType::LBrace, "{".to_string()),
                Token::new(TokenType::RBrace, "}".to_string()),
            ],
        );
        check_lexer(
            "[]".into(),
            vec![
                Token::new(TokenType::LBracket, "[".to_string()),
                Token::new(TokenType::RBracket, "]".to_string()),
            ],
        );
    }

    fn check_lexer(input: String, expected: Vec<Token>) {
        let mut lex = Lexer::new(input);

        for e in expected {
            assert_eq!(lex.next_token(), Some(e));
        }
    }
}
