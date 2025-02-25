use crate::{
    models::{error::AppError, user::User},
    storage::file_storage::FileStorage,
};
use serde::{Deserialize, Serialize};

use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

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
    // 读取用户数据
    let mut users: Vec<User> = storage.read_json("user.json").unwrap_or_default();

    // 查找匹配用户
    // 查找用户索引
    let username = req.username.clone();
    let user_index = users
        .iter()
        .position(|u| u.username == username)
        .ok_or_else(|| warp::reject::custom(AppError::UserNotFound))?;

    // 验证密码（使用索引访问）
    if !storage
        .verify_password(&req.password, &users[user_index].password)
        .map_err(|e| warp::reject::custom(e))?
    {
        return Err(warp::reject::custom(AppError::AuthenticationRequired));
    }

    // 更新token（完全分离可变操作）
    let new_token = uuid::Uuid::new_v4().to_string();

    // 创建独立作用域更新token
    {
        let user = &mut users[user_index];
        user.token = new_token.clone();
    }

    // 先保存users数据
    storage.write_json("user.json", &users)?;

    // 设置Cookie（使用new_token变量）
    let cookie = format!("token={}; Path=/; HttpOnly; Max-Age=2592000", new_token);
    Ok(warp::reply::with_header(
        warp::reply::json(&LoginResponse {
            username: username.clone(),
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
                .map_err(|e| warp::reject::custom(e))
        })
}
