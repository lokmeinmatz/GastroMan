use sqlite::Statement;


#[derive(Debug)]
pub struct User {
  pub id: usize,
  pub session_id: Option<String>,
  pub first_name : String, 
  pub sec_name : String,
  pub user_name: String,
  pub pw_hash: String,
}

impl User {
  pub fn new(id: usize, session_id: Option<String>, user_name : String, first_name : String, sec_name : String, pw_hash: String) -> User {
    User {
      id,
      session_id: Option<String>,
      pw_hash,
      user_name,
      first_name,
      sec_name
    }
  }

  pub fn from_db(statement: Statement) -> Result<User, sqlite::Error> {
    let id = statement.read::<i64>(0)? as usize;
    let first_name = statement.read::<String>(1)?;
    let sec_name = statement.read::<String>(2)?;
    let user_name = statement.read::<String>(3)?;
    let pw_hash = statement.read::<String>(4)?;

    Ok( User {id, first_name, sec_name, user_name, pw_hash})
  }
}