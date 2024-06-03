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

// note: commenting this out because warnings make the code-crafters tests fail
// pub struct Echo {
//     /// The name of the command
//     name: String,
//     /// A brief description of the command
//     description: String,
//     /// The instructions on how to use the command
//     usage: String,
// }

// // Implement the `Default` trait for the `Echo` struct
// impl Default for Echo {
//     fn default() -> Self {
//         Echo {
//             name: String::from("echo"),
//             description: String::from("Print the given arguments to the screen"),
//             usage: String::from("echo [arguments...]"),
//         }
//     }
// }

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
    fn execute(&self, args: Vec<&str>) -> std::io::Result<()> {
        // Skip the first argument (the command name)
        let args = &args[1..];

        // Print the arguments to the screen
        println!("{}", args.join(" "));

        Ok(())
    }
}
