mod lexer;
mod parser;
mod interpreter;

use lexer::{Lexer}; // Lexer and Token types
use parser::Parser;         // Parser type
use interpreter::Interpreter; // Interpreter type

fn main() {
    // Example input expression: "3 + 5 * (10 - 2)"
    let input = "3 + 5 * (10 - 2)";

    // Step 1: Lexical Analysis (tokenization)
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    // Step 2: Parsing (construct AST)
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Step 3: Evaluation (compute result)
    let interpreter = Interpreter;
    let result = interpreter.evaluate(&ast);

    // Print the result
    println!("Result: {}", result); // Expected output: 43.0
}