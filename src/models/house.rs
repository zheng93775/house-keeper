use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct House {
    pub id: String,
    pub name: String,
    pub creator: String,
    pub members: Vec<HouseMember>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateHouseForm {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HouseMember {
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetHouseMembersForm {
    pub usernames: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HouseDetail {
    pub version: String,
    pub name: String,
    pub items: Vec<HouseArea>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HouseArea {
    pub id: String,
    pub name: String,
    pub content: String,
    pub images: Vec<String>,
    pub items: Vec<HouseArea>,
}

impl House {
    pub fn new(name: &str, creator: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            creator: creator.to_string(),
            members: Vec::new(),
        }
    }
}

impl HouseDetail {
    pub fn new() -> Self {
        Self {
            version: Uuid::new_v4().to_string(),
            name: String::new(),
            items: Vec::new(),
        }
    }
}
