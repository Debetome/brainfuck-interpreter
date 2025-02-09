use crate::lexer::Lexer;
use crate::state::State;
use crate::errors::SyntaxError;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    state: &'a mut State,    
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer, state: &'a mut State) -> Self {
        Self { lexer, state }
    }

    pub fn parse(&mut self) -> Result<(), SyntaxError> {        
        let mut loop_indeces: Vec<(usize, usize)> = Vec::new();        

        while let Some(lexer_result) = self.lexer.next_token() {
            if let Err(error) = lexer_result { 
                return Err(error); 
            }

            match lexer_result.unwrap() {
                '+' => self.state.increase_value(),
                '-' => self.state.decrease_value(),
                '>' => self.state.increase_index(),
                '<' => self.state.decrease_index(),
                '[' => {
                    let token_index = self.lexer.get_position() - 1;
                    let cell_index = self.state.get_index();         
                    loop_indeces.push((token_index, cell_index));
                },
                ']' => {                
                    if loop_indeces.last().is_none() {
                        let pos = self.lexer.get_position() - 1;
                        return Err(SyntaxError::new(format!("No matching loop starter '[' at position {}", pos)));                    
                    }
    
                    let loop_index = loop_indeces.last().unwrap().0;
                    let cell_index = loop_indeces.last().unwrap().1;
                    let cell_value = self.state.get_value(cell_index);
    
                    if cell_value == 0 {                    
                        loop_indeces.pop();                                  
                        continue;
                    }
    
                    self.lexer.set_position(loop_index);
                },
                '.' => {                
                    let cell_index = self.state.get_index();
                    let character = self.state.get_value(cell_index) as char;
                    //println!("Value: {}", state.get_value(cell_index));
                    print!("{}", character);
                },            
                _ => continue
            }
        }

        Ok(())         
    }
}