// Library
use std::{env, path::PathBuf};

// -------------------------
// PATH ENVIRONMENT VARIABLE
// -------------------------

/// Find an executable in the `PATH` environment variable.
pub fn find_executable(name: &str) -> Option<String> {
    // Get the `PATH` environment variable
    let path = env::var("PATH").unwrap_or_default();

    // Split the `PATH` environment variable into a list of paths
    let paths: Vec<PathBuf> = env::split_paths(&path).collect();

    // Iterate over the paths and return the first executable found
    for path in paths {
        let executable = path.join(name);
        if executable.exists() {
            if let Some(executable) = executable.to_str() {
                return Some(executable.to_owned()); // Return the executable path
            } else {
                eprintln!("Failed to convert an executable path to a string");
                return None;
            }
        }
    }

    // Return `None` if no executable was found
    None
}
