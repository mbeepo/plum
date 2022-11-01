pub type Ident = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    // general
    LParen,
    LSquare,
    LCurly,
    RParen,
    RSquare,
    RCurly,
    End,

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

    // literal
    Num,
    String,
    True,
    False,
    Array,

    // special
    Ident,
    If,
    Else,
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
    Ident(Ident),
    PrefixOp {
        op: Token,
        expr: Box<Expr>,
    },
    InfixOp {
        op: Token,
        expr: Box<Expr>,
    },
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
