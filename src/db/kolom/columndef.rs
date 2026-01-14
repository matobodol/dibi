use serde::{Deserialize, Serialize};

use crate::db::HeaderType;

use super::flags::Flags;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct ColumnDef {
    name: String,
    tipe: HeaderType,
    flags: Flags,
}
impl ColumnDef {
    // buat baru definisi kolom
    pub(super) fn new(name: &str, tipe: HeaderType, flags: Flags) -> Self {
        Self {
            name: name.to_string(),
            tipe,
            flags,
        }
    }
    // share definisi spesifik kolom
    pub(super) fn get_name(&self) -> &str {
        &self.name
    }
    pub(super) fn header_tipe(&self) -> &HeaderType {
        &self.tipe
    }
}

impl ColumnDef {
    pub(super) fn set_primary_key(&mut self) {
        self.flags.set_primary_key();
    }
    pub(super) fn unset_primary_key(&mut self) {
        self.flags.unset_primary_key();
    }

    pub(super) fn set_nullable(&mut self) {
        self.flags.set_nullable();
    }
    pub(super) fn unset_nullable(&mut self) {
        self.flags.unset_nullable();
    }

    pub(super) fn set_increment(&mut self) {
        self.flags.set_increment();
    }
    pub(super) fn unset_increment(&mut self) {
        self.flags.unset_increment();
    }

    pub(super) fn is_primary_key(&self) -> bool {
        self.flags.is_primary_key()
    }
    pub(super) fn is_nullable(&self) -> bool {
        self.flags.is_nullable()
    }
    pub(super) fn is_increment(&self) -> bool {
        self.flags.is_increment()
    }
}
