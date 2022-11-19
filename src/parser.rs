use chumsky::prelude::*;

use crate::ast::{Expr, InfixOp, Literal, Spanned, Token};

pub fn parse() -> impl Parser<Token, Vec<Spanned>, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let raw_expr = recursive(|raw_expr| {
            let val = select! {
                Token::Num(e) => Expr::from(e.parse::<f64>().unwrap()),
                Token::String(e) => Expr::from(e),
                Token::Bool(e) => Expr::from(e),
                Token::Null => Expr::Literal(Literal::Null),
            }
            .labelled("value")
            .map_with_span(Spanned);

            let ident = select! { Token::Ident(ident) => ident.clone() }.labelled("identifier");

            // Array items
            let items = expr
                .clone()
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing();

            let array = items
                .clone()
                .delimited_by(just(Token::Ctrl('[')), just(Token::Ctrl(']')))
                .map(Literal::Array)
                .map(Expr::Literal)
                .map_with_span(Spanned);

            let assign = ident
                .clone()
                .chain(
                    just(Token::Op("=".to_owned()))
                        .ignore_then(ident)
                        .then_ignore(just(Token::Op("=".to_owned())))
                        .repeated(),
                )
                .then(just(Token::Op("=".to_owned())).ignore_then(expr.clone()))
                .then_ignore(just(Token::Ctrl(';')))
                .map(|(names, val)| Expr::Assign {
                    names,
                    value: Box::new(val),
                })
                .map_with_span(Spanned);

            let single_expr = expr
                .clone()
                .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')')));

            let ident = ident.map(Expr::Ident).map_with_span(Spanned);
            let atom = choice((val, assign, ident, array, single_expr))
                .recover_with(nested_delimiters(
                    Token::Ctrl('('),
                    Token::Ctrl(')'),
                    [
                        (Token::Ctrl('['), Token::Ctrl(']')),
                        (Token::Ctrl('{'), Token::Ctrl('}')),
                    ],
                    |span| Spanned(Expr::Error, span),
                ))
                // Attempt to recover anything that looks like a list but contains errors
                .recover_with(nested_delimiters(
                    Token::Ctrl('['),
                    Token::Ctrl(']'),
                    [
                        (Token::Ctrl('('), Token::Ctrl(')')),
                        (Token::Ctrl('{'), Token::Ctrl('}')),
                    ],
                    |span| Spanned(Expr::Error, span),
                ))
                .boxed();

            let index = atom
                .then(
                    expr.clone()
                        .delimited_by(just(Token::Ctrl('[')), just(Token::Ctrl(']')))
                        .repeated()
                        .labelled("index"),
                )
                .foldl(|lhs, rhs: Spanned| {
                    let span = lhs.1.start..rhs.1.end;

                    Spanned(Expr::Index(Box::new(lhs), Box::new(rhs)), span)
                });

            let op = just(Token::Op("..=".to_owned())).to(InfixOp::IRange);
            let irange = index
                .clone()
                .then(op.then(index).repeated())
                .foldl(|lhs, (op, rhs)| spannify(lhs, op, rhs))
                .boxed();

            let op = just(Token::Op("..".to_owned())).to(InfixOp::Range);
            let range = irange
                .clone()
                .then(op.then(irange).repeated())
                .foldl(|lhs, (op, rhs)| spannify(lhs, op, rhs));

            let op = just(Token::Op("**".to_owned())).to(InfixOp::Pow);
            let pow = range
                .clone()
                .then(op.then(range).repeated())
                .foldl(|lhs, (op, rhs)| spannify(lhs, op, rhs))
                .boxed();

            let op = choice((
                just(Token::Op("*".to_owned()))
                    .labelled("multiply")
                    .to(InfixOp::Mul),
                just(Token::Op("/".to_owned()))
                    .labelled("divide")
                    .to(InfixOp::Div),
                just(Token::Op("%".to_owned()))
                    .labelled("modulus")
                    .to(InfixOp::Mod),
            ));
            let product = pow
                .clone()
                .then(op.then(pow).repeated())
                .foldl(|lhs, (op, rhs)| spannify(lhs, op, rhs));

            let op = choice((
                just(Token::Op("+".to_owned()))
                    .labelled("add")
                    .to(InfixOp::Add),
                just(Token::Op("-".to_owned()))
                    .labelled("subtract")
                    .to(InfixOp::Sub),
            ));
            let sum = product
                .clone()
                .then(op.then(product).repeated())
                .foldl(|lhs, (op, rhs)| spannify(lhs, op, rhs))
                .boxed();

            let op = choice((
                just(Token::Op("==".to_owned()))
                    .labelled("equals")
                    .to(InfixOp::Equals),
                just(Token::Op("<=".to_owned()))
                    .labelled("less than or equal")
                    .to(InfixOp::Lte),
                just(Token::Op(">=".to_owned()))
                    .labelled("greater than or equal")
                    .to(InfixOp::Gte),
                just(Token::Op("<".to_owned()))
                    .labelled("less than")
                    .to(InfixOp::Lt),
                just(Token::Op(">".to_owned()))
                    .labelled("greater than")
                    .to(InfixOp::Gt),
            ));
            let compare = sum
                .clone()
                .then(op.then(sum).repeated())
                .foldl(|lhs, (op, rhs)| spannify(lhs, op, rhs));

            let op = just(Token::Op("in".to_owned()))
                .labelled("in")
                .to(InfixOp::In);
            let contains = compare
                .clone()
                .then(op.then(compare).repeated())
                .foldl(|lhs, (op, rhs)| spannify(lhs, op, rhs))
                .boxed();

            let op = choice((
                just(Token::Op("and".to_owned())),
                just(Token::Op("&&".to_owned())),
            ))
            .labelled("and")
            .to(InfixOp::And);
            let and = contains
                .clone()
                .then(op.then(contains).repeated())
                .foldl(|lhs, (op, rhs)| spannify(lhs, op, rhs));

            let op = choice((
                just(Token::Op("or".to_owned())),
                just(Token::Op("||".to_owned())),
            ))
            .labelled("or")
            .to(InfixOp::Or);
            let or = and
                .clone()
                .then(op.then(and).repeated())
                .foldl(|lhs, (op, rhs)| spannify(lhs, op, rhs));

            or
        });

        let conditional = recursive(|cond| {
            let block = expr
                .clone()
                .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')));

            just(Token::If)
                .ignore_then(expr.clone())
                .then(block.clone())
                .then_ignore(just(Token::Else))
                .then(block.or(cond))
                .map(|((condition, inner), other)| {
                    let span = condition.1.start..other.1.end;

                    Spanned(
                        Expr::Conditional {
                            condition: Box::new(condition),
                            inner: Box::new(inner),
                            other: Box::new(other),
                        },
                        span,
                    )
                })
        });

        conditional.or(raw_expr)
    })
    .repeated()
    .at_least(1)
    .then_ignore(end())
}

