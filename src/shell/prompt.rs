use std::io::Write;

impl super::Shell {
    /// Renders the prompt to the screen
    pub(super) fn render_prompt(&mut self, prompt: &str) -> std::io::Result<()> {
        // Print the prompt
        write!(self.writer, "{}", prompt)?;

        // Flush the output to the screen so the prompt is displayed immediately.
        self.writer.flush()?;

        Ok(())
    }
}
