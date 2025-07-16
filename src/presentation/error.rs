use crate::business::error::CoreError;

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::InternalServerError(msg, _) => write!(f, "Internal server error: {}", msg),
            CoreError::BadRequest(msg, _) => write!(f, "BadRequest: {}", msg),
            CoreError::NotFound(msg, _) => write!(f, "Notfound: {}", msg),
            CoreError::Conflict(msg, _) => write!(f, "Conflict: {}", msg),
            CoreError::Unauthorized(msg, _) => write!(f, "Unauthorized: {}", msg),
            CoreError::Forbidden(msg, _) => write!(f, "Forbidden: {}", msg),
            CoreError::UnprocessableEntity(msg, _) => write!(f, "UnprocessableEntity: {}", msg),
        }
    }
}

