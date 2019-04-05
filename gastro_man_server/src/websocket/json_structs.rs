use serde::{Serialize, Deserialize};
use crate::users::User;


#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub user: String,
    pub password: Option<String>,
    pub sid: Option<String>
}

/// The part of user-struct the client gets
#[derive(Serialize, Deserialize, Debug)]
pub struct WebUser {
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub permissions: Vec<String>,
    password: Option<String> // for user.add request
}

impl From<User> for WebUser {
    fn from(item: User) -> Self {
        WebUser {
            first_name: item.first_name,
            last_name: item.last_name,
            user_name: item.user_name,
            permissions: item.permissions.as_vec(),
            password: None
        }
    }
}