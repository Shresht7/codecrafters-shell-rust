// ----
// EXIT
// ----

/// # The `exit` command.
/// This command will exit the shell with a status code.
///
/// ## Example
///
/// ```sh
/// $ exit 0
/// ```
pub struct Exit;

// Implement the `CommandInfo` trait for the `Exit` struct.
impl super::CommandInfo for Exit {
    /// Get the name of the command.
    fn name(&self) -> String {
        String::from("exit")
    }

    /// Get the description of the command.
    fn description(&self) -> String {
        String::from("Exit the shell with a status code")
    }

    /// Get the usage of the command.
    fn usage(&self) -> String {
        String::from("exit [status_code]")
    }
}

// Implement the `ExecutableCommand` trait for the `Exit` struct.
impl super::ExecutableCommand for Exit {
    /// Execute the `exit` command.
    /// Exits the shell and returns an error code.
    /// ```sh
    /// $ exit 0 # Exit the shell with a status code of 0
    /// ```
    fn execute(&self, args: Vec<String>, _writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Parse the exit code from the arguments
        let exit_code = if args.len() > 1 {
            args[1].parse::<i32>().unwrap_or(1) // Default to 1 if the exit code is invalid
        } else {
            0 // Default to 0 if no exit code is provided
        };
        // Exit the shell
        std::process::exit(exit_code);
    }
}
