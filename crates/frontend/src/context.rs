
#[derive(Debug, Copy, Clone)]
pub struct Context {
    pub col: usize,
    pub row: usize,
}

impl Context {
    pub fn new(row: usize, col: usize) -> Context {
        Context {
            row,
            col,
        }
    }
}