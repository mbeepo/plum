use ariadne::{ColorGenerator, Label, Report, Fmt, Source};

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
}

impl From<Error> for Vec<Error> {
    fn from(f: Error) -> Self {
        vec![f]
    }
}

impl Error {
    fn display<'a>(&self, source_file: &'a str, source: &'a str, offset: usize) {
        let mut colors = ColorGenerator::new();

        match self {
            Self::TypeError {
                expected,
                got,
                context,
            } => match context {
                TypeErrorCtx::InfixOpLhs { op } => {
                    let a = colors.next();
                    let b = colors.next();

                    let note = if expected.len() == 1 {
                        format!(
                            "Operator `{}` only takes values of type {}",
                            op,
                            expected[0].to_string().fg(b)
                        )
                    } else if expected.len() == 0 {
						format!("Uh oh ! Operator `{}` doesn't expect *any* types... Call the dev !", op)
					} else {
						
					}

                    Report::build(ariadne::ReportKind::Error, source, offset)
                        .with_code(1)
                        .with_message(format!("Incompatible types"))
                        .with_label(
                            Label::new((source, got.1))
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
            },
        }
    }
}