fn spannify(lhs: Spanned, op: InfixOp, rhs: Spanned) -> Spanned {
    let span = lhs.1.start..rhs.1.end;

    Spanned(Expr::InfixOp(Box::new(lhs), op, Box::new(rhs)), span)
}

#[cfg(test)]
mod tests {
    use chumsky::{Parser, Stream};

    use crate::{
        ast::{Expr, InfixOp, Literal, Spanned},
        lexer::lexer,
        parser,
    };

    fn parse(input: &str) -> Vec<Spanned> {
        let len = input.len();

        let lexed = lexer().parse(input).unwrap();
        parser::parse()
            .parse(Stream::from_iter(len..len + 1, lexed.into_iter()))
            .unwrap()
    }

    #[test]
    fn parse_neg() {
        let parsed = parse("-23");
        assert_eq!(parsed[0], Spanned::from(-23.0));
    }

    #[test]
    fn parse_frac() {
        let parsed = parse("32.5892");

        assert_eq!(parsed[0], Spanned::from(32.5892));
    }

    #[test]
    fn parse_exp() {
        let parsed = parse("1.82e2");

        assert_eq!(parsed[0], Spanned::from(182.0));
    }

    #[test]
    fn parse_num_array() {
        let parsed = parse("[1, 3.73, 2, 5.98e-2, 4]");

        assert_eq!(
            parsed[0],
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
        let parsed = parse("[1, true, 2, false, 'nice', 935328.478]");

        assert_eq!(
            parsed[0],
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
    fn parse_mul() {
        let parsed = parse("3 * 7");
        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(3.0)),
                InfixOp::Mul,
                Box::new(Spanned::from(7.0))
            )
        );
    }

