use crate::{
    ast::InfixOp,
    error::{Error, TypeErrorCtx},
    value::{SpannedValue, Value, ValueType},
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
                _ => Err(Error::TypeError {
                    expected: ValueType::Num.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Num,
                        op: InfixOp::Pow,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: ValueType::Num.into(),
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::Pow },
            }),
        }
    }

    pub fn mul(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            // idk why i have to clone here but it wont compile if i dont
            Value::Num(lhs) => match other.0.clone() {
                Value::Num(rhs) => Ok(Value::Num(lhs * rhs)),
                Value::String(rhs) => {
                    if lhs == lhs.trunc() {
                        Ok(Value::String(rhs.repeat(lhs as usize)))
                    } else {
                        Err(Error::TypeError {
                            expected: ValueType::Int.into(),
                            got: other,
                            context: TypeErrorCtx::StringMul,
                        })
                    }
                }
                _ => Err(Error::TypeError {
                    expected: vec![ValueType::Num, ValueType::String],
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Num,
                        op: InfixOp::Mul,
                    },
                }),
            },
            Value::String(lhs) => match other.0 {
                Value::Num(rhs) => {
                    if rhs == rhs.trunc() {
                        Ok(Value::String(lhs.repeat(rhs as usize)))
                    } else {
                        Err(Error::TypeError {
                            expected: ValueType::Int.into(),
                            got: other,
                            context: TypeErrorCtx::StringMul,
                        })
                    }
                }
                _ => Err(Error::TypeError {
                    expected: ValueType::Num.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::String,
                        op: InfixOp::Mul,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: vec![ValueType::Num, ValueType::String],
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::Mul },
            }),
        }
    }

    pub fn div(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Num(lhs / rhs)),
                _ => Err(Error::TypeError {
                    expected: ValueType::Num.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Num,
                        op: InfixOp::Div,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: ValueType::Num.into(),
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::Div },
            }),
        }
    }

    pub fn modulus(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Num(lhs % rhs)),
                _ => Err(Error::TypeError {
                    expected: ValueType::Num.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Num,
                        op: InfixOp::Mod,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: ValueType::Num.into(),
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::Mod },
            }),
        }
    }

    pub fn add(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Num(lhs + rhs)),
                _ => Err(Error::TypeError {
                    expected: ValueType::Num.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Num,
                        op: InfixOp::Add,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: ValueType::Num.into(),
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::Add },
            }),
        }
    }

    pub fn sub(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Num(lhs - rhs)),
                _ => Err(Error::TypeError {
                    expected: ValueType::Num.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Num,
                        op: InfixOp::Sub,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: ValueType::Num.into(),
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::Sub },
            }),
        }
    }

    pub fn lt(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Bool(lhs < rhs)),
                _ => Err(Error::TypeError {
                    expected: ValueType::Num.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Num,
                        op: InfixOp::Lt,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: ValueType::Num.into(),
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::Lt },
            }),
        }
    }

    pub fn gt(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Bool(lhs > rhs)),
                _ => Err(Error::TypeError {
                    expected: ValueType::Num.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Num,
                        op: InfixOp::Gt,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: ValueType::Num.into(),
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::Gt },
            }),
        }
    }

    pub fn lte(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Bool(lhs <= rhs)),
                _ => Err(Error::TypeError {
                    expected: ValueType::Num.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Num,
                        op: InfixOp::Lte,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: ValueType::Num.into(),
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::Lte },
            }),
        }
    }

    pub fn gte(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(lhs) => match other.0 {
                Value::Num(rhs) => Ok(Value::Bool(lhs >= rhs)),
                _ => Err(Error::TypeError {
                    expected: ValueType::Num.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Num,
                        op: InfixOp::Gte,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: ValueType::Num.into(),
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::Gte },
            }),
        }
    }

    pub fn and(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Bool(lhs) => match other.0 {
                Value::Bool(rhs) => Ok(Value::Bool(lhs && rhs)),
                _ => Err(Error::TypeError {
                    expected: ValueType::Bool.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Bool,
                        op: InfixOp::And,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: ValueType::Bool.into(),
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::And },
            }),
        }
    }

    pub fn or(self, other: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Bool(lhs) => match other.0 {
                Value::Bool(rhs) => Ok(Value::Bool(lhs || rhs)),
                _ => Err(Error::TypeError {
                    expected: ValueType::Bool.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs: ValueType::Bool,
                        op: InfixOp::Or,
                    },
                }),
            },
            _ => Err(Error::TypeError {
                expected: ValueType::Bool.into(),
                got: self,
                context: TypeErrorCtx::InfixOpLhs { op: InfixOp::Or },
            }),
        }
    }

    pub fn equals(self, other: Self) -> Result<Value, Error> {
        match (self.0.clone(), other.0.clone()) {
            (Value::Num(lhs), Value::Num(rhs)) => Ok(Value::Bool(lhs == rhs)),
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::Bool(lhs == rhs)),
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(lhs == rhs)),
            (Value::Array(lhs), Value::Array(rhs)) => Ok(Value::Bool(lhs == rhs)),
            _ => {
                let lhs = self.0.get_type();

                Err(Error::TypeError {
                    expected: lhs.into(),
                    got: other,
                    context: TypeErrorCtx::InfixOpRhs {
                        lhs,
                        op: InfixOp::Equals,
                    },
                })
            }
        }
    }

    pub fn not_equals(self, other: Self) -> Result<Value, Error> {
        let out = self.equals(other)?;
        SpannedValue::from(out).not()
    }

    pub fn contains(self, other: Self) -> Result<Value, Error> {
        let yes = match other.0 {
            Value::Array(lhs) => lhs.contains(&SpannedValue(self.0, 0..1)),
            Value::String(lhs) => match self.0 {
                Value::String(rhs) => lhs.contains(&rhs),
                _ => {
                    return Err(Error::TypeError {
                        expected: ValueType::String.into(),
                        got: self,
                        context: TypeErrorCtx::InfixOpRhs {
                            lhs: ValueType::String,
                            op: (InfixOp::In),
                        },
                    })
                }
            },
            _ => {
                return Err(Error::TypeError {
                    expected: vec![ValueType::Array, ValueType::String],
                    got: self,
                    context: TypeErrorCtx::InfixOpLhs { op: InfixOp::In },
                })
            }
        };

        if yes {
            Ok(Value::Bool(true))
        } else {
            Ok(Value::Bool(false))
        }
    }

    pub fn not(self) -> Result<Value, Error> {
        match self.0 {
            Value::Bool(e) => Ok(Value::Bool(!e)),
            _ => Err(Error::TypeError {
                expected: ValueType::Bool.into(),
                got: self,
                context: TypeErrorCtx::Not,
            }),
        }
    }

    pub fn index(self, idx: Self) -> Result<Value, Error> {
        let inner = self.0.clone();

        let len = match &inner {
            Value::Array(f) => f.len(),
            Value::String(f) => f.len(),
            _ => {
                return Err(Error::TypeError {
                    expected: vec![ValueType::Array, ValueType::String],
                    got: self,
                    context: TypeErrorCtx::IndexOf,
                })
            }
        };

        match idx.0.clone() {
            Value::Num(e) => {
                if e == e.trunc() {
                    let e = e as isize;
                    let e_abs = e.abs() as usize;
                    let cond = if e < 0 { len >= e_abs } else { len > e_abs };

                    if cond {
                        let e2 = if e < 0 { len - e_abs } else { e_abs };

                        match inner {
                            Value::Array(f) => Ok(f[e2].clone().0),
                            Value::String(f) => {
                                Ok(Value::String(f.chars().nth(e2).unwrap().to_string()))
                            }
                            _ => unreachable!("Checked when checked length at start"),
                        }
                    } else {
                        Err(Error::IndexError {
                            index: e,
                            len,
                            lspan: self.1,
                            rspan: idx.1,
                        })
                    }
                } else {
                    Err(Error::TypeError {
                        expected: vec![ValueType::Num, ValueType::Range],
                        got: self,
                        context: TypeErrorCtx::Index,
                    })
                }
            }
            Value::Range(e) | Value::IRange(e) => {
                let start = e.start;
                let end = e.end;
                let start_abs = start.abs() as usize;
                let end_abs = end.abs() as usize;

                let start_cond = if start < 0 {
                    len >= start_abs
                } else {
                    len > start_abs
                };

                let end_cond = if end < 0 {
                    len >= end_abs
                } else {
                    len > end_abs
                };

                if start_cond && end_cond {
                    let start_norm = if start < 0 {
                        len - start_abs
                    } else {
                        start_abs
                    };
                    let end_norm = if end < 0 { len - end_abs } else { end_abs };

                    if start_norm > end_norm {
                        match inner {
                            Value::Array(mut f) => {
                                f.reverse();
                                let start_norm = len - start_norm - 1;
                                let end_norm = len - end_norm + 1;

                                let out = f[start_norm..=end_norm].to_vec();

                                Ok(Value::Array(out))
                            }
                            Value::String(f) => {
                                let f = f.chars().rev().collect::<String>();
                                dbg!(start, start_abs, start_norm);
                                dbg!(end, end_abs, end_norm);

                                let start_norm = len - start_norm - 1;
                                let end_norm = if let Value::IRange(_) = idx.0 {
                                    end_norm + 1
                                } else {
                                    end_norm
                                };

                                let end_norm = len - end_norm;

                                dbg!(start_norm);
                                dbg!(end_norm);

                                let out = f[start_norm..=end_norm].to_string();

                                Ok(Value::String(out))
                            }
                            _ => unreachable!("Checked when checked length at start"),
                        }
                    } else {
                        match inner {
                            Value::Array(f) => {
                                let out = match idx.0 {
                                    Value::Range(_) => f[start_norm..end_norm].to_vec(),
                                    Value::IRange(_) => f[start_norm..=end_norm].to_vec(),
                                    _ => unreachable!("Outermost match is one of those"),
                                };

                                Ok(Value::Array(out))
                            }
                            Value::String(f) => {
                                let out = match idx.0 {
                                    Value::Range(_) => f[start_norm..end_norm].to_string(),
                                    Value::IRange(_) => f[start_norm..=end_norm].to_string(),
                                    _ => unreachable!("Outermost match is one of those"),
                                };

                                Ok(Value::String(out))
                            }
                            _ => unreachable!("Checked when checked length at start"),
                        }
                    }
                } else {
                    Err(Error::RangeIndexError {
                        index: e,
                        len,
                        lspan: self.1,
                        rspan: idx.1,
                    })
                }
            }
            _ => Err(Error::TypeError {
                expected: vec![ValueType::Num, ValueType::Range],
                got: idx,
                context: TypeErrorCtx::Index,
            }),
        }
    }

    pub fn range(self, rhs: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(e) => {
                if e == e.trunc() {
                    let e = e as isize;

                    match rhs.0 {
                        Value::Num(f) => {
                            if f == f.trunc() {
                                let f = f as isize;

                                Ok(Value::Range(e..f))
                            } else {
                                Err(Error::TypeError {
                                    expected: ValueType::Int.into(),
                                    got: rhs,
                                    context: TypeErrorCtx::Range,
                                })
                            }
                        }
                        _ => Err(Error::TypeError {
                            expected: ValueType::Num.into(),
                            got: rhs,
                            context: TypeErrorCtx::Range,
                        }),
                    }
                } else {
                    Err(Error::TypeError {
                        expected: ValueType::Int.into(),
                        got: self,
                        context: TypeErrorCtx::Range,
                    })
                }
            }
            _ => Err(Error::TypeError {
                expected: ValueType::Num.into(),
                got: self,
                context: TypeErrorCtx::Range,
            }),
        }
    }

    pub fn irange(self, rhs: Self) -> Result<Value, Error> {
        match self.0 {
            Value::Num(e) => {
                if e == e.trunc() {
                    let e = e as isize;

                    match rhs.0 {
                        Value::Num(f) => {
                            if f == f.trunc() {
                                let f = f as isize;

                                Ok(Value::IRange(e..f))
                            } else {
                                Err(Error::TypeError {
                                    expected: ValueType::Int.into(),
                                    got: rhs,
                                    context: TypeErrorCtx::Range,
                                })
                            }
                        }
                        _ => Err(Error::TypeError {
                            expected: ValueType::Num.into(),
                            got: rhs,
                            context: TypeErrorCtx::Range,
                        }),
                    }
                } else {
                    Err(Error::TypeError {
                        expected: ValueType::Int.into(),
                        got: self,
                        context: TypeErrorCtx::Range,
                    })
                }
            }
            _ => Err(Error::TypeError {
                expected: ValueType::Num.into(),
                got: self,
                context: TypeErrorCtx::Range,
            }),
        }
    }
}
