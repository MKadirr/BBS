use crate::context::Context;

type LexResult<T> = Result<T, LexError>;

#[derive(Debug)]
pub struct LexError {
    context: Context,
    reason: LexErrorType,
}

impl LexError {
    pub fn new(reason: LexErrorType, context: Context) -> Self {
        LexError {
            reason,
            context,
        }
    }

    pub fn unwrap(&self) {
        eprintln!("{}:{}: {}", self.context.row, self.context.col, self.reason.to_string())
    }
}

#[derive(Debug)]
pub enum LexErrorType {
    NumberToBigInt,
    IntegerOverFlow,
}

impl LexErrorType {
    pub fn to_string(&self) -> &str {
        match self {
            LexErrorType::NumberToBigInt => { "Number Too big for that type: please add L at the end to cast to wider type" },
            LexErrorType::IntegerOverFlow => { "Number does not fit in a long" }
        }
    }
}