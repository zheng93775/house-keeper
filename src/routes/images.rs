use crate::{
    models::error::AppError, models::user::User, routes::auth::auth_filter,
    storage::file_storage::FileStorage,
};
use bytes::Buf;
use futures::StreamExt;
use std::convert::Infallible;
use std::ffi::OsStr;
use std::path::Path;
use uuid::Uuid;
use warp::{multipart::FormData, Filter, Rejection}; // 引入 Buf 特征

pub fn image_routes(
    storage: FileStorage,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    // 图片上传路由，支持 multipart 文件上传
    let upload_image = warp::path!("images")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5 * 1024 * 1024))
        .and(auth_filter(storage.clone()))
        .and(with_storage(storage.clone()))
        .and_then(upload_image_handler);

    upload_image
}

async fn upload_image_handler(
    mut form: FormData,
    user: User, // 校验登录用户
    storage: FileStorage,
) -> Result<impl warp::Reply, Rejection> {
    // 从表单数据中获取文件
    if let Some(field) = form.next().await {
        let field =
            field.map_err(|e| warp::reject::custom(AppError::FileSystemError(e.to_string())))?;
        if field.name() == "image" {
            // 获取文件名
            let filename = field
                .filename()
                .ok_or_else(|| warp::reject::custom(AppError::ParameterError))?;
            // 解析文件后缀名
            let ext = Path::new(filename)
                .extension()
                .and_then(OsStr::to_str)
                .unwrap_or("");
            // 生成随机文件名
            let random_filename = format!("{}.{}", Uuid::new_v4(), ext);
            let path = format!("images/{}", random_filename);

            // 读取文件内容
            let mut bytes = Vec::new();
            let mut stream = field.stream();
            while let Some(chunk) = stream.next().await {
                let mut chunk = chunk
                    .map_err(|e| warp::reject::custom(AppError::FileSystemError(e.to_string())))?;
                // 修改为使用 copy_to_bytes 方法获取字节切片
                let chunk_slice = chunk.copy_to_bytes(chunk.remaining());
                bytes.extend_from_slice(&chunk_slice);
            }

            // 写入文件
            storage
                .write_file(&path, &bytes)
                .map_err(|e| warp::reject::custom(e))?;

            // 返回文件名给前端
            return Ok(warp::reply::json(&serde_json::json!({
                "file_name": &random_filename,
            })));
        }
    }

    Err(warp::reject::custom(AppError::ParameterError))
}

fn with_storage(
    storage: FileStorage,
) -> impl Filter<Extract = (FileStorage,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}
