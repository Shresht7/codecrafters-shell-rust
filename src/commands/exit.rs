// ------------
// EXIT COMMAND
// ------------

/// Exits the shell and returns an error code
pub fn execute(args: Vec<&str>) {
    // Parse the exit code from the arguments
    let exit_code = if args.len() > 1 {
        args[1].parse::<i32>().unwrap_or(0)
    } else {
        0
    };
    // Exit the shell
    std::process::exit(exit_code);
}
