/// A trait for generating completion suggestions based on current input
pub trait Completer {
    /// Returns a list of possible completions for the given input
    fn complete(&self, input: &str) -> Vec<String>;
}

/// A basic implementation of the [`Completer`] trait that matches on the input prefix
pub struct DefaultCompleter {
    completions: Vec<String>,
}

impl DefaultCompleter {
    pub fn new(completions: Vec<String>) -> Self {
        DefaultCompleter { completions }
    }
}

impl Completer for DefaultCompleter {
    fn complete(&self, input: &str) -> Vec<String> {
        self.completions
            .iter()
            .filter(|cmd| cmd.starts_with(input))
            .cloned()
            .collect()
    }
}
