use std::fmt;

#[derive(Debug)]
struct SyntaxError {
    details: String
}

impl SyntaxError {
    fn new(details: String) -> Self {
        Self { details }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax Error: {}", self.details)
    }
}