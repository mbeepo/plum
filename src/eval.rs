use std::collections::HashMap;

use chumsky::Parser;

use crate::{
    interpreter::{interpret, Value},
    parser::parse,
};

fn addr_of(s: &str) -> usize {
    s.as_ptr() as usize
}

fn split_with_offset<'a>(s: &'a str, on: &'a str) -> impl Iterator<Item = (usize, &'a str)> {
    s.split(on).map(move |sub| (addr_of(sub) - addr_of(s), sub))
}

pub fn eval(input: &str, split_on: &str) -> HashMap<String, Value> {
    let output: HashMap<String, Value> = HashMap::new();
    let input = split_with_offset(input, split_on);

    for (offset, part) in input {
        let parts = split_with_offset(part, "=");

        let (ident_offset, ident) = parts.next().unwrap();
        let (expr_offset, expr) = parts.next().unwrap();

        let ident = ident.trim();
        let expr = expr.trim();

        let parsed = parse().parse(expr);
        let parsed = match parsed {
            Err(e) => {
                for err in e {
                    e.display();
                }

                continue;
            }
            Ok(e) => e,
        };

        let interpreted = interpret(parsed);
        let interpreted = match interpreted {
            Err(e) => {
                for err in e {
                    e.display();
                }

                continue;
            }
            Ok(e) => e,
        };

        output.insert(ident.to_owned(), interpreted);
    }

    output
}
