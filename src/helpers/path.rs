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
        // Check for name executable in each path
        let executable = path.join(name);
        if executable.exists() {
            return Some(executable.to_str().unwrap().to_owned());
        }
        // Check name.exe (for windows) while we are at it
        let executable = path.join(format!("{}.exe", name));
        if executable.exists() {
            return Some(executable.to_str().unwrap().to_owned());
        }
    }

    // Return `None` if no executable was found
    None
}
