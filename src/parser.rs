use chumsky::prelude::*;

use crate::ast::{Expr, InfixOp, Literal, Spanned, Token};

pub fn parse() -> impl Parser<Token, Spanned, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let raw_expr = recursive(|raw_expr| {
            let val = select! {
                Token::Num(e) => Expr::from(e.parse::<f64>().unwrap()),
                Token::String(e) => Expr::from(e),
                Token::Bool(e) => Expr::from(e),
                Token::Null => Expr::Literal(Literal::Null),
            }
            .labelled("value");

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
                .map(Expr::Literal);

            let assign = ident
                .then_ignore(just(Token::Op("=".to_owned())))
                .then(raw_expr.clone())
                .then_ignore(just(Token::Ctrl(';')))
                .then(expr.clone())
                .map(|((name, val), then)| Expr::Assign {
                    name,
                    val: Box::new(val),
                });

            let atom = val
                .or(ident.map(Expr::Ident))
                .or(assign)
                .or(array)
                .map_with_span(Spanned)
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))))
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
                ));

            let index = atom
                .clone()
                .then(
                    raw_expr
                        .clone()
                        .delimited_by(just(Token::Ctrl('[')), just(Token::Ctrl(']')))
                        .repeated()
                        .labelled("index"),
                )
                .foldl(|lhs, rhs| {
                    let span = lhs.1.start()..rhs.1.end();

                    Spanned(Expr::Index(Box::new(lhs), Box::new(rhs)), span)
                });

            let op = just(Token::Op("**".to_owned())).to(InfixOp::Pow);
            let pow = index
                .clone()
                .then(op.then(index).repeated())
                .foldl(|lhs, (op, rhs)| {
                    let span = lhs.1.start..rhs.1.end;

                    Spanned(Expr::InfixOp(Box::new(lhs), op, Box::new(rhs)), span)
                });

            let op = just(Token::Op("*".to_owned()))
                .labelled("multiply")
                .to(InfixOp::Mul)
                .or(just(Token::Op("/".to_owned()))
                    .labelled("divide")
                    .to(InfixOp::Div))
                .or(just(Token::Op("%".to_owned()))
                    .labelled("modulus")
                    .to(InfixOp::Mod));
            let product = pow
                .clone()
                .then(op.then(pow).repeated())
                .foldl(|lhs, (op, rhs)| {
                    let span = lhs.1.start..rhs.1.end;

                    Spanned(Expr::InfixOp(Box::new(lhs), op, Box::new(rhs)), span)
                });

            let op = just(Token::Op("+".to_owned()))
                .labelled("add")
                .to(InfixOp::Add)
                .or(just(Token::Op("-".to_owned()))
                    .labelled("subtract")
                    .to(InfixOp::Sub));
            let sum = product
                .clone()
                .then(op.then(product).repeated())
                .foldl(|lhs, (op, rhs)| {
                    let span = lhs.1.start..rhs.1.end;

                    Spanned(Expr::InfixOp(Box::new(lhs), op, Box::new(rhs)), span)
                });
        });

        raw_expr
    })
    .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use chumsky::{Parser, Stream};

    use crate::{
        ast::{Expr, InfixOp, Literal, Spanned},
        errors::ChumskyAriadne,
        lexer::lexer,
        parser::parse,
    };

    #[test]
    fn parse_neg() {
        let lexed = lexer().parse("-23").unwrap();
        let parsed = parse()
            .parse(Stream::from_iter(3..4, lexed.into_iter()))
            .unwrap();

        assert_eq!(parsed, Spanned::from(-23.0));
    }

    #[test]
    fn parse_frac() {
        let lexed = lexer().parse("32.5892").unwrap();
        let parsed = parse()
            .parse(Stream::from_iter(7..8, lexed.into_iter()))
            .unwrap();

        assert_eq!(parsed, Spanned::from(32.5892));
    }

    #[test]
    fn parse_exp() {
        let lexed = lexer().parse("1.82e2").unwrap();
        let parsed = parse()
            .parse(Stream::from_iter(6..7, lexed.into_iter()))
            .unwrap();

        assert_eq!(parsed, Spanned::from(182.0));
    }

    #[test]
    fn parse_num_array() {
        let lexed = lexer().parse("[1, 3.73, 2, 5.98e-2, 4]").unwrap();
        let parsed = parse()
            .parse(Stream::from_iter(24..25, lexed.into_iter()))
            .unwrap();

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
        let lexed = lexer()
            .parse("[1, true, 2, false, 'nice', 935328.478]")
            .unwrap();
        let parsed = parse()
            .parse(Stream::from_iter(39..40, lexed.into_iter()))
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
    fn parse_mul() {
        let lexed = lexer().parse("3 * 7").unwrap();
        let parsed = parse()
            .parse(Stream::from_iter(5..6, lexed.into_iter()))
            .unwrap();

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
        let lexed = lexer().parse("10 + 83").unwrap();
        let parsed = parse()
            .parse(Stream::from_iter(7..8, lexed.into_iter()))
            .unwrap();

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
        let lexed = lexer().parse("10 + (30 - 5) * 3 ** 2").unwrap();
        let parsed = parse()
            .parse(Stream::from_iter(22..23, lexed.into_iter()))
            .unwrap();

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
        let lexed = lexer().parse("[1, 2, 3, 4][3]").unwrap();
        let parsed = parse()
            .parse(Stream::from_iter(15..16, lexed.into_iter()))
            .unwrap();

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
        let (lexed, errs) = lexer().parse_recovery(input);

        for err in errs {
            err.display("[input]", input, 0)
        }

        let (parsed, errs) =
            parse().parse_recovery(Stream::from_iter(44..45, lexed.unwrap().into_iter()));

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
