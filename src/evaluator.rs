use super::expression::{Atom, Expression, MExpression};

pub trait Evaluate {
    type Output;
    fn eval(&self) -> Self::Output;
}

impl Evaluate for Expression {
    type Output = f64;

    fn eval(&self) -> Self::Output {
        match self {
            Expression::Add(exp1, exp2) => exp1.eval() + exp2.eval(),
            Expression::Subtract(exp1, exp2) => exp1.eval() - exp2.eval(),
            Expression::MExpression(exp) => exp.eval(),
        }
    }
}

impl Evaluate for MExpression {
    type Output = f64;

    fn eval(&self) -> Self::Output {
        match self {
            MExpression::Multiply(atom, exp) => atom.eval() * exp.eval(),
            MExpression::Divide(atom, exp) => atom.eval() / exp.eval(),
            MExpression::Atom(atom) => atom.eval(),
        }
    }
}

impl Evaluate for Atom {
    type Output = f64;

    fn eval(&self) -> Self::Output {
        match self {
            Atom::Expression(exp) => exp.eval(),
            Atom::Integer(int) => *int as f64,
            Atom::Variable(_) =>
                unimplemented!("Variables are not implemented yet!"),
        }
    }
}
