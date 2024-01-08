use std::path::Path;
use library::{ConnectInfo, file_or_folder_rename};
extern crate dotenv;
use dotenv::dotenv;

#[test]
fn file_rename_test() {
  dotenv().ok();
  if let (Ok(user_name), Ok(host), Ok(port), Ok(private_key_path)) = (dotenv::var("SSH_USERNAME"), dotenv::var("SSH_HOST"), dotenv::var("SSH_PORT"), dotenv::var("SSH_PRIVATE_KEY_PATH")) {
    let conn = ConnectInfo::new(user_name, host, port, private_key_path);
    let sess = conn.try_connect().unwrap();
    println!("ssh 연결 성공!");

    let before_file_path = Path::new("/home/ec2-user/my_script.sh");
    let after_file_path = Path::new("/home/ec2-user/f1/f2/f3/my_script.sh");
    file_or_folder_rename(&sess, before_file_path, after_file_path).unwrap();

    sess.disconnect(None, "test end", None).unwrap();
  }
} 

#[test]
fn folder_rename_test() {
  dotenv().ok();
  if let (Ok(user_name), Ok(host), Ok(port), Ok(private_key_path)) = (dotenv::var("SSH_USERNAME"), dotenv::var("SSH_HOST"), dotenv::var("SSH_PORT"), dotenv::var("SSH_PRIVATE_KEY_PATH")) {
    let conn = ConnectInfo::new(user_name, host, port, private_key_path);
    let sess = conn.try_connect().unwrap();
    println!("ssh 연결 성공!");

    let before_file_path = Path::new("/home/ec2-user/kkk");
    let after_file_path = Path::new("/home/ec2-user/kkk3/kkk2/kkk");
    file_or_folder_rename(&sess, before_file_path, after_file_path).unwrap();

    sess.disconnect(None, "test end", None).unwrap();
  }
} 