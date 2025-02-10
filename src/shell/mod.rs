use std::io;

use readline::ReadLine;

use crate::{helpers, parser::Parser};

mod executor;
mod prompt;
mod readline;

/// Struct that encapsulates the shell functionality
pub struct Shell {
    /// The reader to read input from
    // reader: io::BufReader<io::Stdin>,
    /// The writer to write output to
    writer: io::BufWriter<io::Stdout>,
    readline: ReadLine,
}

// Default implementation for the Shell struct
impl Default for Shell {
    fn default() -> Self {
        let mut readline = ReadLine::default();
        readline.with_completions(vec![
            "cd".into(),
            "echo".into(),
            "exit".into(),
            "pwd".into(),
            "type".into(),
        ]);
        let path_completions = helpers::path::get_executables()
            .iter()
            .filter_map(|p| {
                p.file_name()
                    .and_then(|x| Some(x.to_string_lossy().into_owned()))
            })
            .collect();
        readline.with_completions(path_completions);
        Shell {
            // reader: io::BufReader::new(io::stdin()),
            writer: io::BufWriter::new(io::stdout()),
            readline,
        }
    }
}

// Implementation of the Shell struct
impl Shell {
    /// Handles the shell loop
    /// The shell will start a REPL (Read-Eval-Print Loop)
    /// that will keep reading the input and processing commands
    /// until the user exits the shell.
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            // Render the prompt to the screen
            self.render_prompt("$ ")?;

            // Wait for user input and read it into a variable
            let input = self.readline.read()?;
            let input = input.trim();
            if input.is_empty() {
                continue; //Skip this iteration if input was empty
            }

            // Split the input into a vector
            let (args, out_target, err_target) = Parser::parse(&input)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

            // Act on the command-name
            self.execute_command(args, out_target, err_target)?;
        }
    }
}
