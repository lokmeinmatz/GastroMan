use itertools::Itertools;
use ws::{Handler, Handshake, Message, Sender};

pub mod json_structs;
mod methods;
use crate::database::requests;
use json_structs::{LoginRequest, WebUser};
use std::sync::mpsc;

use sha3::{Digest, Sha3_256};


const DB_OFFLINE_ERR : &str = "ws > DB is not reachable";

pub struct WSClient {
  out: Sender,
  db_query: mpsc::Sender<requests::DBRequest>,
}

/// (sessionID, Method, Data)
type RawMsg = (String, String, String);

impl WSClient {
  pub fn new(out: Sender, db_query: mpsc::Sender<requests::DBRequest>) -> Self {
    WSClient { out, db_query }
  }

  /// Takes the sessionID, the method and the data as strings
  fn evaluate_message(&mut self, msg: RawMsg) -> ws::Result<()> {
    match msg.1.as_ref() {
      methods::USER_LOGIN => self.handle_login(msg),
      methods::ADMIN_GETUSERS => self.handle_getusers(msg),
      _ => {eprintln!("ws > unknown method : {}", &msg.1); Err(ws::Error::new(ws::ErrorKind::Protocol, ""))},
    }
  }

  fn handle_login(&mut self, msg: RawMsg) -> ws::Result<()> {
    let payload: LoginRequest =
      serde_json::from_str(msg.2.as_ref()).expect("ws > Error while parsing payload");
    let (prod, cons) = bounded_spsc_queue::make(2);

    // preprare database request
    let req = requests::DBRequest::UserGetRequest(payload.user, prod);
    self.db_query.send(req).expect(DB_OFFLINE_ERR);

    // hash pw while waiting for database
    let mut pw_hashed = Sha3_256::new();
    pw_hashed.input(payload.password);
    let pw_hashed = format!("{:x}", pw_hashed.result());

    let res: requests::UserGetResponse = loop {
      match cons.try_pop() {
        Some(a) => break a,
        None => {
          
          std::thread::sleep(std::time::Duration::from_micros(100));
          continue;
        }
      }
    };


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

          self
            .out
            .send(Self::format(
              &sid,
              methods::USER_LOGIN_SUCCESS,
              &serde_json::to_string(&serial_user).expect("Could not parse serial_user"),
            ))
            ?;
        }
        else {
          // user entered wrong password
          self.out.send(Self::format("0", methods::USER_LOGIN_ERROR, "{\"error\":\"Password incorrect\"}"))?;
        }
      },
      None => {
        self.out.send(Self::format("0", methods::USER_LOGIN_ERROR, "{\"error\":\"Username unknown\"}"))?;
      }
    }
    Ok(())
  }
  fn handle_getusers(&mut self, _msg: RawMsg) -> ws::Result<()> {
    let (prod, cons) = bounded_spsc_queue::make(2);

    // preprare database request
    let req = requests::DBRequest::AdminUserListRequest(prod);
    self.db_query.send(req).expect(DB_OFFLINE_ERR);

    let res: requests::AdminUserListResponse = loop {
      match cons.try_pop() {
        Some(a) => break a,
        None => {
          
          std::thread::sleep(std::time::Duration::from_micros(100));

          continue;
        }
      }
    };

    let wusrs : Vec<WebUser> = res.into_iter().map(|e| e.into()).collect();

    self.out.send(Self::format("0", methods::ADMIN_GETUSERS_RET, &serde_json::to_string(&wusrs).expect("Could not parse userlist")))
  }

  /// packs the session id, the method ad payload into an string
  fn format(session_id: &str, method: &str, payload: &str) -> String {
    format!("{}\x1f{}\x1f{}", session_id, method, payload)
  }
}

impl Handler for WSClient {
  fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
    self.out.send("Hello WebSocket")
  }

  fn on_message(&mut self, msg: Message) -> ws::Result<()> {
    let start_time = std::time::Instant::now();

    match msg {
      Message::Text(t) => {
        // TODO: Better way to get utf8 str than creating a new every time
        if let Some((uid, method, data)) =
          t.split(std::str::from_utf8(&[31]).unwrap()).tuples().next()
        {
          self.evaluate_message((uid.to_owned(), method.to_owned(), data.to_owned()))?;
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
