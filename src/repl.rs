use std::io::{self, Write};

use interpreter::interpret_expression;


pub fn start_repl() {
    println!("Rust Interpreter REPL");
    println!("Type 'exit' to quit.\n");

    loop {
        // Display prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // Read input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input.");
            continue;
        }

        // Trim input and handle exit command
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        // Process input
        match process_input(input) {
            Ok(result) => println!("{}", result),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

pub fn process_input(input: &str) -> Result<f64, String> {
    // This will now return a Result<f64, String>
    let result = interpret_expression(input);
    Ok(result) // wrapping result in Ok
}