use std::collections::HashMap;

#[derive(Debug)]
pub enum CoreError {
    InternalServerError(&'static str, HashMap<String, String>),
    BadRequest(&'static str, HashMap<String, String>),
    NotFound(&'static str, HashMap<String, String>),
    Conflict(&'static str, HashMap<String, String>),
    Unauthorized(&'static str, HashMap<String, String>),
    Forbidden(&'static str, HashMap<String, String>),
    UnprocessableEntity(&'static str, HashMap<String, String>),
}

impl CoreError {
    pub fn internal_server_error(message: &'static str) -> CoreError {
        CoreError::InternalServerError(message, HashMap::new())
    }

    pub fn bad_request(message: &'static str) -> CoreError {
        CoreError::BadRequest(message, HashMap::new())
    }
    pub fn not_found(message: &'static str) -> CoreError {
        CoreError::NotFound(message, HashMap::new())
    }
    pub fn conflict(message: &'static str) -> CoreError {
        CoreError::Conflict(message, HashMap::new())
    }
    pub fn forbidden(message: &'static str) -> CoreError {
        CoreError::Forbidden(message, HashMap::new())
    }
    pub fn unauthorized(message: &'static str) -> CoreError {
        CoreError::Unauthorized(message, HashMap::new())
    }
    pub fn unprocessable_entity(message: &'static str) -> CoreError {
        CoreError::UnprocessableEntity(message, HashMap::new())
    }
}
