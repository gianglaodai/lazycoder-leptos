#![cfg(feature = "ssr")]
use crate::common::error::CoreError;
use actix_web::{http::StatusCode, HttpResponse, Responder};

fn status_from_error(e: &CoreError) -> StatusCode {
    match e {
        CoreError::BadRequest(_, _) => StatusCode::BAD_REQUEST,
        CoreError::Unauthorized(_, _) => StatusCode::UNAUTHORIZED,
        CoreError::Forbidden(_, _) => StatusCode::FORBIDDEN,
        CoreError::NotFound(_, _) => StatusCode::NOT_FOUND,
        CoreError::Conflict(_, _) => StatusCode::CONFLICT,
        CoreError::UnprocessableEntity(_, _) => StatusCode::UNPROCESSABLE_ENTITY,
        CoreError::InternalServerError(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub fn respond_result<T>(result: Result<T, CoreError>) -> impl Responder
where
    T: serde::Serialize,
{
    match result {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(e) => {
            let status = status_from_error(&e);
            log::warn!("{}", e);
            HttpResponse::build(status)
                .content_type("application/json; charset=utf-8")
                .body(e.to_json())
        }
    }
}

pub fn respond_results<T, U, F>(result: Result<Vec<T>, CoreError>, mapper: F) -> impl Responder
where
    F: Fn(T) -> U,
    U: serde::Serialize,
{
    match result {
        Ok(vec) => {
            let mapped = vec.into_iter().map(mapper).collect::<Vec<U>>();
            HttpResponse::Ok().json(mapped)
        }
        Err(e) => {
            let status = status_from_error(&e);
            log::warn!("{}", e);
            HttpResponse::build(status)
                .content_type("application/json; charset=utf-8")
                .body(e.to_json())
        }
    }
}
