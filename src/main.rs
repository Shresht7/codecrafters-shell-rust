// Library
use std::io::{self, Write};

// ----
// MAIN
// ----

/// The main entry point of the application
fn main() {
    // Print the prompt
    print!("$ ");

    // Flush the output to the screen so the prompt is displayed.
    // The `print!` macro (unlike `println!`) does not flush the output automatically.
    io::stdout().flush().unwrap();

    // Wait for user input and read it into a String variable.
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Split the input into a vector
    let args: Vec<&str> = input.trim().split_whitespace().collect();

    // Extract the command name from the vector
    let command = args.get(0);

    // Act on the command-name
    match command {
        Some(x) => println!("{}: command not found", x),
        None => println!("No command provided"),
    }
}
