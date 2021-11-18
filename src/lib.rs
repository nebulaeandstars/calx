mod token;

use strum::IntoEnumIterator;
use token::Token;

pub struct Tokeniser {
    current: Option<Token>,
    buffer:  String,
}

impl Tokeniser {
    pub fn new(buffer: String) -> Self { Tokeniser { current: None, buffer } }
}

impl Iterator for Tokeniser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer = String::from(self.buffer.trim());
        self.current = None;

        if self.buffer.is_empty() {
            return None;
        }

        // For any token with a known format, check whether the start of the
        // buffer matches its String representation. This will match any
        // "static" tokens that have been defined.
        for token in Token::iter() {
            if let Some(display) = token.try_fmt() {
                let chars = self.buffer.chars();
                if !display.chars().zip(chars).any(|(a, b)| a != b) {
                    self.current = Some(token);
                }
            }
        }

        // If a static token wasn't found, try to match an Integer.
        if self.current.is_none() {
            let s: String = self
                .buffer
                .chars()
                .take_while(|&c| c as u8 >= b'0' && c as u8 <= b'9')
                .collect();

            if !s.is_empty() {
                let num = s.parse::<i64>().unwrap();
                self.current = Some(Token::Integer(num));
            }
        }

        // If an Integer wasn't found, try to match a Variable.
        if self.current.is_none() {
            let s: String = self
                .buffer
                .chars()
                .take_while(|&c| c as u8 >= b'A' && c as u8 <= b'z')
                .collect();

            if !s.is_empty() {
                self.current = Some(Token::Variable(s));
            }
        }

        // If a static token *still* hasn't been found, then this isn't an input
        // that the tokeniser can parse.
        if self.current.is_none() {
            panic!("Unknown token!")
        }

        let token_size = match &self.current {
            Some(token) => format!("{}", token).len(),
            None => 0,
        };
        self.buffer = self.buffer.chars().skip(token_size).collect();

        // Finally, return the current token.
        self.current.clone()
    }
}
