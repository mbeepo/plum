use std::{fmt::Display, ops::Range};

use crate::{
    ast::{Expr, InfixOp, Literal, Spanned},
    errors::{Error, TypeErrorCtx},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Num(f64),
    String(String),
    Bool(bool),
    Array(Vec<SpannedValue>),
    Error,
    Assign(String, Box<Value>),
}

#[derive(Clone, Copy, Debug)]
pub enum ValueType {
    Num,
    Int,
    String,
    Bool,
    Array,
    Error,
    Assign,
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
            _ => Value::Error,
        }
    }
}

impl AsRef<Spanned> for Spanned {
    fn as_ref(&self) -> &Spanned {
        &self
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            ValueType::Num => "Num",
            ValueType::Int => "Int",
            ValueType::String => "String",
            ValueType::Bool => "Bool",
            ValueType::Array => "Array",
            ValueType::Error => "[ERROR]",
            ValueType::Assign => "Assign",
        };

        write!(f, "{}", out)
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
            Value::Assign(_, _) => ValueType::Assign,
        }
    }
}

pub fn interpret<T: AsRef<Spanned>>(input: T) -> Result<SpannedValue, Vec<Error>> {
    let input = input.as_ref();
    let mut errors: Vec<Error> = Vec::new();

    match input {
        Spanned(Expr::Literal(literal), span) => match literal {
            Literal::Array(e) => {
                let mut errored = false;

                let new = e
                    .iter()
                    .map(|f| {
                        let interpreted = interpret(f.clone());

                        match interpreted {
                            Ok(e) => e,
                            Err(e) => {
                                errors.extend(e);
                                errored = true;

                                SpannedValue(Value::Error, f.clone().1)
                            }
                        }
                    })
                    .collect();

                if errored {
                    Err(errors)
                } else {
                    Ok(SpannedValue(Value::Array(new), span.clone()))
                }
            }
            _ => Ok(SpannedValue(Value::from(literal.clone()), span.clone())),
        },
        Spanned(Expr::Assign { name, value }, span) => {
            let interpreted = interpret(value);

            match interpreted {
                Ok(spanned) => {
                    if let SpannedValue(Value::Assign(other_name, other_value), other_span) =
                        spanned
                    {
                        let err = Error::TypeError {
                            expected: vec![
                                ValueType::Num,
                                ValueType::String,
                                ValueType::Bool,
                                ValueType::Array,
                            ],
                            got: spanned,
                            context: TypeErrorCtx::AssignToAssign,
                        };

                        errors.push(err);

                        Err(errors)
                    } else {
                        Ok(SpannedValue(Value::Assign(name, interpreted), span.clone()))
                    }
                }
                Err(error) => {
                    errors.extend(error);

                    Err(errors)
                }
            }
        }
        Spanned(Expr::InfixOp(lhs, op, rhs), span) => {
            let lhs = interpret(lhs);
            let lhs = match lhs {
                Err(e) => {
                    errors.extend(e);

                    SpannedValue(Value::Error, 0..1)
                }
                Ok(e) => e.clone(),
            };

            let rhs = interpret(rhs);
            let rhs = match rhs {
                Err(e) => {
                    errors.extend(e);

                    SpannedValue(Value::Error, 0..1)
                }
                Ok(e) => e.clone(),
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
                Ok(e) => Ok(SpannedValue(e, span.clone())),
            }
        }
        Spanned(Expr::Not(rhs), span) => {
            let rhs = interpret(rhs);
            let rhs = match rhs {
                Err(e) => {
                    errors.extend(e);

                    return Err(errors);
                }
                Ok(e) => e.clone(),
            };

            let output = rhs.not();

            match output {
                Err(e) => {
                    errors.push(e);

                    Err(errors)
                }
                Ok(e) => Ok(SpannedValue(e, span.clone())),
            }
        }
        Spanned(Expr::Index(lhs, rhs), span) => {
            let lhs = interpret(lhs);
            let lhs = match lhs {
                Err(e) => {
                    errors.extend(e);

                    SpannedValue(Value::Error, 0..1)
                }
                Ok(e) => e.clone(),
            };

            let rhs = interpret(rhs);
            let rhs = match rhs {
                Err(e) => {
                    errors.extend(e);

                    SpannedValue(Value::Error, 0..1)
                }
                Ok(e) => e.clone(),
            };

            if lhs == Value::Error || rhs == Value::Error {
                return Err(errors);
            }

            let output = lhs.index(rhs);

            match output {
                Err(e) => {
                    errors.push(e);

                    Err(errors)
                }
                Ok(e) => Ok(SpannedValue(e, span.clone())),
            }
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use chumsky::{Parser, Stream};

    use crate::{ast::Spanned, lexer::lexer, parser};

    use super::{interpret, Value};

    fn parse<'a>(input: &'a str) -> Vec<Spanned> {
        let len = input.len();

        let lexed = lexer().parse(input).unwrap();
        parser::parse()
            .parse(Stream::from_iter(len..len + 1, lexed.into_iter()))
            .unwrap()
    }

    #[test]
    fn interpret_num() {
        let parsed = &parse("12")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::Num(12.0))
    }

    #[test]
    fn interpret_addition() {
        let parsed = &parse("12 + 8")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::Num(20.0))
    }

    #[test]
    fn interpret_chained() {
        let parsed = &parse("12 + 8 * 3")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::Num(36.0))
    }

    #[test]
    fn interpret_parens() {
        let parsed = &parse("(12 + 8) / 10")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::Num(2.0))
    }

    #[test]
    fn interpret_complex_chained() {
        let parsed = &parse("10 + (30 - 5) * 3 ** 2")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::Num(235.0))
    }

    #[test]
    fn interpret_string_mul() {
        let parsed = &parse("'nice' * 3")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::String("nicenicenice".to_owned()))
    }

    #[test]
    fn interpret_string_mul_invalid_direct() {
        let parsed = &parse("'nice' * 'cool'")[0];
        let interpreted = interpret(parsed);

        assert!(interpreted.is_err())
    }

    #[test]
    fn interpret_string_mul_invalid_chain() {
        let parsed = &parse("'nice' * (3 * 'cool')")[0];
        let interpreted = interpret(parsed);

        assert!(interpreted.is_err())
    }

    #[test]
    fn interpret_index_array() {
        let parsed = &parse("[1, 2, 3, 4][3]")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::Num(4.0))
    }

    #[test]
    fn interpret_index_string() {
        let parsed = &parse("'nice'[3]")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::String("e".to_owned()))
    }

    #[test]
    fn interpret_and_true() {
        let parsed = &parse("true and true")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::Bool(true))
    }

    #[test]
    fn interpret_and_false() {
        let parsed = &parse("true and false")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::Bool(false))
    }

    #[test]
    fn interpret_or_true() {
        let parsed = &parse("true or false")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::Bool(true))
    }

    #[test]
    fn interpret_or_false() {
        let parsed = &parse("false or false")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::Bool(false))
    }

    #[test]
    fn interpret_chain_and_or() {
        let parsed = &parse("true and false or true and true")[0];
        let interpreted = interpret(parsed).unwrap();

        assert_eq!(interpreted, Value::Bool(true))
    }
}
