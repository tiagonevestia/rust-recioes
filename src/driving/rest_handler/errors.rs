use core::fmt;
use std::fmt::Display;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug, PartialEq)]
pub enum ApiError {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    ValidationError(Vec<String>),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::BadRequest(err)
            | ApiError::InternalServerError(err)
            | ApiError::NotFound(err)
            | ApiError::InvalidData(err)
            | ApiError::Conflict(err)
            | ApiError::Unknown(err) => writeln!(f, "{},", err),
            ApiError::ValidationError(mex_vec) => {
                for err in mex_vec {
                    write!(f, "{}, ", err)?;
                }
                Ok(())
            }
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => HttpResponse::BadRequest().json(error.to_string()),
            ApiError::NotFound(message) => HttpResponse::NotFound().json(message.to_string()),
            ApiError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().json(&errors.to_vec())
            }
            ApiError::InternalServerError(error) => {
                HttpResponse::Unauthorized().json(error.to_string())
            }
            ApiError::Conflict(error) => HttpResponse::Conflict().json(error.to_string()),
            ApiError::InvalidData(error) => HttpResponse::BadRequest().json(error.to_string()),
            ApiError::Unknown(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
