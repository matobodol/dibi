use serde::{Deserialize, Serialize};

use crate::db::{DbError, Header, HeaderType, VRows, Value, flags::Eflags};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tabel {
    pub header: Header,
    pub rows: VRows,
}

impl Tabel {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            rows: VRows::new(),
        }
    }

    pub fn new_header(
        &mut self,
        name: &str,
        tipe: HeaderType,
        eflag: &[Eflags],
    ) -> Result<(), DbError> {
        self.header.add(name, tipe, eflag)?;
        Ok(())
    }

    pub fn insert_rows(&mut self, hrow: Vec<Value>) -> Result<(), DbError> {
        let mut hrow = hrow;
        let values_count = hrow.len();
        let header_count = self.header.header_len();

        if values_count < header_count {
            self.header.validate_nullable(values_count)?;

            for _ in 0..(header_count - values_count) {
                hrow.push(Value::Null);
            }
        }
        if values_count > header_count {
            return Err(DbError::ValuesCountIsGreet);
        }

        self.rows.insert_values(hrow);
        Ok(())
    }
}
