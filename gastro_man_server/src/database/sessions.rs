use sha3::{Digest, Sha3_256};

pub fn generate_session_id(user_name: &String) -> String {
    let cur_time = std::time::SystemTime::now();

    let cur_time_str = cur_time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs().to_string();

    let mut sha = Sha3_256::new();
    sha.input(cur_time_str);
    sha.input(user_name);

    let mut r = format!("{:x}",sha.result());
    r.truncate(10);
    r
}
