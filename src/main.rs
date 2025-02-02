// Modules
mod commands;
mod helpers;
mod parser;
mod shell;

// Library
use helpers::ansi::Colorable;
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
        eprintln!("Error: {}", e.red());
    }
}
