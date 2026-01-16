use crate::{
    Tabel,
    db::{DbError, Value},
};

impl Tabel {
    pub(crate) fn validate_is_primary(&self, name: &str) -> Result<(), DbError> {
        if self.is_primary_key(name) {
            return Err(DbError::PrimaryKeyIsAxist {
                tip: "drop header blocked by primary".into(),
            });
        }
        Ok(())
    }
    pub(crate) fn validate_is_axisting(&self, name: &str) -> Result<(), DbError> {
        if !self.is_axisting(name) {
            return Err(DbError::HeaderNotFound);
        }
        Ok(())
    }
    pub(crate) fn validate_is_empty(&self, names: &[&str]) -> Result<(), DbError> {
        if names.is_empty() {
            return Err(DbError::HeaderNotFound);
        }
        Ok(())
    }
    pub(crate) fn validate_values_count(
        &self,
        headers: &[&str],
        values: &[Value],
    ) -> Result<(), DbError> {
        if headers.len() != values.len() {
            return Err(DbError::ColumnCountMissMatch);
        }
        Ok(())
    }
    pub(super) fn validate_nullable(&self, hrow: &Vec<Value>) -> Result<(), DbError> {
        let names = self.get_list_header_name();

        for (n, v) in names.iter().zip(hrow) {
            if !self.is_nullable(n) && v == &Value::Null {
                return Err(DbError::CannotBeNull {
                    state: self.is_nullable(n),
                    reason: "header is protect null value".into(),
                    trigger: "validate_nullable".into(),
                });
            }
        }
        Ok(())
    }

    pub(super) fn validate_duplicate_header(&self, name: &str) -> Result<(), DbError> {
        if self.get_index_header(name).is_ok() {
            return Err(DbError::DuplicateHeaderName {
                name: name.into(),
                tip: "nama kolom tidak boleh sama.".into(),
            });
        }
        Ok(())
    }
}
