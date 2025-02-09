use crate::commands::Command;

use std::io;

impl super::Shell {
    /// Handles command execution
    pub(super) fn execute_command(
        &mut self,
        args: Vec<String>,
        out_target: Option<(String, bool)>,
        err_target: Option<(String, bool)>,
    ) -> io::Result<()> {
        // Decide the writer for stdout.
        let mut out_writer: Box<dyn io::Write> = if let Some((filename, append)) = out_target {
            let file = std::fs::OpenOptions::new()
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
            let file = std::fs::OpenOptions::new()
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
}
