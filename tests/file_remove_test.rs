use std::path::Path;
use library::{ConnectInfo, remove_file};
extern crate dotenv;
use dotenv::dotenv;

#[test]
fn file_remove_test() {
  dotenv().ok();
  if let (Ok(user_name), Ok(host), Ok(port), Ok(private_key_path)) = (dotenv::var("SSH_USERNAME"), dotenv::var("SSH_HOST"), dotenv::var("SSH_PORT"), dotenv::var("SSH_PRIVATE_KEY_PATH")) {
    let conn = ConnectInfo::new(user_name, host, port, private_key_path);
    let sess = conn.try_connect().unwrap();
    println!("ssh 연결 성공!");

    let remote_file_path = Path::new("/home/ec2-user/a/b/src-legacy/a/b/c.txt");
    match remove_file(&sess, remote_file_path) {
      Ok(_) => {
        println!("파일 삭제 성공");
      },
      Err(error) => {
        println!("파일 삭제 실패 : {:?}", error);
      },
    }

    sess.disconnect(None, "test end", None).unwrap();
  }
} 