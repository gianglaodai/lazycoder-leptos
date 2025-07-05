#[derive(Debug)]
pub enum CoreError {
    DatabaseError(String),
    NotFound,
    ValidationError(String),
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            CoreError::NotFound => write!(f, "Resource not found"),
            CoreError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

