use crate::lexer::Lexer;
use crate::token::Token;

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
        let mut parser = Parser {
            lexer,
            current_token,
            peek_token,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.next_token().ok();
    }
}
