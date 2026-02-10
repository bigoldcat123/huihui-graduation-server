
#[derive(Debug)]
pub enum ServiceError {
    SqlError(sqlx::Error)
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        ServiceError::SqlError(err)
    }
}
