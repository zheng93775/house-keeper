use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub token: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            password: password.to_string(),
            token: Uuid::new_v4().to_string(),
        }
    }
}
