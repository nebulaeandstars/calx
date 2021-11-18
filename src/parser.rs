use std::collections::VecDeque;

use super::expression::{Atom, Expression, MExpression};
use super::token::Token;

pub fn parse_expression(stream: &mut VecDeque<Token>) -> Option<Expression> {
    if let Some(exp1) = parse_m_expression(stream) {
        let token = stream.pop_front();

        match token {
            Some(Token::Add) => parse_expression(stream)
                .map(|exp2| Expression::Add(exp1, Box::new(exp2))),
            Some(Token::Subtract) => parse_expression(stream)
                .map(|exp2| Expression::Subtract(exp1, Box::new(exp2))),

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

fn parse_m_expression(stream: &mut VecDeque<Token>) -> Option<MExpression> {
    if let Some(atom) = parse_atom(stream) {
        let token = stream.pop_front();

        match token {
            Some(Token::Multiply) => parse_m_expression(stream)
                .map(|exp| MExpression::Multiply(atom, Box::new(exp))),
            Some(Token::Divide) => parse_m_expression(stream)
                .map(|exp| MExpression::Divide(atom, Box::new(exp))),

            Some(_) => {
                stream.push_front(token.unwrap());
                if let Some(exp) = parse_m_expression(stream) {
                    Some(MExpression::Multiply(atom, Box::new(exp)))
                }
                else {
                    Some(MExpression::Atom(atom))
                }
            },
            None => Some(MExpression::Atom(atom)),
        }
    }
    else {
        None
    }
}

fn parse_atom(stream: &mut VecDeque<Token>) -> Option<Atom> {
    let token = stream.pop_front();

    match token {
        Some(Token::Integer(num)) => Some(Atom::Integer(num)),
        Some(Token::Variable(var)) => Some(Atom::Variable(var)),

        Some(Token::LeftParenthesis) => {
            if let Some(exp) = parse_expression(stream) {
                if let Some(Token::RightParenthesis) = stream.pop_front() {
                    Some(Atom::Expression(Box::new(exp)))
                }
                else {
                    panic!("Unmatched brackets!")
                }
            }
            else {
                None
            }
        },

        Some(_) => {
            stream.push_front(token.unwrap());
            None
        },
        None => None,
    }
}
