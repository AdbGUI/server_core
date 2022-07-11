#![allow(dead_code)]
use actix_web::{
    error::{BlockingError, ResponseError},
    Error as ActixError, HttpResponse,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, PartialEq, Eq)]
pub enum Error {
    BadRequest(String),
    InternalServer(String),
    Unauthorized,
    Forbidden,
    NotFound(String),
    PoolError(String),
    BlockingError(String),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::BadRequest(message) => {
                let error: ErrorResponse = message.into();
                HttpResponse::BadRequest().json(error)
            }
            Error::NotFound(message) => {
                let error: ErrorResponse = message.into();
                HttpResponse::NotFound().json(error)
            }
            Error::Forbidden => {
                let error: ErrorResponse = "Forbidden".into();
                HttpResponse::Forbidden().json(error)
            }
            _ => {
                error!("Internal server error: {:?}", self);
                let error: ErrorResponse = "Internal Server Error".into();
                HttpResponse::InternalServerError().json(error)
            }
        }
    }
}
// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
}

impl From<&str> for ErrorResponse {
    fn from(error: &str) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

impl From<Vec<String>> for ErrorResponse {
    fn from(error: Vec<String>) -> Self {
        ErrorResponse { errors: error }
    }
}

impl From<BlockingError> for Error {
    fn from(error: BlockingError) -> Error {
        error!("Thread blocking error: {:?}", error);
        Error::BlockingError("Thread blocking error".into())
    }
}

impl From<ActixError> for Error {
    fn from(error: ActixError) -> Error {
        Error::InternalServer(error.to_string())
    }
}
