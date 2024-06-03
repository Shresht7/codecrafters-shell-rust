// -----------------------
// PRINT WORKING DIRECTORY
// -----------------------

/// # The `pwd` command.
/// This command will print the current working directory to the screen.
///
/// ## Example
///
/// ```sh
/// $ pwd
/// ```
/// ```output
/// /path/to/current/directory
/// ```
pub struct PWD;

// Implement the `ExecutableCommand` trait for the `PWD` struct.
impl super::ExecutableCommand for PWD {
    /// Execute the `pwd` command.
    /// This command will print the current working directory to the screen.
    /// ```sh
    /// $ pwd
    /// ```
    /// ```output
    /// /path/to/current/directory
    /// ```
    fn execute(&self, _args: Vec<&str>) -> std::io::Result<()> {
        // Get the current working directory
        let cwd = std::env::current_dir()?;

        // Print the current working directory
        println!("{}", cwd.display());

        Ok(())
    }
}
