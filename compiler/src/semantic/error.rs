use std::fmt;

#[derive(Debug)]
pub struct SemanticError {
    pub message: String,
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Semantic error: {}", self.message)
    }
}
