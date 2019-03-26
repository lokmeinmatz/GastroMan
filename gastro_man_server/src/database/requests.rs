use bounded_spsc_queue::{Producer};
use crate::users::User;

pub enum DBRequest {
  UserGetRequest(String, Producer<ResponseType>),
  DeleteSessionRequest(String) // returns Nothing
}


impl std::fmt::Debug for DBRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      DBRequest::UserGetRequest(req, _) => write!(f, "UserGetRequest {{req: {:?}}}", &req),
      DBRequest::DeleteSessionRequest(req) => write!(f, "DeleteSessionRequest {{req: {:?}}}", &req)
    }
  }
}


pub enum ResponseType {
  UserGetResponse(Option<User>)
}


