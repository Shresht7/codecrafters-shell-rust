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
}
