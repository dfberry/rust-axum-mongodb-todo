use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum CollectionError {
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
    #[error("Document with ID: {0} not found")]
    NotFoundError(String),
    #[error("document does not contain shard key at {0}")]
    ShardKeyError(String)
}

#[derive(Serialize)]
struct ErrorResponse {
    status: &'static str,
    message: String,
}

impl Into<(axum::http::StatusCode, Json<serde_json::Value>)> for CollectionError {
    fn into(self) -> (axum::http::StatusCode, Json<serde_json::Value>) {
        let (status, error_response) = match self {
        CollectionError::MongoErrorKind(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse {
                status: "error",
                message: format!("MongoDB error kind: {}", e),
            },
        ),
        CollectionError::MongoDuplicateError(_) => (
            StatusCode::CONFLICT,
            ErrorResponse {
                status: "fail",
                message: "List with that title already exists".to_string(),
            },
        ),
        CollectionError::InvalidIDError(id) => (
            StatusCode::BAD_REQUEST,
            ErrorResponse {
                status: "fail",
                message: format!("invalid ID: {}", id),
            },
        ),
        CollectionError::NotFoundError(id) => (
            StatusCode::NOT_FOUND,
            ErrorResponse {
                status: "fail",
                message: format!("List with ID: {} not found", id),
            },
        ),
        CollectionError::MongoError(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse {
                status: "error",
                message: format!("MongoDB error: {}", e),
            },
        ),
        CollectionError::MongoQueryError(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse {
                status: "error",
                message: format!("MongoDB error: {}", e),
            },
        ),
        CollectionError::MongoSerializeBsonError(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse {
                status: "error",
                message: format!("MongoDB error: {}", e),
            },
        ),
        CollectionError::MongoDataError(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse {
                status: "error",
                message: format!("MongoDB error: {}", e),
            },
        ),
        CollectionError::ShardKeyError(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse {
                status: "error",
                message: format!("CosmosDB MongoDB sharding error: {}", e),
            },
        ),        
    };
    (status, Json(serde_json::to_value(error_response).unwrap()))
}
}

impl From<CollectionError> for (StatusCode, ErrorResponse) {
    fn from(err: CollectionError) -> (StatusCode, ErrorResponse) {
        err.into()
    }
}