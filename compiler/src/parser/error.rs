use std::fmt;

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.line, self.column)
    }
}
