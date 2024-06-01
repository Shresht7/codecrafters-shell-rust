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

    // Start the Shell's Read-Eval-Print Loop (REPL)
    shell.run();
}
