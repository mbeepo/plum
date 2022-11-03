mod ast;
pub mod errors;
pub mod interpreter;
pub mod operators;
pub mod parser;

use std::collections::HashMap;

use interpreter::Value;
use parser::parse;

pub fn eval<'a>(input: &'a str) -> HashMap<String, Value> {
    let parsed = parse().parse(input).unwrap();
}
