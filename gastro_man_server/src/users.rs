pub struct User {
  pub pw_hash: String,
  pub username: String,
  pub first_name : String, 
  pub sec_name : String,
}

impl User {
  pub fn new(username : String, first_name : String, sec_name : String, pw_hash: String) -> User {
    User {
      pw_hash,
      username,
      first_name,
      sec_name
    }
  }
}