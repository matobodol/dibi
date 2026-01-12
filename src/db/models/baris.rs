use serde::{Deserialize, Serialize};

use crate::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HRows {
    values: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VRows {
    hrow: Vec<HRows>,
}

impl VRows {
    pub fn new() -> Self {
        Self { hrow: Vec::new() }
    }

    pub fn insert_values(&mut self, values: Vec<Value>) {
        let hrow = HRows { values };

        self.hrow.push(hrow);
    }

    pub fn vrow_len(&self) -> usize {
        self.hrow.len()
    }

    pub fn hrow_len(&self, idx_vrow: usize) -> Option<usize> {
        self.hrow.get(idx_vrow).map(|v| v.values.len())
    }

    pub fn hrow_get_read(&self, idx_vrow: usize) -> Option<&[Value]> {
        self.hrow.get(idx_vrow).map(|row| row.values.as_slice())
    }

    pub fn value_get_read(&self, idx_vrow: usize, idx_header: usize) -> Option<&Value> {
        self.hrow.get(idx_vrow)?.values.get(idx_header)
    }
}
