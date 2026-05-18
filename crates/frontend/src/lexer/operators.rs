#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OperatorEnum {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Unary
    Not,
    BinaryNot,

    // assign
    AssignEqual,
    AddEqual,
    SubEqual,
    MulEqual,
    DivEqual,
    BinaryAndEqual,
    BinaryOrEqual,
    BinaryXOREqual,
    LeftShiftEqual,
    RightShiftEqual,

    // boolean
    EqualEqual,
    NotEqual,

    LessThen,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,

    LogicalAnd,
    LogicalOr,

    // Binary
    BinaryAnd,
    BinaryOr,
    BinaryXOR,
    LeftShift,
    RightShift,
}