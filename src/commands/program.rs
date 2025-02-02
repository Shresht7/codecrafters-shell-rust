// Library
use std::io::Write;

// -------
// PROGRAM
// -------

/// The `Program` command is used to execute external programs.
/// These programs are not built-in commands but are external executables.
pub struct Program {
    /// The path to the program
    path: String,
}

impl Program {
    /// Instantiate a new `Program` with the given path.
    pub fn new(path: String) -> Self {
        Program { path }
    }
}

impl std::fmt::Display for Program {
    /// Implement the `Display` trait for the `Program` struct.
    /// This will allow us to print the `Program` struct using the `println!` macro.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

// Implement the `ExecutableCommand` trait for the `Program` struct.
impl super::ExecutableCommand for Program {
    /// Execute the program.
    fn execute(&self, args: Vec<String>, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Execute the program with the given arguments
        let base_path = std::path::PathBuf::from(&self.path);
        let base_path = base_path.file_name().unwrap();
        let output = std::process::Command::new(base_path)
            .args(&args[1..])
            .output()?;

        // Write the output to the standard output
        writer.write_all(&output.stdout)?;
        // Write the error output to the standard error
        std::io::stderr().write_all(&output.stderr)?;

        Ok(())
    }
}
