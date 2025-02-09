use std::process;
use std::fs;
use std::env;
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

struct ScriptState {
    cells: Vec<u8>,
    cell_index: usize,    
}

impl ScriptState {
    fn new() -> Self {
        let mut cells: Vec<u8> = Vec::new();        
        cells.push(0);        
        Self {
            cells,
            cell_index: 0,            
        }
    }    

    pub fn increase_index(&mut self) {
        self.cell_index += 1;
        if self.cells.get(self.cell_index).is_none() {
            self.cells.push(0);
        }
    }
    
    pub fn decrease_index(&mut self) {        
        if self.cell_index == 0 {
            self.cell_index = self.cells.len() - 1;
            return;
        }
        self.cell_index -= 1;
    }

    pub fn increase_value(&mut self) { self.cells[self.cell_index] += 1; } 
    pub fn decrease_value(&mut self) { self.cells[self.cell_index] -= 1; }
    pub fn get_value(&self, index: usize) -> u8 { self.cells[index] }
    pub fn get_index(&self) -> usize { self.cell_index }
}

fn execute_script(state: &mut ScriptState, contents: &String) -> Result<(), SyntaxError> {
    let tokens: Vec<char> = contents.chars().collect();
    let mut token_index = 0;
    let mut loop_indeces: Vec<(usize, usize)> = Vec::new();    

    let ignore = ['\n', ' '];    

    while token_index < contents.len() {
        match tokens[token_index] {
            '+' => state.increase_value(),
            '-' => state.decrease_value(),
            '>' => state.increase_index(),
            '<' => state.decrease_index(),
            '[' => {
                let cell_index = state.get_index();
                loop_indeces.push((token_index, cell_index));                
            },
            ']' => {                
                if loop_indeces.last().is_none() {
                    return Err(SyntaxError::new(format!("No matching loop starter '[' at index {}", token_index)));                    
                }

                let loop_index = loop_indeces.last().unwrap().0;
                let cell_index = loop_indeces.last().unwrap().1;
                let cell_value = state.get_value(cell_index);

                if cell_value == 0 {                    
                    loop_indeces.pop();
                    token_index += 1;          
                    continue;
                }

                token_index = loop_index;           
            },
            '.' => {                
                let cell_index = state.get_index();
                let character = state.get_value(cell_index) as char;
                //println!("Value: {}", state.get_value(cell_index));
                print!("{}", character);
            },            
            _ => {
                if !ignore.contains(&tokens[token_index]) {
                    return Err(SyntaxError::new(format!("Invalid token '{}'", tokens[token_index])))
                }
            }
        }

        //println!("{} - {}", tokens[token_index], token_index);
        token_index += 1;        
    }

    Ok(())
}

fn check_extension(filename: &str) -> Result<(), String> {    
    if let Some((_, ext)) = filename.rsplit_once('.') {
        if ext == "bf" {
            Ok(())
        } else {
            Err(format!("No valid file extension '{}'", ext))
        }
    } else {
        Err("Invalid file extension".to_string())
    }
}

fn main() {    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("[-] No brainf*ck file passed.");
        process::exit(1);
    }

    let filename = &args[1];
    if let Err(err) = check_extension(filename) {
        eprintln!("{}", err);
        process::exit(1);
    }

    match fs::read_to_string(filename) {
        Ok(content) => {
            let mut state = ScriptState::new();
            let stripped = content.lines()
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n");

            if let Err(err) = execute_script(&mut state, &stripped) {
                eprintln!("[-] Error: {}", err);
                process::exit(1);
            }
        },
        Err(err) => {
            eprintln!("[-] Error when reading file {} : {}", filename, err);
            process::exit(1);
        }
    }      
}
