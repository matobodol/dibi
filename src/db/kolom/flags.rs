use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Flags {
    primary_key: bool,
    increment: bool,
    nullable: bool,
}

pub enum Eflags {
    Default,
    Pk,
    Nul,
    Inc,
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
    pub(super) fn set_primary_key(&mut self) {
        self.primary_key = true;
        self.nullable = false;
    }
    pub(super) fn unset_primary_key(&mut self) {
        self.primary_key = false;
        self.nullable = true;
    }

    pub(super) fn set_nullable(&mut self) {
        if !self.primary_key {
            self.nullable = true;
        }
    }
    pub(super) fn unset_nullable(&mut self) {
        self.primary_key = false;
    }

    pub(super) fn set_increment(&mut self) {
        self.increment = true;
    }
    pub(super) fn unset_increment(&mut self) {
        self.increment = false;
    }

    pub(super) fn is_primary_key(&self) -> bool {
        self.primary_key
    }
    pub(super) fn is_nullable(&self) -> bool {
        self.nullable
    }
    pub(super) fn is_increment(&self) -> bool {
        self.increment
    }
}
