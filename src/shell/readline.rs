use std::{io::Write, time};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal, ExecutableCommand,
};

impl super::Shell {
    /// Reads the user input from the command line
    pub(super) fn read_input(&mut self) -> std::io::Result<String> {
        let mut buffer = String::new(); // The buffer that represents the input

        // Enable terminal raw mode with our [RawModeGuard] that will automatically disable when it is dropped
        let _raw_mode = RawModeGuard::new()?;

        loop {
            // Wait for a key event
            if event::poll(time::Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(evt) if evt.kind == KeyEventKind::Press => match evt {

                        // Exit on Ctrl+C or Esc
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

                        // Finish input on Enter or Ctrl+J
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
                            // TODO: Add the complete line to a history buffer
                            break;
                        }

                        // Process backspace
                        KeyEvent {
                            code: KeyCode::Backspace,
                            ..
                        } => {
                            self.handle_backspace(&mut buffer)?;
                        }

                        // Process Tab completion
                        KeyEvent {
                            code: KeyCode::Tab, ..
                        } => {
                            self.handle_tab_completion(&mut buffer)?;
                        }

                        // Process any other character
                        KeyEvent {
                            code: KeyCode::Char(c),
                            ..
                        } => {
                            self.handle_character_input(&mut buffer, c)?;
                        }

                        _ => {} // Ignore other events
                    },
                    _ => {} // Ignore non-key events
                }
            }
        }
        self.writer.execute(cursor::MoveToColumn(0))?; // Move the cursor back to the left-most column if it was ever displaced.

        Ok(buffer.trim().to_string())
    }

    /// Appends a character to the buffer and displays it to the screen
    fn handle_character_input(
        &mut self,
        buffer: &mut String,
        c: char,
    ) -> Result<(), std::io::Error> {
        buffer.push(c);
        write!(self.writer, "{}", c)?;
        self.writer.flush()?;
        Ok(())
    }

    /// Handles the `backspace` key by removing the last character
    fn handle_backspace(&mut self, buffer: &mut String) -> Result<(), std::io::Error> {
        Ok(if !buffer.is_empty() {
            // Remove the last character from the buffer.
            buffer.pop();
            // Move the cursor back, clear the character, and move back again.
            self.writer.execute(cursor::MoveLeft(1))?;
            write!(self.writer, " ")?;
            self.writer.execute(cursor::MoveLeft(1))?;
            self.writer.flush()?;
        })
    }

    /// Handles the `Tab` key and applies completions, if any
    fn handle_tab_completion(&mut self, buffer: &mut String) -> Result<(), std::io::Error> {
        let prefix = buffer.as_str();
        Ok(
            if let Some(suggestion) = self.completions.iter().find(|cmd| cmd.starts_with(prefix)) {
                // Erase the current buffer from the display.
                let len = buffer.len() as u16;
                self.writer.execute(cursor::MoveLeft(len))?;
                for _ in 0..len {
                    write!(self.writer, " ")?;
                }
                self.writer.execute(cursor::MoveLeft(len))?;

                // Replace the buffer with the suggestion.
                *buffer = suggestion.to_string();
                write!(self.writer, "{} ", buffer)?;
                self.writer.flush()?;
            },
        )
    }
}

// RAII guard to enable raw mode and ensure that it's disabled on drop
struct RawModeGuard;

impl RawModeGuard {
    fn new() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        // Ensure raw mode is disabled (even on error) when the guard is dropped
        let _ = terminal::disable_raw_mode();
    }
}
