use serde::{Deserialize, Serialize};

use crate::{
    Flags, HRows, HeaderType, Value,
    db::{Header, VRows},
    error::DbError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tabel {
    pub header: Header,
    pub vrow: VRows,
}

impl Tabel {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            vrow: VRows::new(),
        }
    }

    pub fn new_header(
        &mut self,
        name: &str,
        tipe: HeaderType,
        flags: Flags,
    ) -> Result<(), DbError> {
        self.header.add(name, tipe, flags)?;
        Ok(())
    }

    pub fn insert_rows(&mut self, hrow: HRows) -> Result<(), DbError> {
        let mut hrow = hrow;
        let values_count = hrow.value.len();
        let header_count = self.header.index_header.len();

        if values_count < header_count {
            self.header.validate_nullable(values_count)?;

            for _ in 0..(header_count - values_count) {
                hrow.value.push(Value::Null);
            }
        }
        if values_count > header_count {
            return Err(DbError::ValuesCountIsGreet);
        }

        self.vrow.hrow.push(hrow);
        Ok(())
    }
}
