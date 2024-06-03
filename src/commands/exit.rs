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
pub struct Exit {
    /// The name of the command
    name: String,
    /// A brief description of the command
    description: String,
    /// The instructions on how to use the command
    usage: String,
}

// Implement the `Default` trait for the `Exit` struct
impl Default for Exit {
    fn default() -> Self {
        Exit {
            name: String::from("exit"),
            description: String::from("Exit the shell with a status code"),
            usage: String::from("exit [exit_code]"),
        }
    }
}

// Implement the `ExecutableCommand` trait for the `Exit` struct.
impl super::ExecutableCommand for Exit {
    /// Execute the `exit` command.
    /// Exits the shell and returns an error code.
    /// ```sh
    /// $ exit 0 # Exit the shell with a status code of 0
    /// ```
    fn execute(&self, args: Vec<&str>) -> std::io::Result<()> {
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
