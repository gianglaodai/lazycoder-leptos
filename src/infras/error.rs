#![cfg(feature = "ssr")]

use crate::business::error::CoreError;
use std::collections::HashMap;

impl From<sqlx::Error> for CoreError {
    fn from(error: sqlx::Error) -> Self {
        CoreError::InternalServerError(error.to_string(), HashMap::new())
    }
}
