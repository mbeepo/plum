use ariadne::{ColorGenerator, Fmt, Label, Report, Source};
use chumsky::prelude::Simple;

use crate::{
    ast::InfixOp,
    interpreter::{SpannedValue, ValueType},
};

#[derive(Clone, Copy, Debug)]
pub enum TypeErrorCtx {
    // can't perform <op> on <lhs>
    InfixOpLhs { op: InfixOp },
    // can only perform <op> on <lhs> with <expected>
    InfixOpRhs { lhs: ValueType, op: InfixOp },
    Not,
    StringMul,
    Index,
    IndexOf,
}

#[derive(Clone, Debug)]
pub enum Error {
    TypeError {
        expected: Vec<ValueType>,
        got: SpannedValue,
        context: TypeErrorCtx,
    },
    IndexError {
        index: usize,
        len: usize,
    },
    SyntaxError {},
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
                                Label::new((source, got.clone().1))
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
                    _ => todo!(),
                }
            }
            _ => todo!(),
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
