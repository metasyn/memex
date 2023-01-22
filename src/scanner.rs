use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

// TODO: put these somewhere they can imported non cycylically
use super::{nope, EpiStatus, EpiStatusLookup};

////////////////
// VALIDATORS //
////////////////

trait Validator {
    fn check(&self, c: &char) -> bool;
}

fn validate(c: &char, validators: Vec<&dyn Validator>, invalidators: Vec<&dyn Validator>) -> bool {
    let mut passed_validators = true;
    let mut failed_invalidators = false;

    if !validators.is_empty() {
        passed_validators = validators.iter().map(|v| v.check(c)).any(|x| x);
    }

    if !invalidators.is_empty() {
        failed_invalidators = invalidators.iter().map(|v| v.check(c)).any(|x| x);
    }

    return passed_validators && !failed_invalidators;
}

struct CharMatcher {
    character: char,
}
impl Validator for CharMatcher {
    fn check(&self, c: &char) -> bool {
        return c == &self.character;
    }
}

struct WhitespaceMatcher {}
impl Validator for WhitespaceMatcher {
    fn check(&self, c: &char) -> bool {
        return c.is_whitespace();
    }
}

struct AlphanumericMatcher {}
impl Validator for AlphanumericMatcher {
    fn check(&self, c: &char) -> bool {
        return c.is_numeric() || c.is_alphabetic();
    }
}

struct EOLMatcher {}
impl Validator for EOLMatcher {
    fn check(&self, c: &char) -> bool {
        return c == &'\n' || c == &'\r';
    }
}

struct StandardMatcher {}
impl Validator for StandardMatcher {
    fn check(&self, c: &char) -> bool {
        return validate(
            c,
            vec![&AlphanumericMatcher {}, &CharMatcher { character: '-' }],
            vec![&WhitespaceMatcher {}, &EOLMatcher {}],
        );
    }
}

struct CodeFenceMatcher {}
impl Validator for CodeFenceMatcher {
    fn check(&self, c: &char) -> bool {
        return c != &'`';
    }
}

struct ImageMatcher {}
impl Validator for ImageMatcher {
    fn check(&self, c: &char) -> bool {
        return validate(
            c,
            vec![],
            vec![
                &CharMatcher { character: '&' },
                &WhitespaceMatcher {},
                &EOLMatcher {},
            ],
        );
    }
}

struct AltTitleMatcher {}
impl Validator for AltTitleMatcher {
    fn check(&self, c: &char) -> bool {
        return validate(
            c,
            vec![
                &StandardMatcher {},
                &WhitespaceMatcher {},
                &CharMatcher { character: ',' },
            ],
            vec![
                &CharMatcher { character: '}' },
                &CharMatcher { character: '{' },
                &EOLMatcher {},
            ],
        );
    }
}

struct InternalLinkMatcher {}
impl Validator for InternalLinkMatcher {
    fn check(&self, c: &char) -> bool {
        return validate(
            c,
            vec![&StandardMatcher {}],
            vec![&WhitespaceMatcher {}, &EOLMatcher {}],
        );
    }
}

pub struct Scanner {
    characters: Vec<char>,
    epistemic_lookup: Arc<EpiStatusLookup>,
    cursor: usize,
    output: String,
}

#[derive(Debug)]
enum ScannerError {
    EndOfLine,
}

fn format_md_link_epistemic(
    epistemic_lookup: Arc<EpiStatusLookup>,
    title: String,
    link: String,
) -> String {
    let status = epistemic_lookup.get(link.as_str());

    if status.is_none() {
        nope(format!("invalid link to missing internal page: {}", link));
    }

    let prefix = status.unwrap_or(&EpiStatus::Seedling);

    return format!(
        "[<img alt='icon representing the epistemic certainty of the linked page' class='epistemic-icon' src='resources/img/{}_white.png'/>{}]({}.html)",
        prefix, title, link,
    );
}

