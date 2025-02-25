mod models;
mod routes;
mod storage;

use crate::models::error::AppError;
use std::convert::Infallible;
use storage::file_storage::FileStorage;
use warp::{http::StatusCode, Filter, Reply};

#[tokio::main]
async fn main() {
    // 初始化日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 记录启动开始时间
    let start_time = std::time::Instant::now();

    // 初始化环境变量
    dotenv::dotenv().ok();

    // 输出环境变量
    log::info!("=== Environment Variables ===");
    log::info!("STORAGE_PATH: {}", std::env::var("STORAGE_PATH").unwrap());
    log::info!("STATIC_PATH: {}", std::env::var("STATIC_PATH").unwrap());
    log::info!(
        "PORT: {}",
        std::env::var("PORT").unwrap_or_else(|_| "3030".to_string())
    );

    // 初始化数据目录和文件
    let data_dir = "data";
    std::fs::create_dir_all(format!("{}/house", data_dir))
        .expect("Failed to create house directory");
    std::fs::create_dir_all(format!("{}/images", data_dir))
        .expect("Failed to create images directory");

    for file in &["user.json", "house.json"] {
        let path = format!("{}/{}", data_dir, file);
        if !std::path::Path::new(&path).exists() {
            std::fs::write(&path, "[]").expect(&format!("Failed to initialize {}", file));
        }
    }

    // 初始化存储服务
    let storage_path = std::env::var("STORAGE_PATH").expect("STORAGE_PATH must be set");
    let static_path = std::env::var("STATIC_PATH").expect("STATIC_PATH must be set");
    let file_storage = FileStorage::new(&storage_path);

    // 初始化路由
    let routes = routes::combine_routes(file_storage, static_path).recover(handle_rejection);

    // 启动服务
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse::<u16>()
        .expect("Invalid PORT number");

    log::info!("Server will listen on port: {}", port);

    // 启动服务并记录耗时
    let server = warp::serve(routes);
    log::info!("Starting server...");
    server.run(([0, 0, 0, 0], port)).await;
    log::info!("Server started in {:?}", start_time.elapsed());
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if let Some(e) = err.find::<AppError>() {
        match e {
            AppError::AuthenticationRequired => (StatusCode::UNAUTHORIZED, e.to_string()),
            AppError::PermissionDenied => (StatusCode::FORBIDDEN, e.to_string()),
            AppError::InvalidVersion => (StatusCode::CONFLICT, e.to_string()),
            AppError::FileSystemError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::ParseError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound => (StatusCode::NOT_FOUND, e.to_string()),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, e.to_string()),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
