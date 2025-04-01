use std::{convert::Infallible, fmt};
use warp::{
    http::StatusCode,
    reject::{Reject, Rejection},
    Filter, Reply,
};

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
    HouseNotFound,
    PasswordError,
    VersionMismatch,
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
            AppError::HouseNotFound => write!(f, "House not found"),
            AppError::PasswordError => write!(f, "Invalid password"),
            AppError::VersionMismatch => write!(f, "Version Mismatch"),
        }
    }
}

impl Reject for AppError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if let Some(e) = err.find::<AppError>() {
        (e.status_code(), e.to_string())
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method not allowed".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({
            "message": message
        })),
        code,
    ))
}

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
            AppError::HouseNotFound => StatusCode::NOT_FOUND,
            AppError::PasswordError => StatusCode::UNAUTHORIZED,
            AppError::VersionMismatch => StatusCode::CONFLICT,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystemError(err.to_string())
    }
}
