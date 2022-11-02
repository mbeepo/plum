use std::fmt;

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
    Equals(Box<Expr>, Box<Expr>),
    NotEquals(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Lte(Box<Expr>, Box<Expr>),
    Gte(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
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
