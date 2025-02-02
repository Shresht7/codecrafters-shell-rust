// -------
// UNKNOWN
// -------

/// The `Unknown` command is used to print an error message for unknown commands.
/// These commands are neither built-in commands nor external programs.
/// The `Unknown` command is used as a fallback.
pub struct Unknown;

// Implement the `ExecutableCommand` trait for the `Unknown` struct.
// This will allow use to "execute" the `Unknown` command so that it satisfies the `ExecutableCommand` trait.
impl super::ExecutableCommand for Unknown {
    /// Print an error message for unknown commands.
    fn execute<T>(
        &self,
        args: Vec<String>,
        _out_writer: &mut T,
        err_writer: &mut T,
    ) -> std::io::Result<()>
    where
        T: std::io::Write,
    {
        if let Some(command) = args.get(0) {
            writeln!(err_writer, "{}: command not found", command)?;
        }
        Ok(())
    }
}
