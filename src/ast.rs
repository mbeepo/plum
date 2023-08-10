use std::fmt::{self, Display};
use std::ops::Range;

use crate::value::ValueType;

pub type Span = Range<usize>;

#[derive(Clone, Debug)]
pub struct Spanned(pub Expr, pub Span);

impl PartialEq<Spanned> for Spanned {
    fn eq(&self, other: &Spanned) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialEq<Spanned> for Expr {
    fn eq(&self, other: &Spanned) -> bool {
        self.eq(&other.0)
    }
}

impl PartialEq<Expr> for Spanned {
    fn eq(&self, other: &Expr) -> bool {
        self.0.eq(other)
    }
}

impl From<f64> for Spanned {
    fn from(f: f64) -> Self {
        Self(Expr::Literal(Literal::Num(f)), 0..1)
    }
}

impl<'a> From<&'a str> for Spanned {
    fn from(f: &'a str) -> Self {
        Self::from(f.to_owned())
    }
}

impl From<String> for Spanned {
    fn from(f: String) -> Self {
        Self(Expr::Literal(Literal::String(f)), 0..1)
    }
}

impl From<bool> for Spanned {
    fn from(f: bool) -> Self {
        if f {
            Self(Expr::Literal(Literal::Bool(true)), 0..1)
        } else {
            Self(Expr::Literal(Literal::Bool(false)), 0..1)
        }
    }
}

impl From<Vec<Spanned>> for Spanned {
    fn from(f: Vec<Spanned>) -> Self {
        Self(Expr::Literal(Literal::Array(f)), 0..1)
    }
}

impl From<Expr> for Spanned {
    fn from(f: Expr) -> Self {
        Self(f, 0..1)
    }
}

impl From<Spanned> for Vec<Spanned> {
    fn from(value: Spanned) -> Self {
        vec![value]
    }
}

impl AsRef<Spanned> for Spanned {
    fn as_ref(&self) -> &Spanned {
        &self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Num(f64),
    String(String),
    Bool(bool),
    Array(Vec<Spanned>),
    Null,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InfixOp {
    Equals,
    NotEquals,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
    Pow,
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    In,
    Range,
    IRange,
}

impl Display for InfixOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Equals => "Equals",
            Self::NotEquals => "NotEquals",
            Self::Lt => "Less",
            Self::Gt => "Greater",
            Self::Lte => "LessOrEqual",
            Self::Gte => "GreaterOrEqual",
            Self::And => "And",
            Self::Or => "Or",
            Self::Pow => "Pow",
            Self::Mul => "Mul",
            Self::Div => "Div",
            Self::Mod => "Mod",
            Self::Add => "Add",
            Self::Sub => "Sub",
            Self::In => "In",
            Self::Range => "Range",
            Self::IRange => "IRange",
        };

        write!(f, "{}", out)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Not(Box<Spanned>),
    InfixOp(Box<Spanned>, InfixOp, Box<Spanned>),
    Index(Box<Spanned>, Box<Spanned>),
    Conditional {
        condition: Box<Spanned>,
        inner: Box<Spanned>,
        other: Box<Spanned>,
    },
    Access(Box<Spanned>, Box<Spanned>),
    Call(Box<Spanned>, Vec<Spanned>),
    Assign {
        names: Vec<String>,
        value: Box<Spanned>,
    },
    Error,
    Input(String, ValueType),
}

impl From<f64> for Expr {
    fn from(f: f64) -> Self {
        Self::Literal(Literal::Num(f))
    }
}

impl<'a> From<&'a str> for Expr {
    fn from(f: &'a str) -> Self {
        Self::from(f.to_owned())
    }
}

impl From<String> for Expr {
    fn from(f: String) -> Self {
        Self::Literal(Literal::String(f))
    }
}

impl From<bool> for Expr {
    fn from(f: bool) -> Self {
        Self::Literal(Literal::Bool(f))
    }
}

impl From<Vec<Expr>> for Expr {
    fn from(f: Vec<Expr>) -> Self {
        let mut spanned = Vec::<Spanned>::new();

        for expr in f {
            spanned.push(Spanned::from(expr));
        }

        Self::Literal(Literal::Array(spanned))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Ident(String),
    Ctrl(char),
    Op(String),
    If,
    Else,
    String(String),
    Num(String),
    Bool(bool),
    Input,
    Type(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(e) => write!(f, "{}", e),
            Token::Ctrl(e) => write!(f, "{}", e),
            Token::Op(e) => write!(f, "{}", e),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::String(e) => write!(f, "{}", e),
            Token::Num(e) => write!(f, "{}", e),
            Token::Bool(e) => write!(f, "{}", e),
            Token::Input => write!(f, "Input"),
            Token::Type(_) => write!(f, "TypeName"),
        }
    }
}
