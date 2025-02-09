use std::process;
use std::fs;
use std::env;

mod parser;
mod lexer;
mod state;
mod errors;

use parser::Parser;
use lexer::Lexer;
use state::State;

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
            let stripped = content.lines()
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n");

            let mut lexer = Lexer::new(&stripped);
            let mut state = State::new();            
            let mut parser = Parser::new(&mut lexer, &mut state);

            if let Err(err) = parser.parse() {
                eprintln!("[-] Error: {}", err);
                process::exit(1);
            }
        },
        Err(err) => {
            eprintln!("[-] Error when reading the file {} : {}", filename, err);
            process::exit(1);
        }
    }      
}
