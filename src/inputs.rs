use chumsky::{Parser, Stream};

use crate::{error::Error, interpreter::Output, lexer, parser};

impl Output {
    fn set_input(name: &str, value: &str) -> Result<Self, Vec<Error>> {
        let len = value.len();

        let (lexed, errs) = lexer::lexer().parse_recovery(value);

        if errs.len() > 0 {
            return Err(errs.iter().map(|e| Error::SyntaxError(e.clone())).collect());
        }

        let (parsed, errs) = parser::parse()
            .parse_recovery(Stream::from_iter(len..len + 1, lexed.unwrap().into_iter()));

        if errs.len() > 0 {
            return Err(errs
                .iter()
                .map(|e| Error::ParsingError(e.clone()))
                .collect());
        }

        let parsed = parsed.unwrap()[0];
    }
}
