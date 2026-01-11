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

    pub fn insert_rows(&mut self, rows: HRows) -> Result<(), DbError> {
        let mut rows = rows;

        if rows.value.len() < self.header.index_header.len() {
            let idx = rows.value.len();
            let nullable = self.header.column[idx].flags.nullable;
            let isprimary = self.header.column[idx].flags.primary_key;

            if !nullable || isprimary {
                return Err(DbError::CannotBeNull);
            }
            rows.value.push(Value::Null);
        }
        self.vrow.hrow.push(rows);
        Ok(())
    }
}
