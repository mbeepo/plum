use std::collections::HashMap;

use chumsky::Parser;

use crate::parser::parse;

pub fn interpret<'a>(input: &'a str) {
    let parsed = parse().parse(input);
}
