mod lexer;
mod token;
mod value;
mod parser;
mod ast;
mod interpreter;

use std::io::{self, Write};
use lexer::Lexer;
use parser::Parser;
use interpreter::{Interpreter, Environment};

fn main() {
    println!("ENU Interpreter");
    println!("Sexagecimal Programming Language with Cuneiform bindings");
    println!("Type 'exit' to quit\n");
    
    start_repl();
}

fn start_repl() {
    let mut environment = Environment::new();
    let mut interpreter = Interpreter::new();
    
    loop {
        print!("𒀜> ");
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
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(e) => {
                println!("Lexer Error: {}", e);
                continue;
            }
        };
        
        // Parse tokens into AST
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse();
        
        match parse_result {
            Ok(program) => {
                println!("AST: {}", program);
                
                // Evaluate the program
                match interpreter.eval_program(&program, &mut environment) {
                    Ok(result) => {
                        if let Some(value) = result {
                            println!("Result: {}", value);
                        }
                    }
                    Err(e) => {
                        println!("Runtime Error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Parser Error: {}", e);
            }
        }
    }
    
    println!("𒆠𒂗𒈾 (Goodbye!)");
}