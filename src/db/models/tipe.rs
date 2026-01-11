use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Str(String),
    Float(f64),
    Date(chrono::NaiveDate),
    Enum { variant: String },
    Null,
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Str(value)
    }
}
impl From<chrono::NaiveDate> for Value {
    fn from(value: chrono::NaiveDate) -> Self {
        Value::Date(value)
    }
}
impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Str(value.to_string())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "-"),
            Value::Int(v) => write!(f, "{}", v),
            Value::Str(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Date(v) => write!(f, "{}", v),
            Value::Enum { variant } => write!(f, "Enum: {}", variant),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeaderType {
    Int,
    Str,
    Float,
    Date,
    Enum { variant: Vec<String> },
}
