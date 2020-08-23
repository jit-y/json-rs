use crate::lexer::Lexer;
use crate::token::Token;

use anyhow::Result;

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
