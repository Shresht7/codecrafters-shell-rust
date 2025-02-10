use std::{
    io::{self, BufWriter, Write},
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
    prompt: String,
    completers: Vec<Box<dyn Completer>>,
    tab_count: u8,
    writer: BufWriter<io::Stdout>,
    poll_interval: time::Duration,
}

impl Default for ReadLine {
    fn default() -> Self {
        Self {
            prompt: String::from("$ "),
            completers: Vec::new(),
            tab_count: 0,
            poll_interval: time::Duration::from_millis(100),
            writer: BufWriter::new(std::io::stdout()),
        }
    }
}

impl ReadLine {
    /// Set the prompt
    pub fn with_prompt(&mut self, prompt: &str) -> &mut Self {
        self.prompt = prompt.to_owned();
        self
    }

    /// Render the prompt to the screen
    fn render_prompt(&mut self) -> std::io::Result<()> {
        write!(self.writer, "{}", self.prompt)?;
        self.writer.flush()?;
        Ok(())
    }

    /// Register a new completer
    pub fn register_completer(&mut self, completer: Box<dyn Completer>) -> &mut Self {
        self.completers.push(completer);
        self
    }

    /// A convenience helper function to create a [`DefaultCompleter`] from a list of strings
    pub fn with_completions(&mut self, completions: Vec<String>) -> &mut Self {
        self.register_completer(Box::new(DefaultCompleter::new(completions)));
        self
    }

    /// Read the next line the user inputs
    pub(super) fn read(&mut self) -> std::io::Result<String> {
        let mut buffer = String::new(); // The buffer that represents the input

        // Enable terminal raw mode with our `RawModeGuard` that will automatically disable when it is dropped
        let _raw_mode = raw_mode::RawModeGuard::new()?;

        // Render the prompt
        self.render_prompt()?;

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

        // Move the cursor back to the left-most column if it somehow ends up in a weird place.
        self.writer.execute(cursor::MoveToColumn(0))?;

        Ok(buffer)
    }
}
