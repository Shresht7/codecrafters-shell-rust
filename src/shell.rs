// Library
use crate::commands::Command;
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

    /// Parses the input into a vector of arguments
    fn parse_input(&mut self, input: &str) -> Vec<String> {
        let mut args = Vec::new(); // Vector to store the resulting args

        let mut word = String::new(); // Bucket to store the current word
        let mut in_single_quotes = false; // Boolean indicating whether we are currently in a single-quoted string
        let mut in_double_quotes = false; // Boolean indicating whether we are currently in a double-quoted string
        let mut chars = input.trim().chars(); // Iterator to walk over
        while let Some(ch) = chars.next() {
            match ch {
                '\'' => {
                    if !in_double_quotes {
                        in_single_quotes = !in_single_quotes;
                    } else {
                        word.push(ch);
                    }
                }
                '"' => {
                    if !in_single_quotes {
                        in_double_quotes = !in_double_quotes;
                    } else {
                        word.push(ch);
                    }
                }
                ' ' => {
                    if !word.is_empty() {
                        if !in_single_quotes && !in_double_quotes {
                            args.push(word.clone());
                            word.clear();
                        } else {
                            word.push(' ');
                        }
                    }
                }
                ch => word.push(ch),
            }
        }
        if !word.is_empty() {
            args.push(word.clone()); // Push any remaining word after the loop onto args
        }

        args
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

            // Split the input into a vector
            let args = self.parse_input(&input);

            // Act on the command-name
            self.execute_command(args)?;
        }
    }
}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let mut shell = Shell::default();
        let input = "command arg1 arg2";
        let actual = shell.parse_input(input);
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_no_args() {
        let mut shell = Shell::default();
        let input = "command";
        let actual = shell.parse_input(input);
        let expected = vec!["command"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_empty() {
        let mut shell = Shell::default();
        let input = "";
        let actual = shell.parse_input(input);
        let expected: Vec<&str> = vec![];
        assert_eq!(actual, expected);
    }

    // TODO: Implement escaped quotes
    #[test]
    #[ignore = "Not implemented yet"]
    fn test_parse_input_with_quoted_args() {
        let mut shell = Shell::default();
        let input = "command \"arg1 arg2\"";
        let actual = shell.parse_input(input);
        let expected = vec!["command", "arg1 arg2"];
        assert_eq!(actual, expected);
    }
}
