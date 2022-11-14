use crate::ast::{Expr, InfixOp, Literal, Spanned};

impl InfixOp {
    fn get_binding_power(&self) -> u8 {
        match self {
            InfixOp::IRange => 68,
            InfixOp::Range => 66,
            InfixOp::Pow => 64,
            InfixOp::Mul | InfixOp::Div | InfixOp::Mod => 62,
            InfixOp::Add | InfixOp::Sub => 60,
            InfixOp::Equals
            | InfixOp::NotEquals
            | InfixOp::Lt
            | InfixOp::Gt
            | InfixOp::Lte
            | InfixOp::Gte => 58,
            InfixOp::In => 56,
            InfixOp::And => 54,
            InfixOp::Or => 52,
        }
    }
}

impl From<InfixOp> for String {
    fn from(f: InfixOp) -> Self {
        match f {
            InfixOp::Pow => " ** ",
            InfixOp::Mul => " * ",
            InfixOp::Div => " / ",
            InfixOp::Mod => " % ",
            InfixOp::Add => " + ",
            InfixOp::Sub => " - ",
            InfixOp::Equals => " == ",
            InfixOp::NotEquals => " != ",
            InfixOp::Lt => " < ",
            InfixOp::Gt => " > ",
            InfixOp::Lte => " <= ",
            InfixOp::Gte => " >= ",
            InfixOp::In => " in ",
            InfixOp::And => " && ",
            InfixOp::Or => " || ",
            InfixOp::Range => "..",
            InfixOp::IRange => "..=",
        }
        .to_owned()
    }
}

impl From<&Spanned> for String {
    fn from(input: &Spanned) -> Self {
        match input {
            Spanned(Expr::Assign { names, value }, _) => {
                let out = names.join(" = ");

                out + &String::from(*value.clone()) + ";"
            }
            Spanned(Expr::Literal(value), _) => match value {
                Literal::Array(inner) => {
                    let inner: Vec<String> = inner.iter().map(|e| String::from(e)).collect();

                    "[".to_owned() + &inner.join(", ") + "]"
                }
                Literal::Bool(inner) => {
                    if *inner {
                        "true".to_owned()
                    } else {
                        "false".to_owned()
                    }
                }
                Literal::String(inner) => r#"""#.to_owned() + inner + r#"""#,
                Literal::Num(inner) => inner.to_string(),
                Literal::Null => "[NULL]".to_owned(),
            },
            Spanned(Expr::Ident(name), _) => name.clone(),
            Spanned(Expr::Not(inner), _) => {
                let out = String::from(*inner.clone());

                "!(".to_owned() + &out + ")"
            }
            Spanned(Expr::InfixOp(lhs, op, rhs), _) => {
                let lhs_str = String::from(*lhs.clone());
                let rhs_str = String::from(*rhs.clone());
                let op_str = String::from(op.clone());

                let lhs_str = match *lhs.clone() {
                    Spanned(Expr::InfixOp(_, _, _), _) => "(".to_owned() + &lhs_str + ")",
                    _ => lhs_str,
                };

                let rhs_str = match *rhs.clone() {
                    Spanned(Expr::InfixOp(_, _, _), _) => "(".to_owned() + &rhs_str + ")",
                    _ => rhs_str,
                };

                lhs_str + &op_str + &rhs_str
            }
            Spanned(Expr::Index(lhs, idx), _) => {
                let lhs_str = String::from(*lhs.clone());
                let idx_str = String::from(*idx.clone());

                let lhs_str = match *lhs.clone() {
                    Spanned(Expr::InfixOp(_, _, _), _) => "(".to_owned() + &lhs_str + ")",
                    _ => lhs_str,
                };

                lhs_str + "[" + &idx_str + "]"
            }
            Spanned(Expr::Error, _) => "[ERROR]".to_owned(),
            Spanned(
                Expr::Conditional {
                    condition,
                    inner,
                    other,
                },
                _,
            ) => {
                let cond_str = String::from(*condition.clone());
                let inner_str = String::from(*inner.clone());
                let other_str = String::from(*other.clone());

                let other_str = match *other.clone() {
                    Spanned(
                        Expr::Conditional {
                            condition: _,
                            inner: _,
                            other: _,
                        },
                        _,
                    ) => other_str,
                    _ => "{ ".to_owned() + &other_str,
                };

                "if ".to_owned() + &cond_str + " { " + &inner_str + " } else " + &other_str + " }"
            }
            _ => todo!(),
        }
    }
}

impl From<Spanned> for String {
    fn from(input: Spanned) -> Self {
        (&input).into()
    }
}
