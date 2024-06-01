// ----
// ECHO
// ----

/// Execute the echo command
/// This command will print the arguments to the screen
pub fn execute(args: Vec<&str>) {
    // Skip the first argument (the command name)
    let args = &args[1..];

    // Print the arguments to the screen
    println!("{}", args.join(" "));
}
