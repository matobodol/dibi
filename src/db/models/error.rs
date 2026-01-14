use crate::db::Value;

#[derive(Debug, PartialEq)]
pub enum DbError {
    ProblemAddHeader,
    CannotBeNull(String),
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
    ColumnCountMissMatch,
    ValuesCountIsLess,
    ValuesCountIsGreet,
    ValurNotFound,
}
