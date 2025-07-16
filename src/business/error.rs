use std::collections::HashMap;

#[derive(Debug)]
pub enum CoreError {
    InternalServerError(String, HashMap<String, String>),
    BadRequest(String, HashMap<String, String>),
    NotFound(String, HashMap<String, String>),
    Conflict(String, HashMap<String, String>),
    Unauthorized(String, HashMap<String, String>),
    Forbidden(String, HashMap<String, String>),
    UnprocessableEntity(String, HashMap<String, String>),
}
