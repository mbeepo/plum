use std::ops::Range;

use chumsky::Span;

use crate::interpreter::Value;

#[derive(Clone, Debug)]
pub enum Error {
    WrongType {
        expected: Value,
        got: Value,
        span: Range<usize>,
    },
}

impl From<Error> for Vec<Error> {
    fn from(f: Error) -> Self {
        vec![f]
    }
}
