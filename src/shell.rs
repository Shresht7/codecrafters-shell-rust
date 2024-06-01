// Library
use std::io::{self, Write};

/// Struct that encapsulates the shell functionality
pub struct Shell {
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
    pub fn render_prompt(&mut self, prompt: &str) {
        // Print the prompt
        write!(self.writer, "{}", prompt).unwrap();

        // Flush the output to the screen so the prompt is displayed.
        // The `print!` macro (unlike `println!`) does not flush the output automatically.
        self.writer.flush().unwrap();
    }

    /// Reads the user input from the command line
    pub fn read_input(&mut self) -> String {
        let mut input = String::new(); // Create a string buffer to hold the input
        self.reader.read_line(&mut input).unwrap(); // Read the input into the buffer
        return input;
    }

    /// Parses the input into a vector of arguments
    pub fn parse_input<'a>(&mut self, input: &'a str) -> Vec<&'a str> {
        input.trim().split_whitespace().collect()
    }

    /// Handles command execution
    pub fn execute_command(&mut self, args: Vec<&str>) {
        // Extract the command name from the vector
        let command = args.get(0);
        // Match on the command name
        match command {
            Some(x) => println!("{}: command not found", x),
            None => println!("No command provided"),
        }
    }
}
