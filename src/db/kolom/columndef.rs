use serde::{Deserialize, Serialize};

use crate::db::{HeaderType, flags::EFlags};

use super::flags::Flags;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct ColumnDef {
    name: String,
    tipe: HeaderType,
    flags: Flags,
}
impl ColumnDef {
    pub(super) fn new(name: &str, tipe: HeaderType, flags: Flags) -> Self {
        Self {
            name: name.to_string(),
            tipe,
            flags,
        }
    }

    pub(super) fn flag(&mut self, eflag: EFlags) {
        self.flags.flag(eflag);
    }
    pub(super) fn unflag(&mut self, eflag: EFlags) {
        self.flags.unflag(eflag);
    }
    pub(super) fn readflg(&self, eflag: EFlags) -> bool {
        self.flags.readflag(eflag)
    }
}
