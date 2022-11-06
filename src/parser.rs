use chumsky::prelude::*;

use crate::ast::{Expr, InfixOp, Literal, Spanned};

pub fn parse() -> impl Parser<char, Spanned, Error = Simple<char>> {
    recursive(|expr| {
        let raw_expr = recursive(|raw_expr| {
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
                .map(Literal::Num)
                .map(Expr::Literal)
                .map_with_span(Spanned)
                .labelled("number");

            // string stuff
            let escape = just('\\').ignore_then(
                just('\\')
                    .or(just('/'))
                    .or(just('"'))
                    .or(just('\''))
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

            let string = d_string
                .or(s_string)
                .map(Literal::String)
                .map(Expr::Literal)
                .map_with_span(Spanned);

            // array stuff
            let array = raw_expr
                .clone()
                .separated_by(just(',').padded())
                .allow_trailing()
                .delimited_by(just('['), just(']'))
                .map(Literal::Array)
                .map(Expr::Literal)
                .map_with_span(Spanned)
                .labelled("array");

            // identifiers
            let ident = text::ident().map(Expr::Ident).map_with_span(Spanned);

            // any single value
            let single = raw_expr
                .clone()
                .delimited_by(just('('), just(')'))
                .or(just("true")
                    .to(Expr::Literal(Literal::Bool(true)))
                    .map_with_span(Spanned))
                .labelled("true")
                .or(just("false")
                    .to(Expr::Literal(Literal::Bool(false)))
                    .map_with_span(Spanned))
                .labelled("false")
                .or(num)
                .or(string)
                .or(array)
                .or(ident);

            // operators
            let op = |c| just(c).padded();

            let index = single
                .clone()
                .then(
                    single
                        .clone()
                        .delimited_by(just('['), just(']'))
                        .repeated()
                        .labelled("index"),
                )
                .foldl(|lhs, rhs| {
                    let range = lhs.1.start()..rhs.1.end();

                    Spanned(Expr::Index(Box::new(lhs), Box::new(rhs)), range)
                });

            let exp = index
                .clone()
                .then(
                    just("**")
                        .padded()
                        .to(InfixOp::Pow)
                        .then(index)
                        .repeated()
                        .labelled("exponent"),
                )
                .foldl(|lhs, (op, rhs)| {
                    let range = lhs.1.start()..rhs.1.end();

                    Spanned(Expr::InfixOp(Box::new(lhs), op, Box::new(rhs)), range)
                });

            let product = exp
                .clone()
                .then(
                    op('*')
                        .to(InfixOp::Mul)
                        .labelled("mul")
                        .or(op('/').to(InfixOp::Div).labelled("div"))
                        .or(op('%').to(InfixOp::Mod).labelled("mod"))
                        .then(exp)
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| {
                    let range = lhs.1.start()..rhs.1.end();

                    Spanned(Expr::InfixOp(Box::new(lhs), op, Box::new(rhs)), range)
                });

            let sum = product
                .clone()
                .then(
                    op('+')
                        .to(InfixOp::Add)
                        .labelled("add")
                        .or(op('-').to(InfixOp::Sub).labelled("sub"))
                        .then(product)
                        .repeated(),
                )
                .foldl(|lhs, (op, rhs)| {
                    let range = lhs.1.start()..rhs.1.end();

                    Spanned(Expr::InfixOp(Box::new(lhs), op, Box::new(rhs)), range)
                });

            sum
        });

        // block for conditionals
        let block = expr
            .clone()
            .then_ignore(just(';'))
            .padded()
            .repeated()
            .at_least(1)
            .map(Expr::Block)
            .map_with_span(Spanned)
            .labelled("block");

        // conditionals
        let conditional = recursive(|cond| {
            text::keyword("if")
                .ignore_then(raw_expr.clone())
                .then_ignore(just('{').padded())
                .then(block.clone())
                .then_ignore(just('{').padded())
                .then(
                    text::keyword("else")
                        .padded()
                        .ignore_then(just('{').padded().ignore_then(block))
                        .or(cond)
                        .then_ignore(just('}').padded()),
                )
                .map(|((condition, inner), other)| Expr::Conditional {
                    condition: Box::new(condition),
                    inner: Box::new(inner),
                    other: Box::new(other),
                })
                .map_with_span(Spanned)
                .labelled("conditional")
        });

        conditional.or(raw_expr)
    })
    .then_ignore(end().recover_with(skip_then_retry_until([])))
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        ast::{Expr, InfixOp, Literal, Spanned},
        errors::ChumskyAriadne,
        parser::parse,
    };

    #[test]
    fn parse_int() {
        let parsed = parse().parse("89").unwrap();

        assert_eq!(parsed, Spanned::from(89.0));
    }

    #[test]
    fn parse_neg() {
        let parsed = parse().parse("-23").unwrap();

        assert_eq!(parsed, Spanned::from(-23.0));
    }

    #[test]
    fn parse_frac() {
        let parsed = parse().parse("32.5892").unwrap();

        assert_eq!(parsed, Spanned::from(32.5892));
    }

    #[test]
    fn parse_sci() {
        let parsed = parse().parse("3e14").unwrap();

        assert_eq!(parsed, Spanned::from(300000000000000.0));
    }

    #[test]
    fn parse_exp() {
        let parsed = parse().parse("1.82e2").unwrap();

        assert_eq!(parsed, Spanned::from(182.0));
    }

    #[test]
    fn parse_d_string() {
        let parsed = parse().parse("\"nice\"").unwrap();

        assert_eq!(parsed, Spanned::from("nice"));
    }

    #[test]
    fn parse_s_string() {
        let parsed = parse().parse("'cool'").unwrap();

        assert_eq!(parsed, Spanned::from("cool"));
    }

    #[test]
    fn escaped_s_string() {
        let parsed = parse().parse("'this is \\'nice\\' and \"cool\"'").unwrap();

        assert_eq!(parsed, Spanned::from("this is 'nice' and \"cool\""));
    }

    #[test]
    fn escaped_d_string() {
        let parsed = parse()
            .parse("\"this is 'nice' and \\\"cool\\\"\"")
            .unwrap();

        assert_eq!(parsed, Spanned::from("this is 'nice' and \"cool\""));
    }

    #[test]
    fn parse_true() {
        let parsed = parse().parse("true").unwrap();

        assert_eq!(parsed, Spanned::from(true));
    }

    #[test]
    fn parse_false() {
        let parsed = parse().parse("false").unwrap();

        assert_eq!(parsed, Spanned::from(false));
    }

    #[test]
    fn parse_num_array() {
        let parsed = parse().parse("[1, 3.73, 2, 5.98e-2, 4]").unwrap();

        assert_eq!(
            parsed.0,
            Expr::Literal(Literal::Array(vec![
                Spanned::from(1.0),
                Spanned::from(3.73),
                Spanned::from(2.0),
                Spanned::from(0.0598),
                Spanned::from(4.0)
            ]))
        );
    }

    #[test]
    fn parse_mixed_array() {
        let parsed = parse()
            .parse("[1, true, 2, false, 'nice', 935328.478]")
            .unwrap();

        assert_eq!(
            parsed.0,
            Expr::Literal(Literal::Array(vec![
                Spanned::from(1.0),
                Spanned::from(true),
                Spanned::from(2.0),
                Spanned::from(false),
                Spanned::from("nice"),
                Spanned::from(935328.478)
            ]))
        );
    }

    #[test]
    fn parse_ident() {
        let parsed = parse().parse("nice").unwrap();

        assert_eq!(parsed.0, Expr::Ident("nice".to_owned()));
    }

    #[test]
    fn parse_mul() {
        let parsed = parse().parse("3 * 7").unwrap();

        assert_eq!(
            parsed.0,
            Expr::InfixOp(
                Box::new(Spanned::from(3.0)),
                InfixOp::Mul,
                Box::new(Spanned::from(7.0))
            )
        );
    }

    #[test]
    fn parse_add() {
        let parsed = parse().parse("10 + 83").unwrap();

        assert_eq!(
            parsed.0,
            Expr::InfixOp(
                Box::new(Spanned::from(10.0)),
                InfixOp::Add,
                Box::new(Spanned::from(83.0))
            )
        );
    }

    #[test]
    fn parse_chained() {
        let parsed = parse().parse("10 + (30 - 5) * 3 ** 2").unwrap();

        assert_eq!(
            parsed.0,
            Expr::InfixOp(
                Box::new(Spanned::from(10.0)),
                InfixOp::Add,
                Box::new(Spanned::from(Expr::InfixOp(
                    Box::new(Spanned::from(Expr::InfixOp(
                        Box::new(Spanned::from(30.0)),
                        InfixOp::Sub,
                        Box::new(Spanned::from(5.0))
                    ))),
                    InfixOp::Mul,
                    Box::new(Spanned::from(Expr::InfixOp(
                        Box::new(Spanned::from(3.0)),
                        InfixOp::Pow,
                        Box::new(Spanned::from(2.0))
                    )))
                )))
            )
        );
    }

    #[test]
    fn parse_array_index() {
        let parsed = parse().parse("[1, 2, 3, 4][3]").unwrap();

        assert_eq!(
            parsed.0,
            Expr::Index(
                Box::new(Spanned::from(vec![
                    Spanned::from(1.0),
                    Spanned::from(2.0),
                    Spanned::from(3.0),
                    Spanned::from(4.0)
                ])),
                Box::new(Spanned::from(3.0))
            )
        )
    }

    #[test]
    fn parse_conditional() {
        let input = "if cool {
			36;
		} else {
			nice;
		}";
        let (parsed, errs) = parse().parse_recovery_verbose(input);

        for err in errs {
            err.display("[input]", input, 0)
        }

        assert_eq!(
            parsed.unwrap(),
            Expr::Conditional {
                condition: Box::new(Spanned::from(Expr::Ident("cool".to_owned()))),
                inner: Box::new(Spanned::from(Expr::Block(vec![Spanned::from(36.0)]))),
                other: Box::new(Spanned::from(Expr::Block(vec![Spanned::from(
                    Expr::Ident("nice".to_owned())
                )])))
            }
        )
    }
}
