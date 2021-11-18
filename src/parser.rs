use std::collections::VecDeque;
use std::fmt;

use super::token::Token;

#[derive(Debug)]
pub enum Expression {
    Add(MExpression, Box<Expression>),
    Subtract(MExpression, Box<Expression>),
    MExpression(MExpression),
}

#[derive(Debug)]
pub enum MExpression {
    Multiply(Atoms, Box<MExpression>),
    Divide(Atoms, Box<MExpression>),
    Atoms(Atoms),
}

#[derive(Debug)]
pub enum Atoms {
    Atoms(Atom, Box<Atoms>),
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
            MExpression::Atoms(atoms) => write!(f, "{}", atoms),
        }
    }
}

impl fmt::Display for Atoms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atoms::Atoms(a1, a2) => write!(f, "{}{}", a1, a2),
            Atoms::Atom(atom) => write!(f, "{}", atom),
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
        parse_expression(&mut stream).unwrap()
    }
}

fn parse_expression<'a>(stream: &mut VecDeque<Token>) -> Option<Expression> {
    if let Some(exp1) = parse_m_expression(stream) {
        let token = stream.pop_front();

        match token {
            Some(Token::Add) =>
                if let Some(exp2) = parse_expression(stream) {
                    Some(Expression::Add(exp1, Box::new(exp2)))
                }
                else {
                    None
                },
            Some(Token::Subtract) =>
                if let Some(exp2) = parse_expression(stream) {
                    Some(Expression::Subtract(exp1, Box::new(exp2)))
                }
                else {
                    None
                },

            Some(_) => {
                stream.push_front(token.unwrap());
                Some(Expression::MExpression(exp1))
            },
            None => Some(Expression::MExpression(exp1)),
        }
    }
    else {
        None
    }
}

fn parse_m_expression<'a>(stream: &mut VecDeque<Token>) -> Option<MExpression> {
    if let Some(atoms) = parse_atoms(stream) {
        let token = stream.pop_front();

        match token {
            Some(Token::Multiply) =>
                if let Some(exp) = parse_m_expression(stream) {
                    Some(MExpression::Multiply(atoms, Box::new(exp)))
                }
                else {
                    None
                },
            Some(Token::Divide) =>
                if let Some(exp) = parse_m_expression(stream) {
                    Some(MExpression::Divide(atoms, Box::new(exp)))
                }
                else {
                    None
                },

            Some(_) => {
                stream.push_front(token.unwrap());
                Some(MExpression::Atoms(atoms))
            },
            None => Some(MExpression::Atoms(atoms)),
        }
    }
    else {
        None
    }
}

fn parse_atoms<'a>(stream: &mut VecDeque<Token>) -> Option<Atoms> {
    if let Some(atom) = parse_atom(stream) {
        if let Some(atoms) = parse_atoms(stream) {
            Some(Atoms::Atoms(atom, Box::new(atoms)))
        }
        else {
            Some(Atoms::Atom(atom))
        }
    }
    else {
        None
    }
}

fn parse_atom<'a>(stream: &mut VecDeque<Token>) -> Option<Atom> {
    let token = stream.pop_front();

    match token {
        Some(Token::Integer(num)) => Some(Atom::Integer(num)),
        Some(Token::Variable(var)) => Some(Atom::Variable(var.clone())),

        Some(_) => {
            stream.push_front(token.unwrap());
            None
        },
        None => None,
    }
}
