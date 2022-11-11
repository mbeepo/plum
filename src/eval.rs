use std::collections::HashMap;

use chumsky::{Parser, Stream};

use crate::{
    error::Error,
    interpreter::{interpret, SpannedValue, Value},
    lexer, parser,
};

pub fn eval(input: impl Into<String>) -> Result<HashMap<String, Value>, Vec<Error>> {
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

    let mut vars: HashMap<String, Value> = HashMap::new();

    for expr in parsed.unwrap() {
        let interpreted = interpret(expr, vars.clone())?;

        match interpreted {
            SpannedValue(Value::Assign(name, value), _) => {
                vars.insert(name, *value);
            }
            _ => {}
        }
    }

    Ok(vars)
}