    #[test]
    fn parse_add() {
        let parsed = parse("10 + 83");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(10.0)),
                InfixOp::Add,
                Box::new(Spanned::from(83.0))
            )
        );
    }

    #[test]
    fn parse_chained() {
        let parsed = parse("10 + (30 - 5) * 3 ** 2");

        assert_eq!(
            parsed[0],
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
    fn parse_assign() {
        let parsed = parse("nice = 12;");

        assert_eq!(
            parsed[0],
            Expr::Assign {
                names: vec!["nice".to_owned()],
                value: Box::new(Spanned::from(12.0))
            }
        )
    }

    #[test]
    fn parse_array_index() {
        let parsed = parse("[1, 2, 3, 4][3]");

        assert_eq!(
            parsed[0],
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
        let parsed = parse(
            "if cool {
			36
		} else {
			nice
		}",
        );

        assert_eq!(
            parsed[0],
            Expr::Conditional {
                condition: Box::new(Spanned::from(Expr::Ident("cool".to_owned()))),
                inner: Box::new(Spanned::from(36.0)),
                other: Box::new(Spanned::from(Expr::Ident("nice".to_owned())))
            }
        )
    }

    #[test]
    fn parse_and() {
        let parsed = parse("cool and good");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(Expr::Ident("cool".to_owned()))),
                InfixOp::And,
                Box::new(Spanned::from(Expr::Ident("good".to_owned())))
            )
        )
    }

    #[test]
    fn parse_or() {
        let parsed = parse("cool or good");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(Expr::Ident("cool".to_owned()))),
                InfixOp::Or,
                Box::new(Spanned::from(Expr::Ident("good".to_owned())))
            )
        )
    }

    #[test]
    fn parse_mixed_and() {
        let parsed = parse("cool and good && nice");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(Expr::InfixOp(
                    Box::new(Spanned::from(Expr::Ident("cool".to_owned()))),
                    InfixOp::And,
                    Box::new(Spanned::from(Expr::Ident("good".to_owned())))
                ))),
                InfixOp::And,
                Box::new(Spanned::from(Expr::Ident("nice".to_owned()))),
            )
        )
    }

    #[test]
    fn parse_multiple_assign() {
        let parsed = parse(
            "cool = 3;
		nice = cool + 7;
		sick = false;",
        );

        assert_eq!(
            parsed,
            vec![
                Expr::Assign {
                    names: vec!["cool".to_owned()],
                    value: Box::new(Spanned::from(3.0))
                },
                Expr::Assign {
                    names: vec!["nice".to_owned()],
                    value: Box::new(Spanned::from(Expr::InfixOp(
                        Box::new(Spanned::from(Expr::Ident("cool".to_owned()))),
                        InfixOp::Add,
                        Box::new(Spanned::from(7.0))
                    )))
                },
                Expr::Assign {
                    names: vec!["sick".to_owned()],
                    value: Box::new(Spanned::from(false)),
                }
            ]
        )
    }

    #[test]
    fn parse_assign_to_conditional() {
        let parsed = parse(
            "cool = if nice {
			30
		} else {
			10
		};",
        );

        assert_eq!(
            parsed[0],
            Expr::Assign {
                names: vec!["cool".to_owned()],
                value: Box::new(Spanned::from(Expr::Conditional {
                    condition: Box::new(Spanned::from(Expr::Ident("nice".to_owned()))),
                    inner: Box::new(Spanned::from(30.0)),
                    other: Box::new(Spanned::from(10.0))
                }))
            }
        )
    }

    #[test]
    fn parse_equals() {
        let parsed = parse("10 == 12");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(10.0)),
                InfixOp::Equals,
                Box::new(Spanned::from(12.0))
            )
        )
    }

    #[test]
    fn parse_lt() {
        let parsed = parse("10 < 12");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(10.0)),
                InfixOp::Lt,
                Box::new(Spanned::from(12.0))
            )
        )
    }

    #[test]
    fn parse_gt() {
        let parsed = parse("10 > 12");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(10.0)),
                InfixOp::Gt,
                Box::new(Spanned::from(12.0))
            )
        )
    }

    #[test]
    fn parse_lte() {
        let parsed = parse("10 <= 12");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(10.0)),
                InfixOp::Lte,
                Box::new(Spanned::from(12.0))
            )
        )
    }

    #[test]
    fn parse_gte() {
        let parsed = parse("10 >= 12");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(10.0)),
                InfixOp::Gte,
                Box::new(Spanned::from(12.0))
            )
        )
    }

    #[test]
    fn parse_contains() {
        let parsed = parse("12 in [10, 11, 12, 13, 14]");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(12.0)),
                InfixOp::In,
                Box::new(Spanned::from(vec![
                    Spanned::from(10.0),
                    Spanned::from(11.0),
                    Spanned::from(12.0),
                    Spanned::from(13.0),
                    Spanned::from(14.0)
                ]))
            )
        )
    }

    #[test]
    fn parse_assign_chain() {
        let parsed = parse("these = are = all = 12;");

        assert_eq!(
            parsed[0],
            Expr::Assign {
                names: vec!["these".to_owned(), "are".to_owned(), "all".to_owned()],
                value: Box::new(Spanned::from(12.0))
            }
        )
    }

    #[test]
    fn parse_inclusive_range() {
        let parsed = parse("0..=10");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(0.0)),
                InfixOp::IRange,
                Box::new(Spanned::from(10.0))
            )
        )
    }

    #[test]
    fn parse_exclusive_range() {
        let parsed = parse("0..10");

        assert_eq!(
            parsed[0],
            Expr::InfixOp(
                Box::new(Spanned::from(0.0)),
                InfixOp::Range,
                Box::new(Spanned::from(10.0))
            )
        )
    }
}
