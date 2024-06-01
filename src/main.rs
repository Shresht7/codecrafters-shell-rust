// Modules
mod shell;

// Library
use shell::Shell;

// ----
// MAIN
// ----

/// The main entry point of the application
fn main() {
    // Initialize the Shell
    let mut shell = Shell::default();

    // Render the prompt to the screen
    shell.render_prompt("$ ");

    // Wait for user input and read it into a variable
    let input = shell.read_input();

    // Split the input into a vector
    let args: Vec<&str> = shell.parse_input(&input);

    // Act on the command-name
    shell.execute_command(args);
}
