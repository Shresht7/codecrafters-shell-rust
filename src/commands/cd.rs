// Library
use crate::helpers;
use std::path::PathBuf;

// ----------------
// CHANGE DIRECTORY
// ----------------

/// # The `cd` command.
/// This command will change the current working directory.
///
/// ## Example
///
/// ```sh
/// $ cd /path/to/directory
/// ```
pub struct CD;

// Implement the `CommandInfo` trait for the `CD` struct.
impl super::CommandInfo for CD {
    /// Get the name of the command.
    fn name(&self) -> String {
        String::from("cd")
    }

    /// Get the description of the command.
    fn description(&self) -> String {
        String::from("Change the current working directory")
    }

    /// Get the usage of the command.
    fn usage(&self) -> String {
        String::from("cd [path]")
    }
}

// Implement the `ExecutableCommand` trait for the `CD` struct.
impl super::ExecutableCommand for CD {
    /// Execute the `cd` command.
    /// This command will change the current working directory.
    /// ```sh
    /// $ cd /path/to/directory
    /// ```
    fn execute(&self, args: Vec<String>) -> std::io::Result<()> {
        // Get the path from the arguments
        let path = match args.get(1) {
            Some(arg) => {
                if arg.starts_with("~") {
                    let home_path = helpers::home::get().expect("cd: HOME path not found");
                    let path = PathBuf::from(arg).components().skip(1).collect::<PathBuf>();
                    home_path.join(path)
                } else {
                    PathBuf::from(arg)
                }
            }
            None => {
                eprintln!("cd: missing argument");
                return Ok(());
            }
        };

        // Change the current working directory
        if let Err(_) = std::env::set_current_dir(&path) {
            eprintln!("{}: No such file or directory", path.display());
        }

        Ok(())
    }
}
