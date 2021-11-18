use std::fmt;

use strum_macros::EnumIter;
use Token::*;

#[derive(Clone, Debug, EnumIter)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
    Integer(i64),
    Variable(String),
}

impl Token {
    pub fn try_fmt(&self) -> Option<String> {
        match self {
            Add => Some("+"),
            Subtract => Some("-"),
            Multiply => Some("*"),
            Divide => Some("/"),
            LeftParenthesis => Some("("),
            RightParenthesis => Some(")"),
            _ => None,
        }
        .map(String::from)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Integer(num) => format!("{}", num),
            Variable(var) => var.clone(),
            _ => self.try_fmt().unwrap(),
        };

        write!(f, "{}", s)
    }
}
