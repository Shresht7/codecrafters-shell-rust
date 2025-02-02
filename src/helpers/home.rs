// Library
use std::{env, path::PathBuf};

// ---------
// HOME PATH
// ---------

// Get the home path from the `HOME` or `HOMEPATH` environment variable
pub fn get() -> Option<PathBuf> {
    // Get the home path from the `HOME` environment variable
    let env_home_path = env::var("HOME").ok();

    // If the `HOME` environment variable is set, return it
    if let Some(home_path) = env_home_path {
        return Some(PathBuf::from(home_path));
    }

    // If the `HOME` environment variable is not set, check the `HOMEPATH` variable
    let env_home_path = env::var("HOMEPATH").ok();

    // If the `HOMEPATH` environment variable is set, return it
    if let Some(home_path) = env_home_path {
        return Some(PathBuf::from(home_path));
    }

    // Return `None` if no home path is found
    None
}
