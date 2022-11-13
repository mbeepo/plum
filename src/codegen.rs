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
            Spanned(Expr::InfixOp(lhs, op, rhs), _) => {}
        }
    }
}

impl From<Spanned> for String {
    fn from(input: Spanned) -> Self {
        (&input).into()
    }
}
