pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
    output: String,
}

pub enum Action<T> {
    /// If next iteration returns None, return T without advancing
    /// the cursor.
    Request(T),

    /// If the next iteration returns None, return None without advancing
    /// the cursor.
    Require,

    /// Immediately advance the cursor and return T.
    Return(T),
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
            output: String::new(),
        }
    }

    /// Returns the current cursor. Useful for reporting errors.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns the next character without advancing the cursor.
    /// AKA "lookahead"
    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    /// Returns true if further progress is not possible.
    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
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
        let mut matched_terminal = false;

        'taker: while self.peek().is_some() && validator(self.peek().unwrap()) {
            let char = self.pop();

            if char.is_some() && char.unwrap() == term {
                matched_terminal = true;
                break 'taker;
            }

            out.push(*char.unwrap());
        }

        if matched_terminal {
            // We've extracted up to the first terminal character
            if terminal.len() == 1 {
                return Ok(out);
            }

            // Iterate but skip first
            let mut it = terminal.chars();
            it.next();

            // Loop through remaining chars(
            while let Some(t) = it.next() {
                let next = self.pop();

                // If we 're out of text, we didn't match
                if next.is_none() || &t != next.unwrap() {
                    // Advance one and continue from where we started
                    self.cursor = original_cursor + 1;
                    return Err(Error::EndOfLine);
                }
            }
            // The fall through means all terminal chars were matched
            return Ok(out);
        }

        // Advance one and continue from where we started
        self.cursor = original_cursor + 1;
        return Err(Error::EndOfLine);
    }

    pub fn scan<T>(&mut self, cb: impl Fn(&str) -> Option<Action<T>>) -> Result<Option<T>, Error> {
        let mut sequence = String::new();
        let mut require = false;
        let mut request = None;

        loop {
            match self.characters.get(self.cursor) {
                Some(target) => {
                    sequence.push(*target);

                    match cb(&sequence) {
                        Some(Action::Return(result)) => {
                            self.cursor += 1;
                            break Ok(Some(result));
                        }
                        Some(Action::Request(result)) => {
                            self.cursor += 1;
                            require = false;
                            request = Some(result);
                        }
                        Some(Action::Require) => {
                            self.cursor += 1;
                            require = true;
                        }
                        None => {
                            if require {
                                break Err(Error::Character(self.cursor));
                            } else {
                                break Ok(request);
                            }
                        }
                    }
                }
                None => {
                    if require {
                        break Err(Error::EndOfLine);
                    } else {
                        break Ok(request);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // import namespace above here
    use super::*;

    #[test]
    fn test_parser_take_if_until_single_term() {
        let mut s = Scanner::new("asdf ");
        let t = |c: &char| -> bool { c != &'z' };
        let r = s.take_if_until(&t, " ");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), String::from("asdf"));
    }

    #[test]
    fn test_parser_take_if_until_double_term() {
        let mut s = Scanner::new("asdf]]");
        let t = |c: &char| -> bool { c != &'z' };
        let r = s.take_if_until(&t, "]]");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), String::from("asdf"));
    }

    #[test]
    fn test_parser_take_if_until_error() {
        let mut s = Scanner::new("asdf");
        let t = |c: &char| -> bool { c != &'a' };
        let r = s.take_if_until(&t, "]]");
        assert!(r.is_err());
    }
}
