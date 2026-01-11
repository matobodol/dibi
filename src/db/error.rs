use crate::Value;

#[derive(Debug)]
pub enum DbError {
    CannotBeNull,
    E(String),
    PrimaryKeyIsAxist {
        tip: String,
    },
    DuplicateTableName,
    DuplicateHeaderName {
        name: String,
        tip: String,
    },
    HeaderNotFound,
    TypeMissMatch {
        expected: Value,
        reasoon: String,
        found: Value,
    },
    ColumnCountMissMatch {
        reason: String,
    },
    ValurNotFound,
}
