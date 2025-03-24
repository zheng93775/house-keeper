use crate::storage::file_storage::FileStorage;
use warp::Filter;

pub fn combine_routes(
    file_storage: FileStorage,
    static_path: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // 添加 /api 前缀
    let api_prefix = warp::path("api");

    let auth = auth_routes(file_storage.clone());
    let houses = houses_routes(file_storage.clone());
    let static_files = static_files_routes(static_path);

    // 组合所有路由并添加前缀
    api_prefix.and(auth.or(houses).or(static_files))
}

pub mod auth;
pub mod houses;
pub mod static_files;

use self::auth::auth_routes;
use self::houses::houses_routes;
use self::static_files::static_files_routes;
