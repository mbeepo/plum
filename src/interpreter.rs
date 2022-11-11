use std::collections::HashMap;

use chumsky::{Parser, Stream};

use crate::{
    ast::{Expr, Span, Spanned},
    error::Error,
    eval::{eval, SpannedValue, Value},
    lexer, parser,
};

pub fn interpret(input: impl Into<String>) -> Result<HashMap<String, Value>, Vec<Error>> {
    let input = input.into();
    let len = input.len();

    let (lexed, errs) = lexer::lexer().parse_recovery(input);

    if errs.len() > 0 {
        return Err(errs.iter().map(|e| Error::SyntaxError(e.clone())).collect());
    }

    let (parsed, errs) =
        parser::parse().parse_recovery(Stream::from_iter(len..len + 1, lexed.unwrap().into_iter()));

    if errs.len() > 0 {
        return Err(errs
            .iter()
            .map(|e| Error::ParsingError(e.clone()))
            .collect());
    }

    let parsed = parsed.unwrap();
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    let mut spans: HashMap<String, Span> = HashMap::new();
    let mut errs: Vec<Error> = Vec::new();

    // check dependencies of variables
    for expr in parsed.iter() {
        match expr {
            Spanned(Expr::Assign { names, value }, span) => {
                let value_deps = get_deps(value);

                for name in names {
                    if let Some(old_span) = spans.get(name) {
                        let err = Error::ReassignError {
                            name: name.to_string(),
                            old_span: old_span.clone(),
                            new_span: span.clone(),
                        };
                        errs.push(err);
                    } else {
                        spans.insert(name.clone(), span.clone());
                        deps.insert(name.clone(), value_deps.clone());
                    }
                }
            }
            _ => {}
        }
    }

    if errs.len() > 0 {
        return Err(errs);
    }

    let mut vars: HashMap<String, Value> = HashMap::new();

    // actually evaluate the variables
    for expr in parsed.iter() {
        let interpreted = eval(expr, vars.clone())?;

        match interpreted {
            SpannedValue(Value::Assign(names, value), _) => {
                for name in names {
                    vars.insert(name, *value.clone());
                }
            }
            _ => {}
        }
    }

    if errs.len() > 0 {
        Err(errs)
    } else {
        Ok(vars)
    }
}

fn get_deps(expr: &Spanned) -> Vec<String> {
    let mut deps: Vec<String> = Vec::new();

    match expr {
        Spanned(Expr::Ident(name), _) => {
            deps.push(name.clone());
        }
        Spanned(Expr::InfixOp(lhs, _, rhs), _) => {
            let lhs_deps = get_deps(lhs);
            let rhs_deps = get_deps(rhs);

            deps.extend(lhs_deps);
            deps.extend(rhs_deps);
        }
        Spanned(Expr::Index(lhs, idx), _) => {
            let lhs_deps = get_deps(lhs);
            let idx_deps = get_deps(idx);

            deps.extend(lhs_deps);
            deps.extend(idx_deps);
        }
        Spanned(Expr::Not(rhs), _) => {
            let rhs_deps = get_deps(rhs);

            deps.extend(rhs_deps);
        }
        Spanned(Expr::Literal(_), _) => {}
        Spanned(Expr::Assign { names, value }, _) => {
            unreachable!("Assigns can never be in the value of an assignment")
        }
        Spanned(Expr::Error, _) => {}
        _ => todo!(),
    }

    deps
}

#[cfg(test)]
mod tests {
    use crate::eval::Value;

    use super::interpret;

    #[test]
    fn interpret_assign_chain() {
        let interpreted = interpret("these = are = all = 12;").unwrap();
        let value = Value::Num(12.0);

        assert_eq!(interpreted.get("these").unwrap(), &value);
        assert_eq!(interpreted.get("are").unwrap(), &value);
        assert_eq!(interpreted.get("all").unwrap(), &value)
    }
}
