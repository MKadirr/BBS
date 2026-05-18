use crate::context::Context;
use crate::lexer::operators::*;

#[derive(Clone, PartialEq, Debug)]
pub enum NumberType {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    Id(String), // for var names

    // Symbol
    Dot,
    SemiColon,
    Colon,

    // Operators
    Operator(OperatorEnum),

    // Keyword
    Return,
    Break,
    If,
    Else,
    While,
    For,
    Spawn,
    Join,

    // Brackets
    LeftParentheses,
    RightParentheses,

    LeftCurlyBracket,
    RightCurlyBracket,

    LeftSquareBracket,
    RightSquareBracket,

    // types
    String(String),
    Number(NumberType),

    // System
    EOL,
    EOF,
    Space,

    Error,
    NotStarted,
}

#[derive(Clone)]
pub struct Token {
    pub context: Context,

    pub ty: TokenType,

    pub matched: String,
}

impl Token {
    pub fn new(token_type: TokenType, context: Context, matched: String) -> Token {
        Token {
            ty: token_type,
            context,
            matched,
        }
    }
}