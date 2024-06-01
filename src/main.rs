// Library
use std::io::{self, Write};

// ----
// MAIN
// ----

/// The main entry point of the application
fn main() {
    // Render the prompt to the screen
    render_prompt("$ ");

    // Wait for user input and read it into a variable
    let input = read_input();

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

/// Renders the prompt to the screen
fn render_prompt(str: &str) {
    // Print the prompt
    print!("{}", str);

    // Flush the output to the screen so the prompt is displayed.
    // The `print!` macro (unlike `println!`) does not flush the output automatically.
    io::stdout().flush().unwrap();
}

/// Reads the user input from the command line
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input;
}
