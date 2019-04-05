use super::{methods, RawMsg, WSClient, DB_OFFLINE_ERR};
use crate::database::requests;
use crate::websocket::json_structs::WebUser;

pub fn handle_admin_msg(wsc: &mut WSClient, msg: RawMsg) {
  //check if client has admin rights
  let (prod, cons) = bounded_spsc_queue::make(1);
  wsc
    .db_query
    .send(requests::DBRequest::PermissionsGetRequest(
      msg.0.clone(),
      prod,
    ))
    .expect(DB_OFFLINE_ERR);

  match WSClient::wait_for_query_res(cons) {
    Some(perm_flags) => {
      if !perm_flags.is_admin() {
        return wsc.send_permission_error("You are not an admin.");
      }

      // user is admin

      println!("ws > Handling admin request : {}", &msg.1);

      match msg.1.as_ref() {
        methods::ADMIN_GETUSERS => {
          return get_users(wsc);
        }
        not_known => eprintln!("ws > admin command {} not known", not_known),
      }
    }
    None => {
      wsc.send_permission_error("Can't find permissions for you.");
    }
  }

}

fn get_users(wsc: &mut WSClient) {
  let (prod, cons) = bounded_spsc_queue::make(2);

  // preprare database request
  let req = requests::DBRequest::AdminUserListRequest(prod);
  wsc.db_query.send(req).expect(DB_OFFLINE_ERR);

  let res: requests::AdminUserListResponse = loop {
    match cons.try_pop() {
      Some(a) => break a,
      None => {
        std::thread::sleep(std::time::Duration::from_micros(100));

        continue;
      }
    }
  };

  let wusrs: Vec<WebUser> = res.into_iter().map(|e| e.into()).collect();

  wsc.send(methods::ADMIN_GETUSERS_RET, &serde_json::to_string(&wusrs).expect("Could not parse userlist"))
}
