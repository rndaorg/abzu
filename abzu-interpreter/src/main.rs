mod lexer;
mod token;

use std::io::{self, Write};
use lexer::Lexer;
use token::Token;

fn main() {
    println!("ENU Interpreter - Day 1");
    println!("Cuneiform Programming Language");
    println!("Type 'exit' to quit\n");
    
    start_repl();
}

fn start_repl() {
    loop {
        print!("𒀜> ");  // Cuneiform-style prompt
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("exit") {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        // Create lexer and tokenize input
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        match tokens {
            Ok(tokens) => {
                println!("Tokens: {:?}", tokens);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    
    println!("𒆠𒂗𒈾 (Goodbye!)");
}