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
            ch @ ':' => self.build_token(TokenType::Colon, ch.to_string()),
            ch @ '+' => self.build_token(TokenType::Plus, ch.to_string()),
            ch @ '-' => self.build_token(TokenType::Minus, ch.to_string()),
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
            ],
        );
    }

    fn check_tokenize(input: String, expected: Vec<Token>) {
        let mut lex = Lexer::new(input);
        let mut tokens = vec![];
        while let Some(tok) = lex.next_token() {
            tokens.push(tok);
        }

        assert_eq!(tokens.len(), expected.len());

        for i in 0..expected.len() {
            assert_eq!(tokens[i], expected[i]);
        }
    }
}
