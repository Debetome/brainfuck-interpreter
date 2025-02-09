use crate::errors::SyntaxError;

pub struct Lexer {
    input: Vec<char>,
    position: usize
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0
        }
    }

    pub fn next_token(&mut self) -> Option<Result<char, SyntaxError>> {
        while let Some(&c) = self.input.get(self.position) {
            self.position += 1;            
            
            if "\n ".contains(c) { continue }
            if !"><+-[].".contains(c) {
                return Some(Err(SyntaxError::new(format!("Invalid token: '{}' at {}", c, self.position))));
            }

            return Some(Ok(c))
        }
        None
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn get_position(&self) -> usize { self.position }
}