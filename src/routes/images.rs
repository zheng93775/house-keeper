use crate::storage::file_storage::FileStorage;
use std::convert::Infallible;
use warp::{Filter, Rejection};

pub fn image_routes(
    storage: FileStorage,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    // 图片上传路由
    let upload_image = warp::path!("images")
        .and(warp::post())
        .and(warp::body::bytes())
        .and(with_storage(storage.clone()))
        .and_then(upload_image_handler);

    upload_image
}

async fn upload_image_handler(
    bytes: bytes::Bytes,
    storage: FileStorage,
) -> Result<impl warp::Reply, Rejection> {
    // TODO: 实现图片上传逻辑
    Ok(warp::reply::json(&"Not implemented"))
}

fn with_storage(
    storage: FileStorage,
) -> impl Filter<Extract = (FileStorage,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}
