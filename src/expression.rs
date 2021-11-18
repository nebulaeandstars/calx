use std::collections::VecDeque;
use std::fmt;

use super::parser;
use super::token::Token;

#[derive(Debug)]
pub enum Expression {
    Add(MExpression, Box<Expression>),
    Subtract(MExpression, Box<Expression>),
    MExpression(MExpression),
}

#[derive(Debug)]
pub enum MExpression {
    Multiply(Atom, Box<MExpression>),
    Divide(Atom, Box<MExpression>),
    Atom(Atom),
}

#[derive(Debug)]
pub enum Atom {
    Expression(Box<Expression>),
    Integer(i64),
    Variable(String),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Add(e1, e2) => write!(f, "{} + {}", e1, e2),
            Expression::Subtract(e1, e2) => write!(f, "{} - {}", e1, e2),
            Expression::MExpression(exp) => write!(f, "{}", exp),
        }
    }
}

impl fmt::Display for MExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MExpression::Multiply(e1, e2) => write!(f, "({} * {})", e1, e2),
            MExpression::Divide(e1, e2) => write!(f, "({} / {})", e1, e2),
            MExpression::Atom(atoms) => write!(f, "{}", atoms),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Expression(exp) => write!(f, "({})", exp),
            Atom::Integer(int) => write!(f, "{}", int),
            Atom::Variable(var) => write!(f, "{}", var),
        }
    }
}

impl From<Vec<Token>> for Expression {
    fn from(stream: Vec<Token>) -> Self {
        let mut stream: VecDeque<Token> = stream.into();
        parser::parse_expression(&mut stream).unwrap()
    }
}
