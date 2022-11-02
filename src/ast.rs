use std::fmt;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Spanned(pub Expr, pub Range<usize>);

impl PartialEq<Spanned> for Spanned {
    fn eq(&self, other: &Spanned) -> bool {
        self.0.eq(&other.0)
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
            Self(Expr::Literal(Literal::True), 0..1)
        } else {
            Self(Expr::Literal(Literal::False), 0..1)
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
    True,
    False,
    Array(Vec<Spanned>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Equals(Box<Spanned>, Box<Spanned>),
    NotEquals(Box<Spanned>, Box<Spanned>),
    Lt(Box<Spanned>, Box<Spanned>),
    Gt(Box<Spanned>, Box<Spanned>),
    Lte(Box<Spanned>, Box<Spanned>),
    Gte(Box<Spanned>, Box<Spanned>),
    And(Box<Spanned>, Box<Spanned>),
    Or(Box<Spanned>, Box<Spanned>),
    Not(Box<Spanned>),
    Exp(Box<Spanned>, Box<Spanned>),
    Mul(Box<Spanned>, Box<Spanned>),
    Div(Box<Spanned>, Box<Spanned>),
    Mod(Box<Spanned>, Box<Spanned>),
    Add(Box<Spanned>, Box<Spanned>),
    Sub(Box<Spanned>, Box<Spanned>),
    Index(Box<Spanned>, Box<Spanned>),
    Conditional {
        condition: Box<Spanned>,
        inner: Box<Spanned>,
        other: Box<Spanned>,
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
        let spanned = Vec::<Spanned>::new();

        for expr in f.iter() {
            spanned.push(Spanned::from(*expr));
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

impl fmt::Display for TokenKind {
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
