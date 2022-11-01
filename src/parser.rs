use chumsky::prelude::*;

use crate::ast::{Expr, Literal};

// a lot of this is very heavily based on the Chumsky JSON example
// https://github.com/zesterer/chumsky/blob/master/examples/json.rs
pub fn parse() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr| {
        let frac = just('.').chain(text::digits(10));
        let exp = just('e')
            .or(just('E'))
            .chain(just('+').or(just('-')).or_not())
            .chain::<char, _, _>(text::digits(10));
        let num = just('-')
            .or_not()
            .chain::<char, _, _>(text::int(10))
            .chain::<char, _, _>(frac.or_not().flatten())
            .chain::<char, _, _>(exp.or_not().flatten())
            .collect::<String>()
            .from_str()
            .unwrapped()
            .labelled("number");

        // string stuff
        let escape = just('\\').ignore_then(
            just('\\')
                .or(just('/'))
                .or(just('"'))
                .or(just('b').to('\x08'))
                .or(just('f').to('\x0C'))
                .or(just('n').to('\n'))
                .or(just('r').to('\r'))
                .or(just('t').to('\t'))
                .or(just('u').ignore_then(
                    filter(|c: &char| c.is_digit(16))
                        .repeated()
                        .exactly(4)
                        .collect::<String>()
                        .validate(|digits, span, emit| {
                            char::from_u32(u32::from_str_radix(&digits, 16).unwrap())
                                .unwrap_or_else(|| {
                                    emit(Simple::custom(span, "invalid unicode character"));
                                    '\u{FFFD}' // unicode replacement character
                                })
                        }),
                )),
        );

        let d_string = just('"')
            .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape).repeated())
            .then_ignore(just('"'))
            .collect::<String>()
            .labelled("string");

        let s_string = just('\'')
            .ignore_then(filter(|c| *c != '\\' && *c != '\'').or(escape).repeated())
            .then_ignore(just('\''))
            .collect::<String>()
            .labelled("string");

        let string = d_string.or(s_string);

        // array stuff
        let array = expr
            .clone()
            .chain(just(',').padded().ignore_then(expr.clone()).repeated())
            .or_not()
            .flatten()
            .delimited_by(just('['), just(']'))
            .map(Literal::Array)
            .map(Expr::Literal)
            .labelled("array");

        //
        just("true")
            .to(Expr::Literal(Literal::True))
            .labelled("true")
            .or(just("false").to(Expr::Literal(Literal::False)))
            .labelled("false")
            .or(num.map(Literal::Num).map(Expr::Literal))
            .or(string.map(Literal::String).map(Expr::Literal))
            .or(array)
            .or(text::ident().map(Expr::Ident))
    })
    .then_ignore(end().recover_with(skip_then_retry_until([])))
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        ast::{Expr, Literal},
        parser::parse,
    };

    #[test]
    fn parse_int() {
        let parsed = parse().parse("89");

        assert_eq!(parsed, Ok(Expr::from(89.0)));
    }

    #[test]
    fn parse_neg() {
        let parsed = parse().parse("-23");

        assert_eq!(parsed, Ok(Expr::from(-23.0)));
    }

    #[test]
    fn parse_frac() {
        let parsed = parse().parse("32.5892");

        assert_eq!(parsed, Ok(Expr::from(32.5892)));
    }

    #[test]
    fn parse_exp() {
        let parsed = parse().parse("1.82e2");

        assert_eq!(parsed, Ok(Expr::from(182.0)));
    }

    #[test]
    fn parse_d_string() {
        let parsed = parse().parse("\"nice\"");

        assert_eq!(parsed, Ok(Expr::from("nice")));
    }

    #[test]
    fn parse_s_string() {
        let parsed = parse().parse("'cool'");

        assert_eq!(parsed, Ok(Expr::from("cool")));
    }

    #[test]
    fn parse_true() {
        let parsed = parse().parse("true");

        assert_eq!(parsed, Ok(Expr::from(true)));
    }

    #[test]
    fn parse_false() {
        let parsed = parse().parse("false");

        assert_eq!(parsed, Ok(Expr::from(false)));
    }

    #[test]
    fn parse_num_array() {
        let parsed = parse().parse("[1, 3.73, 2, 5.98e-2, 4]");

        assert_eq!(
            parsed,
            Ok(Expr::Literal(Literal::Array(vec![
                Expr::from(1.0),
                Expr::from(3.73),
                Expr::from(2.0),
                Expr::from(0.0598),
                Expr::from(4.0)
            ])))
        );
    }

    #[test]
    fn parse_mixed_array() {
        let parsed = parse().parse("[1, true, 2, false, 'nice', 935328.478]");

        assert_eq!(
            parsed,
            Ok(Expr::Literal(Literal::Array(vec![
                Expr::from(1.0),
                Expr::from(true),
                Expr::from(2.0),
                Expr::from(false),
                Expr::from("nice"),
                Expr::from(935328.478)
            ])))
        );
    }

    #[test]
    fn escaped_s_string() {
        let parsed = parse().parse("'this is \\'nice\\' and \"cool\"'");

        assert_eq!(parsed, Ok(Expr::from("this is 'nice' and \"cool\"")));
    }

    #[test]
    fn escaped_d_string() {
        let parsed = parse().parse("\"this is 'nice' and \\\"cool\\\"\"");

        assert_eq!(parsed, Ok(Expr::from("this is 'nice' and \"cool\"")));
    }
}
