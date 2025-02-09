use std::{
    io::{self, BufWriter, Write},
    time,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal, ExecutableCommand,
};

pub(super) struct ReadLine {
    writer: BufWriter<io::Stdout>,
    poll_interval: time::Duration,
    completions: Vec<String>,
}

impl Default for ReadLine {
    fn default() -> Self {
        Self {
            writer: BufWriter::new(std::io::stdout()),
            poll_interval: time::Duration::from_millis(100),
            completions: Vec::new(),
        }
    }
}

impl ReadLine {
    pub fn with_completions(&mut self, completions: Vec<String>) -> &mut Self {
        self.completions = completions;
        self
    }

    pub(super) fn read(&mut self) -> std::io::Result<String> {
        let mut buffer = String::new(); // The buffer that represents the input

        // Enable terminal raw mode with our [RawModeGuard] that will automatically disable when it is dropped
        let _raw_mode = RawModeGuard::new()?;

        loop {
            // Wait for a key-event
            if event::poll(self.poll_interval)? {
                match event::read()? {
                    // Only check for key-presses to prevent double-trigger on both press and release
                    Event::Key(evt) if evt.kind == KeyEventKind::Press => {
                        let done = self.handle_key_press(evt, &mut buffer)?;
                        if done {
                            break; // If we are done processing the line, then exit the loop and continue onwards!
                        }
                    }
                    _ => {} // Ignore non-key events
                }
            }
        }

        self.writer.execute(cursor::MoveToColumn(0))?; // Move the cursor back to the left-most column if it was ever displaced.

        Ok(buffer)
    }

    fn handle_key_press(
        &mut self,
        evt: KeyEvent,
        mut buffer: &mut String,
    ) -> std::io::Result<bool> {
        match evt {
            // Exit on Ctrl+C or Esc
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }
            | KeyEvent {
                code: KeyCode::Esc, ..
            } => {
                return Ok(true); // Exit the loop immediately
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
                return Ok(true);
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
        }

        Ok(false)
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
                *buffer = suggestion.to_string() + " ";
                write!(self.writer, "{}", buffer)?;
                self.writer.flush()?;
            } else {
                write!(self.writer, "\x07")?; // If the tab-completion could not be performed, ring the bell 🔔!
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
