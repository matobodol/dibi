use crate::{
    Value,
    db::baris::baris::{HRows, VRows},
};

pub enum Op {
    Eq,
    Gt,
    Lt,
}

pub struct Filter {
    pub column: usize,
    pub op: Op,
    pub value: Value,
}

impl Filter {
    pub fn f(column: usize, op: Op, value: Value) -> Self {
        Self { column, op, value }
    }
}

pub fn match_row(hrow: &HRows, filter: &Filter) -> bool {
    let Some(value) = hrow.get(filter.column) else {
        return false;
    };

    match (&filter.op, value, &filter.value) {
        (Op::Eq, Value::Null, Value::Null) => true,
        (Op::Eq, Value::Int(a), Value::Int(b)) => a == b,
        (Op::Eq, Value::Str(a), Value::Str(b)) => a == b,
        (Op::Eq, Value::Float(a), Value::Float(b)) => a == b,
        (Op::Eq, Value::Date(a), Value::Date(b)) => a == b,
        (Op::Eq, Value::Enum { variant: a }, Value::Enum { variant: b }) => a == b,

        (Op::Gt, Value::Int(a), Value::Int(b)) => a > b,
        (Op::Lt, Value::Int(a), Value::Int(b)) => a < b,
        _ => false,
    }
}

pub(crate) fn filter<'a>(vrow: &'a VRows, filter: &Filter) -> Vec<&'a HRows> {
    vrow.vrow_iter()
        .filter(|hrow| match_row(hrow, filter))
        .collect()
}

pub struct Update {
    pub column: usize,
    pub value: Value,
}
impl Update {
    pub fn u(column: usize, value: Value) -> Self {
        Self { column, value }
    }
}
