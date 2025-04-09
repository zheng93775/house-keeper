use crate::{models::error::AppError, storage::backup_manager::BackupManager};
use std::env;
use std::fs;
use std::path::Path;
use warp::{Filter, Rejection, Reply};

// 定义备份路由
pub fn backup_routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("backup")
        .and(warp::post())
        .and_then(backup_handler)
}

// 备份处理函数
async fn backup_handler() -> Result<impl Reply, Rejection> {
    // 获取备份目录
    let backup_path = env::var("HOUSE_KEEPER_BACKUP_PATH")
        .map_err(|e| warp::reject::custom(AppError::BackupError))?;
    // 获取备份目录
    let storage_base_path = env::var("HOUSE_KEEPER_STORAGE_PATH")
        .map_err(|e| warp::reject::custom(AppError::BackupError))?;

    let backup_manager = BackupManager::new(&backup_path, &storage_base_path);

    // 备份 user.json
    backup_file(&backup_manager, "user.json").await?;

    // 备份 house.json
    backup_file(&backup_manager, "house.json").await?;

    // 备份 house/{house-id}.json
    let house_dir = Path::new(&storage_base_path).join("house");
    if house_dir.exists() && house_dir.is_dir() {
        for entry in fs::read_dir(house_dir).map_err(|e| warp::reject::custom(AppError::from(e)))? {
            let entry = entry.map_err(|e| warp::reject::custom(AppError::from(e)))?;
            let path = entry.path();
            // 修改此处传参
            if path.is_file() && path.extension().map(|s| s == "json").unwrap_or(false) {
                let relative_path = Path::new("house").join(path.file_name().unwrap());
                backup_file(&backup_manager, relative_path.to_str().unwrap()).await?;
            }
        }
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({
            "message": "Backup completed successfully"
        })),
        warp::http::StatusCode::OK,
    ))
}

// 备份单个文件的函数
async fn backup_file(backup_manager: &BackupManager, file_path: &str) -> Result<(), Rejection> {
    backup_manager
        .create_backup(file_path)
        .map_err(|e| warp::reject::custom(AppError::from(e)))?;
    Ok(())
}
