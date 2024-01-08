use std::{env, path::Path};
use library::{ConnectInfo, upload_folder};
extern crate dotenv;
use dotenv::dotenv;

#[test]
fn folder_upload_test() {
  dotenv().ok();
  if let (Ok(user_name), Ok(host), Ok(port), Ok(private_key_path)) = (dotenv::var("SSH_USERNAME"), dotenv::var("SSH_HOST"), dotenv::var("SSH_PORT"), dotenv::var("SSH_PRIVATE_KEY_PATH")) {
    let conn = ConnectInfo::new(user_name, host, port, private_key_path);
    let sess = conn.try_connect().unwrap();
    println!("ssh 연결 성공!");

    let local_folder_path_buf = env::current_dir().unwrap().join("src");
    let local_folder_path = local_folder_path_buf.as_path();
    let remote_folder_path = Path::new("/home/ec2-user/a/b/src");
    upload_folder(&sess, local_folder_path, &remote_folder_path).unwrap();
    
    sess.disconnect(None, "test end", None).unwrap();
  }
} 