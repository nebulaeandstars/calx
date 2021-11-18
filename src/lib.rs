mod token;

use std::ops::RangeInclusive;

use strum::IntoEnumIterator;
use token::Token;

pub struct Tokeniser {
    current: Option<Token>,
    buffer:  String,
}

impl Tokeniser {
    pub fn new(buffer: String) -> Self { Tokeniser { current: None, buffer } }

    fn match_static(&self) -> Option<Token> {
        // For any token with a known format, check whether the start of the
        // buffer matches its String representation. This will match any
        // "static" tokens that have been defined.
        for token in Token::iter() {
            if let Some(display) = token.try_fmt() {
                let chars = self.buffer.chars();
                if !display.chars().zip(chars).any(|(a, b)| a != b) {
                    return Some(token);
                }
            }
        }

        None
    }

    fn match_dynamic(&self) -> Option<Token> {
        // Try to match an Integer.
        let s = self.substring_for_range('0'..='9');
        if !s.is_empty() {
            let num = s.parse::<i64>().unwrap();
            return Some(Token::Integer(num));
        }

        // If that didn't work, try to match a Variable.
        let s = self.substring_for_range('A'..='z');
        if !s.is_empty() {
            return Some(Token::Variable(s));
        }

        None
    }

    fn substring_for_range(&self, range: RangeInclusive<char>) -> String {
        self.buffer.chars().take_while(|c| range.contains(c)).collect()
    }
}

impl Iterator for Tokeniser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer = String::from(self.buffer.trim());

        // If the buffer is empty, stop here.
        if self.buffer.is_empty() {
            return None;
        }

        // Otherwise, try to match a static token.
        self.current = self.match_static();

        // If a static token wasn't found, try to match an Integer or Variable.
        if self.current.is_none() {
            self.current = self.match_dynamic();
        }

        // If a token *still* hasn't been found, then this isn't an input that
        // the tokeniser can parse.
        if self.current.is_none() {
            panic!("Unknown token!")
        }

        // Finally, remove the token from the buffer, and return it.
        let token_size = match &self.current {
            Some(token) => format!("{}", token).len(),
            None => 0,
        };
        self.buffer = self.buffer.chars().skip(token_size).collect();

        self.current.clone()
    }
}
