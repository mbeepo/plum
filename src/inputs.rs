use std::collections::HashMap;

use chumsky::{Parser, Stream};

use crate::{error::Error, interpreter::VarStore, lexer, parser};

impl VarStore {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            inputs: Vec::new(),
            deps: HashMap::new(),
            dependents: HashMap::new(),
            source: HashMap::new(),
            cached: HashMap::new(),
            intermediate: HashMap::new(),
        }
    }

    pub fn set_input(name: &str, value: &str) -> Result<Self, Vec<Error>> {
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

        let parsed = &parsed.unwrap()[0];

        Ok(Self::new())
    }
}
