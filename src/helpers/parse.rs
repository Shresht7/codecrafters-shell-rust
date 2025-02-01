pub struct Parser {
    word: String,
    in_single_quotes: bool,
    in_double_quotes: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            word: String::new(),
            in_single_quotes: false,
            in_double_quotes: false,
        }
    }

    pub fn parse(input: &str) -> Vec<String> {
        let mut p = Parser::new();
        p._parse(input)
    }

    /// Parses the input into a vector of arguments
    pub fn _parse(&mut self, input: &str) -> Vec<String> {
        let mut args = Vec::new(); // Vector to store the resulting args

        let mut chars = input.trim().chars().peekable(); // Iterator to walk over
        while let Some(ch) = chars.next() {
            match ch {
                '\'' => self.handle_single_quote(ch),
                '"' => self.handle_double_quotes(ch),
                '\\' => self.handle_backslash(&mut chars),
                ' ' => self.handle_space(&mut args),
                ch => self.word.push(ch),
            }
        }

        if !self.word.is_empty() {
            args.push(self.word.clone()); // Push any remaining word after the loop onto args
        }

        args
    }

    fn handle_single_quote(&mut self, ch: char) {
        if !self.in_double_quotes {
            self.in_single_quotes = !self.in_single_quotes;
        } else {
            self.word.push(ch);
        }
    }

    fn handle_double_quotes(&mut self, ch: char) {
        if !self.in_single_quotes {
            self.in_double_quotes = !self.in_double_quotes;
        } else {
            self.word.push(ch);
        }
    }

    fn handle_backslash(&mut self, chars: &mut std::iter::Peekable<std::str::Chars<'_>>) {
        if self.in_single_quotes {
            self.word.push('\\');
        } else if self.in_double_quotes {
            if let Some(c) = chars.peek() {
                if c == &'\\' || c == &'$' || c == &'\n' || c == &'"' {
                    self.word.push(chars.next().unwrap());
                } else {
                    self.word.push(c.clone());
                }
            }
        } else if !self.in_single_quotes && !self.in_double_quotes {
            if let Some(c) = chars.next() {
                self.word.push(c);
            }
        } else {
            self.word.push('\\');
        }
    }

    fn handle_space(&mut self, args: &mut Vec<String>) {
        if !self.word.is_empty() {
            if !self.in_single_quotes && !self.in_double_quotes {
                args.push(self.word.clone());
                self.word.clear();
            } else {
                self.word.push(' ');
            }
        }
    }
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
        let actual = parse::Parser::parse(input);
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_no_args() {
        let input = "command";
        let actual = parse::Parser::parse(input);
        let expected = vec!["command"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_empty() {
        let input = "";
        let actual = parse::Parser::parse(input);
        let expected: Vec<&str> = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_quoted_args() {
        let input = "command \"arg1 arg2\"";
        let actual = parse::Parser::parse(input);
        let expected = vec!["command", "arg1 arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_single_quoted_args() {
        let input = "command 'arg1 arg2'";
        let actual = parse::Parser::parse(input);
        let expected = vec!["command", "arg1 arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_mixed_quotes() {
        let input = "command \"arg1 'arg2'\"";
        let actual = parse::Parser::parse(input);
        let expected = vec!["command", "arg1 'arg2'"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_escaped_quotes() {
        let input = "command \\\"arg1\\\" arg2";
        let actual = parse::Parser::parse(input);
        let expected = vec!["command", "\"arg1\"", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_multiple_spaces() {
        let input = "command    arg1     arg2";
        let actual = parse::Parser::parse(input);
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_trailing_spaces() {
        let input = "command arg1 arg2   ";
        let actual = parse::Parser::parse(input);
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input_with_leading_spaces() {
        let input = "   command arg1 arg2";
        let actual = parse::Parser::parse(input);
        let expected = vec!["command", "arg1", "arg2"];
        assert_eq!(actual, expected);
    }
}
