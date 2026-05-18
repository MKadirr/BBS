use std::str::Chars;
use crate::context::Context;
use crate::lexer::lex_error::LexError;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::token::TokenType::NotStarted;

pub struct Parser {
    pub _current: Token,
    pub _next: Token,

    pub _data: String,

    pub _idx: usize,
    pub _row: usize,
    pub _col: usize,

    _is_init: bool,
}

impl Parser {
    pub fn new(string: String) -> Result<Parser, LexError> {

        let mut parser = Parser {
            _current: Token::new(TokenType::NotStarted, Context::new(0,0), String::new()),
            _next: Token::new(TokenType::NotStarted, Context::new(0,0), String::new()),

            _data: string,

            _idx: 0,
            _row: 0,
            _col: 0,
            _is_init: false,
        };

        let current = parser.parse_header()?;
        let next = parser.parse_header()?;

        parser._current = current;
        parser._next = next;

        Ok(parser)
    }

    pub fn get(&self) -> Token {
        self._current.clone()
    }

    pub fn pop(&mut self) -> Result<Token, LexError> {
        let tmp = self._current.clone();
        self._current = self._next.clone();

        let next = self.parse_header()?;

        self._next = next;

        Ok(tmp)
    }

    pub fn peak(&mut self) -> Token {
        self._next.clone()
    }
}