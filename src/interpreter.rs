use crate::{ast::Literal, errors::Error};

#[derive(Clone, Debug)]
pub enum Value {
    Num(f64),
    String(String),
    True,
    False,
    Array(Vec<Value>),
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Self::Num(f)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(f: &'a str) -> Self {
        Self::from(f.to_owned())
    }
}

impl From<String> for Value {
    fn from(f: String) -> Self {
        Self::String(f)
    }
}

impl From<bool> for Value {
    fn from(f: bool) -> Self {
        if f {
            Self::True
        } else {
            Self::False
        }
    }
}

impl From<Vec<Value>> for Value {
    fn from(f: Vec<Value>) -> Self {
        Self::Array(f)
    }
}

impl Value {
    fn pow(self, other: Self) -> Result<Self, Error> {
        match self {
            Self::Num(lhs) => match other {
                Self::Num(rhs) => {
                    if lhs == lhs as i64 && rhs == rhs as i64 {
                        Ok(lhs.powi(rhs))
                    } else {
                        Ok(lhs.powf(rhs))
                    }
                }
                kind => Error::WrongType(format!("Expect Num on right side of **, got {}", kind)),
            },
            kind => Error::WrongType(format!("Expected Num on left side of **, got {}", kind)),
        }
    }
}

pub fn interpret<'a>(input: Value) -> Result<Value, crate::errors::Error> {
    match input {
        Value::Literal(literal) => Ok(Value::from(literal)),
        Value::Exp(lhs, rhs) => interpret(lhs).pow(interpret(rhs)),
        Value::Add(lhs, rhs) => interpret(lhs) + interpret(rhs),
        Value::Sub(lhs, rhs) => interpret(lhs) - interpret(rhs),
    }
}
