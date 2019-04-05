use bounded_spsc_queue::{Producer};

use crate::users::{User, UserPermissionFlags};


pub enum DBRequest {
  UserGetRequest(String, Producer<UserGetResponse>),
  PermissionsGetRequest(String, Producer<PermissionsGetResponse>),
  DeleteSessionRequest(String), // returns Nothing
  AdminUserListRequest(Producer<AdminUserListResponse>),
}

  
pub type UserGetResponse = Option<User>;
pub type AdminUserListResponse = Vec<User>;
pub type PermissionsGetResponse = Option<UserPermissionFlags>;


impl std::fmt::Debug for DBRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      DBRequest::UserGetRequest(req, _) => write!(f, "UserGetRequest {{req: {:?}}}", &req),
      DBRequest::DeleteSessionRequest(req) => write!(f, "DeleteSessionRequest {{req: {:?}}}", &req),
      DBRequest::AdminUserListRequest(_) => write!(f, "AdminUserListRequest {{}}"),
      DBRequest::PermissionsGetRequest(req, _) => write!(f, "DeleteSessionRequest {{req: {:?}}}", &req),
    }
  }
}


