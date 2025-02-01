/// Parses the input into a vector of arguments
pub fn input(input: &str) -> Vec<String> {
    let mut args = Vec::new(); // Vector to store the resulting args

    let mut word = String::new(); // Bucket to store the current word
    let mut in_single_quotes = false; // Boolean indicating whether we are currently in a single-quoted string
    let mut in_double_quotes = false; // Boolean indicating whether we are currently in a double-quoted string
    let mut chars = input.trim().chars(); // Iterator to walk over
    while let Some(ch) = chars.next() {
        match ch {
            '\'' => {
                if !in_double_quotes {
                    in_single_quotes = !in_single_quotes;
                } else {
                    word.push(ch);
                }
            }
            '"' => {
                if !in_single_quotes {
                    in_double_quotes = !in_double_quotes;
                } else {
                    word.push(ch);
                }
            }
            '\\' => {
                if in_single_quotes {
                    word.push('\\');
                } else if !in_single_quotes && !in_double_quotes {
                    if let Some(c) = chars.next() {
                        word.push(c);
                    }
                } else {
                    word.push('\\');
                }
            }
            ' ' => {
                if !word.is_empty() {
                    if !in_single_quotes && !in_double_quotes {
                        args.push(word.clone());
                        word.clear();
                    } else {
                        word.push(' ');
                    }
                }
            }
            ch => word.push(ch),
        }
    }
    if !word.is_empty() {
        args.push(word.clone()); // Push any remaining word after the loop onto args
    }

    args
}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use crate::helpers::parse;

    #[test]
    fn test_parse_input() {
        let input = "command arg1 arg2";
        let actual = parse::input(input);
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_no_args() {
        let input = "command";
        let actual = parse::input(input);
        let expected = vec!["command"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_empty() {
        let input = "";
        let actual = parse::input(input);
        let expected: Vec<&str> = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_quoted_args() {
        let input = "command \"arg1 arg2\"";
        let actual = parse::input(input);
        let expected = vec!["command", "arg1 arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_single_quoted_args() {
        let input = "command 'arg1 arg2'";
        let actual = parse::input(input);
        let expected = vec!["command", "arg1 arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_mixed_quotes() {
        let input = "command \"arg1 'arg2'\"";
        let actual = parse::input(input);
        let expected = vec!["command", "arg1 'arg2'"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_escaped_quotes() {
        let input = "command \\\"arg1\\\" arg2";
        let actual = parse::input(input);
        let expected = vec!["command", "\"arg1\"", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_multiple_spaces() {
        let input = "command    arg1     arg2";
        let actual = parse::input(input);
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_trailing_spaces() {
        let input = "command arg1 arg2   ";
        let actual = parse::input(input);
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_leading_spaces() {
        let input = "   command arg1 arg2";
        let actual = parse::input(input);
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }
}
