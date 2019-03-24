use sqlite::Statement;
use crate::database::sessions;

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
      session_id,
      pw_hash,
      user_name,
      first_name,
      sec_name
    }
  }

  pub fn from_db(statement: Statement, session_map: &mut std::collections::HashMap<usize, String>) -> Result<User, sqlite::Error> {
    let id = statement.read::<i64>(0)? as usize;
    let first_name = statement.read::<String>(1)?;
    let sec_name = statement.read::<String>(2)?;
    let user_name = statement.read::<String>(3)?;
    let pw_hash = statement.read::<String>(4)?;

    // get session id
    let session_id = match session_map.get(&id) {
      Some(ref_sid) => Some(ref_sid.clone()),
      None => {
        // create hashed session id and store it together with user (for privelege test)
        let sid = sessions::generate_session_id(&user_name);
        session_map.insert(id, sid.clone());
        Some(sid)
      }
    };

    Ok( User {id, first_name, session_id, sec_name, user_name, pw_hash})
  }
}