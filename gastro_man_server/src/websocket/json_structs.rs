use serde::{Serialize, Deserialize};
use crate::users::User;


#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub user: String,
    pub password: String,
}


/// Payload contains either session-id or the cause of error
// TODO: find cleaner way to manage Responses
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub success: bool,
    pub payload: String,
}

/// The part of user-struct the client gets
#[derive(Serialize, Deserialize, Debug)]
pub struct WebUser {
    pub first_name: String,
    pub sec_name: String,
    pub user_name: String
}

impl From<User> for WebUser {
    fn from(item: User) -> Self {
        WebUser {
            first_name: item.first_name,
            sec_name: item.sec_name,
            user_name: item.user_name
        }
    }
}