#![cfg(feature = "ssr")]

use log::error;
use sqlx::Error;
use sqlx::Error::{Database, RowNotFound};
use crate::common::error::CoreError;

impl From<Error> for CoreError {
    fn from(error: Error) -> Self {
        match &error {
            Database(db_err) => {
                let code = db_err
                    .code()
                    .map(|c| c.to_string())
                    .unwrap_or("".to_string());
                let constraint = db_err.constraint().unwrap_or("");
                error!(
                    "SQLx Database error: message='{}', code={:?}, constraint='{}'",
                    db_err.message(),
                    code,
                    constraint
                );
            }
            RowNotFound => {
                error!("SQLx error: Row not found");
            }
            other => {
                error!("SQLx error: {:?}", other);
            }
        }

        CoreError::internal_server_error("error.db.unknown")
    }
}
