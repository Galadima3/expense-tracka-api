use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation Failed")]
    Validation(#[from] validator::ValidationErrors),

    #[error("Expense not found")]
    NotFound,

    #[error("Expense already exists")]
    Conflict,

    #[error("Database error")]
    Database,

   
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Expense not found").into_response(),

            AppError::Conflict => (StatusCode::CONFLICT, "Expense already exists").into_response(),

            AppError::Database => StatusCode::INTERNAL_SERVER_ERROR.into_response(),

            AppError::Validation(_) => (
                StatusCode::BAD_REQUEST,
               
            )
                .into_response(),    
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            other => {
                tracing::error!("Database error: {other:?}");
                AppError::Database
            }
        }
    }
}

// impl From<validator::ValidationError> for AppError {
//     fn from(err: validator::ValidationError) -> Self {
        
//     }
// }