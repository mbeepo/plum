use std::ops::Range;

use crate::{
    ast::{Expr, Literal, Spanned},
    errors::Error,
};

#[derive(Clone, Debug)]
pub enum Value {
    Num(f64),
    String(String),
    True,
    False,
    Array(Vec<Value>),
}

#[derive(Clone, Debug)]
pub struct SpannedValue(Value, Range<usize>);

impl From<Literal> for Value {
    fn from(f: Literal) -> Self {
        match f {
            Literal::Num(e) => Self::Num(e),
            Literal::String(e) => Self::String(e),
            Literal::True => Self::True,
            Literal::False => Self::False,
            Literal::Array(e) => Self::Array(e),
        }
    }
}

impl SpannedValue {
    fn pow(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Num(lhs) => match other {
                Self::Num(rhs) => {
                    if lhs == lhs.trunc() && rhs == rhs.trunc() {
                        Ok(Value::Num(lhs.powi(rhs as i32)))
                    } else {
                        Ok(Value::Num(lhs.powf(rhs)))
                    }
                }
                kind => Err(Error::WrongType {
                    expected: Value::Num(0),
                    got: kind,
                    span: other.1,
                }),
            },
            kind => Err(Error::WrongType {
                expected: Value::Num(0),
                got: kind,
                span: self.1,
            }),
        }
    }
}

pub fn interpret(input: Spanned) -> Result<Value, Vec<Error>> {
    match input {
        Spanned(Expr::Literal(literal), span) => Ok(Value::from(literal)),
        Spanned(Expr::Exp(lhs, rhs), span) => Ok(flatten(*lhs)?.pow(flatten(*rhs)?)?),
        Spanned(Expr::Add(lhs, rhs), span) => Ok(flatten(*lhs)? + flatten(*rhs)?),
        Spanned(Expr::Sub(lhs, rhs), span) => Ok(flatten(*lhs)? - flatten(*rhs)?),
    }
}

pub fn flatten(input: Spanned) -> Result<SpannedValue, Vec<Error>> {}
