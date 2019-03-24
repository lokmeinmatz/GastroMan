use ws::{Handler, Handshake, Message, Result, Sender};
use itertools::Itertools;

pub mod json_structs;
use json_structs::LoginRequest;
use serde::de::Deserialize;
use std::sync::mpsc;
use crate::database::requests;

use sha3::{Digest, Sha3_256};

#[derive(Debug)]
pub struct ParsedMessage<T> {
  session_id : String,
  method: String,
  payload: T
}

impl<'de, T> ParsedMessage<T> where T: Deserialize<'de> {
  pub fn new(sid: String, method: String, payload: T) -> Self {
    ParsedMessage {
      session_id: sid,
      method,
      payload 
    }
  }
}




pub struct WSClient {
  out: Sender,
  db_query: mpsc::Sender<requests::DatabaseRequest>
}

/// (sessionID, Method, Data)
type RawMsg = (String, String, String);

impl WSClient {
  
  pub fn new(out: Sender, db_query: mpsc::Sender<requests::DatabaseRequest>) -> Self {
    WSClient {
      out,
      db_query
    }
  }

  /// Takes the sessionID, the method and the data as strings
  fn evaluate_message(&mut self, msg : RawMsg) {
    match msg.1.as_ref() {
            "user.login" => {self.handle_login(msg)},
            _ => eprintln!("ws > unknown method : {}", &msg.1)
          }
  }

  fn handle_login(&mut self, msg: RawMsg) {
    let pl : LoginRequest = serde_json::from_str(msg.2.as_ref()).expect("ws > Error while parsing payload");
    let (prod, cons) = bounded_spsc_queue::make(2);


    // preprare database request
    let req = requests::DatabaseRequest::new(requests::RequestType::UserRequest(pl.user), prod);
    self.db_query.send(req).expect("ws > Database down!!!");

  	// hash pw while waiting for database
    let mut pw_hashed = Sha3_256::new();
    pw_hashed.input(pl.password);
    let pw_hashed = format!("{:x}", pw_hashed.result());

    println!("ws > waiting for db response...");
    let res : requests::ResponseType = loop {
      match cons.try_pop() {
        Some(a) => break a,
        None => {std::thread::yield_now(); continue}
      }
    };

    match res {
      requests::ResponseType::UserGetResponse(opt_user) => {
        if let Some(user) = opt_user {
          println!("ws > User: {:?}", user);
          self.out.send(format!("|user.login.success|{}", &pl.user)).expect("User channel closed");
        }
      },
      _ => eprintln!("ws > Database is dumb: send wrong Response!")
    }
  }
}

impl Handler for WSClient {
  fn on_open(&mut self, _: Handshake) -> Result<()> {
    self.out.send("Hello WebSocket")
  }


  fn on_message(&mut self, msg: Message) -> Result<()> {
    // Close the connection when we get a response from the server
    println!("Got message: {}", msg);
    
    match msg {
      Message::Text(t) => {
        // TODO: Better way to get utf8 str than creating a new every time
        if let Some((uid, method, data)) = t.split(std::str::from_utf8(&[31]).unwrap()).tuples().next() {
          self.evaluate_message((uid.to_owned(), method.to_owned(), data.to_owned()));
        }
        else {
          eprintln!("ws > Received invalid message. Too many or few | splitters");
        }
      },
      Message::Binary(_) => {eprintln!("ws > Cannot decode binary data");}
    }

    Ok(())
  }
}
