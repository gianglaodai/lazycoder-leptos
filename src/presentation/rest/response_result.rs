#![cfg(feature = "ssr")]
use crate::business::error::CoreError;
use actix_web::{HttpResponse, Responder};

pub fn respond_result<T>(result: Result<T, CoreError>) -> impl Responder
where
    T: serde::Serialize,
{
    match result {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(e) => {
            log::error!("Error: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
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
            log::error!("Error: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
