pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

#[derive(Debug)]
pub enum Error {
    EndOfLine,
    Character(usize),
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }

    pub fn advance(&mut self) {
        self.cursor += 1
    }

    /// Returns the next character without advancing the cursor.
    /// AKA "lookahead"
    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    /// Returns the next character (if available) and advances the cursor.
    pub fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;

                Some(character)
            }
            None => None,
        }
    }

    /// Returns true if the char matches the logic at the current cursor position,
    /// and advances the cursor.
    /// Otherwise, returns false leaving the cursor unchanged.
    pub fn take_with_validator(&mut self, validator: &dyn Fn(&char) -> bool) -> bool {
        match self.characters.get(self.cursor) {
            Some(character) => {
                if validator(character) {
                    self.cursor += 1;

                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Returns true if the `target` is found at the current cursor position,
    /// and advances the cursor.
    /// Otherwise, returns false leaving the cursor unchanged.
    pub fn take_char(&mut self, target: &char) -> bool {
        let t = |c: &char| -> bool {
            return c == target;
        };
        return self.take_with_validator(&t);
    }

    /// Returns a string
    pub fn take_if_until(
        &mut self,
        validator: &dyn Fn(&char) -> bool,
        terminal: &str,
    ) -> Result<String, Error> {
        // Buf to return
        let mut out = String::new();

        // Backup to restore
        let original_cursor = self.cursor;

        // First character of terminal sequence to stop on
        let term = &terminal.chars().nth(0).expect("invalid terminal string");

        while self.peek().is_some() && validator(self.peek().unwrap()) {
            let char = self.pop();
            out.push(*char.unwrap());
        }

        // Determining point; do we have a terminal value at the end here?
        let char = self.peek();

        if char.is_some() && char.unwrap() == term {
            // We've extracted up to the first terminal character
            self.advance();

            // Early exit
            if terminal.len() == 1 {
                return Ok(out);
            }

            // Iterate but skip first, since we already checked
            let mut it = terminal.chars();
            it.next();

            // Loop through remaining chars(
            while let Some(t) = it.next() {
                let next = self.pop();

                // If we 're out of text, we didn't match
                if next.is_none() || &t != next.unwrap() {
                    // Reset cursor, no match
                    self.cursor = original_cursor;
                    return Err(Error::EndOfLine);
                }
            }
            // The fall through means all terminal chars were matched
            return Ok(out);
        }

        // Reset cursor, no match
        self.cursor = original_cursor;
        return Err(Error::EndOfLine);
    }
}

fn parse(s: &str) -> String {
    let mut scanner = Scanner::new(s);
    let mut out = String::new();

    let alt_title_validator = |c: &char| -> bool { c.is_alphabetic() || c == &' ' };
    let link_validator = |x: &char| -> bool { return x.is_alphabetic() };

    while let Some(&c) = scanner.pop() {
        match c {
            '{' => {
                // We need double consumption so keep track of cursor
                let original_cursor = scanner.cursor;
                let mut reset = false;

                // Check for title
                let title = scanner.take_if_until(&alt_title_validator, "}[[");
                if title.is_ok() {
                    // Check for link
                    let link = scanner.take_if_until(&alt_title_validator, "]]");
                    if link.is_ok() {
                        // Add to buffer
                        out.push_str(
                            format!("[{}]({}.html)", title.unwrap(), link.unwrap(),).as_str(),
                        );
                    } else {
                        reset = true;
                    }
                } else {
                    reset = true;
                }

                if reset {
                    // No match, reset cursor
                    scanner.cursor = original_cursor;
                    out.push(c);
                }
            }
            '[' => {
                // Second match
                if scanner.take_char(&c) {
                    // Check for link
                    match scanner.take_if_until(&link_validator, "]]") {
                        Ok(val) => out.push_str(format!("[{}]({}.html)", val, val).as_str()),
                        _ => {
                            // Inital match and second take
                            out.push(c);
                            out.push(c);
                        }
                    }
                } else {
                    // No second match; just add first match
                    out.push(c);
                }
            }
            _ => {
                out.push(c);
            }
        }
    }
    return out;
}

#[cfg(test)]
mod tests {
    // import namespace above here
    use super::*;

    #[test]
    fn test_parser_take_if_until_single_term() {
        let mut s = Scanner::new("asdf ");
        let logic = |x: &char| -> bool { return x.is_alphabetic() };
        let r = s.take_if_until(&logic, " ");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), String::from("asdf"));
    }

    #[test]
    fn test_parser_take_if_until_double_term() {
        let mut s = Scanner::new("asdf]]");
        let logic = |x: &char| -> bool { return x.is_alphabetic() };
        let r = s.take_if_until(&logic, "]]");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), String::from("asdf"));
    }

    #[test]
    fn test_parser_take_if_until_many_term() {
        let mut s = Scanner::new("asdf1234567890");
        let logic = |x: &char| -> bool { return x.is_alphabetic() };
        let r = s.take_if_until(&logic, "1234567890");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), String::from("asdf"));
    }

    #[test]
    fn test_parser_take_if_until_error() {
        let logic = |x: &char| -> bool { return x.is_alphabetic() };
        let mut s = Scanner::new(" asdf]]");
        let r = s.take_if_until(&logic, "]]");
        assert!(r.is_err());
    }

    #[test]
    fn test_parser_parse_basic() {
        assert_eq!(parse("[[something]]"), "[something](something.html)");
    }

    #[test]
    fn test_parser_parse_basic_noop() {
        assert_eq!(parse("[something]"), "[something]");
    }

    #[test]
    fn test_parser_parse_unique_title() {
        assert_eq!(parse("{test}[[foo]]"), "[test](foo.html)");
    }

    #[test]
    fn test_parser_parse_ignore_partial_link() {
        let s = "something [[test";
        assert_eq!(parse(s), s);
    }

    #[test]
    fn test_parser_parse_ignore_partial_link_alt() {
        let s = "something {alt";
        assert_eq!(parse(s), s);
    }

    // Its trash time, bitches
    #[test]
    fn test_parser_parse_ignore_trash_1() {
        let s = "something {alt}[[test";
        assert_eq!(parse(s), s);
    }

    #[test]
    fn test_parser_parse_ignore_trash_2() {
        let s = "something {alt}[[test]";
        assert_eq!(parse(s), s);
    }

    #[test]
    fn test_parser_parse_ignore_trash_3() {
        let s = "something {alt}[[test}]]";
        assert_eq!(parse(s), s);
    }

    #[test]
    fn test_parser_parse_ignore_trash_4() {
        let s = "{test this}[[seems like it]{cshould][{}[{]} work";
        assert_eq!(parse(s), s);
    }
}
