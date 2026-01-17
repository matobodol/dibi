use serde::{Deserialize, Serialize};

use crate::db::{
    Value,
    baris::filter::{Filter, Update, match_row},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HRows {
    values: Vec<Value>,
}

impl HRows {
    pub(super) fn get(&self, idx: usize) -> Option<&Value> {
        self.values.get(idx)
    }
    pub(super) fn get_mut(&mut self, idx: usize) -> Option<&mut Value> {
        self.values.get_mut(idx)
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = &Value> {
        self.values.iter()
    }
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

    pub(crate) fn apply_delete(&mut self, filter: &Filter) {
        self.hrow.retain(|hrow| !match_row(hrow, filter));
    }
    pub(crate) fn apply_update(&mut self, update: Update, filter: &Filter) {
        for hrow in self.hrow.iter_mut() {
            if match_row(hrow, filter) {
                if let Some(value) = hrow.get_mut(update.column) {
                    *value = update.value.clone();
                }
            }
        }
    }

    pub(crate) fn vrow_len(&self) -> usize {
        self.hrow.len()
    }

    pub(crate) fn hrow_len(&self, idx_vrow: usize) -> Option<usize> {
        self.hrow.get(idx_vrow).map(|v| v.values.len())
    }

    pub fn vrow_iter(&self) -> impl Iterator<Item = &HRows> {
        self.hrow.iter()
    }

    pub(crate) fn get_read_hrows(&self, idx_vrow: usize) -> Option<&[Value]> {
        self.hrow.get(idx_vrow).map(|row| row.values.as_slice())
    }

    pub(crate) fn get_read_values(&self, idx_vrow: usize, idx_header: usize) -> Option<&Value> {
        self.hrow.get(idx_vrow)?.values.get(idx_header)
    }
}
