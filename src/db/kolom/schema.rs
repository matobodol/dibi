use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::db::{
    DbError, HeaderType,
    columndef::ColumnDef,
    flags::{Eflags, Flags},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Header {
    column: Vec<ColumnDef>,
    index_header: HashMap<String, usize>,
    index_primary: Option<usize>,
}

impl Header {
    pub(crate) fn new() -> Self {
        Self {
            column: Vec::new(),
            index_header: HashMap::new(),
            index_primary: None,
        }
    }

    pub(crate) fn add(
        &mut self,
        name: &str,
        tipe: HeaderType,
        eflags: &[Eflags],
    ) -> Result<(), DbError> {
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

    pub(crate) fn drop_header_by_index(&mut self, idx: usize) {
        self.column.remove(idx);
    }

    pub(crate) fn update_index_header(&mut self) {
        self.index_header.clear();
        for (i, v) in self.column.iter().enumerate() {
            self.index_header.insert(v.get_name().to_string(), i);
        }
    }

    // pub(crate) fn get_name(&self, index: usize) -> &str {
    //     self.column[index].get_name()
    // }

    pub(crate) fn header_len(&self) -> usize {
        self.index_header.len()
    }

    // pub(crate) fn get_index_pk(&self) -> Option<usize> {
    //     self.index_primary
    // }

    pub(crate) fn is_axisting(&self, name: &str) -> bool {
        self.get_list_names().contains(&name)
    }

    pub(crate) fn get_list_names(&self) -> Vec<&str> {
        self.column.iter().map(|k| k.get_name()).collect()
    }

    pub(crate) fn get_header_type(&self, name: &str) -> Result<&HeaderType, DbError> {
        self.to_column_ref(name, |c| c.header_tipe())
    }

    pub(crate) fn get_header_index(&self, name: &str) -> Result<usize, DbError> {
        Ok(self.header_id(name)?.get())
    }

    pub(crate) fn is_primary_key(&self, name: &str) -> bool {
        self.to_column(name, |c| c.is_primary_key())
            .unwrap_or(false)
    }
    pub(crate) fn is_nullable(&self, name: &str) -> bool {
        self.to_column(name, |c| c.is_nullable()).unwrap_or(false)
    }
    pub(crate) fn is_increment(&self, name: &str) -> bool {
        self.to_column(name, |c| c.is_increment()).unwrap_or(false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeaderIdx(usize);
impl HeaderIdx {
    fn get(self) -> usize {
        self.0
    }
}

// helper
impl Header {
    // get index ccolumn by name
    fn header_id(&self, name: &str) -> Result<HeaderIdx, DbError> {
        self.index_header
            .get(name)
            .copied()
            .map(HeaderIdx)
            .ok_or(DbError::HeaderNotFound)
    }

    // predicate for Ccolumn
    fn to_column<T>(&self, name: &str, f: impl FnOnce(&ColumnDef) -> T) -> Result<T, DbError> {
        let idx = self.header_id(name)?;
        Ok(f(&self.column[idx.get()]))
    }
    fn to_column_ref<'a, T>(
        &'a self,
        name: &str,
        f: impl FnOnce(&'a ColumnDef) -> T,
    ) -> Result<T, DbError> {
        let idx = self.header_id(name)?;
        Ok(f(&self.column[idx.get()]))
    }
    fn to_column_mut<T>(
        &mut self,
        name: &str,
        f: impl FnOnce(&mut ColumnDef) -> T,
    ) -> Result<T, DbError> {
        let idx = self.header_id(name)?;
        Ok(f(&mut self.column[idx.get()]))
    }
}

// flags services
impl Header {
    pub(crate) fn set_primary_key(&mut self, name: &str) -> Result<(), DbError> {
        let &new_idx = self.index_header.get(name).ok_or(DbError::HeaderNotFound)?;

        if let Some(old_idx) = self.index_primary {
            self.column[old_idx].unset_primary_key();
        }

        // rebuild pk
        self.column[new_idx].set_primary_key();
        self.index_primary = Some(new_idx);

        Ok(())
    }
    pub fn unset_primary_key(&mut self) -> Result<(), DbError> {
        let idx = self
            .index_primary
            .ok_or_else(|| DbError::PrimaryKeyNotAxist)?;

        self.column[idx].unset_primary_key();
        self.index_primary = None;
        Ok(())
    }

    pub fn set_nullable(&mut self, name: &str) -> Result<(), DbError> {
        self.to_column_mut(name, |k| k.set_nullable())?;
        Ok(())
    }
    pub fn unset_nullable(&mut self, name: &str) -> Result<(), DbError> {
        self.to_column_mut(name, |k| k.unset_nullable())?;
        Ok(())
    }
    pub fn set_increment(&mut self, name: &str) -> Result<(), DbError> {
        self.to_column_mut(name, |k| k.set_increment())?;
        Ok(())
    }
    pub fn unset_increment(&mut self, name: &str) -> Result<(), DbError> {
        self.to_column_mut(name, |k| k.unset_increment())?;
        Ok(())
    }
}
