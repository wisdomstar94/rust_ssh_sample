use std::path::Path;
use library::{ConnectInfo, remove_folder};
extern crate dotenv;
use dotenv::dotenv;

/// 폴더 밑에 파일 또는 폴더가 존재할 경우 삭제 안됨!
#[test]
fn folder_remove_test() {
  dotenv().ok();
  if let (Ok(user_name), Ok(host), Ok(port), Ok(private_key_path)) = (dotenv::var("SSH_USERNAME"), dotenv::var("SSH_HOST"), dotenv::var("SSH_PORT"), dotenv::var("SSH_PRIVATE_KEY_PATH")) {
    let conn = ConnectInfo::new(user_name, host, port, private_key_path);
    let sess = conn.try_connect().unwrap();
    println!("ssh 연결 성공!");

    let remote_folder_path = Path::new("/home/ec2-user/b");
    match remove_folder(&sess, remote_folder_path) {
      Ok(_) => {
        println!("폴더 삭제 성공");
      },
      Err(error) => {
        println!("폴더 삭제 실패 : {:?}", error);
      },
    }

    sess.disconnect(None, "test end", None).unwrap();
  }
} 