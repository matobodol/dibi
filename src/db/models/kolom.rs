use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{db::HeaderType, error::DbError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags {
    primary_key: bool,
    increment: bool,
    nullable: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            primary_key: false,
            increment: false,
            nullable: true,
        }
    }
}
impl Flags {
    pub fn primary_key(&mut self, set: bool) {
        if set {
            self.primary_key = true;
            self.nullable(false);
        } else {
            self.primary_key = false;
            self.nullable(true);
        }
    }
    pub fn increment(&mut self, set: bool) {
        self.increment = set;
    }
    pub fn nullable(&mut self, set: bool) {
        if !self.primary_key {
            self.nullable = set
        }
        self.nullable = self.is_nul();
    }

    // read
    pub fn is_pk(&self) -> bool {
        self.primary_key
    }
    pub fn is_inc(&self) -> bool {
        self.increment
    }
    pub fn is_nul(&self) -> bool {
        self.nullable
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDef {
    pub name: String,
    pub tipe: HeaderType,
    pub flags: Flags,
}
impl ColumnDef {
    fn new(name: &str, tipe: HeaderType, flags: Flags) -> Self {
        Self {
            name: name.to_string(),
            tipe,
            flags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub column: Vec<ColumnDef>,
    pub index_header: HashMap<String, usize>,
    pub index_primary: Option<usize>,
}

impl Header {
    pub fn new() -> Self {
        Self {
            column: Vec::new(),
            index_header: HashMap::new(),
            index_primary: None,
        }
    }

    pub fn add(&mut self, name: &str, tipe: HeaderType, flags: Flags) -> Result<(), DbError> {
        self.validate_new_header(name)?;

        if flags.primary_key {
            self.set_pk(name)?;
        }

        let idx = self.column.len();
        let col = ColumnDef::new(name, tipe, flags);

        self.index_header.insert(name.to_string(), idx);
        self.column.push(col);

        Ok(())
    }

    pub fn set_pk(&mut self, name: &str) -> Result<(), DbError> {
        let &new_idx = self
            .index_header
            .get(name)
            .ok_or_else(|| DbError::HeaderNotFound)?;

        if let Some(old_pk) = self.index_primary {
            self.column[old_pk].flags.primary_key(false);
        }

        self.column[new_idx].flags.primary_key(true);
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
        let nullable = self.column[idx_column].flags.is_nul();
        let isprimary = self.column[idx_column].flags.is_pk();

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
}
