use colored::*;
use sqlite::Readable;
use sha3::{Digest, Sha3_256};

use std::collections::HashSet;
use std::sync::mpsc;
use crate::users::User;
use bounded_spsc_queue::{Producer};

pub mod requests;
use requests::{DatabaseRequest, RequestType, ResponseType};

#[derive(Debug)]
pub enum UserError {
  AllreadyExists,
  DBError
}



pub struct DBManager {
  con: sqlite::Connection,
  db_query: (mpsc::Sender<DatabaseRequest>, mpsc::Receiver<DatabaseRequest>)
}

impl DBManager {
  pub fn new() -> Result<DBManager, sqlite::Error> {
    let sq = sqlite::open("current_data.sqlite")?;

    let query_cannel = mpsc::channel();

    let db = DBManager { con: sq, db_query: query_cannel };

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

        let username = String::from("stduser");
        let password: String = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis().to_string();


        let password = &password[(password.len()-6)..];
        println!("\ndbm > Username: {}", username);
        println!("\ndbm > Password: {}", password);
        let mut h = Sha3_256::new();
        h.input(password);

        let usr = User::new(username, String::from("std"), String::from("usr"), format!("{:x}", h.result()));

        match db.add_user(usr) {
            Ok(_) => println!("dbm > You can now log in with this credentials."),
            Err(e) => eprintln!("dbm > {:?}", e)
        }
    }

    Ok(db)
  }

  pub fn get_query_sender(&mut self) -> mpsc::Sender<DatabaseRequest> {
    self.db_query.0.clone()
  }

  pub fn run(&mut self) {
    loop {
      match self.db_query.1.recv() {
        Ok(req) => {
          println!("dbm > Processing Request {:?}", req);
          match req.req {
            RequestType::UserExists(user, pw_hashed) => self.handle_user_exists(user, pw_hashed, req.answer),
            _ => {unimplemented!()}
          }
        },
        Err(e) => eprintln!("dbm > Error while getting Request, {:?}", e)
      }

      std::thread::yield_now();
    }
  }

  fn handle_user_exists(&mut self, user: String, password_hashed: String, answer: Producer<ResponseType>) {
    let mut stmt = self.con.prepare("SELECT * FROM users WHERE user_name = {} AND pw_hash = {}").expect("dbm > error while creating Statement");

    if let sqlite::State::Row
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
    let mut stmt = self.con.prepare("SELECT COUNT(*) FROM users").unwrap();

    // go to the first and only result
    stmt.next().unwrap();

    stmt.read(0).expect("Error while reading COUNT of users")
  }

  pub fn add_user(&self, usr : User) -> Result<(), UserError> {
    // check if user allready exists
    let mut qry_usr = self.con.prepare(format!("SELECT * FROM users WHERE user_name = '{}'", usr.username)).map_err(|_| UserError::DBError)?;
   
   
    match qry_usr.next().unwrap() {
      sqlite::State::Done => {},
      sqlite::State::Row => {
        eprintln!("User {} allready exits!", usr.username);
        return Err(UserError::AllreadyExists);
      }
    }

    self.con.execute(format!("INSERT INTO users (first_name, sec_name, user_name, pw_hash) VALUES ('{}', '{}', '{}', '{}')", usr.first_name, usr.sec_name, usr.username, usr.pw_hash)).map_err(|_| UserError::DBError)?;

    Ok(())
  }
}
