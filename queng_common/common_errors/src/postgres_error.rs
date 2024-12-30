use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PostgresDBError {
    ConnectionFailed(String),
    CountFailed(String),
    CheckFailed(String),
    CheckIfExistsFailed(String),
    InsertFailed(String),
    SetFieldFailed(String),
    UpdateFailed(String),
    DeleteFailed(String),
    QueryFailed(String),
    TableDoesNotExist(String, String),
    DataRecordDoesNotExist(String),
    TableSanitizeError(String),
    TransactionRollback(String),
    MigrationFailed(String),
    UnknownError(String),
}

impl Error for PostgresDBError {}

impl fmt::Display for PostgresDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed(e) => {
                write!(
                    f,
                    "[PostgresDBError]: Connection to Postgres DB failed with error: {e}"
                )
            }

            Self::CountFailed(e) => {
                write!(f, "[PostgresDBError]: Count of DB Table has failed: {e}")
            }

            Self::CheckFailed(e) => {
                write!(f, "[PostgresDBError]: Check if DB Table has failed: {e}")
            }

            Self::CheckIfExistsFailed(e) => {
                write!(f, "[PostgresDBError]: Check if DB Table exists failed: {e}")
            }

            Self::InsertFailed(e) => {
                write!(f, "[PostgresDBError]: Insert into DB failed: {e}")
            }

            Self::SetFieldFailed(e) => {
                write!(f, "[PostgresDBError]: Set field failed: {e}")
            }

            Self::UpdateFailed(e) => {
                write!(f, "[PostgresDBError]: DB Update failed: {e}")
            }

            Self::DeleteFailed(e) => {
                write!(f, "[PostgresDBError]: Delete failed: {e}")
            }
            Self::QueryFailed(e) => {
                write!(f, "[PostgresDBError]: DB Query failed: {e}")
            }

            Self::TableDoesNotExist(table_name, err) => {
                write!(
                    f,
                    "Table does not exist: Table {table_name} does not exist. Error: {err}"
                )
            }

            Self::TableSanitizeError(e) => {
                write!(f, "[PostgresDBError]: Table sanitization error: {e}")
            }

            Self::TransactionRollback(e) => {
                write!(
                    f,
                    "[PostgresDBError]: Transaction failed and rolled back: {e}"
                )
            }

            Self::DataRecordDoesNotExist(e) => {
                write!(f, "[PostgresDBError]: Data record does not exist: {e}")
            }

            Self::MigrationFailed(e) => {
                write!(f, "[PostgresDBError]: Migration failed: {e}")
            }

            Self::UnknownError(e) => {
                write!(f, "[PostgresDBError]: Unknown error: {e}")
            }
        }
    }
}
