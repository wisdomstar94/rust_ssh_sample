use library::{ConnectInfo, run_command};
extern crate dotenv;
use dotenv::dotenv;

#[test]
fn run_command_test() {
  dotenv().ok();
  if let (Ok(user_name), Ok(host), Ok(port), Ok(private_key_path)) = (dotenv::var("SSH_USERNAME"), dotenv::var("SSH_HOST"), dotenv::var("SSH_PORT"), dotenv::var("SSH_PRIVATE_KEY_PATH")) {
    let conn = ConnectInfo::new(user_name, host, port, private_key_path);
    let sess = conn.try_connect().unwrap();
    println!("ssh 연결 성공!");

    // let result = run_command(&sess, format!("source /home/ec2-user/my_script.sh")).unwrap();
    // println!("result is {}", result);

    let result = run_command(&sess, format!("ls -al")).unwrap();
    println!("\n[ result ]\n\n{}", result);

    sess.disconnect(None, "test end", None).unwrap();
  }
} 
