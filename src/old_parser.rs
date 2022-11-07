pub fn parse() -> impl Parser<Token, Spanned, Error = Simple<char>> {
    recursive(|expr| {
        let raw_expr = recursive(|raw_expr| {
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

            // any single value
            let atomic = raw_expr
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

            let index = atomic
                .clone()
                .then(
                    atomic
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

        let conditional = recursive(|cond| {
            // block for conditionals
            let block = expr
                .clone()
                .then_ignore(just(';'))
                .padded()
                .or(cond.clone())
                .repeated()
                .at_least(1)
                .map(Expr::Block)
                .map_with_span(Spanned)
                .labelled("block");

            // conditionals
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
