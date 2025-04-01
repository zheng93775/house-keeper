use crate::models::user::User;
use crate::storage::file_storage::FileStorage;
use warp::Filter;

pub fn combine_routes(
    file_storage: FileStorage,
    static_path: String,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let api_prefix = warp::path("api");

    let auth = auth_routes(file_storage.clone());
    let houses = houses_routes(file_storage.clone());

    let api_routes = api_prefix.and(auth.or(houses));

    api_routes.or(static_files_routes(static_path))
}

pub mod auth;
pub mod houses;
pub mod static_files;

use self::auth::auth_filter;
use self::auth::auth_routes;
use self::houses::houses_routes;
use self::static_files::static_files_routes;
