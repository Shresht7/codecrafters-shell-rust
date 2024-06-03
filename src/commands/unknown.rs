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
    fn execute(&self, args: Vec<&str>) -> std::io::Result<()> {
        let command = args.get(0).unwrap_or(&"");
        eprintln!("{}: command not found", command);
        Ok(())
    }
}
