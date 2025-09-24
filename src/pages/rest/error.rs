use crate::common::error::CoreError;
use serde::Serialize;
use serde_json::to_string;
use std::collections::HashMap;

#[derive(Serialize)]
struct CoreErrorDto<'a> {
    code: &'a str,
    message: &'a str,
    details: &'a HashMap<String, String>,
}

impl CoreError {
    pub fn to_json(&self) -> String {
        let dto = match self {
            CoreError::InternalServerError(msg, map) => CoreErrorDto {
                code: "INTERNAL_SERVER_ERROR",
                message: msg,
                details: map,
            },
            CoreError::BadRequest(msg, map) => CoreErrorDto {
                code: "BAD_REQUEST",
                message: msg,
                details: map,
            },
            CoreError::NotFound(msg, map) => CoreErrorDto {
                code: "NOT_FOUND",
                message: msg,
                details: map,
            },
            CoreError::Conflict(msg, map) => CoreErrorDto {
                code: "CONFLICT",
                message: msg,
                details: map,
            },
            CoreError::Unauthorized(msg, map) => CoreErrorDto {
                code: "UNAUTHORIZED",
                message: msg,
                details: map,
            },
            CoreError::Forbidden(msg, map) => CoreErrorDto {
                code: "FORBIDDEN",
                message: msg,
                details: map,
            },
            CoreError::UnprocessableEntity(msg, map) => CoreErrorDto {
                code: "UNPROCESSABLE_ENTITY",
                message: msg,
                details: map,
            },
        };
        to_string(&dto).unwrap_or_else(|_| "{}".into())
    }
}
