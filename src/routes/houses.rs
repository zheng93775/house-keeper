use crate::{
    models::{
        house::{House},
    },
    storage::file_storage::FileStorage,
};
use std::convert::Infallible;
use warp::{Filter, Rejection};

pub fn houses_routes(
    storage: FileStorage,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    warp::path!("houses")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_storage(storage.clone()))
        .and_then(create_house_handler)
}

async fn create_house_handler(
    _new_house: House,
    _storage: FileStorage,
) -> Result<impl warp::Reply, Rejection> {
    // TODO: 实现创建房屋逻辑
    Ok(warp::reply::json(&"Not implemented"))
}

fn with_storage(
    storage: FileStorage,
) -> impl Filter<Extract = (FileStorage,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}
