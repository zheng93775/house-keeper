use crate::{
    models::{error::AppError, house::House},
    storage::file_storage::FileStorage,
};
use serde_json::json;
use std::convert::Infallible;
use uuid;
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
    new_house: House,
    storage: FileStorage,
) -> Result<impl warp::Reply, Rejection> {
    // 生成唯一ID
    let house_id = uuid::Uuid::new_v4().to_string();

    // 读取现有房屋数据并处理错误
    let mut houses: Vec<House> = storage
        .read_json("house.json")
        .map_err(|e| warp::reject::custom(AppError::from(e)))?;

    // 创建带ID的新房屋对象
    let mut house_with_id = new_house;
    house_with_id.id = house_id.clone();

    // 添加到列表并保存
    houses.push(house_with_id);
    storage
        .write_json("house.json", &houses)
        .map_err(|e| warp::reject::custom(AppError::from(e)))?;

    // 返回创建成功的响应
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({
            "id": house_id,
            "message": "House created successfully"
        })),
        warp::http::StatusCode::CREATED,
    ))
}

fn with_storage(
    storage: FileStorage,
) -> impl Filter<Extract = (FileStorage,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}
