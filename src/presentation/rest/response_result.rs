#![cfg(feature = "ssr")]
use actix_web::{HttpResponse, Responder};

pub fn respond_result<T, E>(result: Result<T, E>) -> impl Responder
where
    T: serde::Serialize,
    E: std::fmt::Display,
{
    match result {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(e) => {
            log::error!("Error: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub fn respond_results<T, U, E, F>(result: Result<Vec<T>, E>, mapper: F) -> impl Responder
where
    F: Fn(T) -> U,
    U: serde::Serialize,
    E: std::fmt::Display,
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
