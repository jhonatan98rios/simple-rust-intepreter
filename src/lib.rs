pub mod lexer;
pub mod parser;
pub mod interpreter;

// Re-export common items for convenience
pub use lexer::{Lexer, Token};
pub use parser::{Parser, Expr};
pub use interpreter::Interpreter;


pub fn interpret_expression(input: &str) -> f64 {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    
    let interpreter = Interpreter;
    let result = interpreter.evaluate(&ast);

    return result;
}