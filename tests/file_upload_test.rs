use std::{path::Path, fs, env, io::Write};

use library::ConnectInfo;
extern crate dotenv;
use dotenv::dotenv;

#[test]
fn file_upload_test() {
  dotenv().ok();
  if let (Ok(user_name), Ok(host), Ok(port), Ok(private_key_path)) = (dotenv::var("SSH_USERNAME"), dotenv::var("SSH_HOST"), dotenv::var("SSH_PORT"), dotenv::var("SSH_PRIVATE_KEY_PATH")) {
    let conn = ConnectInfo::new(user_name, host, port, private_key_path);
    let sess = conn.try_connect().unwrap();
    println!("ssh 연결 성공!");

    let local_file_path = env::current_dir().unwrap().join("Cargo.toml");
    let local_file_byte_vec = fs::read(local_file_path.as_path()).unwrap();
    let local_file_size = local_file_byte_vec.len();
    let mut remote_file = sess.scp_send(Path::new("/home/ec2-user/Cargo.toml"), 0o644, local_file_size.try_into().unwrap(), None).unwrap();
    
    remote_file.write(&local_file_byte_vec).unwrap();
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();
    sess.disconnect(None, "test end", None).unwrap();
  }
} 