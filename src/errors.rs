use crate::interpreter::{SpannedValue, ValueType};

#[derive(Clone, Debug)]
pub enum Error {
    WrongType {
        expected: Vec<ValueType>,
        got: SpannedValue,
    },
    NeedsInt {
        got: SpannedValue,
    },
}

impl From<Error> for Vec<Error> {
    fn from(f: Error) -> Self {
        vec![f]
    }
}
