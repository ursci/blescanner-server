//! Defining custom errors

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}
#[derive(Debug, Error)]
pub enum BleScannerApiError {
    #[error("An internal error occurred. Please try again later.")]
    InternalError,
}

impl BleScannerApiError {
    pub fn name(&self) -> String {
        match self {
            Self::InternalError => "InternalError".to_string(),
        }
    }
}

impl ResponseError for BleScannerApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

#[derive(Error, Debug)]
pub enum BleScnnerDbError {
    #[error("failed to open given file")]
    Diesel(#[from] r2d2::Error),
    #[error("failed to get a successful response: {0}")]
    R2d2(#[from] diesel::result::Error),
}
