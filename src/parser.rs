mod state;
mod errors;

use state::ScriptState;
use errors::SyntaxError;

pub struct Parser {
    state: &mut ScriptState,
    tokens: Vec<char>,    
}

impl Parser {
    fn new(state: &mut ScriptState, tokens: Vec<char>) -> Self {
        Self { state, tokens }
    }

    pub fn parse(&mut self) -> Result<(), SyntaxError> {        
        let mut loop_indeces: Vec<(usize, usize)> = Vec::new();
        let mut token_index = 0;

        while token_index < contents.len() {
            match self.tokens[token_index] {
                '+' => self.state.increase_value(),
                '-' => self.state.decrease_value(),
                '>' => self.state.increase_index(),
                '<' => self.state.decrease_index(),
                '[' => {
                    let cell_index = self.state.get_index();
                    loop_indeces.push((token_index, cell_index));
                },
                ']' => {                
                    if loop_indeces.last().is_none() {
                        return Err(SyntaxError::new(format!("No matching loop starter '[' at index {}", token_index)));                    
                    }
    
                    let loop_index = loop_indeces.last().unwrap().0;
                    let cell_index = loop_indeces.last().unwrap().1;
                    let cell_value = self.state.get_value(cell_index);
    
                    if cell_value == 0 {                    
                        loop_indeces.pop();
                        token_index += 1;          
                        continue;
                    }
    
                    token_index = loop_index;           
                },
                '.' => {                
                    let cell_index = self.state.get_index();
                    let character = self.state.get_value(cell_index) as char;
                    //println!("Value: {}", state.get_value(cell_index));
                    print!("{}", character);
                },            
                _ => {                    
                    if !['\n', ' '].contains(&self.tokens[token_index]) {
                        return Err(SyntaxError::new(format!("Invalid token '{}'", self.tokens[token_index])));
                    }
                }
            }
        }            
    }
}