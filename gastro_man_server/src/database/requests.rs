use bounded_spsc_queue::{Producer};
use crate::users::User;

#[derive(Debug)]
pub enum RequestType {
  UserGetRequest(String)
}



pub enum ResponseType {
  UserGetResponse(Option<User>)
}

pub struct DatabaseRequest {
  pub req: RequestType,
  pub answer: Producer<ResponseType>
}

impl DatabaseRequest {
  pub fn new(req: RequestType, answer: Producer<ResponseType>) -> Self {
    DatabaseRequest {req, answer}
  }
}

impl std::fmt::Debug for DatabaseRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "DatabaseRequest {{req: {:?}}}", &self.req)
  }
}