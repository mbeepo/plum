use chumsky::primitive::Container;

use crate::ast::{Expr, InfixOp, Literal, Spanned};

impl InfixOp {
    fn get_binding_power(&self) -> u8 {
        match self {
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
                Literal::String(inner) => inner.clone(),
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

                let (lhs_str, rhs_str) = match (*lhs.clone(), *rhs.clone()) {
                    (
                        Spanned(Expr::InfixOp(_, op1, _), _),
                        Spanned(Expr::InfixOp(_, op2, _), _),
                    ) => {
						let lhs_bp = op1.get_binding_power();
						let rhs_bp = op2.get_binding_power();

						if lhs_bp > rhs_bp {

						}
					}
                }

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
            _ => todo!(),
        }
    }
}

impl From<Spanned> for String {
    fn from(input: Spanned) -> Self {
        (&input).into()
    }
}
