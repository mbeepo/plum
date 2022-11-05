use std::fmt::{self, Display};
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Spanned(pub Expr, pub Range<usize>);

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

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Num(f64),
    String(String),
    Bool(bool),
    Array(Vec<Spanned>),
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
    Block(Vec<Spanned>),
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    // ===== general =====
    Ident,
    Assign,
    End,
    Comma,
    Access,
    Whitespace,
    Comment,
    Error,
    EOF,

    // ===== containers =====
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    LParen,
    RParen,

    // ===== logic =====
    Equals,
    NotEquals,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
    Not,
    If,
    Else,

    // ===== math =====
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,

    // ===== literal =====
    String,
    Num,
    True,
    False,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<&TokenKind> for String {
    fn from(other: &TokenKind) -> String {
        let out = match other {
            TokenKind::Ident => "Ident",
            TokenKind::Assign => "Assign",
            TokenKind::End => "End",
            TokenKind::Comma => "Comma",
            TokenKind::Access => "Access",
            TokenKind::Whitespace => "",
            TokenKind::Comment => "Comment",
            TokenKind::Error => "Error",
            TokenKind::EOF => "EOF",
            TokenKind::LCurly => "LCurly",
            TokenKind::RCurly => "RCurly",
            TokenKind::LSquare => "LSquare",
            TokenKind::RSquare => "RSquare",
            TokenKind::LParen => "LParen",
            TokenKind::RParen => "RParen",
            TokenKind::Equals => "Equals",
            TokenKind::NotEquals => "NotEquals",
            TokenKind::Lt => "Lt",
            TokenKind::Gt => "Gt",
            TokenKind::Lte => "Lte",
            TokenKind::Gte => "Gte",
            TokenKind::And => "And",
            TokenKind::Or => "Or",
            TokenKind::Not => "Not",
            TokenKind::If => "If",
            TokenKind::Else => "Else",
            TokenKind::Add => "Add",
            TokenKind::Sub => "Sub",
            TokenKind::Mul => "Mul",
            TokenKind::Div => "Div",
            TokenKind::Mod => "Mod",
            TokenKind::Exp => "Exp",
            TokenKind::String => "String",
            TokenKind::Num => "Num",
            TokenKind::True => "True",
            TokenKind::False => "False",
        };

        out.to_owned()
    }
}
