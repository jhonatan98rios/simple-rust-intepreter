// Define the token types
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),        // e.g., 123.45
    Plus,               // '+'
    Minus,              // '-'
    Star,               // '*'
    Slash,              // '/'
    LeftParen,          // '('
    RightParen,         // ')'
    EOF,                // End of file/input
}

// The Lexer struct will manage the input and current current
pub struct Lexer<'a> {
    input: &'a str,
    current: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.current < self.input.len() {
            match self.current_char() {
                '0'..='9' => tokens.push(self.consume_number()),
                '+' => { self.advance(); tokens.push(Token::Plus); },
                '-' => { self.advance(); tokens.push(Token::Minus); },
                '*' => { self.advance(); tokens.push(Token::Star); },
                '/' => { self.advance(); tokens.push(Token::Slash); },
                '(' => { self.advance(); tokens.push(Token::LeftParen); },
                ')' => { self.advance(); tokens.push(Token::RightParen); },
                ' ' | '\t' | '\n' => self.advance(), // Ignore whitespace
                _ => panic!("Unexpected character: {}", self.current_char()),
            }
        }

        tokens.push(Token::EOF); // End of file marker
        tokens
    }

    fn current_char(&self) -> char {
        self.input[self.current..].chars().next().unwrap_or('\0')
    }

    fn advance(&mut self) {
        self.current += self.current_char().len_utf8();
    }

    fn consume_number(&mut self) -> Token {
        let start = self.current;
        while self.current_char().is_digit(10) || self.current_char() == '.' {
            self.advance();
        }
        let number: f64 = self.input[start..self.current].parse().unwrap();
        Token::Number(number)
    }
}