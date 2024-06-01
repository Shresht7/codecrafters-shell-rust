// Modules
mod commands;
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
    if let Err(e) = shell.run() {
        eprintln!("\u{001b}[31mError: {}\u{001b}[0m", e);
    }
}
