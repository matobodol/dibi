use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct Flags {
    primary_key: bool,
    increment: bool,
    nullable: bool,
}

pub enum EFlags {
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
    pub(super) fn flag(&mut self, eflag: EFlags) {
        match eflag {
            EFlags::Pk => {
                self.primary_key = true;
                self.nullable = false;
            }
            EFlags::Nul => {
                if !self.primary_key {
                    self.nullable = true;
                }
            }
            EFlags::Inc => self.increment = true,
            EFlags::Default => {
                Flags::default();
                {}
            }
        }
    }
    pub(super) fn unflag(&mut self, eflag: EFlags) {
        match eflag {
            EFlags::Pk => {
                self.primary_key = false;
                self.nullable = true;
            }
            EFlags::Nul => {
                if self.primary_key {
                    self.nullable = false;
                }
            }
            EFlags::Inc => self.increment = false,
            EFlags::Default => {
                Flags::default();
                {}
            }
        }
    }

    // read
    pub(super) fn readflag(&self, eflag: EFlags) -> bool {
        match eflag {
            EFlags::Pk => self.primary_key,
            EFlags::Nul => self.nullable,
            EFlags::Inc => self.increment,
            EFlags::Default => false,
        }
    }
}
