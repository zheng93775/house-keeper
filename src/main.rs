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
    log::info!(
        "HOUSE_KEEPER_STORAGE_PATH: {}",
        std::env::var("HOUSE_KEEPER_STORAGE_PATH").unwrap()
    );
    log::info!(
        "HOUSE_KEEPER_STATIC_PATH: {}",
        std::env::var("HOUSE_KEEPER_STATIC_PATH").unwrap()
    );
    log::info!(
        "HOUSE_KEEPER_PORT: {}",
        std::env::var("HOUSE_KEEPER_PORT").unwrap_or_else(|_| "3030".to_string())
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
    let storage_path =
        std::env::var("HOUSE_KEEPER_STORAGE_PATH").expect("HOUSE_KEEPER_STORAGE_PATH must be set");
    let static_path =
        std::env::var("HOUSE_KEEPER_STATIC_PATH").expect("HOUSE_KEEPER_STATIC_PATH must be set");
    let file_storage: FileStorage = FileStorage::new(&storage_path);

    // 初始化路由
    let routes = routes::combine_routes(file_storage, static_path).recover(handle_rejection);

    // 启动服务
    let port = std::env::var("HOUSE_KEEPER_PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse::<u16>()
        .expect("Invalid HOUSE_KEEPER_PORT number");

    log::info!("Server will listen on port: {}", port);

    // 启动服务并记录耗时
    let server = warp::serve(routes);
    log::info!("Starting server...");
    server.run(([0, 0, 0, 0], port)).await;
    log::info!("Server started in {:?}", start_time.elapsed());
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, Infallible> {
    if let Some(app_err) = err.find::<AppError>() {
        let json = warp::reply::json(&serde_json::json!({
            "error": app_err.to_string()
        }));
        return Ok(warp::reply::with_status(json, StatusCode::UNAUTHORIZED));
    }
    let json = warp::reply::json(&serde_json::json!({
        "error": "Internal server error"
    }));
    Ok(warp::reply::with_status(
        json,
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
