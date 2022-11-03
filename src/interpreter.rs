use std::ops::Range;

use crate::{
    ast::{Expr, InfixOp, Literal, Spanned},
    errors::Error,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Num(f64),
    String(String),
    Bool(bool),
    Array(Vec<SpannedValue>),
    Error,
}

#[derive(Clone, Copy, Debug)]
pub enum ValueType {
    Num,
    String,
    Bool,
    Array,
    Error,
}

#[derive(Clone, Debug)]
pub struct SpannedValue(pub Value, pub Range<usize>);

impl From<ValueType> for Vec<ValueType> {
    fn from(f: ValueType) -> Self {
        vec![f]
    }
}

impl PartialEq<SpannedValue> for SpannedValue {
    fn eq(&self, other: &SpannedValue) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialEq<Value> for SpannedValue {
    fn eq(&self, other: &Value) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<SpannedValue> for Value {
    fn eq(&self, other: &SpannedValue) -> bool {
        self.eq(&other.0)
    }
}

impl From<Literal> for Value {
    fn from(f: Literal) -> Self {
        match f {
            Literal::Num(e) => Value::Num(e),
            Literal::String(e) => Value::String(e),
            Literal::Bool(e) => Value::Bool(e),
            Literal::Array(_) => Value::Error,
        }
    }
}

impl Value {
    pub fn get_type(&self) -> ValueType {
        match self {
            Value::Num(_) => ValueType::Num,
            Value::String(_) => ValueType::String,
            Value::Bool(_) => ValueType::Bool,
            Value::Array(_) => ValueType::Array,
            Value::Error => ValueType::Error,
        }
    }
}

pub fn interpret(input: Spanned) -> Result<SpannedValue, Vec<Error>> {
    let errors: Vec<Error> = Vec::new();

    match input {
        Spanned(Expr::Literal(literal), span) => match literal {
            Literal::Array(e) => {
                let mut errored = false;

                let new = e
                    .iter()
                    .map(|f| {
                        let interpreted = interpret(*f);

                        match interpreted {
                            Ok(e) => e,
                            Err(e) => {
                                errors.extend(e);
                                errored = true;

                                SpannedValue(Value::Error, f.1)
                            }
                        }
                    })
                    .collect();

                if errored {
                    Err(errors)
                } else {
                    Ok(SpannedValue(Value::Array(new), span))
                }
            }
            _ => Ok(SpannedValue(literal.into(), span)),
        },
        Spanned(Expr::InfixOp(lhs, op, rhs), span) => {
            let lhs = interpret(*lhs);

            let lhs = match lhs {
                Err(e) => {
                    errors.extend(e);

                    SpannedValue(Value::Error, 0..1)
                }
                Ok(e) => e,
            };

            let rhs = interpret(*rhs);

            let rhs = match rhs {
                Err(e) => {
                    errors.extend(e);

                    SpannedValue(Value::Error, 0..1)
                }
                Ok(e) => e,
            };

            if lhs == Value::Error || rhs == Value::Error {
                return Err(errors);
            }

            let output = match op {
                InfixOp::Pow => lhs.pow(rhs),
                InfixOp::Mul => lhs.mul(rhs),
                InfixOp::Div => lhs.div(rhs),
                InfixOp::Mod => lhs.modulus(rhs),
                InfixOp::Add => lhs.add(rhs),
                InfixOp::Sub => lhs.sub(rhs),
                InfixOp::Equals => lhs.equals(rhs),
                InfixOp::NotEquals => lhs.equals(rhs),
                InfixOp::Lt => lhs.lt(rhs),
                InfixOp::Gt => lhs.gt(rhs),
                InfixOp::Lte => lhs.lte(rhs),
                InfixOp::Gte => lhs.gte(rhs),
                InfixOp::And => lhs.and(rhs),
                InfixOp::Or => lhs.or(rhs),
                InfixOp::In => lhs.is_in(rhs),
            };

            match output {
                Err(e) => {
                    errors.push(e);

                    Err(errors)
                }
                Ok(e) => Ok(SpannedValue(e, span)),
            }
        }
    }
}
