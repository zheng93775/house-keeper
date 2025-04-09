use crate::storage::file_storage::FileStorage;
use warp::Filter;

// 声明 images 模块
pub mod images;

pub fn combine_routes(
    file_storage: FileStorage,
    static_path: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let api_prefix = warp::path("api");

    let auth = auth_routes(file_storage.clone());
    let houses = houses_routes(file_storage.clone());
    let images = image_routes(file_storage.clone());
    let backup = backup::backup_routes();

    let api_routes = api_prefix.and(auth.or(houses).or(images).or(backup));

    api_routes.or(static_files_routes(static_path))
}

pub mod auth;
pub mod backup;
pub mod houses;
pub mod static_files;

use self::auth::auth_routes;
use self::houses::houses_routes;
use self::images::image_routes;
use self::static_files::static_files_routes;
