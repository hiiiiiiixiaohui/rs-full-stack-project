use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum SysError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct SysErrorResponse {
    error_message: String,
}

impl SysError {
    fn error_response(&self) -> String {
        match self {
            SysError::DBError(msg) => {
                println!("Database Error Occurred: {:?}", msg);
                "Database Error".into()
            }
            SysError::ActixError(msg) => {
                println!("Server Error Occurred: {:?}", msg);
                "Internal Server Error".into()
            }
            SysError::NotFound(msg) => {
                println!("Not Found Error Occurred: {:?}", msg);
                msg.into()
            }
            SysError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for SysError {
    fn status_code(&self) -> StatusCode {
        match self {
            SysError::DBError(_msg) | SysError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            SysError::NotFound(_msg) => StatusCode::NOT_FOUND,
            SysError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(SysErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for SysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // write!(f, "{}", self)
        match self {
            SysError::DBError(msg) => write!(f, "{}", msg),
            SysError::ActixError(msg) => write!(f, "{}", msg),
            SysError::NotFound(msg) => write!(f, "{}", msg),
            SysError::InvalidInput(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<actix_web::error::Error> for SysError {
    fn from(err: actix_web::error::Error) -> Self {
        SysError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for SysError {
    fn from(value: SQLxError) -> Self {
        SysError::DBError(value.to_string())
    }
}
