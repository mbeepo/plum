use crate::{
    errors::Error,
    interpreter::{SpannedValue, Value, ValueType},
};

// da big SpannedValue operation set
impl SpannedValue {
    pub fn pow(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => {
                    if lhs == lhs.trunc() && rhs == rhs.trunc() {
                        Ok(Value::Num(lhs.powi(rhs as i32)))
                    } else {
                        Ok(Value::Num(lhs.powf(rhs)))
                    }
                }
                _ => Err(Error::WrongType {
                    expected: ValueType::Num.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Num.into(),
                got: self,
            }),
        }
    }

    pub fn mul(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Num(lhs * rhs)),
                Value::String(rhs) => {
                    if lhs == lhs.trunc() {
                        Ok(Value::String(rhs.repeat(lhs as usize)))
                    } else {
                        Err(Error::NeedsInt { got: self })
                    }
                }
                _ => Err(Error::WrongType {
                    expected: ValueType::Num.into(),
                    got: other,
                }),
            },
            Value::String(lhs) => match other.0 {
                Value::Num(rhs) => {
                    if rhs == rhs.trunc() {
                        Ok(Value::String(lhs.repeat(rhs as usize)))
                    } else {
                        Err(Error::NeedsInt { got: other })
                    }
                }
                _ => Err(Error::WrongType {
                    expected: ValueType::Num.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Num.into(),
                got: self,
            }),
        }
    }

    pub fn div(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Num(lhs / rhs)),
                _ => Err(Error::WrongType {
                    expected: ValueType::Num.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Num.into(),
                got: self,
            }),
        }
    }

    pub fn modulus(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Num(lhs % rhs)),
                _ => Err(Error::WrongType {
                    expected: ValueType::Num.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Num.into(),
                got: self,
            }),
        }
    }

    pub fn add(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Num(lhs + rhs)),
                _ => Err(Error::WrongType {
                    expected: ValueType::Num.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Num.into(),
                got: self,
            }),
        }
    }

    pub fn sub(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Num(lhs - rhs)),
                _ => Err(Error::WrongType {
                    expected: ValueType::Num.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Num.into(),
                got: self,
            }),
        }
    }

    pub fn lt(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Bool(lhs < rhs)),
                _ => Err(Error::WrongType {
                    expected: ValueType::Num.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Num.into(),
                got: self,
            }),
        }
    }

    pub fn gt(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Bool(lhs > rhs)),
                _ => Err(Error::WrongType {
                    expected: ValueType::Num.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Num.into(),
                got: self,
            }),
        }
    }

    pub fn lte(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Bool(lhs <= rhs)),
                _ => Err(Error::WrongType {
                    expected: ValueType::Num.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Num.into(),
                got: self,
            }),
        }
    }

    pub fn gte(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Bool(lhs >= rhs)),
                _ => Err(Error::WrongType {
                    expected: ValueType::Num.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Num.into(),
                got: self,
            }),
        }
    }

    pub fn and(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Bool(lhs) => match other.0 {
                Value::Bool(rhs) => Ok(Value::Bool(lhs && rhs)),
                _ => Err(Error::WrongType {
                    expected: ValueType::Bool.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Bool.into(),
                got: self,
            }),
        }
    }

    pub fn or(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Bool(lhs) => match other.0 {
                Value::Bool(rhs) => Ok(Value::Bool(lhs || rhs)),
                _ => Err(Error::WrongType {
                    expected: ValueType::Bool.into(),
                    got: other,
                }),
            },
            _ => Err(Error::WrongType {
                expected: ValueType::Bool.into(),
                got: self,
            }),
        }
    }

    pub fn equals(self, other: Self) -> Result<Value, Error> {
        match (self.0, other.0) {
            (Value::Num(lhs), Value::Num(rhs)) => Ok(Value::Bool(lhs == rhs)),
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::Bool(lhs == rhs)),
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(lhs == rhs)),
            (Value::Array(lhs), Value::Array(rhs)) => Ok(Value::Bool(lhs == rhs)),
            _ => Err(Error::WrongType {
                expected: self.0.get_type().into(),
                got: other,
            }),
        }
    }

    pub fn is_in(self, other: Self) -> Result<Value, Error> {
        todo!()
    }
}
