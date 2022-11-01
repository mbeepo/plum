#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    // prefix
    Not,

    // infix
    Equals,
    NotEquals,
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    Lt,
    Gt,
    Lte,
    Gte,

    // postfix
    Index,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Num(f64),
    String(String),
    True,
    False,
    Array(Vec<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Exp(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Conditional {
        condition: Box<Expr>,
        inner: Box<Expr>,
        other: Box<Expr>,
    },
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
        if f {
            Self::Literal(Literal::True)
        } else {
            Self::Literal(Literal::False)
        }
    }
}

impl From<Vec<Expr>> for Expr {
    fn from(f: Vec<Expr>) -> Self {
        Self::Literal(Literal::Array(f))
    }
}
