use crate::{
    models::{
        error::AppError,
        house::{CreateHouseForm, House, HouseDetail},
        user::User,
    },
    routes::auth::auth_filter,
    storage::file_storage::FileStorage,
};
use log::info;
use std::convert::Infallible;
use uuid;
use warp::{Filter, Rejection};

pub fn houses_routes(
    storage: FileStorage,
) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let create_house = warp::path!("houses")
        .and(warp::post())
        .and(warp::body::json())
        .and(auth_filter(storage.clone()))
        .and(with_storage(storage.clone()))
        .and_then(create_house_handler);

    let get_my_houses = warp::path!("houses")
        .and(warp::get())
        .and(auth_filter(storage.clone()))
        .and(with_storage(storage.clone()))
        .and_then(get_my_houses_handler);

    // 新增删除房屋的路由
    let delete_house = warp::path!("houses" / String)
        .and(warp::delete())
        .and(auth_filter(storage.clone()))
        .and(with_storage(storage.clone()))
        .and_then(delete_house_handler);

    create_house.or(get_my_houses).or(delete_house)
}

async fn create_house_handler(
    create_house_form: CreateHouseForm,
    user: User,
    storage: FileStorage,
) -> Result<impl warp::Reply, Rejection> {
    // 生成唯一ID
    let house_id = uuid::Uuid::new_v4().to_string();

    // 读取现有房屋数据并处理错误
    let mut houses: Vec<House> = storage
        .read_json("house.json")
        .map_err(|e| warp::reject::custom(AppError::from(e)))?;

    let new_house: House = House {
        id: house_id.clone(),
        name: create_house_form.name.clone(),
        creator: user.id,
        members: Vec::new(),
    };

    // 添加到列表并保存
    houses.push(new_house);
    storage
        .write_json("house.json", &houses)
        .map_err(|e| warp::reject::custom(AppError::from(e)))?;

    // 创建 house/{house-id}.json 空数据文件
    let empty_house_detail = HouseDetail {
        version: uuid::Uuid::new_v4().to_string(),
        data: Vec::new(),
    };
    storage
        .write_json(&format!("house/{}.json", house_id), &empty_house_detail)
        .map_err(|e| warp::reject::custom(AppError::from(e)))?;

    // 返回创建成功的响应
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({
            "id": house_id.clone(),
            "message": "House created successfully"
        })),
        warp::http::StatusCode::CREATED,
    ))
}

async fn get_my_houses_handler(
    user: User,
    storage: FileStorage,
) -> Result<impl warp::Reply, Rejection> {
    // 记录进入方法时的传参
    info!("Entering get_my_houses_handler user: {:?}", user);

    // 读取房屋数据
    let houses: Vec<House> = storage
        .read_json("house.json")
        .map_err(|e| warp::reject::custom(AppError::from(e)))?;

    // 过滤属于当前用户的房屋
    let my_houses = houses
        .into_iter()
        .filter(|house| {
            house.creator == user.id || house.members.iter().any(|member| member.user_id == user.id)
        })
        .collect::<Vec<_>>();

    Ok(warp::reply::json(&my_houses))
}

async fn delete_house_handler(
    house_id: String,
    user: User,
    storage: FileStorage,
) -> Result<impl warp::Reply, Rejection> {
    // 读取房屋数据
    let mut houses: Vec<House> = storage
        .read_json("house.json")
        .map_err(|e| warp::reject::custom(AppError::from(e)))?;

    // 查找要删除的房屋
    if let Some(index) = houses.iter().position(|house| house.id == house_id) {
        let house = &houses[index];
        // 校验房屋创建人是否为当前用户
        if house.creator != user.id {
            return Err(warp::reject::custom(AppError::PermissionDenied));
        }

        // 移除房屋记录
        houses.remove(index);
        storage
            .write_json("house.json", &houses)
            .map_err(|e| warp::reject::custom(AppError::from(e)))?;

        // 删除 house/{house-id}.json 文件
        if let Err(e) = storage.delete_file(&format!("house/{}.json", house_id)) {
            return Err(warp::reject::custom(AppError::from(e)));
        }

        // 返回删除成功的响应
        return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({
                "message": "House deleted successfully"
            })),
            warp::http::StatusCode::OK,
        ));
    }

    // 若未找到房屋，返回错误
    Err(warp::reject::custom(AppError::HouseNotFound))
}

fn with_storage(
    storage: FileStorage,
) -> impl Filter<Extract = (FileStorage,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}

fn with_user(user: User) -> impl Filter<Extract = (User,), Error = Infallible> + Clone {
    warp::any().map(move || user.clone())
}
