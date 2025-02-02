// Library
use crate::{commands::Command, helpers::parse};
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
    fn render_prompt(&mut self, prompt: &str) -> io::Result<()> {
        // Print the prompt
        write!(self.writer, "{}", prompt)?;

        // Flush the output to the screen so the prompt is displayed.
        // The `print!` macro (unlike `println!`) does not flush the output automatically.
        self.writer.flush()?;

        Ok(())
    }

    /// Reads the user input from the command line
    fn read_input(&mut self) -> io::Result<String> {
        let mut input = String::new(); // Create a string buffer to hold the input
        self.reader.read_line(&mut input)?; // Read the input into the buffer
        Ok(input) // Return the input
    }

    /// Handles command execution
    fn execute_command(&mut self, args: Vec<String>) -> io::Result<()> {
        // Extract the command name from the vector
        if let Some(command) = args.get(0) {
            // Try to parse the command into a Command enum
            if let Ok(cmd) = command.parse::<Command>() {
                cmd.execute(args)?; // Execute the command
            }
        }
        // If no command is provided, continue as if nothing happened
        // Since this is a shell repl, we don't want to error out if no command is provided
        Ok(()) // Return and continue on
    }

    /// Handles the shell loop
    /// The shell will start a REPL (Read-Eval-Print Loop)
    /// that will keep reading the input and processing commands
    /// until the user exits the shell.
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            // Render the prompt to the screen
            self.render_prompt("$ ")?;

            // Wait for user input and read it into a variable
            let input = self.read_input()?;
            let input = input.trim();
            if input.is_empty() {
                continue; //Skip this iteration if input was empty
            }

            // Split the input into a vector
            let args = parse::Parser::parse(&input)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

            // Act on the command-name
            self.execute_command(args)?;
        }
    }
}
