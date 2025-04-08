use crate::{
    models::{
        error::AppError,
        house::{CreateHouseForm, House, HouseDetail, HouseMember, SetHouseMembersForm},
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

    let delete_house = warp::path!("houses" / String)
        .and(warp::delete())
        .and(auth_filter(storage.clone()))
        .and(with_storage(storage.clone()))
        .and_then(delete_house_handler);

    let set_house_members = warp::path!("houses" / String / "members")
        .and(warp::put())
        .and(warp::body::json())
        .and(auth_filter(storage.clone()))
        .and(with_storage(storage.clone()))
        .and_then(set_house_members_handler);

    let get_house_detail = warp::path!("houses" / String / "detail")
        .and(warp::get())
        .and(auth_filter(storage.clone()))
        .and(with_storage(storage.clone()))
        .and_then(get_house_detail_handler);

    // 新增修改房屋详细数据的路由
    let update_house_detail = warp::path!("houses" / String / "detail")
        .and(warp::put())
        .and(warp::body::json())
        .and(auth_filter(storage.clone()))
        .and(with_storage(storage.clone()))
        .and_then(update_house_detail_handler);

    create_house
        .or(get_my_houses)
        .or(delete_house)
        .or(set_house_members)
        .or(get_house_detail)
        .or(update_house_detail)
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
        name: create_house_form.name.clone(),
        items: Vec::new(),
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

    Ok(warp::reply::json(&serde_json::json!({
        "user_id": user.id,
        "houses": &my_houses
    })))
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

async fn set_house_members_handler(
    house_id: String,
    form: SetHouseMembersForm,
    user: User,
    storage: FileStorage,
) -> Result<impl warp::Reply, Rejection> {
    // 读取房屋数据
    let mut houses: Vec<House> = storage
        .read_json("house.json")
        .map_err(|e| warp::reject::custom(AppError::from(e)))?;

    // 查找要修改的房屋
    if let Some(house) = houses.iter_mut().find(|h| h.id == house_id) {
        // 校验房屋创建人是否为当前用户
        if house.creator != user.id {
            return Err(warp::reject::custom(AppError::PermissionDenied));
        }

        // 读取用户数据
        let users: Vec<User> = storage
            .read_json("user.json")
            .map_err(|e| warp::reject::custom(AppError::from(e)))?;

        // 根据用户名列表找到相应的用户
        let members = form
            .usernames
            .iter()
            .filter_map(|username| {
                users.iter().find(|u| u.username == *username).map(|u| {
                    // Create a HouseMember instance here.
                    // Assume HouseMember has fields id and username.
                    HouseMember {
                        user_id: u.id.clone(),
                        username: u.username.clone(),
                    }
                })
            })
            .collect::<Vec<HouseMember>>();

        // 修改房屋的 members 字段
        house.members = members;

        // 保存修改后的房屋数据
        storage
            .write_json("house.json", &houses)
            .map_err(|e| warp::reject::custom(AppError::from(e)))?;

        // 返回修改成功的响应
        return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({
                "message": "House members updated successfully"
            })),
            warp::http::StatusCode::OK,
        ));
    }

    // 若未找到房屋，返回错误
    Err(warp::reject::custom(AppError::HouseNotFound))
}

async fn get_house_detail_handler(
    house_id: String,
    user: User,
    storage: FileStorage,
) -> Result<impl warp::Reply, Rejection> {
    // 读取房屋数据
    let houses: Vec<House> = storage
        .read_json("house.json")
        .map_err(|e| warp::reject::custom(AppError::from(e)))?;

    // 查找要查询的房屋
    if let Some(house) = houses.iter().find(|h| h.id == house_id) {
        // 校验当前用户是否为房屋的创建人或成员
        if house.creator != user.id && !house.members.iter().any(|member| member.user_id == user.id)
        {
            return Err(warp::reject::custom(AppError::PermissionDenied));
        }

        // 读取房屋详细数据
        let house_detail: HouseDetail = storage
            .read_json(&format!("house/{}.json", house_id))
            .map_err(|e| warp::reject::custom(AppError::from(e)))?;

        // 返回房屋详细数据
        return Ok(warp::reply::json(&house_detail));
    }

    // 若未找到房屋，返回错误
    Err(warp::reject::custom(AppError::HouseNotFound))
}

async fn update_house_detail_handler(
    house_id: String,
    new_house_detail: HouseDetail,
    user: User,
    storage: FileStorage,
) -> Result<impl warp::Reply, Rejection> {
    // 读取房屋数据
    let houses: Vec<House> = storage
        .read_json("house.json")
        .map_err(|e| warp::reject::custom(AppError::from(e)))?;

    // 查找要修改的房屋
    if let Some(house) = houses.iter().find(|h| h.id == house_id) {
        // 校验当前用户是否为房屋的创建人或成员
        if house.creator != user.id && !house.members.iter().any(|member| member.user_id == user.id)
        {
            return Err(warp::reject::custom(AppError::PermissionDenied));
        }

        // 读取房屋详细数据
        let mut current_house_detail: HouseDetail = storage
            .read_json(&format!("house/{}.json", house_id))
            .map_err(|e| warp::reject::custom(AppError::from(e)))?;

        // 检查版本号是否匹配
        if current_house_detail.version != new_house_detail.version {
            return Err(warp::reject::custom(AppError::VersionMismatch));
        }

        // 生成新的版本号
        let new_version = uuid::Uuid::new_v4().to_string();
        current_house_detail.version = new_version.clone();
        current_house_detail.items = new_house_detail.items;

        // 写入新的房屋详细数据
        storage
            .write_json(&format!("house/{}.json", house_id), &current_house_detail)
            .map_err(|e| warp::reject::custom(AppError::from(e)))?;

        // 返回新的版本号
        return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({
                "version": new_version
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
