use crate::business::error::CoreError;

impl From<sqlx::Error> for CoreError {
    fn from(error: sqlx::Error) -> Self {
        CoreError::DatabaseError(error.to_string())
    }
}
