use crate::db::Value;

#[derive(Debug, PartialEq)]
pub enum DbError {
    PrimaryKeyNotAxist,
    RowNotFound(usize),
    ProblemAddHeader,
    CannotBeNull {
        state: bool,
        reason: String,
        trigger: String,
    },
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
