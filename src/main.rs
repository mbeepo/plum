use std::ops::Range;

use chumsky::prelude::*;

#[derive(Clone, Debug)]
enum Expr {
    Ident(String),
    Num(u64),
    String(String),
    Bool(bool),
    Block(Vec<Spanned>),
    Conditional {
        condition: Box<Spanned>,
        inner: Box<Spanned>,
        other: Box<Spanned>,
    },
}

#[derive(Clone, Debug)]
struct Spanned(Expr, Range<usize>);

fn main() {
    let parsed = parse()
        .parse(r#"if cool { true; } else { "nice"; false; }"#)
        .unwrap();

    println!("{:?}", parsed);
}

fn parse() -> impl Parser<char, Spanned, Error = Simple<char>> {
    let ident = text::ident::<_, Simple<char>>()
        .padded()
        .map(Expr::Ident)
        .map_with_span(Spanned);

    let int = text::int(10)
        .padded()
        .from_str()
        .unwrapped()
        .map(Expr::Num)
        .map_with_span(Spanned);

    let string = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Expr::String)
        .map_with_span(Spanned);

    let boolean = text::keyword("true")
        .to(Expr::Bool(true))
        .or(text::keyword("false").to(Expr::Bool(false)))
        .map_with_span(Spanned);

    let atomic = ident.or(int).or(string).or(boolean);

    let block = just('{')
        .padded()
        .ignore_then(atomic.clone().then_ignore(just(';')).repeated())
        .then_ignore(just('}').padded())
        .map(Expr::Block)
        .map_with_span(Spanned);

    let conditional = text::keyword("if")
        .ignore_then(atomic.clone())
        .then(block.clone())
        .then_ignore(text::keyword("else"))
        .then(block)
        .map(|((condition, inner), other)| Expr::Conditional {
            condition: Box::new(condition),
            inner: Box::new(inner),
            other: Box::new(other),
        })
        .map_with_span(Spanned)
        .then_ignore(end());

    conditional.or(atomic)
}
