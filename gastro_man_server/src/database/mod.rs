use colored::*;
use sha3::{Digest, Sha3_256};

use std::collections::HashSet;
use std::sync::mpsc;
use std::collections::HashMap;
use crate::users::{User, UserPermissionFlags};
use bounded_spsc_queue::{Producer};

pub mod requests;
use requests::{DBRequest};

pub mod sessions;

#[derive(Debug)]
pub enum UserError {
  AllreadyExists,
  DBError,
  CannotAdd,
}



pub struct DBManager {
  con: sqlite::Connection,
  db_query: (mpsc::Sender<DBRequest>, mpsc::Receiver<DBRequest>),
  sessions: HashMap<usize, String>
}

impl DBManager {
  pub fn new() -> Result<DBManager, sqlite::Error> {
    let sq = sqlite::open("current_data.sqlite")?;

    let query_cannel = mpsc::channel();
   

    
    let db = DBManager { con: sq, db_query: query_cannel, sessions: HashMap::new() };

    let missing_tables = db.get_missing_required_tables();
    let usr_count = db.get_user_count();


    if missing_tables.len() > 0 {
        println!("\ndbm > The following tables are missing in current_data.sqlite: ");
        for missing_table in &missing_tables {
            println!(" > {}", missing_table.red());
        }
    }

    if missing_tables.contains(&"users") || usr_count <= 0 {
        println!("dbm > This program needs users to function and to get setup!");
        println!("\ndbm > You will receive a default login to configure the system.");
        println!(
            "dbm > >>{}<<\n",
            " Change this immidiately as it is a risk of security "
                .red()
                .reversed()
        );

        let username = String::from("admin");
        let password: String = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis().to_string();


        let password = &password[(password.len()-6)..];
        println!("\ndbm > Username: {}", username);
        println!("\ndbm > Password: {}", password);
        let mut h = Sha3_256::new();
        h.input(password);

        let usr = User::new(
          0, 
          String::new(), 
          username, 
          String::from("admin"), 
          String::from("account"), 
          format!("{:x}", h.result()),
          Some(UserPermissionFlags::ADMIN));

        match db.add_user(usr) {
            Ok(_) => println!("dbm > You can now log in with this credentials."),
            Err(e) => eprintln!("dbm > Error while adding User: {:?}", e)
        }
    }

    Ok(db)
  }

  pub fn get_query_sender(&mut self) -> mpsc::Sender<DBRequest> {
    self.db_query.0.clone()
  }

  pub fn run(&mut self) {
    println!("dbm > database-manager running");
    loop {
      match self.db_query.1.recv() {
        Ok(req) => {
          dbg!(&req);
          match req {
            DBRequest::UserGetRequest(user_name, answer) => self.handle_user_get(user_name, answer),
            DBRequest::DeleteSessionRequest(sid) => self.delete_session(sid),
            DBRequest::AdminUserListRequest(answer) => self.handle_userlist(answer)
          }
        },
        Err(e) => eprintln!("dbm > Error while getting Request, {:?}", e)
      }

      std::thread::yield_now();
    }
  }

  fn handle_user_get(&mut self, user: String, answer: Producer<Option<User>>) {
    let mut stmt = self.con.prepare(format!("SELECT * FROM users WHERE user_name = '{}'", user)).expect("dbm > Cant create User exists query");
    if let sqlite::State::Row = stmt.next().unwrap() {
      match User::from_db(&stmt, &self.sessions) {
        Ok(usr) => {
          self.sessions.insert(usr.id.expect("User::from_db did not set the id"), usr.session_id.clone().expect("User::from_db did not set a session_id"));
          answer.push(Some(usr));
        },
        Err(_) => {answer.push(None)} // no user found
      }
    }
    else {
      // no use matching
      // TODO: implement websocket response if no user matches
    }
  }

  fn handle_userlist(&mut self, answer: Producer<requests::AdminUserListResponse>) {
    let mut users : Vec<User> = Vec::new();
    
    let mut stmt = self.con.prepare("SELECT * FROM users").expect("dbm > Cant create Userlist query");

    while let sqlite::State::Row = stmt.next().unwrap() {
      match User::from_db(&stmt, &self.sessions) {
        Ok(usr) => {users.push(usr)},
        Err(e) => eprintln!("{:?}", e)
      }
  }

    answer.push(users);
  }

  fn delete_session(&mut self, sid: String) {
    let key = match self.sessions.iter().find_map(|(k, v)| {if v == &sid {Some(k)} else {None}}) {
      Some(key) => *key,
      None => return
    };
    self.sessions.remove(&key);
  }


  pub fn get_missing_required_tables(&self) -> Vec<&str> {
    // check if db has necessary tables
    let mut required_tables: HashSet<&str> = HashSet::new();

    required_tables.insert("users");
    //required_tables.insert("meals");

    self
      .con
      .iterate(
        "SELECT * FROM sqlite_master WHERE type = \"table\"",
        |row| {
          let tab_name = row[1].1.unwrap();
          required_tables.remove(tab_name);
          true
        },
      )
      .expect("Error while reading db");

    required_tables.into_iter().collect()
  }

  pub fn get_user_count(&self) -> i64 {
    let mut stmt = self.con.prepare("SELECT COUNT(*) FROM users").expect("dbm > can't count users :(");

    // go to the first and only result
    stmt.next().unwrap();

    stmt.read(0).expect("Error while reading COUNT of users")
  }

  pub fn add_user(&self, usr : User) -> Result<(), UserError> {
    // check if user allready exists
    let mut qry_usr = self.con.prepare(format!("SELECT * FROM users WHERE user_name = '{}'", usr.user_name)).map_err(|_| UserError::DBError)?;
   
   
    match qry_usr.next().unwrap() {
      sqlite::State::Done => {},
      sqlite::State::Row => {
        eprintln!("User {} allready exits!", usr.user_name);
        return Err(UserError::AllreadyExists);
      }
    }
    match usr.pw_hash {
      Some(pw_hash) => {
        self.con.execute(format!("INSERT INTO users (first_name, sec_name, user_name, pw_hash, perm_flags) VALUES ('{}', '{}', '{}', '{}', {})", usr.first_name, usr.last_name, usr.user_name, pw_hash, usr.permissions.0)).map_err(|_| UserError::CannotAdd)?;
      },
      None => return Err(UserError::CannotAdd)
    }
    

    Ok(())
  }
}
