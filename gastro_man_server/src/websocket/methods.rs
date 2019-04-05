/// Naming convention: requests are "basic"
/// if only one possible outcome exists: _RET (.ret) , else _SUCCESS (.success) and _ERROR (.error)

pub const USER_LOGIN : &str           = "user.login";
pub const USER_LOGIN_SUCCESS : &str   = "user.login.success";
pub const USER_LOGIN_ERROR : &str     = "user.login.error";
pub const ADMIN_GETUSERS : &str       = "admin.getusers";
pub const ADMIN_GETUSERS_RET : &str    = "admin.getusers.ret";
pub const PERMISSION_ERROR : &str    = "permissionerror";