/// Heavily inspired by https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust/
impl Scanner {
    pub fn new(string: &str, epistemic_lookup: Option<Arc<EpiStatusLookup>>) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
            epistemic_lookup: epistemic_lookup.unwrap_or(Arc::new(HashMap::new())),
            output: String::new(),
        }
    }

    fn advance(&mut self) {
        self.cursor += 1
    }

    /// Returns the next character without advancing the cursor.
    /// AKA "lookahead"
    fn peek(&self) -> Option<&char> {
        return self.peek_n(0);
    }

    fn peek_n(&self, n: usize) -> Option<&char> {
        return self.characters.get(self.cursor + n);
    }

    /// Returns the previous character without advancing the cursor.
    /// AKA "lookabehind"
    fn peekback(&self) -> Option<&char> {
        // We're generally already advanced when we're peeking back,
        // so -1 == current letter, +1 == next letter, and -2 is previous.
        if self.cursor < 2 {
            return None;
        }
        self.characters.get(self.cursor - 2)
    }

    /// Returns the next character (if available) and advances the cursor.
    fn pop(&mut self) -> Option<&char> {
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
    fn take_with_validator(&mut self, validator: &dyn Fn(&char) -> bool) -> bool {
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
    fn take_char(&mut self, target: &char) -> bool {
        let t = |c: &char| -> bool {
            return c == target;
        };
        return self.take_with_validator(&t);
    }

    /// Returns a string if we can match characters (using the validator) until they
    /// terminal characters. If the terminal characters are not matched, the cursor
    /// is reset, making this lookahead idempotent in the case of failure.
    /// TODO: add version of this function that also returns the terminal char
    /// sequence and updates the cursor. This is needed for octothrope and backtick.
    fn take_if_until(
        &mut self,
        validator: &impl Validator,
        terminal: &str,
    ) -> std::result::Result<String, ScannerError> {
        // Buf to return in positive case
        let mut out = String::new();

        // Backup to restore in negative case
        let original_cursor = self.cursor;

        // First character of terminal sequence to stop on
        let term = &terminal.chars().nth(0).expect("invalid terminal string");

        while let Some(&c) = self.peek() {
            match validator.check(&c) {
                true => {
                    // We've already peeked
                    self.advance();
                    out.push(c);
                }
                false => break,
            }
        }

        // Determining point; do we have a terminal value at the end here?
        match self.peek() {
            Some(&char) if &char == term => {
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
                    // If we're out of text, we didn't match
                    if next.is_none() || &t != next.unwrap() {
                        // Reset cursor, no match
                        self.cursor = original_cursor;
                        return Err(ScannerError::EndOfLine);
                    }
                }
                // The fall through means all terminal chars were matched
                return Ok(out);
            }
            _ => {
                // Reset cursor, no match
                self.cursor = original_cursor;
                return Err(ScannerError::EndOfLine);
            }
        };
    }

    fn handle_left_square_bracket(&mut self, c: char) {
        // Second match
        match self.take_char(&c) {
            false => self.output.push(c),
            true => {
                // Check for link
                match self.take_if_until(&InternalLinkMatcher {}, "]]") {
                    Ok(val) => self.output.push_str(
                        format_md_link_epistemic(
                            Arc::clone(&self.epistemic_lookup),
                            val.clone(),
                            val.clone(),
                        )
                        .as_str(),
                    ),
                    _ => {
                        // Inital match and second take
                        self.output.push(c);
                        self.output.push(c);
                    }
                }
            }
        }
    }

    fn handle_left_curly_bracket(&mut self, c: char) {
        // We need double consumption so keep track of cursor manually
        // because the first one could update the first successfully
        // and the second one could fail, resulting in an updated cursor
        let original_cursor = self.cursor;

        // Check for title
        let title_res = self.take_if_until(&AltTitleMatcher {}, "}[[");
        let link_res = self.take_if_until(&InternalLinkMatcher {}, "]]");

        match (title_res, link_res) {
            (Ok(title), Ok(link)) => {
                self.output.push_str(
                    format_md_link_epistemic(Arc::clone(&self.epistemic_lookup), title, link)
                        .as_str(),
                );
            }
            _ => {
                self.cursor = original_cursor;
                self.output.push(c);
            }
        }
    }

    fn handle_ampersand(&mut self, c: char) {
        match self.take_if_until(&ImageMatcher {}, "&") {
            Ok(val) => self
                .output
                .push_str(format!("<img src='resources/img/{}'/>", val).as_str()),
            _ => {
                self.output.push(c);
            }
        }
    }

    fn handle_octothrope(&mut self, c: char) {
        match self.peekback() {
            // Will _not_ match the first character
            Some('\n') => {
                // Matched header
                self.output.push(c);

                // In case of ##, ###, and so on
                let more = self.take_if_until(&CharMatcher { character: '#' }, " ");

                if more.is_ok() {
                    self.output.push_str(more.unwrap().as_str());
                }

                let text = self.take_if_until(&AltTitleMatcher {}, "\n");

                if text.is_ok() {
                    let content = text.unwrap();
                    let clean = Regex::new(r"[^\w-]+")
                        .unwrap()
                        .replace_all(&content.trim(), "-")
                        .to_lowercase();

                    self.output
                        .push_str(format!(" <a name='{}'>{}</a>\n", clean, &content).as_str());
                }
            }
            _ => self.output.push(c),
        }
    }

    fn handle_backtick(&mut self, c: char) {
        let original_cursor = self.cursor;
        match self.peekback() {
            // Will _not_ match the first character
            Some('\n') => match (self.peek(), self.peek_n(1), self.peek_n(2)) {
                (Some('`'), Some('`'), Some('\n')) => {
                    // Advance cursor 3 chars, for what we've peeked.
                    // we'll add them manually later if we match.
                    self.cursor += 3;

                    // Look for the text + ending sequence
                    match self.take_if_until(&CodeFenceMatcher {}, "```\n") {
                        Ok(text) => {
                            // Original matched character
                            self.output.push(c);
                            // Peeked characters
                            self.output.push_str("``\n");
                            // The above is advanced already for the cursor

                            // Captured content - also advancved
                            self.output.push_str(text.as_str());

                            // Ending - this ending isn't advanced
                            // however I don't want it to match again
                            // so we manually add it and advance the cursor
                            self.output.push_str("```\n");
                            self.cursor += 4;
                        }
                        _ => {
                            self.cursor = original_cursor;
                            self.output.push(c);
                        }
                    }
                }
                _ => self.output.push(c),
            },
            _ => self.output.push(c),
        }
    }

    pub fn convert(&mut self) -> String {
        while let Some(&c) = self.pop() {
            match c {
                '{' => self.handle_left_curly_bracket(c),
                '[' => self.handle_left_square_bracket(c),
                '&' => self.handle_ampersand(c),
                '#' => self.handle_octothrope(c),
                '`' => self.handle_backtick(c),
                _ => self.output.push(c),
            }
        }
        return self.output.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_matcher() {
        assert!(CharMatcher { character: 'c' }.check(&'c'));
        assert!(!CharMatcher { character: 'c' }.check(&'d'));
    }

    #[test]
    fn test_whitespace_matcher() {
        assert!(WhitespaceMatcher {}.check(&' '));
        assert!(!WhitespaceMatcher {}.check(&'a'));
    }

    #[test]
    fn test_alphabetic_matcher() {
        assert!(AlphanumericMatcher {}.check(&'a'));
        assert!(!AlphanumericMatcher {}.check(&' '));
        assert!(AlphanumericMatcher {}.check(&'9'));
    }

    #[test]
    fn test_eol_matcher() {
        assert!(!EOLMatcher {}.check(&'a'));
        assert!(EOLMatcher {}.check(&'\n'));
        assert!(EOLMatcher {}.check(&'\r'));
    }

    #[test]
    fn test_validate() {
        // Empty
        assert!(validate(&'c', vec![], vec![]));
        // Single match
        assert!(validate(
            &'c',
            vec![&CharMatcher { character: 'c' }],
            vec![]
        ));
        // Match both
        assert!(validate(
            &'c',
            vec![&CharMatcher { character: 'c' }],
            vec![&WhitespaceMatcher {}]
        ));

        // Match second only - passes
        assert!(validate(
            &' ',
            vec![],
            vec![&CharMatcher { character: 'c' }],
        ));

        // Match second only - fails
        assert!(!validate(
            &'c',
            vec![],
            vec![&CharMatcher { character: 'c' }],
        ));

        // Disagree - fails
        assert!(!validate(
            &'c',
            vec![&CharMatcher { character: 'c' }],
            vec![&CharMatcher { character: 'c' }],
        ));
    }

    #[test]
    fn test_standard_matcher() {
        assert!(StandardMatcher {}.check(&'a'));
        assert!(StandardMatcher {}.check(&'-'));
        assert!(!StandardMatcher {}.check(&' '));
        assert!(!StandardMatcher {}.check(&'\n'));
    }

    #[test]
    fn test_image_matcher() {
        assert!(ImageMatcher {}.check(&'a'));
        assert!(ImageMatcher {}.check(&'-'));
        assert!(!ImageMatcher {}.check(&'&'));
    }

    #[test]
    fn test_alttitle_matcher() {
        assert!(AltTitleMatcher {}.check(&'a'));
        assert!(AltTitleMatcher {}.check(&' '));
        assert!(!AltTitleMatcher {}.check(&'}'));
        assert!(!AltTitleMatcher {}.check(&'{'));
    }

    #[test]
    fn test_internal_link_matcher() {
        assert!(InternalLinkMatcher {}.check(&'a'));
        assert!(InternalLinkMatcher {}.check(&'-'));

        for char in vec![' ', '[', ']', '{', '}', '\n'] {
            assert!(!InternalLinkMatcher {}.check(&char));
        }
    }

    #[test]
    fn test_scanner_take_if_until_single_term() {
        let mut s = Scanner::new("asdf ", None);
        let r = s.take_if_until(&InternalLinkMatcher {}, " ");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), String::from("asdf"));
    }

    #[test]
    fn test_scanner_take_if_until_double_term() {
        let mut s = Scanner::new("asdf]]", None);
        let r = s.take_if_until(&InternalLinkMatcher {}, "]]");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), String::from("asdf"));
    }

    #[test]
    fn test_scanner_take_if_until_many_term() {
        let mut s = Scanner::new("asdf}1234567890", None);
        let r = s.take_if_until(&InternalLinkMatcher {}, "}");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), String::from("asdf"));
    }

    #[test]
    fn test_scanner_take_if_until_error() {
        let mut s = Scanner::new(" asdf]]", None);
        let r = s.take_if_until(&ImageMatcher {}, "]]");
        assert!(r.is_err());
    }

    #[test]
    fn test_scanner_convert_basic() {
        assert_eq!(
            Scanner::new("[[something]]", None).convert(),
            "[<img alt='icon representing the epistemic certainty of the linked page' class='epistemic-icon' src='resources/img/seedling_white.png'/>something](something.html)"
        );
    }

    #[test]
    fn test_scanner_convert_image_basic() {
        assert_eq!(
            Scanner::new("&test.png&", None).convert(),
            "<img src='resources/img/test.png'/>",
        );
    }

    #[test]
    fn test_scanner_convert_basic_noop() {
        assert_eq!(Scanner::new("[something]", None).convert(), "[something]");
    }

    #[test]
    fn test_scanner_convert_unique_title() {
        assert_eq!(
            Scanner::new("{test}[[foo]]", None).convert(),
            "[<img alt='icon representing the epistemic certainty of the linked page' class='epistemic-icon' src='resources/img/seedling_white.png'/>test](foo.html)"
        );
    }

    #[test]
    fn test_scanner_convert_ignore_partial_link() {
        let s = "something [[test";
        assert_eq!(Scanner::new(s, None).convert(), s);
    }

    #[test]
    fn test_scanner_convert_ignore_partial_link_alt() {
        let s = "something {alt";
        assert_eq!(Scanner::new(s, None).convert(), s);
    }

    // Its trash time, bitches
    #[test]
    fn test_scanner_convert_ignore_trash_1() {
        let s = "something {alt}[[test";
        assert_eq!(Scanner::new(s, None).convert(), s);
    }

    #[test]
    fn test_scanner_convert_ignore_trash_2() {
        let s = "something {alt}[[test]";
        assert_eq!(Scanner::new(s, None).convert(), s);
    }

    #[test]
    fn test_scanner_convert_ignore_trash_3() {
        let s = "something {alt}[[test}]]";
        assert_eq!(Scanner::new(s, None).convert(), s);
    }

    #[test]
    fn test_scanner_convert_ignore_trash_4() {
        let s = "{test this}[[seems like it]{cshould][{}[{]} work";
        assert_eq!(Scanner::new(s, None).convert(), s);
    }

    #[test]
    fn test_scanner_convert_header_link() {
        let s = "\n# Hello World\n";
        let e = "\n# <a name='hello-world'>Hello World</a>\n";
        let o = Scanner::new(s, None).convert();
        assert_eq!(o, e);
    }

    #[test]
    fn test_scanner_convert_code_fence() {
        let s = "\n```\n[[test]]\n```\n";
        let o = Scanner::new(s, None).convert();
        assert_eq!(s, o);
    }
}
