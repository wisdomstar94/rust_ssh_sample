use std::{net::TcpStream, path::Path};

use ssh2::{Session, ErrorCode};

#[derive(Debug)]
pub struct ConnectInfo {
  user_name: String,
  host: String,
  port: String,
  private_key_path: String,
}

#[derive(Debug)]
pub struct Error<'a> {
  pub code: Option<ErrorCode>,
  pub msg: &'a str,
} 

impl ConnectInfo {
  pub fn new(user_name: String, host: String, port: String, private_key_path: String) -> Self {
    ConnectInfo {
      user_name,
      host,
      port,
      private_key_path, 
    }
  }

  pub fn try_connect(&self) -> Result<Session, Error> {
    let tcp = TcpStream::connect(format!("{}:{}", self.host, self.port)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    // sess.userauth_pubkey_memory(&info.user_name, None, &info.private_key_path, None).unwrap();
    sess.userauth_pubkey_file(&self.user_name, None, Path::new(&self.private_key_path), None).unwrap();
    if sess.authenticated() {
      Ok(sess)
    } else {
      Err(Error {
        code: None,
        msg: "ssh 연결 인증에 실패하였습니다.",
      })
    }
  }
}