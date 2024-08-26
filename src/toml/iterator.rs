use std::cmp::min;

use super::{TomlError, TomlResult};

pub struct TomlIterator<'a> {
    remaining: &'a str,
}

impl<'a> TomlIterator<'a> {
    pub fn new(str: &'a str) -> TomlIterator<'a> {
        TomlIterator { remaining: str }
    }
}

impl<'a> TomlIterator<'a> {
    fn get_section(&mut self) -> TomlResult<(&'a str, &'a str)> {
        let mut idx = 0;
        while idx < self.remaining.len() && !self.remaining[..=idx].ends_with('\n') {
            idx += 1;
        }

        if !self.remaining[..idx].ends_with("]") {
            return Err(TomlError::InvalidToml);
        }

        let ident = &self.remaining[..idx];

        self.remaining = &self.remaining[idx..];
        idx = 0;

        while idx < self.remaining.len() && !self.remaining[..=idx].ends_with("\n[") {
            idx += 1;
        }

        let value = &self.remaining[..idx].trim_start();
        self.remaining = &self.remaining[idx..];

        return Ok((ident, value));
    }

    fn skip_comments(&mut self) {
        let mut idx = 0;

        while self.remaining.starts_with('#') {
            while idx < self.remaining.len() && !self.remaining[..=idx].ends_with('\n') {
                idx += 1;
            }
            idx = min(self.remaining.len() - 1, idx + 1);
            self.remaining = self.remaining[idx..].trim_start();
            idx = 0;
        }
    }

    fn get_value(&mut self, str: &'a str) -> TomlResult<&'a str> {
        let mut matching = "\n";
        let mut pair_matcher = PairMatcher::new();

        let mut idx = 0;
        while idx < str.len() && !str[..=idx].ends_with(matching) {
            let curr_val = &str[..=idx];

            let last_char = curr_val.chars().last().unwrap();

            pair_matcher.add_char(last_char)?;

            if matching == "\n" {
                if matching != "\"\"\"\n" && curr_val.ends_with("\"\"\"") {
                    matching = "\"\"\"\n";
                    idx = min(self.remaining.len() - 1, idx + 1);
                } else if curr_val.ends_with('{') {
                    matching = "}\n";
                } else if curr_val.ends_with('[') {
                    matching = "]\n";
                }
            }

            idx = min(self.remaining.len() - 1, idx + 1);
        }

        if !pair_matcher.valid() {
            return Err(TomlError::InvalidToml);
        }

        let value = str[..idx].trim();

        self.remaining = &str[idx..];

        Ok(value)
    }
}

impl<'a> Iterator for TomlIterator<'a> {
    type Item = TomlResult<(&'a str, &'a str)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.remaining = self.remaining.trim_start();

        self.skip_comments();

        // No more content to parse
        if self.remaining.is_empty() {
            return None;
        }

        // Check for section
        if self.remaining.starts_with('[') {
            return Some(self.get_section());
        }

        let Some(split) = self.remaining.split_once('=') else {
            return Some(Err(TomlError::InvalidToml));
        };

        let ident = split.0.trim();

        let Ok(value) = self.get_value(split.1) else {
            return Some(Err(TomlError::InvalidToml));
        };

        return Some(Ok((ident, value)));
    }
}

#[derive(Debug)]
struct PairMatcher {
    pairs_queue: Vec<char>,
    str_n: usize,
}

impl PairMatcher {
    pub fn new() -> Self {
        Self {
            pairs_queue: Vec::new(),
            str_n: 0,
        }
    }

    pub fn valid(&self) -> bool {
        return self.pairs_queue.len() == self.str_n;
    }

    /// Adds a character to the pairs if it can be paired
    /// Errors if a pair closes before the first opened
    pub fn add_char(&mut self, c: char) -> TomlResult<()> {
        if !"\"[]{}".contains(c) {
            return Ok(());
        }

        match (c, self.pairs_queue.last()) {
            ('[' | '{', Some('"')) | ('"', None) => {
                if c == '"' {
                    self.str_n += 1;
                }
                self.pairs_queue.push(c);
            }
            ('"', Some('"')) | (']', Some('[')) | ('}', Some('{')) => {
                if c == '"' {
                    self.str_n -= 1;
                }
                self.pairs_queue.pop();
            }
            _ => {
                return Err(TomlError::InvalidToml);
            }
        }

        Ok(())
    }
}
