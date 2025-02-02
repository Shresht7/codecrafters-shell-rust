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

impl super::CommandInfo for PWD {
    /// Get the name of the command.
    fn name(&self) -> String {
        String::from("pwd")
    }

    /// Get the description of the command.
    fn description(&self) -> String {
        String::from("Print the current working directory to the screen")
    }

    /// Get the usage of the command.
    fn usage(&self) -> String {
        String::from("pwd")
    }
}

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
    fn execute(&self, _args: Vec<String>, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Get the current working directory
        let cwd = std::env::current_dir()?;

        // Print the current working directory
        writeln!(writer, "{}", cwd.display())?;

        Ok(())
    }
}
