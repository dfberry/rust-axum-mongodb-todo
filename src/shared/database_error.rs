use serde::Serialize;
use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum MyDBError {
    #[error("MongoDB error")]
    MongoError(#[from] mongodb::error::Error),
    #[error("duplicate key error: {0}")]
    MongoErrorKind(mongodb::error::ErrorKind),
    #[error("duplicate key error: {0}")]
    MongoDuplicateError(mongodb::error::Error),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("error serializing BSON")]
    MongoSerializeBsonError(#[from] mongodb::bson::ser::Error),
    #[error("validation error")]
    MongoDataError(#[from] mongodb::bson::document::ValueAccessError),
    #[error("invalid ID: {0}")]
    InvalidIDError(String),
    #[error("Doc with ID: {0} not found")]
    NotFoundError(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    status: &'static str,
    message: String,
}

impl From<MyDBError> for ErrorResponse {
    fn from(err: MyDBError) -> ErrorResponse {
        ErrorResponse {
            status: "error",
            message: err.to_string(),
        }
    }
}
