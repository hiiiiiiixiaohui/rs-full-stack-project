use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum SysError {
    ActixError(String),
    NotFound(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct SysErrorResponse {
    error_message: String,
}

impl std::error::Error for SysError {}

impl SysError {
    fn error_response(&self) -> String {
        match self {
            SysError::ActixError(msg) => {
                println!("Server Error Occurred: {:?}", msg);
                "Internal Server Error".into()
            }
            SysError::TeraError(msg) => {
                println!("Error in rendering the template: {:?}", msg);
                msg.into()
            }
            SysError::NotFound(msg) => {
                println!("Not Found Error Occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for SysError {
    fn status_code(&self) -> StatusCode {
        match self {
            SysError::ActixError(_msg) | SysError::TeraError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            SysError::NotFound(_msg) => StatusCode::NOT_FOUND,
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
        match self {
            SysError::ActixError(msg) => write!(f, "{}", msg),
            SysError::NotFound(msg) => write!(f, "{}", msg),
            SysError::TeraError(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<actix_web::error::Error> for SysError {
    fn from(err: actix_web::error::Error) -> Self {
        SysError::ActixError(err.to_string())
    }
}

impl From<tera::Error> for SysError {
    fn from(err: tera::Error) -> Self {
        SysError::TeraError(err.to_string())
    }
}
