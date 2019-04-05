use itertools::Itertools;
use ws::{Handler, Handshake, Message, Sender};

pub mod json_structs;
mod methods;
mod admin_handler;
use crate::database::requests;
use json_structs::{LoginRequest, WebUser};
use std::sync::mpsc;

use sha3::{Digest, Sha3_256};


pub const DB_OFFLINE_ERR : &str = "ws > DB is not reachable";

pub struct WSClient {
  out: Sender,
  sid: Option<String>,
  pub db_query: mpsc::Sender<requests::DBRequest>,
}

/// (sessionID, Method, Data)
pub type RawMsg = (String, String, String);

impl WSClient {
  pub fn new(out: Sender, db_query: mpsc::Sender<requests::DBRequest>) -> Self {
    WSClient { out, db_query, sid: None }
  }

  /// blocks efficiently until db has answered
  pub fn wait_for_query_res<T>(cons: bounded_spsc_queue::Consumer<T>) -> T {
    let mut start = std::time::Instant::now();
    loop {
      match cons.try_pop() {
        Some(a) => break a,
        None => {
          
          std::thread::sleep(std::time::Duration::from_micros(200));
          if start.elapsed().as_secs() > 1 {
            eprintln!("ws > db-query takes wayy to long!");
            start = std::time::Instant::now();

          }
          continue;
        }
      }
    }
  }

  /// Takes the sessionID, the method and the data as strings
  fn evaluate_message(&mut self, msg: RawMsg) {
    println!("ws > Handling request : {}", &msg.1);

    if msg.1.starts_with("admin") {
      return admin_handler::handle_admin_msg(self, msg)
    }

    match msg.1.as_ref() {
      methods::USER_LOGIN_PW => self.handle_login_by_pw(msg),
      methods::USER_LOGIN_SID => self.handle_login_by_sid(msg),
      _ => {eprintln!("ws > unknown method : {}", &msg.1);}
    }
  }

  pub fn send(&mut self, method: &str, payload: &str) {
    match self.sid.as_ref() {
      Some(sid) => self.out.send(Self::format(sid, method, payload)).expect("WS closed"),
      None => self.out.send(Self::format("0", method, payload)).expect("WS closed")
    }
  }

  fn send_permission_error(&mut self, err: &str) {
    self.send(methods::PERMISSION_ERROR, &format!("{{\"msg\": \"Permission-Error: {}\"}}", err));
  }

  fn handle_login_by_pw(&mut self, msg: RawMsg) {
    let payload: LoginRequest =
      serde_json::from_str(msg.2.as_ref()).expect("ws > Error while parsing payload");
    let password = payload.password.unwrap_or(String::new());
    let (prod, cons) = bounded_spsc_queue::make(2);

    // preprare database request
    let req = requests::DBRequest::UserGetRequest(payload.user, prod);
    self.db_query.send(req).expect(DB_OFFLINE_ERR);

    // hash pw while waiting for database
    let mut pw_hashed = Sha3_256::new();
    pw_hashed.input(password);
    let pw_hashed = format!("{:x}", pw_hashed.result());

    let res: requests::UserGetResponse = Self::wait_for_query_res(cons);

    match res {
      Some(user) => {

        if user.pw_hash == Some(pw_hashed) {
          // if request came with diffrent sessionID, make sure to delete, as well as other sessions of this user
          if msg.0.len() > 2 {
            self.db_query.send(requests::DBRequest::DeleteSessionRequest(msg.0)).expect(DB_OFFLINE_ERR);
            }
  
          let sid = user
            .session_id
            .as_ref()
            .expect("ws > user has no sessionid")
            .clone();
          let serial_user: WebUser = user.into();
          self.sid = Some(sid);

          self.send(methods::USER_LOGIN_SUCCESS, &serde_json::to_string(&serial_user).expect("Could not parse serial_user"));
        }
        else {
          // user entered wrong password
          self.send(methods::USER_LOGIN_ERROR, "{\"error\":\"Password incorrect\"}");
        }
      },
      None => {
        self.send(methods::USER_LOGIN_ERROR, "{\"error\":\"Username unknown\"}");
      }
    }
  }

  fn handle_login_by_sid(&mut self, msg: RawMsg) {
    let payload: LoginRequest =
      serde_json::from_str(msg.2.as_ref()).expect("ws > Error while parsing payload");
    let (prod, cons) = bounded_spsc_queue::make(2);

    // preprare database request
    let req = requests::DBRequest::UserGetRequest(payload.user, prod);
    self.db_query.send(req).expect(DB_OFFLINE_ERR);
    let res: requests::UserGetResponse = Self::wait_for_query_res(cons);

    match res {
      Some(user) => {

        if user.session_id == payload.sid {
          // if request came with diffrent sessionID, make sure to delete, as well as other sessions of this user
  
          let serial_user: WebUser = user.into();
          self.sid = payload.sid;

          self.send(methods::USER_LOGIN_SUCCESS, &serde_json::to_string(&serial_user).expect("Could not parse serial_user"));
        }
        else {
          // user entered wrong password
          self.send(methods::USER_LOGIN_ERROR, "{\"error\":\"Session invalid\"}");
        }
      },
      None => {
        self.send(methods::USER_LOGIN_ERROR, "{\"error\":\"Username unknown\"}");
      }
    }
  }

  /// packs the session id, the method ad payload into an string
  fn format(session_id: &str, method: &str, payload: &str) -> String {
    format!("{}\x1f{}\x1f{}", session_id, method, payload)
  }
}

impl Handler for WSClient {
  fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
    Ok(())
  }

  fn on_message(&mut self, msg: Message) -> ws::Result<()> {
    let start_time = std::time::Instant::now();

    match msg {
      Message::Text(t) => {
        // TODO: Better way to get utf8 str than creating a new every time
        if let Some((uid, method, data)) =
          t.split(std::str::from_utf8(&[31]).unwrap()).tuples().next()
        {
          self.evaluate_message((uid.to_owned(), method.to_owned(), data.to_owned()));
        } else {
          eprintln!("ws > Received invalid message. Too many or few | splitters");
        }
      }
      Message::Binary(_) => {
        eprintln!("ws > Cannot decode binary data");
      }
    }


    println!("Message handling took {}s", start_time.elapsed().as_float_secs());
    Ok(())
  }
}
