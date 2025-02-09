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
        self.writer.execute(cursor::MoveToColumn(0))?; // Move the cursor back to the left-most column if it was ever displaced.

        Ok(buffer.trim().to_string())
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
