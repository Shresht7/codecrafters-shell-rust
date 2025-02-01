// ----
// ECHO
// ----

/// # The `echo` command.
/// This command will print the given arguments to the screen.
///
/// ## Example
///
/// ```sh
/// $ echo Hello World!
/// ```
/// ```output
/// Hello World!
/// ```
pub struct Echo;

// Implement the `CommandInfo` trait for the `Echo` struct.
impl super::CommandInfo for Echo {
    /// Get the name of the command.
    fn name(&self) -> String {
        String::from("echo")
    }

    /// Get the description of the command.
    fn description(&self) -> String {
        String::from("Print the given arguments to the screen")
    }

    /// Get the usage of the command.
    fn usage(&self) -> String {
        String::from("echo [arguments...]")
    }
}

// Implement the `ExecutableCommand` trait for the `Echo` struct.
impl super::ExecutableCommand for Echo {
    /// Execute the `echo` command.
    /// This command will print the arguments to the screen.
    /// ```sh
    /// $ echo Hello World!
    /// ```
    /// ```output
    /// Hello World!
    /// ```
    fn execute(&self, args: Vec<String>) -> std::io::Result<()> {
        // Skip the first argument (the command name)
        let args = &args[1..];

        // Print the arguments to the screen
        println!("{}", args.join(" "));

        Ok(())
    }
}
