use std::collections::HashMap;

use crate::{
    ast::{Expr, InfixOp, Literal, Spanned},
    error::{Error, TypeErrorCtx},
    value::{SpannedValue, Value, ValueType},
};

pub fn eval<T: AsRef<Spanned>>(
    input: T,
    vars: HashMap<String, Value>,
) -> Result<SpannedValue, Vec<Error>> {
    let input = input.as_ref();
    let mut errors: Vec<Error> = Vec::new();

    match input {
        Spanned(Expr::Literal(literal), span) => match literal {
            Literal::Array(e) => {
                let mut errored = false;

                let new = e
                    .iter()
                    .map(|f| {
                        let evaluated = eval(f.clone(), vars.clone());

                        match evaluated {
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
        Spanned(Expr::Assign { names, value }, span) => {
            let evaluated = eval(value, vars);

            match evaluated {
                Ok(spanned) => Ok(SpannedValue(
                    Value::Assign(names.clone(), Box::new(spanned.0)),
                    span.clone(),
                )),
                Err(error) => {
                    errors.extend(error);

                    Err(errors)
                }
            }
        }
        Spanned(Expr::InfixOp(lhs, op, rhs), span) => {
            let lhs = eval(lhs, vars.clone());
            let lhs = match lhs {
                Err(e) => {
                    errors.extend(e);

                    SpannedValue(Value::Error, 0..1)
                }
                Ok(e) => e.clone(),
            };

            let rhs = eval(rhs, vars);
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
                InfixOp::NotEquals => lhs.not_equals(rhs),
                InfixOp::Lt => lhs.lt(rhs),
                InfixOp::Gt => lhs.gt(rhs),
                InfixOp::Lte => lhs.lte(rhs),
                InfixOp::Gte => lhs.gte(rhs),
                InfixOp::And => lhs.and(rhs),
                InfixOp::Or => lhs.or(rhs),
                InfixOp::In => lhs.contains(rhs),
                InfixOp::Range => lhs.range(rhs),
                InfixOp::IRange => lhs.irange(rhs),
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
            let rhs = eval(rhs, vars);
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
            let lhs = eval(lhs, vars.clone());
            let lhs = match lhs {
                Err(e) => {
                    errors.extend(e);

                    SpannedValue(Value::Error, 0..1)
                }
                Ok(e) => e.clone(),
            };

            let rhs = eval(rhs, vars);
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
        Spanned(Expr::Ident(name), span) => {
            let out = vars.get(name);

            match out {
                Some(out) => Ok(SpannedValue(out.clone(), span.clone())),
                None => {
                    let err = Error::ReferenceError {
                        name: name.clone(),
                        span: span.clone(),
                    };
                    errors.push(err);

                    Err(errors)
                }
            }
        }
        Spanned(
            Expr::Conditional {
                condition,
                inner,
                other,
            },
            _,
        ) => {
            let evaluated = eval(condition, vars.clone())?;

            match evaluated {
                SpannedValue(Value::Bool(enter), _) => {
                    if enter {
                        eval(inner, vars)
                    } else {
                        eval(other, vars)
                    }
                }
                _ => {
                    let err = Error::TypeError {
                        expected: ValueType::Bool.into(),
                        got: evaluated,
                        context: TypeErrorCtx::Condition,
                    };

                    errors.push(err);

                    Err(errors)
                }
            }
        }
        Spanned(Expr::Input(_, kind), span) => Ok(SpannedValue(Value::Input(*kind), span.clone())),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chumsky::{Parser, Stream};

    use crate::{ast::Spanned, error::Error, lexer::lexer, parser};

    use super::{SpannedValue, Value};

    fn parse<'a>(input: &'a str) -> Vec<Spanned> {
        let len = input.len();

        let lexed = lexer().parse(input).unwrap();
        parser::parse()
            .parse(Stream::from_iter(len..len + 1, lexed.into_iter()))
            .unwrap()
    }

    fn evaluate<T: AsRef<Spanned>>(input: T) -> Result<SpannedValue, Vec<Error>> {
        super::eval(input, HashMap::new())
    }

    #[test]
    fn evaluate_num() {
        let parsed = &parse("12")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Num(12.0))
    }

    #[test]
    fn evaluate_addition() {
        let parsed = &parse("12 + 8")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Num(20.0))
    }

    #[test]
    fn evaluate_chained() {
        let parsed = &parse("12 + 8 * 3")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Num(36.0))
    }

    #[test]
    fn evaluate_parens() {
        let parsed = &parse("(12 + 8) / 10")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Num(2.0))
    }

    #[test]
    fn evaluate_complex_chained() {
        let parsed = &parse("10 + (30 - 5) * 3 ** 2")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Num(235.0))
    }

    #[test]
    fn evaluate_string_mul() {
        let parsed = &parse("'nice' * 3")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::String("nicenicenice".to_owned()))
    }

    #[test]
    fn evaluate_string_mul_invalid_direct() {
        let parsed = &parse("'nice' * 'cool'")[0];
        let evaluated = evaluate(parsed);

        assert!(evaluated.is_err())
    }

    #[test]
    fn evaluate_string_mul_invalid_chain() {
        let parsed = &parse("'nice' * (3 * 'cool')")[0];
        let evaluated = evaluate(parsed);

        assert!(evaluated.is_err())
    }

    #[test]
    fn evaluate_index_array() {
        let parsed = &parse("[1, 2, 3, 4][3]")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Num(4.0))
    }

    #[test]
    fn evaluate_index_string() {
        let parsed = &parse("'nice'[3]")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::String("e".to_owned()))
    }

    #[test]
    fn evaluate_and_true() {
        let parsed = &parse("true and true")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Bool(true))
    }

    #[test]
    fn evaluate_and_false() {
        let parsed = &parse("true and false")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Bool(false))
    }

    #[test]
    fn evaluate_or_true() {
        let parsed = &parse("true or false")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Bool(true))
    }

    #[test]
    fn evaluate_or_false() {
        let parsed = &parse("false or false")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Bool(false))
    }

    #[test]
    fn evaluate_chain_and_or() {
        let parsed = &parse("true and false or true and true")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Bool(true))
    }

    #[test]
    fn evaluate_assign() {
        let parsed = &parse("nice = 'cool';")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(
            evaluated,
            Value::Assign(
                vec!["nice".to_owned()],
                Box::new(Value::String("cool".to_owned()))
            )
        )
    }

    #[test]
    fn evaluate_equals() {
        let parsed = &parse("60 == 60")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Bool(true))
    }

    #[test]
    fn evaluate_equals_false() {
        let parsed = &parse("50 == 60")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Bool(false))
    }

    #[test]
    fn evaluate_equals_fail() {
        let parsed = &parse("50 == [50]")[0];
        let evaluated = evaluate(parsed);

        assert!(evaluated.is_err())
    }

    #[test]
    fn evaluate_contains() {
        let parsed = &parse("12 in [10, 11, 12, 13, 14]")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Bool(true))
    }

    #[test]
    fn evaluate_index_array_fail() {
        let parsed = &parse("[1, 2, 3][3]")[0];
        let evaluated = evaluate(parsed);

        assert!(evaluated.is_err())
    }

    #[test]
    fn evaluate_index_string_fail() {
        let parsed = &parse(r#""nice"[10]"#)[0];
        let evaluated = evaluate(parsed);

        assert!(evaluated.is_err())
    }

    #[test]
    fn evaluate_assigned() {
        let parsed = parse(r#"cool = 23; nice = cool * 3;"#);
        let mut vars: HashMap<String, Value> = HashMap::new();

        let evaluated1 = evaluate(&parsed[0]).unwrap();

        match evaluated1 {
            SpannedValue(Value::Assign(names, value), _) => {
                for name in names {
                    vars.insert(name, *value.clone());
                }
            }
            _ => {
                assert!(false);
            }
        }

        let evaluated2 = super::eval(&parsed[1], vars).unwrap();

        assert_eq!(
            evaluated2,
            Value::Assign(vec!["nice".to_owned()], Box::new(Value::Num(69.0)))
        )
    }

    #[test]
    fn evaluate_assign_chain() {
        let parsed = &parse("these = are = all = 12;")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(
            evaluated,
            Value::Assign(
                vec!["these".to_owned(), "are".to_owned(), "all".to_owned()],
                Box::new(Value::Num(12.0))
            )
        )
    }

    #[test]
    fn evaluate_range() {
        let parsed = &parse("0..5")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::Range(0..5))
    }

    #[test]
    fn evaluate_range_as_index() {
        let parsed = &parse("['nice', 'cool', 'wicked', 'sick'][1..3]")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(
            evaluated,
            Value::Array(vec![
                Value::String("cool".to_owned()).into(),
                Value::String("wicked".to_owned()).into()
            ])
        )
    }
    #[test]
    fn evaluate_irange_as_index() {
        let parsed = &parse("['nice', 'cool', 'wicked', 'sick'][1..=2]")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(
            evaluated,
            Value::Array(vec![
                Value::String("cool".to_owned()).into(),
                Value::String("wicked".to_owned()).into()
            ])
        )
    }

    #[test]
    fn evaluate_backwards_range_as_index() {
        let parsed = &parse("'wonderful'[-1..4]")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::String("lufr".to_owned()));
    }

    #[test]
    fn evaluate_backwards_irange_as_index() {
        let parsed = &parse("'sickening'[-4..=3]")[0];
        let evaluated = evaluate(parsed).unwrap();

        assert_eq!(evaluated, Value::String("nek".to_owned()));
    }
}
