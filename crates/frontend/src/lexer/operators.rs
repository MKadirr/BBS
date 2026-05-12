pub enum BinaryOperatorEnum {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // boolean
    Equal,
    NotEqual,

    LessThen,
    LessOrEqual,
    GreaterThan,
    GreatorOrEqual,

    LogicalAnd,
    LogicalOr,

    // Binary
    BinaryAnd
    BinaryOr,
    BinaryXOR,
    LeftShift,
    RightShift,
}

pub enum UnaryOperatorEnum {
    Plus,
    Minus,
    Not,
    BinaryNot
}

pub enum AssignOperatorEnum {
    Equal,
    AddEqual,
    SubEqual,
    MulEqual,
    DivEqual,
    BinaryAndEqual,
    BinaryOrEqual,
    BinaryXOREqual,
    LeftShiftEqual,
    RightShiftEqual,
}