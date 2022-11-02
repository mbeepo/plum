use std::ops::Range;

use chumsky::Span;

use crate::ast::TokenKind;

#[derive(Clone, Debug)]
pub enum Error {
    WrongType {
        expected: TokenKind,
        got: TokenKind,
        span: Range<usize>,
    },
}
