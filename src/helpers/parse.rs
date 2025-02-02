// Library
use std::iter::Peekable;
use std::str::Chars;

/// Represents the various states the parser can be in
#[derive(Debug)]
enum ParseState {
    Normal,
    InSingleQuote,
    InDoubleQuote,
}

#[derive(Debug)]
/// A `Parser` struct that holds the state and context for parsing operations.
pub struct Parser<'a> {
    /// A collection to store the resulting arguments
    args: Vec<String>,
    /// A string representing the current token being processed
    current: String,
    /// The current state of the parser, represented by the `ParseState` enum
    state: ParseState,
    /// An iterator over the characters of the input string, allowing for peeking at the next character
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    /// Instantiate a new Parser with the initial conditions
    fn new(input: &str) -> Parser {
        Parser {
            args: Vec::new(),
            current: String::new(),
            state: ParseState::Normal,
            chars: input.trim().chars().peekable(),
        }
    }

    /// Parses an input string into a vector of arguments, handling quotes and escapes.
    pub fn parse(input: &str) -> Result<(Vec<String>, Option<String>, Option<String>), String> {
        let mut parser = Parser::new(input); // Initialize the parser

        // Iterate over the characters...
        while let Some(ch) = parser.chars.next() {
            parser.state = match parser.state {
                ParseState::Normal => parser.handle_normal(ch)?,
                ParseState::InSingleQuote => parser.handle_in_single_quote(ch),
                ParseState::InDoubleQuote => parser.handle_in_double_quote(ch)?,
            }
            // Update the parser state, as necessary
        }

        // Once the iteration is complete, put any remaining tokens in current as the final argument
        if !parser.current.is_empty() {
            parser.args.push(parser.current);
        }

        let (args, stdout_target, stderr_target) = extract_redirection(parser.args);

        // Return the resulting vector of arguments
        Ok((args, stdout_target, stderr_target))
    }

    /// Handles a character in the Normal state.
    /// Returns the new state after processing the character.
    fn handle_normal(&mut self, ch: char) -> Result<ParseState, String> {
        match ch {
            '\\' => {
                // Escape the next character if present.
                if let Some(escaped) = self.chars.next() {
                    self.current.push(escaped);
                } else {
                    return Err("Trailing backslash".into());
                }
                Ok(ParseState::Normal)
            }
            '\'' => Ok(ParseState::InSingleQuote),
            '"' => Ok(ParseState::InDoubleQuote),
            c if c.is_whitespace() => {
                if !self.current.is_empty() {
                    self.args.push(std::mem::take(&mut self.current));
                }
                Ok(ParseState::Normal)
            }
            _ => {
                self.current.push(ch);
                Ok(ParseState::Normal)
            }
        }
    }

    /// Handles a character in the InSingleQuote state.
    /// Returns the new state after processing the character.
    fn handle_in_single_quote(&mut self, ch: char) -> ParseState {
        if ch == '\'' {
            ParseState::Normal
        } else {
            self.current.push(ch);
            ParseState::InSingleQuote
        }
    }

    /// Handles a character in the InDoubleQuote state.
    /// Returns the new state after processing the character.
    fn handle_in_double_quote(&mut self, ch: char) -> Result<ParseState, String> {
        match ch {
            '"' => Ok(ParseState::Normal),
            '\\' => {
                // Only escape certain characters within double quotes.
                if let Some(&next_ch) = self.chars.peek() {
                    match next_ch {
                        '\\' | '"' | '$' | '\n' => {
                            self.current.push(self.chars.next().unwrap());
                        }
                        _ => {
                            self.current.push('\\');
                        }
                    }
                    Ok(ParseState::InDoubleQuote)
                } else {
                    Err("Trailing backslash in double quotes".into())
                }
            }
            _ => {
                self.current.push(ch);
                Ok(ParseState::InDoubleQuote)
            }
        }
    }
}

// ----------------
// HELPER FUNCTIONS
// ----------------

