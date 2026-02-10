
#[derive(Debug)]
pub enum ServiceError {
    SqlError(sqlx::Error),
    JwtError(jsonwebtoken::errors::Error),
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
