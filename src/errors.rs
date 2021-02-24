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
pub enum BleScannerError {
    #[error("Bad Request")]
    BadRequest,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Request Timeout")]
    RequestTimeout,
    #[error("An internal error occurred. Please try again later.")]
    InternalError,
}

impl BleScannerError {
    pub fn name(&self) -> String {
        match self {
            Self::BadRequest => "Bad Request".to_string(),
            Self::Forbidden => "Forbidden".to_string(),
            Self::NotFound => "Not Found".to_string(),
            Self::RequestTimeout => "Request Timeout".to_string(),
            Self::InternalError => "InternalError".to_string(),
        }
    }
}

impl ResponseError for BleScannerError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
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
