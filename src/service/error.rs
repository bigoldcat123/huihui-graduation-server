
#[derive(Debug)]
pub enum ServiceError {
    SqlError(sqlx::Error),
    JwtError(jsonwebtoken::errors::Error),
    JsonError(serde_json::Error),
    PermissionDenied(String),
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        ServiceError::SqlError(err)
    }
}

impl From<jsonwebtoken::errors::Error> for ServiceError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        ServiceError::JwtError(err)
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(err: serde_json::Error) -> Self {
        ServiceError::JsonError(err)
    }
}
