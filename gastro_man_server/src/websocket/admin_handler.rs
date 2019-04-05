use super::{WSClient, RawMsg, DB_OFFLINE_ERR};
use crate::database::requests;

pub fn handle_admin_msg(wsc: &mut WSClient, msg: RawMsg) -> ws::Result<()> {

  //check if client has admin rights
  let (prod, cons) = bounded_spsc_queue::make(1);
  wsc.db_query.send(requests::DBRequest::PermissionsGetRequest(msg.0.clone(), prod)).expect(DB_OFFLINE_ERR);

  match WSClient::wait_for_query_res(cons) {
    Some(perm_flags) => {
      if perm_flags.is_admin() {return wsc.send_permission_error(&msg.0)}

      
    },
    None => {
      wsc.send_permission_error(&msg.0)?;
    }
  }

  Ok(())
}