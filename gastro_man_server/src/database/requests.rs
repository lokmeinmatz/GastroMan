use bounded_spsc_queue::{Producer};


#[derive(Debug)]
pub enum RequestType {
  UserExists(String, String)
}

pub enum ResponseType {
  UserExists(bool)
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