use serde::{Deserialize, Serialize};

use crate::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HRows {
    pub value: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VRows {
    pub hrow: Vec<HRows>,
}

impl VRows {
    pub fn new() -> Self {
        Self { hrow: Vec::new() }
    }

    pub fn add(&mut self, baris: HRows) {
        self.hrow.push(baris);
    }
}
