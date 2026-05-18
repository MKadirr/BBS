use std::ops::Index;
use crate::lexer::operators::{OperatorEnum};
use crate::lexer::parser::parser::Parser;
use crate::lexer::token::{NumberType, Token, TokenType};
use crate::lexer::token::TokenType::{Operator};

use regex::Regex;
use crate::context::Context;
use crate::lexer::lex_error::{LexError, LexErrorType};



impl Parser {
    fn get_context(&self) -> Context {
        Context::new(self._row, self._col)
    }

    pub fn parse_header(&mut self) -> Result<Token, LexError> {
        let token = self.parse()?;

        let mut nb_back_carriage: usize = 0;
        let mut idx_last: usize = 0;
        for (i, c) in token.matched.chars().enumerate() {
            if c == '\n' { // TODO find a way to test for windows user so that they have the good row value ("\r\n" | "\n\r")
                nb_back_carriage += 1;
                idx_last = i
            }
        }

        if nb_back_carriage != 0 {
            self._col += token.matched.len() - idx_last;
        }

        self._row = token.matched.len() - idx_last;

        self._idx += token.matched.len();

        Ok(token)
    }

    fn parse(&mut self) -> Result<Token, LexError> {
        let context: Context = self.get_context();

        if self._idx == self._data.len() {
            return Ok(Token::new(TokenType::EOF, context, String::from("")));
        }

        if let Some(t) = self.parse_id()? {
            return Ok(t);
        }

        if let Some(t) = self.parse_spaces()? {
            return Ok(t);
        }

        if let Some(t) = self.parse_number()? {
            return Ok(t);
        }

        if let Some(t) = self.parse_operator()? {
            return Ok(t);
        }

        if let Some(t) = self.parse_string()? {
            return Ok(t);
        }

        todo!()
    }

    fn parse_symbols(&mut self) -> Result<Option<Token>, LexError> {
        match self._data.chars().nth(self._idx).unwrap() {
            '{' => Ok(Some(Token::new(TokenType::LeftCurlyBracket,   self.get_context(), String::from(&self._data[0..1])))),
            '}' => Ok(Some(Token::new(TokenType::RightCurlyBracket,  self.get_context(), String::from(&self._data[0..1])))),

            '[' => Ok(Some(Token::new(TokenType::LeftSquareBracket,  self.get_context(), String::from(&self._data[0..1])))),
            ']' => Ok(Some(Token::new(TokenType::RightSquareBracket, self.get_context(), String::from(&self._data[0..1])))),

            '(' => Ok(Some(Token::new(TokenType::LeftParentheses,    self.get_context(), String::from(&self._data[0..1])))),
            ')' => Ok(Some(Token::new(TokenType::RightParentheses,   self.get_context(), String::from(&self._data[0..1])))),

            '.' => Ok(Some(Token::new(TokenType::Dot,                self.get_context(), String::from(&self._data[0..1])))),
            ':' => Ok(Some(Token::new(TokenType::Colon,              self.get_context(), String::from(&self._data[0..1])))),
            ';' => Ok(Some(Token::new(TokenType::SemiColon,          self.get_context(), String::from(&self._data[0..1])))),
            _ => Ok(None)
        }
    }

    fn parse_spaces(&mut self) -> Result<Option<Token>, LexError> {
        let data = &self._data[self._idx..self._data.len()];

        // Match basic spaces
        let regex = Regex::new(r"^[ \t]+").unwrap();
        if let Some(m) = regex.find(&data) {
            let matched = String::from(&data[m.start()..m.end()]);
            return Ok(Some(Token::new(TokenType::Space, self.get_context(), matched)))
        }

        // match '\n'
        let regex = Regex::new(r"^\n").unwrap();
        if let Some(m) = regex.find(&data) {
            let matched = String::from(&data[m.start()..m.end()]);
            return Ok(Some(Token::new(TokenType::EOL, self.get_context(), matched)))
        }

        Ok(None)
    }

    fn parse_operator(&mut self) -> Result<Option<Token>, LexError> {
        todo!()
    }

    fn parse_string(&mut self) -> Result<Option<Token>, LexError> {
        todo!()
    }

    fn parse_number(&mut self) -> Result<Option<Token>, LexError> {
        // 0x       hexa
        // 0123     octal
        // 123      deci
        // 103123L  long variant

        // .1...2f float
        // .1...3  double

        // hexa
        let data = &self._data[self._idx..self._data.len() - 1];
        let regex = Regex::new(r"^0x[0-9A-Fa-f]+L?").unwrap();
        if let Some(m) = regex.find(&data) {
            let data = &data[m.start()..m.end()].to_lowercase();

            let mut nb: u64 = 0;

            let mut last : usize = data.len();

            let is_long = data.ends_with('L');

            if is_long {
                last -= 1;
            }

            for c in data[2..last].chars() {
                nb = nb.checked_mul(16)
                    .ok_or_else(|| LexError::new(LexErrorType::NumberToBigInt, self.get_context()))?;

                nb = nb.checked_add(c.to_digit(16).unwrap() as u64)
                    .ok_or_else(|| LexError::new(LexErrorType::NumberToBigInt, self.get_context()))?;
            }

            if is_long {
                let val : i64 = nb.try_into().or(Err(LexError::new(LexErrorType::IntegerOverFlow, self.get_context())))?;
                return Ok(Some(Token::new(TokenType::Number(NumberType::Long(val)), self.get_context(), data.clone())));
            }
            else {
                let val : i32 = nb.try_into().or(Err(LexError::new(LexErrorType::IntegerOverFlow, self.get_context())))?;
                return Ok(Some(Token::new(TokenType::Number(NumberType::Int(val)), self.get_context(), data.clone())));
            }
        }

        // octal
        let data = &self._data[self._idx..self._data.len() - 1];
        let regex = Regex::new(r"^0[0-9A-Fa-f]").unwrap();
        if let Some(m) = regex.find(&data) {
            let data = data[m.start()..m.end()].to_lowercase();

            let mut nb = 0;
            for c in data[2..].chars() {
                nb *= 16;
                nb += c.to_digit(16).unwrap();
            }
        }
        
        Ok(None)
    }

    fn parse_id(&mut self) -> Result<Option<Token>, LexError> {
        // match an id

        let data = &self._data[self._idx..self._data.len() - 1];
        let regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
        if let Some(m) = regex.find(&data) {
            let data = &data[m.start()..m.end()];

            // match Keyword
            let token_type = match data {
                "return" => { TokenType::Return }
                "break" => { TokenType::Break }
                "if" => { TokenType::If }
                "else" => { TokenType::Else }
                "while" => { TokenType::While }
                "for" => { TokenType::For }
                "spawn" => { TokenType::Spawn }
                "join" => { TokenType::Join }

                n => { TokenType::Id(String::from("n")) }
            };

            self._row += m.len();

            return Ok(Some(Token::new(token_type, self.get_context(), data.to_string())));
        }

        Ok(None)
    }
}

