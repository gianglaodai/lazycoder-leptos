#[derive(Debug)]
pub enum CoreError {
    DatabaseError(String),
    NotFound,
    ValidationError(String),
}
