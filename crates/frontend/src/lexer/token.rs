use crate::lexer::operators::*;
pub enum TokenType {

    // Operators
    BinaryOperator(BinaryOperator)

    // Brackets
    LeftParentheses,
    RightParentheses,

    LeftCurlyBracket,
    RightCurlyParenthesis,

    LeftSquareBracket,
    RightSquareParenthesis,


}

pub struct Token {
    col: usize,
    row: usize,

    ty: TokenType,
}