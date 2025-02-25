use std::fmt;
use warp::{http::StatusCode, reject::Reject};

#[derive(Debug)]
pub enum AppError {
    AuthenticationRequired,
    PermissionDenied,
    InvalidVersion,
    FileSystemError(String),
    ParseError(String),
    NotFound,
    InternalServerError,
    UserNotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::AuthenticationRequired => write!(f, "Authentication required"),
            AppError::PermissionDenied => write!(f, "Permission denied"),
            AppError::InvalidVersion => write!(f, "Invalid version"),
            AppError::FileSystemError(msg) => write!(f, "File system error: {}", msg),
            AppError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::InternalServerError => write!(f, "Internal server error"),
            AppError::UserNotFound => write!(f, "User not found"),
        }
    }
}

impl Reject for AppError {}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::AuthenticationRequired => StatusCode::UNAUTHORIZED,
            AppError::PermissionDenied => StatusCode::FORBIDDEN,
            AppError::InvalidVersion => StatusCode::CONFLICT,
            AppError::FileSystemError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ParseError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UserNotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystemError(err.to_string())
    }
}
