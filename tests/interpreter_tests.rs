use interpreter::{interpret_expression, Lexer, Token};

#[test]
fn test_lexer_simple_expression() {
    let mut lexer = Lexer::new("1 + 2");
    let tokens = lexer.tokenize();

    assert_eq!(
        tokens,
        vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::EOF,
        ]
    );
}

#[test]
fn test_interpreter_simple_expression() {
    assert_eq!(interpret_expression("1 + 2"), 3.0);
    assert_eq!(interpret_expression("3 * 4"), 12.0);
    assert_eq!(interpret_expression("10 / 2"), 5.0);
    assert_eq!(interpret_expression("8 - 3"), 5.0);
}

#[test]
fn test_interpreter_nested_expression() {
    assert_eq!(interpret_expression("(2 + 3) * 4"), 20.0);
    assert_eq!(interpret_expression("10 / (5 - 3)"), 5.0);
    assert_eq!(interpret_expression("7 + 3 * (10 - 8)"), 13.0);
    assert_eq!(interpret_expression("256 * (512 / 16) * (256 + 128)"), 3145728.0);
}