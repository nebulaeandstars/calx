use std::fmt;

use strum_macros::EnumIter;
use Token::*;

#[derive(Clone, Debug, EnumIter)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
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
            _ => None,
        }
        .map(|s| String::from(s))
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Integer(num) => format!("{}", num),
            Variable(var) => var.clone(),
            _ => String::from(self.try_fmt().unwrap()),
        };

        write!(f, "{}", s)
    }
}
