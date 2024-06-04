// ----------------
// CHANGE DIRECTORY
// ----------------

/// # The `cd` command.
/// This command will change the current working directory.
///
/// ## Example
///
/// ```sh
/// $ cd /path/to/directory
/// ```
pub struct CD;

// Implement the `ExecutableCommand` trait for the `CD` struct.
impl super::ExecutableCommand for CD {
    /// Execute the `cd` command.
    /// This command will change the current working directory.
    /// ```sh
    /// $ cd /path/to/directory
    /// ```
    fn execute(&self, args: Vec<&str>) -> std::io::Result<()> {
        // Get the path from the arguments
        let path = match args.get(1) {
            Some(path) => std::path::Path::new(path),
            None => {
                eprintln!("cd: missing argument");
                return Ok(());
            }
        };

        // Change the current working directory
        if let Err(_) = std::env::set_current_dir(path) {
            eprintln!("{}: No such file or directory", path.display());
        }

        Ok(())
    }
}