/// Given a vector of tokens, extracts redirection targets and returns a tuple:
/// (remaining arguments, stdout target, stderr target)
fn extract_redirection(tokens: Vec<String>) -> (Vec<String>, Option<String>, Option<String>) {
    let mut args = Vec::new();
    let mut stdout_target: Option<String> = None;
    let mut stderr_target: Option<String> = None;

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            ">" | "1>" => {
                if i + 1 < tokens.len() {
                    stdout_target = Some(tokens[i + 1].clone());
                    i += 2; // Skip both the redirection operator and the filename.
                } else {
                    // If there’s no filename, just break or decide how to handle the error.
                    break;
                }
            }
            "2>" => {
                if i + 1 < tokens.len() {
                    stderr_target = Some(tokens[i + 1].clone());
                    i += 2;
                } else {
                    break;
                }
            }
            _ => {
                args.push(tokens[i].clone());
                i += 1;
            }
        }
    }
    (args, stdout_target, stderr_target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "command arg1 arg2";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_no_args() {
        let input = "command";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_empty() {
        let input = "";
        let actual = Parser::parse(input).unwrap();
        let expected: Vec<&str> = vec![];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_with_quoted_args() {
        let input = "command \"arg1 arg2\"";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1 arg2"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_with_single_quoted_args() {
        let input = "command 'arg1 arg2'";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1 arg2"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_with_mixed_quotes() {
        let input = "command \"arg1 'arg2'\"";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1 'arg2'"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_with_escaped_quotes() {
        let input = "command \\\"arg1\\\" arg2";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "\"arg1\"", "arg2"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_with_multiple_spaces() {
        let input = "command    arg1     arg2";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_with_trailing_spaces() {
        let input = "command arg1 arg2   ";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_with_leading_spaces() {
        let input = "   command arg1 arg2";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_with_escaped_backslash() {
        let input = "command arg1 \\\\arg2";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1", "\\arg2"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_with_nested_quotes() {
        let input = "command \"arg1 'nested arg2'\"";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1 'nested arg2'"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_with_unclosed_quotes() {
        let input = "command \"arg1 arg2";
        let actual = Parser::parse(input).unwrap();
        // In this implementation, unclosed quotes are accepted and treated as literal.
        let expected = vec!["command", "arg1 arg2"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_input_with_special_characters() {
        let input = "command arg1!@# arg2$%^";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1!@#", "arg2$%^"];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_redirection_with_both() {
        let input = "ls -l > out.txt 2> err.txt";
        let tokens: Vec<String> = input.split_whitespace().map(String::from).collect();
        let (args, stdout_target, stderr_target) = extract_redirection(tokens);
        assert_eq!(args, vec!["ls", "-l"]);
        assert_eq!(stdout_target, Some("out.txt".to_string()));
        assert_eq!(stderr_target, Some("err.txt".to_string()));
    }

    #[test]
    fn test_parse_redirection_stdout_only() {
        let input = "echo Hello World > output.txt";
        let tokens: Vec<String> = input.split_whitespace().map(String::from).collect();
        let (args, stdout_target, stderr_target) = extract_redirection(tokens);
        assert_eq!(args, vec!["echo", "Hello", "World"]);
        assert_eq!(stdout_target, Some("output.txt".to_string()));
        assert_eq!(stderr_target, None);
    }

    #[test]
    fn test_parse_redirection_stderr_only() {
        let input = "grep foo file.txt 2> errors.log";
        let tokens: Vec<String> = input.split_whitespace().map(String::from).collect();
        let (args, stdout_target, stderr_target) = extract_redirection(tokens);
        assert_eq!(args, vec!["grep", "foo", "file.txt"]);
        assert_eq!(stdout_target, None);
        assert_eq!(stderr_target, Some("errors.log".to_string()));
    }

    #[test]
    fn test_parse_redirection_without_any() {
        let input = "pwd";
        let tokens: Vec<String> = input.split_whitespace().map(String::from).collect();
        let (args, stdout_target, stderr_target) = extract_redirection(tokens);
        assert_eq!(args, vec!["pwd"]);
        assert_eq!(stdout_target, None);
        assert_eq!(stderr_target, None);
    }
}
