use std::{
    io::{self, BufWriter},
    time,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyEventKind},
    ExecutableCommand,
};

mod completer;
mod key_press;
mod raw_mode;

pub use completer::*;

pub(super) struct ReadLine {
    writer: BufWriter<io::Stdout>,
    poll_interval: time::Duration,
    completers: Vec<Box<dyn Completer>>,
    tab_count: u8,
}

impl Default for ReadLine {
    fn default() -> Self {
        Self {
            writer: BufWriter::new(std::io::stdout()),
            poll_interval: time::Duration::from_millis(100),
            completers: Vec::new(),
            tab_count: 0,
        }
    }
}

impl ReadLine {
    /// Register a new completer
    pub fn register_completer(&mut self, completer: Box<dyn Completer>) -> &mut Self {
        self.completers.push(completer);
        self
    }

    /// Convenience Helper Function: Set a default completer from a list of strings
    pub fn with_completions(&mut self, completions: Vec<String>) -> &mut Self {
        self.register_completer(Box::new(DefaultCompleter::new(completions)));
        self
    }

    pub(super) fn read(&mut self) -> std::io::Result<String> {
        let mut buffer = String::new(); // The buffer that represents the input

        // Enable terminal raw mode with our `RawModeGuard` that will automatically disable when it is dropped
        let _raw_mode = raw_mode::RawModeGuard::new()?;

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
}
