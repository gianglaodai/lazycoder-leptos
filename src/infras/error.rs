#![cfg(feature = "ssr")]

use crate::business::error::CoreError;

impl From<sqlx::Error> for CoreError {
    fn from(error: sqlx::Error) -> Self {
        // Log details about the SQLx error for diagnostics
        match &error {
            sqlx::Error::Database(db_err) => {
                let code = db_err
                    .code()
                    .map(|c| c.to_string())
                    .unwrap_or("".to_string());
                let constraint = db_err.constraint().unwrap_or("");
                // message() is human-readable DB message
                log::error!(
                    "SQLx Database error: message='{}', code={:?}, constraint='{}'",
                    db_err.message(),
                    code,
                    constraint
                );
            }
            sqlx::Error::RowNotFound => {
                log::error!("SQLx error: Row not found");
            }
            other => {
                // Fallback to Debug for other variants (Io, PoolTimedOut, Decode, etc.)
                log::error!("SQLx error: {:?}", other);
            }
        }

        // Map all SQL errors to a generic internal server error for now.
        // If needed later, map specific variants to more specific CoreError kinds.
        CoreError::internal_server_error("error.db.unknown")
    }
}
