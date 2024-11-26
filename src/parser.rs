use crate::lexer::Token; // Import Token and other necessary items from the lexer module


// Define the AST node types
#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal(f64),
}

// The Parser struct manages tokens and current
pub struct Parser {
    tokens: Vec<Token>, // Tokens from the lexer
    current: usize,    // Current current in tokens
}

impl Parser {
    // Create a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // Helper to peek at the current token
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    // Helper to advance and consume the current token
    fn advance(&mut self) -> &Token {
        self.current += 1;
        self.tokens.get(self.current - 1).unwrap()
    }

    // Parse a full expression
    pub fn parse(&mut self) -> Expr {
        self.parse_expression()
    }

    // Parse the expression, handling both addition, subtraction, multiplication, and division
    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_term(); // Parse the first term (handles multiplication, division)

        while let Some(op) = self.peek() {
            match op {
                Token::Plus | Token::Minus => {
                    let operator = self.advance().clone(); // Get the operator
                    let right = self.parse_term(); // Parse the next term
                    left = Expr::Binary {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    }; // Combine the terms
                },
                _ => break, // Stop when we encounter an operator that isn't + or -
            }
        }

        left
    }

    // Parse a term, which handles multiplication and division
    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor(); // Parse the first factor (numbers or parenthesized expressions)

        while let Some(op) = self.peek() {
            match op {
                Token::Star | Token::Slash => {
                    let operator = self.advance().clone(); // Get the operator
                    let right = self.parse_factor(); // Parse the next factor
                    left = Expr::Binary {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    }; // Combine the terms
                },
                _ => break, // Stop when we encounter an operator that isn't * or /
            }
        }

        left
    }

    // Parse a factor, which handles numbers and parenthesized expressions
    fn parse_factor(&mut self) -> Expr {
        if let Some(Token::Number(n)) = self.peek() {
            let n_value = *n; // Dereference the reference to get the value
            self.consume(); // Consume the number
            Expr::Literal(n_value) // Return the number as a literal expression
        } else if let Some(Token::LeftParen) = self.peek() {
            self.consume(); // Consume the '('
            let expr = self.parse_expression(); // Parse the expression inside the parentheses
            self.consume(); // Consume the ')'
            expr
        } else {
            panic!("Unexpected token: {:?}", self.peek()); // Panic if token is unexpected
        }
    }

    fn consume(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            Some(self.advance().clone())
        } else {
            None
        }
    }
}
