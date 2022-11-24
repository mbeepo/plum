use std::ops::Range;

use ariadne::{ColorGenerator, Fmt, Label, Report, Source};
use chumsky::prelude::Simple;

use crate::{
    ast::{InfixOp, Span, Token},
    interpreter::SpannedIdent,
    value::{SpannedValue, ValueType},
};

#[derive(Clone, Copy, Debug)]
pub enum TypeErrorCtx {
    // can't perform <op> on <lhs>
    // for when <lhs> is an invalid type for <op> in any use case
    InfixOpLhs { op: InfixOp },
    // can only perform <op> on <lhs> with <expected>
    // for when <rhs> is an invalid type for <op> with <lhs>
    InfixOpRhs { lhs: ValueType, op: InfixOp },
    Not,
    StringMul,
    Index,
    IndexOf,
    AssignToAssign,
    Condition,
    Range,
}

#[derive(Clone, Debug)]
pub enum Error {
    TypeError {
        expected: Vec<ValueType>,
        got: SpannedValue,
        context: TypeErrorCtx,
    },
    IndexError {
        index: isize,
        len: usize,
        lspan: Span,
        rspan: Span,
    },
    SyntaxError(Simple<char>),
    ParsingError(Simple<Token>),
    ReferenceError {
        name: String,
        span: Span,
    },
    ReassignError {
        name: String,
        old_span: Span,
        new_span: Span,
    },
    RecursionError {
        chain: Vec<SpannedIdent>,
    },
    RangeIndexError {
        index: Range<isize>,
        len: usize,
        lspan: Span,
        rspan: Span,
    },
}

impl From<Error> for Vec<Error> {
    fn from(f: Error) -> Self {
        vec![f]
    }
}

pub trait ChumskyAriadne {
    fn display<'a>(&self, source_file: &'a str, source: &'a str, offset: usize);
}

impl ChumskyAriadne for Simple<char> {
    fn display<'a>(&self, source_file: &'a str, source: &'a str, offset: usize) {
        Report::build(ariadne::ReportKind::Error, source, offset)
            .with_code(1)
            .with_message("SyntaxError: Unexpected token")
            .with_label(
                Label::new((source_file, self.span()))
                    .with_message(format!("{}", self))
                    .with_color(ariadne::Color::Green),
            )
            .with_note(if let Some(e) = self.label() {
                format!("Label is `{}`", e)
            } else {
                "No label".to_owned()
            })
            .finish()
            .eprint((source_file, Source::from(source)))
            .unwrap();
    }
}

impl ChumskyAriadne for Simple<Token> {
    fn display<'a>(&self, source_file: &'a str, source: &'a str, offset: usize) {
        Report::build(ariadne::ReportKind::Error, source, offset)
            .with_code(1)
            .with_message("SyntaxError: Unexpected token")
            .with_label(
                Label::new((source_file, self.span()))
                    .with_message(format!("{}", self))
                    .with_color(ariadne::Color::Green),
            )
            .with_note(if let Some(e) = self.label() {
                format!("Label is `{}`", e)
            } else {
                "No label".to_owned()
            })
            .finish()
            .eprint((source_file, Source::from(source)))
            .unwrap();
    }
}

impl ChumskyAriadne for Error {
    fn display<'a>(&self, source_file: &'a str, source: &'a str, offset: usize) {
        let mut colors = ColorGenerator::new();

        match self {
            Self::TypeError {
                expected,
                got,
                context,
            } => {
                match context {
                    TypeErrorCtx::InfixOpLhs { op } => {
                        let a = colors.next();
                        let b = colors.next();

                        let note = if expected.len() == 1 {
                            format!(
                                "Operator `{}` only accept operands of type {}",
                                op,
                                expected[0].to_string().fg(b)
                            )
                        } else if expected.len() == 0 {
                            format!("Uh oh ! Operator `{}` doesn't accept *any* types... Call the dev !", op)
                        } else {
                            format!(
                                "Operator `{}` only accepts operands of type [{}]",
                                op,
                                stringify_expected(expected).fg(b)
                            )
                        };

                        Report::build(ariadne::ReportKind::Error, source, offset)
                            .with_code(2)
                            .with_message("Incompatible types")
                            .with_label(
                                Label::new((source_file, got.clone().1))
                                    .with_message(format!(
                                        "This is of type {}",
                                        got.0.get_type().to_string().fg(a)
                                    ))
                                    .with_color(a),
                            )
                            .with_note(note)
                            .finish()
                            .eprint((source_file, Source::from(source)))
                            .unwrap();
                    }
                    e => {
                        println!("{:#?}", e);
                        dbg!(self);
                        todo!()
                    }
                }
            }
            Self::IndexError {
                index,
                len,
                lspan: lhs,
                rspan: rhs,
            } => {
                let a = colors.next();
                let b = colors.next();

                let note = if *index > 0 {
                    let uindex = *index as usize;

                    if *len == uindex {
                        let c = colors.next();
                        let max_idx = index - 1;

                        format!(
                            "Arrays are zero-indexed, the index of the last element is {}",
                            max_idx.fg(c)
                        )
                    } else {
                        "Index must fit within the bounds of the accessed sequence".to_owned()
                    }
                } else {
                    "Index must fit within the bounds of the accessed sequence".to_owned()
                };

                Report::build(ariadne::ReportKind::Error, source, offset)
                    .with_code(3)
                    .with_message("Index out of bounds")
                    .with_label(
                        Label::new((source_file, lhs.clone()))
                            .with_message(format!("This is of length {}", len.fg(a)))
                            .with_color(a),
                    )
                    .with_label(
                        Label::new((source_file, rhs.clone()))
                            .with_message(format!("This is {}", index.fg(b)))
                            .with_color(b),
                    )
                    .with_note(note)
                    .finish()
                    .eprint((source_file, Source::from(source)))
                    .unwrap();
            }
            Self::ReassignError {
                name,
                old_span,
                new_span,
            } => {
                let a = colors.next();
                let b = colors.next();

                Report::build(ariadne::ReportKind::Error, source, offset)
                    .with_code(4)
                    .with_message("Attempt to reassign variable")
                    .with_label(
                        Label::new((source_file, old_span.clone()))
                            .with_message(format!(
                                "`{}` was first assigned {}",
                                name.clone().fg(a),
                                "here".fg(a)
                            ))
                            .with_color(a),
                    )
                    .with_label(
                        Label::new((source_file, new_span.clone()))
                            .with_message(format!("It was then reassigned {}", "here".fg(b)))
                            .with_color(b),
                    )
                    .with_note("Variables may only be assigned once")
                    .finish()
                    .eprint((source_file, Source::from(source)))
                    .unwrap()
            }
            e => {
                dbg!(e);
                todo!()
            }
        }
    }
}

fn stringify_expected(expected: &Vec<ValueType>) -> String {
    let out = expected
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    out[..out.len() - 2].to_string()
}
