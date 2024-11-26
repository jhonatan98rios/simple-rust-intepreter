use crate::lexer::Token;  // Import Token from the lexer module
use crate::parser::Expr;

pub struct Interpreter;

impl Interpreter {
    // Function to evaluate an expression
    pub fn evaluate(&self, expr: &Expr) -> f64 {
        match expr {
            Expr::Literal(value) => self.evaluate_literal(value),
            Expr::Binary { left, operator, right } => self.evaluate_binary(left, operator, right),
        }
    }

    // Function to evaluate a literal (just returns its value)
    fn evaluate_literal(&self, value: &f64) -> f64 {
        *value
    }

    // Function to evaluate a binary expression (e.g., 3 + 5)
    fn evaluate_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> f64 {
        let left_value = self.evaluate(left);   // Recursively evaluate the left expression
        let right_value = self.evaluate(right); // Recursively evaluate the right expression

        // Apply the operator to the evaluated values
        match operator {
            Token::Plus => self.evaluate_addition(left_value, right_value),
            Token::Minus => self.evaluate_subtraction(left_value, right_value),
            Token::Star => self.evaluate_multiplication(left_value, right_value),
            Token::Slash => self.evaluate_division(left_value, right_value),
            _ => panic!("Unknown operator: {:?}", operator),
        }
    }

    // Function to evaluate addition
    fn evaluate_addition(&self, left: f64, right: f64) -> f64 {
        left + right
    }

    // Function to evaluate subtraction
    fn evaluate_subtraction(&self, left: f64, right: f64) -> f64 {
        left - right
    }

    // Function to evaluate multiplication
    fn evaluate_multiplication(&self, left: f64, right: f64) -> f64 {
        left * right
    }

    // Function to evaluate division
    fn evaluate_division(&self, left: f64, right: f64) -> f64 {
        if right == 0.0 {
            panic!("Division by zero is not allowed");
        }
        left / right
    }
}
