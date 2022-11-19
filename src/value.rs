use std::{fmt::Display, ops::Range};

use crate::ast::Literal;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Num(f64),
    String(String),
    Bool(bool),
    Array(Vec<SpannedValue>),
    Error,
    Assign(Vec<String>, Box<Value>),
    Range(Range<isize>),
}

#[derive(Clone, Copy, Debug)]
pub enum ValueType {
    Num,
    Int,
    String,
    Bool,
    Array,
    Error,
    Assign,
    Range,
}

#[derive(Clone, Debug)]
pub struct SpannedValue(pub Value, pub Range<usize>);

impl From<ValueType> for Vec<ValueType> {
    fn from(f: ValueType) -> Self {
        vec![f]
    }
}

impl PartialEq<SpannedValue> for SpannedValue {
    fn eq(&self, other: &SpannedValue) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialEq<Value> for SpannedValue {
    fn eq(&self, other: &Value) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<SpannedValue> for Value {
    fn eq(&self, other: &SpannedValue) -> bool {
        self.eq(&other.0)
    }
}

impl From<Literal> for Value {
    fn from(f: Literal) -> Self {
        match f {
            Literal::Num(e) => Value::Num(e),
            Literal::String(e) => Value::String(e),
            Literal::Bool(e) => Value::Bool(e),
            _ => Value::Error,
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            ValueType::Num => "Num",
            ValueType::Int => "Int",
            ValueType::String => "String",
            ValueType::Bool => "Bool",
            ValueType::Array => "Array",
            ValueType::Error => "[ERROR]",
            ValueType::Assign => "Assign",
            ValueType::Range => "Range",
        };

        write!(f, "{}", out)
    }
}

impl Value {
    pub fn get_type(&self) -> ValueType {
        match self {
            Value::Num(_) => ValueType::Num,
            Value::String(_) => ValueType::String,
            Value::Bool(_) => ValueType::Bool,
            Value::Array(_) => ValueType::Array,
            Value::Error => ValueType::Error,
            Value::Assign(_, _) => ValueType::Assign,
            Value::Range(_) => ValueType::Range,
        }
    }
}