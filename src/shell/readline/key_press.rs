use std::io::Write;

use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyModifiers},
    ExecutableCommand,
};

impl super::ReadLine {
    pub(super) fn handle_key_press(
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

    /// Extended Tab-completion:
    /// - If there is progress (the longest common prefix of suggestions is longer than current buffer),
    ///   update the buffer.
    /// - If not, on first Tab press, ring the bell.
    /// - On second consecutive Tab press, print all suggestions.
    fn handle_tab_completion(&mut self, buffer: &mut String) -> std::io::Result<()> {
        // Aggregate suggestions from all completers.
        let mut suggestions = Vec::new();
        for completer in &self.completers {
            suggestions.extend(completer.complete(&buffer));
        }
        suggestions.sort();
        suggestions.dedup();

        if suggestions.is_empty() {
            // No suggestions: ring the bell.
            write!(self.writer, "\x07")?;
            self.writer.flush()?;
            self.tab_count = 0;
            return Ok(());
        }

        // Compute longest common prefix (LCP) among suggestions.
        let lcp = longest_common_prefix(&suggestions);

        if lcp.len() > buffer.len() {
            // There is progress; update buffer to LCP.
            let len = buffer.len() as u16;
            self.writer.execute(cursor::MoveLeft(len))?;
            write!(self.writer, "{}", " ".repeat(len as usize))?;
            self.writer.execute(cursor::MoveLeft(len))?;
            if suggestions.len() > 1 {
                *buffer = lcp;
            } else {
                *buffer = lcp + " ";
            }
            write!(self.writer, "{}", buffer)?;
            self.writer.flush()?;
            self.tab_count = 0;
        } else {
            // No progress: if multiple suggestions, use tab_count to decide.
            self.tab_count += 1;
            if self.tab_count == 1 {
                // First Tab press: ring the bell.
                write!(self.writer, "\x07")?;
                self.writer.flush()?;
            } else {
                // Second consecutive Tab press: print all suggestions.
                writeln!(self.writer)?;
                self.writer.execute(cursor::MoveToColumn(0))?;
                for suggestion in &suggestions {
                    write!(self.writer, "{}  ", suggestion)?;
                }
                writeln!(self.writer)?;
                self.writer.execute(cursor::MoveToColumn(0))?;
                // Reprint the prompt with the current buffer (assumed prompt "$ ").
                write!(self.writer, "$ {}", buffer)?;
                self.writer.flush()?;
                self.tab_count = 0;
            }
        }
        Ok(())
    }
}

/// Helper function: returns the longest common prefix among the provided strings.
fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = strings[0].clone();
    for s in strings.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                break;
            }
        }
    }
    prefix
}
