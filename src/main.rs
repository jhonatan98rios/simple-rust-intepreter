mod lexer;
mod parser;
mod interpreter;
mod repl;

use lexer::Lexer; // Lexer and Token types
use parser::Parser;         // Parser type
use interpreter::Interpreter; // Interpreter type

use repl::start_repl;

fn main() {
    
    println!("Rust Interpreter");
    println!("1. Run REPL");
    println!("2. Evaluate a single expression");
    println!("3. Exit");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    match choice.trim() {
        "1" => start_repl(),
        "2" => evaluate_expression(),
        "3" => println!("Goodbye!"),
        _ => println!("Invalid choice. Exiting."),
    }
}

fn evaluate_expression() {
    println!("Enter your expression:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match repl::process_input(input.trim()) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}