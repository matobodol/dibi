use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::db::{
    DbError, HeaderType,
    columndef::ColumnDef,
    flags::{EFlags, Flags},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    column: Vec<ColumnDef>,
    index_header: HashMap<String, usize>,
    index_primary: Option<usize>,
}

impl Header {
    pub fn new() -> Self {
        Self {
            column: Vec::new(),
            index_header: HashMap::new(),
            index_primary: None,
        }
    }

    pub fn add(&mut self, name: &str, tipe: HeaderType, eflags: EFlags) -> Result<(), DbError> {
        self.validate_new_header(name)?;

        let mut flags = Flags::default();
        flags.flag(eflags);

        let idx = self.column.len();
        let col = ColumnDef::new(name, tipe, flags.clone());

        self.index_header.insert(name.to_string(), idx);
        self.column.push(col);
        if flags.readflag(EFlags::Pk) {
            self.set_pk(name)?;
        }

        Ok(())
    }

    pub fn set_pk(&mut self, name: &str) -> Result<(), DbError> {
        let &new_idx = self
            .index_header
            .get(name)
            .ok_or_else(|| DbError::HeaderNotFound)?;

        if let Some(old_pk) = self.index_primary {
            self.column[old_pk].unflag(EFlags::Pk);
        }

        self.column[new_idx].flag(EFlags::Pk);
        self.index_primary = Some(new_idx);

        Ok(())
    }

    fn validate_new_header(&self, name: &str) -> Result<(), DbError> {
        if self.index_header.contains_key(name) {
            return Err(DbError::DuplicateHeaderName {
                name: name.into(),
                tip: "nama header tidak boleh sama.".into(),
            });
        }
        Ok(())
    }

    pub fn validate_nullable(&self, idx_column: usize) -> Result<(), DbError> {
        let nullable = self.column[idx_column].readflg(EFlags::Nul);
        let isprimary = self.column[idx_column].readflg(EFlags::Pk);

        if !nullable || isprimary {
            return Err(DbError::CannotBeNull(
                "Protected by 'fn validate_nullable()'".into(),
            ));
        }

        Ok(())
    }

    pub fn validate_values_count(&self, row_len: usize) -> Result<(), DbError> {
        let header_len = self.index_header.len();

        if row_len < header_len {
            return Err(DbError::ValuesCountIsLess);
        } else if row_len > header_len {
            return Err(DbError::ValuesCountIsGreet);
        }

        Ok(())
    }

    pub fn header_len(&self) -> usize {
        self.index_header.len()
    }
}
