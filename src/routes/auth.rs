use crate::{
    models::{error::AppError, user::User},
    storage::file_storage::FileStorage,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{
    http::StatusCode,
    reject::{Reject, Rejection},
    Filter, Reply,
};

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    username: String,
}

pub fn auth_routes(
    file_storage: FileStorage,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let login = warp::path!("login")
        .and(warp::post())
        .and(json_body())
        .and(with_storage(file_storage.clone()))
        .and_then(handle_login);

    login
}

async fn handle_login(req: LoginRequest, storage: FileStorage) -> Result<impl Reply, Rejection> {
    let mut users: Vec<User> = storage.read_json("user.json").map_err(|e| {
        log::error!("Failed to read user.json: {}", e);
        AppError::FileSystemError(e.to_string())
    })?;
    log::info!("users count: {}", users.len());

    let user_index = users
        .iter()
        .position(|u| u.username == req.username)
        .ok_or_else(|| AppError::UserNotFound)?;

    if !storage
        .verify_password(&req.password, &users[user_index].password)
        .map_err(|_| AppError::PasswordError)?
    {
        return Err(AppError::PasswordError.into());
    }

    let new_token = uuid::Uuid::new_v4().to_string();
    users[user_index].token = new_token.clone();

    storage
        .write_json("user.json", &users)
        .map_err(|e| AppError::FileSystemError(e.to_string()))?;

    let cookie = format!("token={}; Path=/; HttpOnly; Max-Age=2592000", new_token);
    Ok(warp::reply::with_header(
        warp::reply::json(&LoginResponse {
            username: req.username,
        }),
        "Set-Cookie",
        cookie,
    ))
}

fn json_body() -> impl Filter<Extract = (LoginRequest,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn with_storage(
    storage: FileStorage,
) -> impl Filter<Extract = (FileStorage,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}

// 认证中间件
pub fn auth_filter(
    storage: FileStorage,
) -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
    warp::any()
        .and(warp::cookie::optional("token"))
        .and(with_storage(storage))
        .and_then(|token: Option<String>, storage: FileStorage| async move {
            let token = token.ok_or(AppError::AuthenticationRequired)?;
            let users: Vec<User> = storage
                .read_json("user.json")
                .map_err(|_| AppError::AuthenticationRequired)?;
            users
                .into_iter()
                .find(|u| u.token == token)
                .ok_or_else(|| AppError::AuthenticationRequired)
                .map_err(|e: AppError| -> Rejection { e.into() })
        })
}
