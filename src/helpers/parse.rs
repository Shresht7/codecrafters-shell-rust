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
    pub fn parse(input: &str) -> Result<Vec<String>, String> {
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

        // Return the resulting vector of arguments
        Ok(parser.args)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "command arg1 arg2";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_no_args() {
        let input = "command";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_empty() {
        let input = "";
        let actual = Parser::parse(input).unwrap();
        let expected: Vec<&str> = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_quoted_args() {
        let input = "command \"arg1 arg2\"";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1 arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_single_quoted_args() {
        let input = "command 'arg1 arg2'";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1 arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_mixed_quotes() {
        let input = "command \"arg1 'arg2'\"";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1 'arg2'"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_escaped_quotes() {
        let input = "command \\\"arg1\\\" arg2";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "\"arg1\"", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_multiple_spaces() {
        let input = "command    arg1     arg2";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_trailing_spaces() {
        let input = "command arg1 arg2   ";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_leading_spaces() {
        let input = "   command arg1 arg2";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_escaped_backslash() {
        let input = "command arg1 \\\\arg2";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1", "\\arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_nested_quotes() {
        let input = "command \"arg1 'nested arg2'\"";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1 'nested arg2'"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_unclosed_quotes() {
        let input = "command \"arg1 arg2";
        let actual = Parser::parse(input).unwrap();
        // In this implementation, unclosed quotes are accepted and treated as literal.
        let expected = vec!["command", "arg1 arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_special_characters() {
        let input = "command arg1!@# arg2$%^";
        let actual = Parser::parse(input).unwrap();
        let expected = vec!["command", "arg1!@#", "arg2$%^"];
        assert_eq!(actual, expected);
    }
}
