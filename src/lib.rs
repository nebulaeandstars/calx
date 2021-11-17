use std::fmt;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum StaticTokenType {
    AdditionSymbol,
    SubtractionSymbol,
    MultiplicationSymbol,
    DivisionSymbol,
}

#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    Static(StaticTokenType),
    Integer,
}

impl fmt::Display for StaticTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StaticTokenType::*;

        let s = match self {
            AdditionSymbol => "+",
            SubtractionSymbol => "-",
            MultiplicationSymbol => "*",
            DivisionSymbol => "/",
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    value:      String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }

    pub fn get_type(&self) -> TokenType { self.token_type }
    pub fn get_value(&self) -> &String { &self.value }
}

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

        // Match any static tokens.
        for token_type in StaticTokenType::iter() {
            let s = token_type.to_string();
            if self.buffer.len() >= s.len() {
                let substring: String =
                    self.buffer.chars().take(s.len()).collect();
                if substring == s {
                    let token_type = TokenType::Static(token_type);
                    self.current = Some(Token::new(token_type, s));
                    break;
                }
            }
        }

        // If a static token wasn't found, match the next Integer.
        if self.current.is_none() {
            let s: String = self
                .buffer
                .chars()
                .take_while(|&c| c as u8 >= b'0' && c as u8 <= b'9')
                .collect();

            if !s.is_empty() {
                self.current = Some(Token::new(TokenType::Integer, s));
            }
        }

        // If a static token *still* hasn't been found, then this isn't an input
        // that the tokeniser can parse.
        if self.current.is_none() {
            panic!("Unknown token!")
        }

        let token_size = match &self.current {
            Some(token) => token.get_value().len(),
            None => 0,
        };
        self.buffer = self.buffer.chars().skip(token_size).collect();

        // Finally, return the current token.
        self.current.clone()
    }
}
