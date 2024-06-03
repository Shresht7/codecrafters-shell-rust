// ----
// TYPE
// ----

/// # The `type` command.
/// This command will print the type of the given argument.
/// This can be used to determine if the argument is a built-in command, an alias, or an external program.
pub struct Type;

// note: commenting this out because warnings make the code-crafters tests fail
// pub struct Type {
//     /// The name of the command
//     name: String,
//     /// A brief description of the command
//     description: String,
//     /// The instructions on how to use the command
//     usage: String,
// }

// // Implement the `Default` trait for the `Type` struct
// impl Default for Type {
//     fn default() -> Self {
//         Type {
//             name: String::from("type"),
//             description: String::from("Print the type of the given argument"),
//             usage: String::from("type [argument]"),
//         }
//     }
// }

// Implement the `ExecutableCommand` trait for the `Type` struct.
impl super::ExecutableCommand for Type {
    /// Execute the `type` command.
    /// This command will print the type of the given argument.
    /// This can be used to determine if the argument is a built-in command, an alias, or an external program.
    /// ```sh
    /// $ type echo
    /// ```
    /// ```output
    /// echo is a shell builtin
    /// ```
    fn execute(&self, args: Vec<&str>) -> std::io::Result<()> {
        // Skip the first argument (the command name)
        let args = &args[1..];

        // Get the first argument
        if let Some(arg) = args.first() {
            match arg.parse::<super::Command>() {
                Ok(super::Command::Builtin(_)) => {
                    println!("{} is a shell builtin", arg);
                }
                Ok(super::Command::Program(path)) => {
                    println!("{} is {}", arg, path);
                }
                Ok(super::Command::Unknown) => {
                    println!("{} not found", arg);
                }
                Err(_) => {
                    println!("{} is not a valid command", arg);
                }
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No argument provided",
            ))?;
        }

        Ok(())
    }
}
