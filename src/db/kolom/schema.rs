use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::db::{
    DbError, HeaderType,
    columndef::ColumnDef,
    flags::{Eflags, Flags},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    column: Vec<ColumnDef>,
    index_header: HashMap<String, usize>,
    index_primary: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeaderIdx(usize);
impl HeaderIdx {
    fn get(self) -> usize {
        self.0
    }
}

impl Header {
    fn header_id(&self, name: &str) -> Result<HeaderIdx, DbError> {
        self.index_header
            .get(name)
            .copied()
            .map(HeaderIdx)
            .ok_or(DbError::HeaderNotFound)
    }

    pub fn new() -> Self {
        Self {
            column: Vec::new(),
            index_header: HashMap::new(),
            index_primary: None,
        }
    }

    pub fn add(&mut self, name: &str, tipe: HeaderType, eflags: &[Eflags]) -> Result<(), DbError> {
        self.validate_duplicate_header(name)?;

        let mut flags = Flags::default();
        for f in eflags {
            match f {
                Eflags::Pk => flags.set_primary_key(),
                Eflags::Nul => flags.set_nullable(),
                Eflags::Inc => flags.set_increment(),
                Eflags::Default => {}
            }
        }

        if flags.is_primary_key() && self.index_primary.is_some() {
            return Err(DbError::PrimaryKeyIsAxist { tip: "".into() });
        }

        // index harus sebelum terjadi push
        let idx = self.column.len();

        let col = ColumnDef::new(name, tipe, flags);

        self.column.push(col);

        if self.column[idx].is_primary_key() {
            self.index_primary = Some(idx);
        }
        self.index_header.insert(name.to_string(), idx);

        Ok(())
    }
    pub fn set_primary_key(&mut self, name: &str) -> Result<(), DbError> {
        let &new_idx = self.index_header.get(name).ok_or(DbError::HeaderNotFound)?;

        if let Some(old_idx) = self.index_primary {
            self.column[old_idx].unset_primary_key();
        }

        // rebuild pk
        self.column[new_idx].set_primary_key();
        self.index_primary = Some(new_idx);

        Ok(())
    }

    fn validate_duplicate_header(&self, name: &str) -> Result<(), DbError> {
        if self.index_header.contains_key(name) {
            return Err(DbError::DuplicateHeaderName {
                name: name.into(),
                tip: "nama header tidak boleh sama.".into(),
            });
        }
        Ok(())
    }

    pub fn validate_nullable(&self, idx_column: usize) -> Result<(), DbError> {
        let nullable = self.column[idx_column].is_nullable();
        let isprimary = self.column[idx_column].is_primary_key();

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

    pub fn get_header_name(&self) -> Vec<&str> {
        self.column.iter().map(|k| k.get_name()).collect()
    }
    pub fn get_header_type(&self, name: &str) -> Result<&HeaderType, DbError> {
        self.header_id(name)
            .map(|idx| self.column[idx.get()].header_tipe())
    }

    pub fn is_primary_key(&self, name: &str) -> bool {
        self.header_id(name)
            .map(|idx| self.index_primary == Some(idx.get()))
            .unwrap_or(false)
    }
    pub fn is_nullable(&self, name: &str) -> bool {
        self.header_id(name)
            .map(|idx| self.column[idx.get()].is_nullable())
            .unwrap_or(false)
    }
    pub fn is_increment(&self, name: &str) -> bool {
        self.header_id(name)
            .map(|idx| self.column[idx.get()].is_increment())
            .unwrap_or(false)
    }
}
