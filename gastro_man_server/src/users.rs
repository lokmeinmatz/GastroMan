use sqlite::Statement;
use crate::database::sessions;
use crate::websocket::json_structs;

#[derive(Debug)]
pub struct User {
  pub id: Option<usize>,
  pub session_id: Option<String>,
  pub first_name : String, 
  pub last_name : String,
  pub user_name: String,
  pub pw_hash: Option<String>,
  pub permissions: UserPermissionFlags
}

#[derive(Debug)]
pub struct UserPermissionFlags (pub i64);

impl UserPermissionFlags {
  pub const ADMIN: UserPermissionFlags = UserPermissionFlags(0b001000);

  pub fn as_vec(&self) -> Vec<String> {
    let mut elmts = Vec::new();

    if self.0 & 0b000001 != 0 {elmts.push("waiter");}
    if self.0 & 0b000010 != 0 {elmts.push("cook");}
    if self.0 & 0b000100 != 0 {elmts.push("manager");}
    if self.0 & 0b001000 != 0 {elmts.push("admin");}

    elmts.iter().map(|e| String::from(*e)).collect()
  }

  pub fn is_admin(&self) -> bool {
    self.0 & 0b001000 != 0
  }
}

impl From<Vec<String>> for UserPermissionFlags {
  fn from(vec: Vec<String>) -> Self {
    let mut res = 0i64;
    for el in &vec {
      match el.as_str() {
        "waiter"  => res |= 0b000001,
        "cook"    => res |= 0b000010,
        "manager" => res |= 0b000100,
        "admin"   => res |= 0b001000,
        _ => {}
      }
    }


    UserPermissionFlags(res)
  }
}

impl User {
  pub fn new(id: usize, session_id: String, user_name : String, first_name : String, last_name : String, pw_hash: String, perm_flags: Option<UserPermissionFlags>) -> User {
    User {
      id: Some(id),
      session_id: Some(session_id),
      pw_hash: Some(pw_hash),
      user_name,
      first_name,
      last_name,
      permissions: perm_flags.unwrap_or(UserPermissionFlags(0b0001i64))
    }
  }

  pub fn from_web(user_name: String, first_name: String, last_name: String, perm_flags: Option<UserPermissionFlags>) -> Self {
    User {
      id: None,
      session_id: None,
      pw_hash: None,
      user_name,
      first_name,
      last_name,
      permissions: perm_flags.unwrap_or(UserPermissionFlags(0b0001i64))
    }
  }

  /// Does construct a new User from Database statement, but doesn't delete sessions!!
  pub fn from_db(statement: &Statement, session_map: &std::collections::HashMap<String, usize>) -> Result<User, sqlite::Error> {
    let id = statement.read::<i64>(0)? as usize;
    let first_name = statement.read::<String>(1)?;
    let last_name = statement.read::<String>(2)?;
    let user_name = statement.read::<String>(3)?;
    let pw_hash = statement.read::<String>(4)?;
    let perm_flags = statement.read::<i64>(5)?;

    // get session id
    let session_id : Option<String> = match session_map.iter().find(|(k, v)| *v == &id) {
      Some(ref_sid) => Some(ref_sid.0.clone()),
      None => {
        // create hashed session id and store it together with user (for privelege test)
        let sid = sessions::generate_session_id(&user_name);
        Some(sid)
      }
    };

    Ok( User {id : Some(id), first_name, session_id, last_name, user_name, pw_hash: Some(pw_hash), permissions: UserPermissionFlags(perm_flags)})
  }
}

impl From<json_structs::WebUser> for User {
  fn from(wu : json_structs::WebUser) -> Self {
    User {
      id: None,
      session_id: None,
      pw_hash: None,
      user_name: wu.user_name,
      permissions: wu.permissions.into(),
      first_name: wu.first_name,
      last_name: wu.last_name
    }
  }
}