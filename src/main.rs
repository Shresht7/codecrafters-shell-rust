// Library
use std::io::{self, Write};

// ----
// MAIN
// ----

/// The main entry point of the application
fn main() {
    // Initialize the Shell
    let mut shell = Shell::default();

    // Render the prompt to the screen
    shell.render_prompt("$ ");

    // Wait for user input and read it into a variable
    let input = shell.read_input();

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

/// Struct that encapsulates the shell functionality
struct Shell {
    /// The reader to read input from
    reader: io::Stdin,
    /// The writer to write output to
    writer: io::Stdout,
}

// Default implementation for the Shell struct
impl Default for Shell {
    fn default() -> Self {
        Shell {
            reader: io::stdin(),
            writer: io::stdout(),
        }
    }
}

// Implementation of the Shell struct
impl Shell {
    /// Renders the prompt to the screen
    fn render_prompt(&mut self, prompt: &str) {
        // Print the prompt
        write!(self.writer, "{}", prompt).unwrap();

        // Flush the output to the screen so the prompt is displayed.
        // The `print!` macro (unlike `println!`) does not flush the output automatically.
        self.writer.flush().unwrap();
    }

    /// Reads the user input from the command line
    fn read_input(&mut self) -> String {
        let mut input = String::new(); // Create a string buffer to hold the input
        self.reader.read_line(&mut input).unwrap(); // Read the input into the buffer
        return input;
    }
}
