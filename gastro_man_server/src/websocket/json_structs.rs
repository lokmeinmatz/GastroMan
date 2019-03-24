use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub user: String,
    pub password: String,
}