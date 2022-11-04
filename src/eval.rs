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

pub fn eval(source: &str, split_on: &str) -> HashMap<String, Value> {
    let mut output: HashMap<String, Value> = HashMap::new();
    let input = split_with_offset(source, split_on);

    for (offset, part) in input {
        let mut parts = split_with_offset(part, "=");

        let (_ident_offset, ident) = parts.next().unwrap();
        let (expr_offset, expr) = parts.next().unwrap();

        let ident = ident.trim();
        let expr = expr.trim();

        let parsed = parse().parse(expr);
        let parsed = match parsed {
            Err(e) => {
                for err in e {
                    eprintln!("{}", err);
                }

                continue;
            }
            Ok(e) => e,
        };

        let interpreted = interpret(parsed);
        let interpreted = match interpreted {
            Err(e) => {
                for err in e {
                    err.display("[stdin]", source, expr_offset + offset);
                }

                continue;
            }
            Ok(e) => e.0,
        };

        output.insert(ident.to_owned(), interpreted);
    }

    output
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::interpreter::Value;

    use super::eval;

    #[test]
    fn assign_one() {
        let out = eval("e = 45", "#split");

        let expected = [("e".to_owned(), Value::Num(45.0))];
        let expected: HashMap<String, Value> = expected.into_iter().collect();

        assert_eq!(out, expected)
    }

    #[test]
    fn assign_multiple() {
        let out = eval(
            r#"
			nice = 23
			#split
			cool = "epic"
			#split
			gnar = "sickening"
			#split
			wicked = true
		"#,
            "#split",
        );

        let expected = [
            ("nice".to_owned(), Value::Num(23.0)),
            ("cool".to_owned(), Value::String("epic".to_owned())),
            ("gnar".to_owned(), Value::String("sickening".to_owned())),
            ("wicked".to_owned(), Value::Bool(true)),
        ];
        let expected: HashMap<String, Value> = expected.into_iter().collect();

        assert_eq!(out, expected)
    }
}
