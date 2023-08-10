use std::env;

use chumsky::{Stream, Parser};
use plum::{lexer, error::{Error, ChumskyAriadne}, parser, Spanned};

fn main() {
    let path = &env::args().collect::<Vec<String>>()[1];
    let file = std::fs::read(path).unwrap();
    let source = String::from_utf8(file).unwrap();
    let source = source.as_ref();

    let ast = read(source);

    match ast {
        Err(errs) => {
            for err in errs {
                err.display(path, source, 0);
            }
        }
        Ok(out) => println!("{:#?}", out),
    }
}

fn read(input: &str) -> Result<Vec<Spanned>, Vec<Error>> {
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

    Ok(parsed.unwrap())
}