use std::{path::Path, env};
use library::{ConnectInfo, upload_file};
extern crate dotenv;
use dotenv::dotenv;

#[test]
fn file_upload_test() {
  dotenv().ok();
  if let (Ok(user_name), Ok(host), Ok(port), Ok(private_key_path)) = (dotenv::var("SSH_USERNAME"), dotenv::var("SSH_HOST"), dotenv::var("SSH_PORT"), dotenv::var("SSH_PRIVATE_KEY_PATH")) {
    let conn = ConnectInfo::new(user_name, host, port, private_key_path);
    let sess: ssh2::Session = conn.try_connect().unwrap();
    println!("ssh 연결 성공!");

    let local_file_path_buf = env::current_dir().unwrap().join("Cargo.toml");
    let local_file_path = local_file_path_buf.as_path();
    let remote_file_path = Path::new("/home/ec2-user/Cargo.toml");
    
    upload_file(&sess, local_file_path, remote_file_path).unwrap();
    sess.disconnect(None, "test end", None).unwrap();
  }
} 