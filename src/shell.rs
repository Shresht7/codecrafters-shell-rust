// Library
use crate::{commands::Command, parser::Parser};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal, ExecutableCommand,
};
use std::{fs, io, time};

// Traits
use std::io::Write;

/// Struct that encapsulates the shell functionality
pub struct Shell {
    /// The reader to read input from
    // reader: io::BufReader<io::Stdin>,
    /// The writer to write output to
    writer: io::BufWriter<io::Stdout>,
    completions: Vec<&'static str>,
}

// Default implementation for the Shell struct
impl Default for Shell {
    fn default() -> Self {
        Shell {
            // reader: io::BufReader::new(io::stdin()),
            writer: io::BufWriter::new(io::stdout()),
            completions: vec!["cd", "echo", "exit", "pwd", "type"],
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
        let mut buffer = String::new(); // The buffer that represents the input

        terminal::enable_raw_mode()?; // Enable raw mode to override key input processing
        self.writer.execute(cursor::Hide)?;

        loop {
            // Wait for a key event
            if event::poll(time::Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(evt) if evt.kind == KeyEventKind::Press => match evt {
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        }
                        | KeyEvent {
                            code: KeyCode::Esc, ..
                        } => {
                            break; // Exit the loop immediately
                        }

                        KeyEvent {
                            code: KeyCode::Enter,
                            ..
                        }
                        // Important: Looks like codecrafters use Ctrl+J to enter the line.
                        // So, if this case isn't handled, all input pickup a trailing j causing everything to crash and burn
                        | KeyEvent {
                            code: KeyCode::Char('j'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        } => {
                            // On Enter, finish the line.
                            writeln!(self.writer, "")?;
                            self.writer.flush()?;
                            break;
                        }

                        KeyEvent {
                            code: KeyCode::Backspace,
                            ..
                        } => {
                            if !buffer.is_empty() {
                                // Remove the last character from the buffer.
                                buffer.pop();
                                // Move the cursor back, clear the character, and move back again.
                                self.writer.execute(cursor::MoveLeft(1))?;
                                write!(self.writer, " ")?;
                                self.writer.execute(cursor::MoveLeft(1))?;
                                self.writer.flush()?;
                            }
                        }

                        KeyEvent {
                            code: KeyCode::Tab, ..
                        } => {
                            let prefix = buffer.as_str();

                            if let Some(suggestion) =
                                self.completions.iter().find(|cmd| cmd.starts_with(prefix))
                            {
                                // Erase the current buffer from the display.
                                let len = buffer.len() as u16;
                                self.writer.execute(cursor::MoveLeft(len))?;
                                for _ in 0..len {
                                    write!(self.writer, " ")?;
                                }
                                self.writer.execute(cursor::MoveLeft(len))?;

                                // Replace the buffer with the suggestion.
                                buffer = suggestion.to_string();
                                write!(self.writer, "{} ", buffer)?;
                                self.writer.flush()?;
                            }
                        }

                        KeyEvent {
                            code: KeyCode::Char(c),
                            ..
                        } => {
                            // Append the character to our buffer.
                            buffer.push(c);
                            write!(self.writer, "{}", c)?;
                            self.writer.flush()?;
                        }

                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        self.writer.execute(cursor::Show)?;
        terminal::disable_raw_mode()?;

        Ok(buffer.trim().to_string())
    }

    /// Handles command execution
    fn execute_command(
        &mut self,
        args: Vec<String>,
        out_target: Option<(String, bool)>,
        err_target: Option<(String, bool)>,
    ) -> io::Result<()> {
        // Decide the writer for stdout.
        let mut out_writer: Box<dyn io::Write> = if let Some((filename, append)) = out_target {
            let file = fs::OpenOptions::new()
                .write(true)
                .append(append)
                .create(true)
                .open(filename)?;
            Box::new(io::BufWriter::new(file))
        } else {
            Box::new(io::BufWriter::new(io::stdout()))
        };

        // Decide the writer for stderr.
        let mut err_writer: Box<dyn io::Write> = if let Some((filename, append)) = err_target {
            let file = fs::OpenOptions::new()
                .write(true)
                .append(append)
                .create(true)
                .open(filename)?;
            Box::new(io::BufWriter::new(file))
        } else {
            Box::new(io::BufWriter::new(io::stderr()))
        };

        // Extract the command name from the vector
        if let Some(command) = args.get(0) {
            // Try to parse the command into a Command enum
            return match command.parse::<Command>() {
                Ok(cmd) => Ok(cmd.execute(args, &mut out_writer, &mut err_writer)?),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unexpected command! {command}"),
                )),
            };
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
            let (args, out_target, err_target) = Parser::parse(&input)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

            // Act on the command-name
            self.execute_command(args, out_target, err_target)?;
        }
    }
}
