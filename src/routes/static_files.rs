use std::path::PathBuf;
use warp::Filter;

pub fn static_files_routes(
    static_path: String,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let static_dir = PathBuf::from(static_path);
    warp::fs::dir(static_dir)
}
