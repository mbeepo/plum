use std::{collections::HashMap, fmt::Display, ops::Range};

use serde::Serialize;

use crate::ast::Literal;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Value {
    Num(f64),
    String(String),
    Bool(bool),
    Array(Vec<SpannedValue>),
    Error,
    Assign(Vec<String>, Box<Value>),
    Range(Range<isize>),
    IRange(Range<isize>),
    Input(String, ValueType, Box<Value>),
    None,
}

#[derive(Clone, Debug, Serialize)]
pub struct ValueMap {
    #[serde(flatten)]
    pub values: HashMap<String, Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ValueType {
    Num,
    Int,
    String,
    Bool,
    Array,
    Error,
    Assign,
    Range,
    IRange,
    Input,
    Any,
    Null,
}

#[derive(Clone, Debug, Serialize)]
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

impl From<Value> for SpannedValue {
    fn from(f: Value) -> Self {
        Self(f, 0..1)
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
            ValueType::IRange => "IRange",
            ValueType::Input => "Input",
            ValueType::Any => "Any",
            ValueType::Null => "Null",
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
            Value::IRange(_) => ValueType::IRange,
            Value::Input(_, _, _) => ValueType::Input,
            Value::None => ValueType::Null,
        }
    }
}
