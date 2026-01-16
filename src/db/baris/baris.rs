use serde::{Deserialize, Serialize};

use crate::db::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HRows {
    values: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct VRows {
    hrow: Vec<HRows>,
}

impl VRows {
    pub(crate) fn new() -> Self {
        Self { hrow: Vec::new() }
    }

    pub(crate) fn default(&self, len: usize) -> Vec<Value> {
        vec![Value::Null; len]
    }

    pub(crate) fn insert_values(&mut self, values: Vec<Value>) {
        let hrow = HRows { values };

        self.hrow.push(hrow);
    }

    pub(crate) fn drop_value(&mut self, idx: usize) {
        self.hrow.iter_mut().for_each(|v| {
            v.values.remove(idx);
        });
    }
    pub(crate) fn drop_vrow(&mut self, idx: usize) {
        self.hrow.remove(idx);
    }

    pub(crate) fn vrow_len(&self) -> usize {
        self.hrow.len()
    }

    pub(crate) fn hrow_len(&self, idx_vrow: usize) -> Option<usize> {
        self.hrow.get(idx_vrow).map(|v| v.values.len())
    }

    pub(crate) fn get_read_hrows(&self, idx_vrow: usize) -> Option<&[Value]> {
        self.hrow.get(idx_vrow).map(|row| row.values.as_slice())
    }

    pub(crate) fn get_read_values(&self, idx_vrow: usize, idx_header: usize) -> Option<&Value> {
        self.hrow.get(idx_vrow)?.values.get(idx_header)
    }
}
