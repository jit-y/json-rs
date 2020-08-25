use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::value::Value;

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,

    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let lexer = lexer;
        let current_token = None;
        let peek_token = None;
        let parser = Parser {
            lexer,
            current_token,
            peek_token,
        };

        parser
    }

    pub fn parse(&mut self) -> Result<Value> {
        let value = match self.current_token.as_ref() {
            None => return Err(anyhow!("parse error")),
            Some(tok) => match &tok.token_type {
                TokenType::Null => self.parse_null()?,
                TokenType::String => self.parse_string()?,
                TokenType::Integer => self.parse_number()?,
                TokenType::LBracket => self.parse_array()?,
                TokenType::LBrace => self.parse_object()?,
                TokenType::True => self.parse_boolean()?,
                TokenType::False => self.parse_boolean()?,
                _ => return Err(anyhow!("parse error")),
            },
        };

        Ok(value)
    }

    fn parse_null(&self) -> Result<Value> {
        Ok(Value::Null)
    }

    fn parse_number(&self) -> Result<Value> {
        unimplemented!()
    }

    fn parse_string(&self) -> Result<Value> {
        unimplemented!()
    }

    fn parse_boolean(&self) -> Result<Value> {
        unimplemented!()
    }

    fn parse_array(&mut self) -> Result<Value> {
        unimplemented!()
    }

    fn parse_object(&mut self) -> Result<Value> {
        unimplemented!()
    }

    pub fn build(lexer: Lexer) -> Result<Self> {
        let mut parser = Self::new(lexer);

        parser.next_token()?;
        parser.next_token()?;

        Ok(parser)
    }

    fn next_token(&mut self) -> Result<()> {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.next_token().map(|t| Some(t))?;

        Ok(())
    }
}
