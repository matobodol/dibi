use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{db::Tabel, error::DbError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub tabel: HashMap<String, Tabel>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tabel: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, nama: &str) -> Result<(), DbError> {
        if self.tabel.contains_key(nama) {
            return Err(DbError::DuplicateTableName);
        }
        self.tabel.insert(nama.into(), Tabel::new());

        Ok(())
    }
}
