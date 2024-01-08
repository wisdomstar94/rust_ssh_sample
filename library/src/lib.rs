use std::{net::{TcpStream, SocketAddr, ToSocketAddrs}, path::Path, fs::{self, DirEntry}, io::{self, Write}, time::Duration};
use ssh2::{Session, ErrorCode, Sftp};

#[derive(Debug)]
pub struct ConnectInfo {
  user_name: String,
  host: String,
  port: String,
  private_key_path: String,
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
    let sockets = format!("{}:{}", self.host, self.port).to_socket_addrs().unwrap();
    let socket_vec: Vec<SocketAddr> = sockets.collect();
    let socket = socket_vec.first().unwrap();
    // let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(ip_fragments.get(0).unwrap().parse().unwrap(), ip_fragments.get(1).unwrap().parse().unwrap(), ip_fragments.get(2).unwrap().parse().unwrap(), ip_fragments.get(3).unwrap().parse().unwrap())), self.port.parse::<u16>().unwrap());
    let tcp = TcpStream::connect_timeout(socket, Duration::new(5, 0)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_timeout(5000);
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
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

#[derive(Debug)]
pub struct Error<'a> {
  pub code: Option<ErrorCode>,
  pub msg: &'a str,
} 

pub fn get_dir_entry_list(target_folder_path: &Path) -> Result<Vec<DirEntry>, io::Error> {
  match fs::read_dir(target_folder_path) {
    Ok(result) => {
      let vec: Vec<io::Result<DirEntry>> = result.collect();
      let mut dir_entry_list: Vec<DirEntry> = Vec::new();
      for item in vec {
        if let Ok(v) = item {
          dir_entry_list.push(v);
        }
      }
      Ok(dir_entry_list)
    },
    Err(error) => Err(error),
  }
}

pub fn upload_file(sess: &Session, local_file_path: &Path, remote_file_path: &Path) -> Result<bool, ssh2::Error> {
  let local_file_byte_vec = fs::read(local_file_path).unwrap();
  let local_file_size = local_file_byte_vec.len();
  let mut remote_file = sess.scp_send(remote_file_path, 0o644, local_file_size.try_into().unwrap(), None).unwrap();
  remote_file.write(&local_file_byte_vec).unwrap();
  remote_file.send_eof().unwrap();
  remote_file.wait_eof().unwrap();
  remote_file.close().unwrap();
  remote_file.wait_close().unwrap();
  Ok(true)
}

pub fn upload_folder(sess: &Session, target_local_folder_path: &Path, remote_folder_path: &Path) -> Result<bool, ssh2::Error> {
  let sftp = sess.sftp().unwrap();
  remote_mkdir_recursive(&sftp, remote_folder_path);
  dir_logic(sess, &sftp, target_local_folder_path, remote_folder_path, remote_folder_path, true);
  Ok(true)
}

pub fn remote_mkdir_recursive(sftp: &Sftp, folder_path: &Path) -> Vec<bool> {
  let mut result: Vec<bool> = Vec::new();
  let remote_folder_path_vec: Vec<&Path> = convert_path_to_vec(folder_path);
  let has_root = folder_path.has_root();
  let mut i = 0;
  for target_path in remote_folder_path_vec {
    if i == 0 && has_root {
      result.push(false);
      i = i + 1;
      continue;
    }
    match sftp.mkdir(target_path, 0o755) {
      Ok(_) => result.push(true),
      Err(_) => result.push(false),
    }
    i = i + 1;
  }
  result
}

pub fn convert_path_to_vec(path: &Path) -> Vec<&Path> {
  let mut result: Vec<&Path> = Vec::new();
  result.push(path);
  let mut target_path: &Path = path;
  loop {
    let temp = target_path.parent();
    // dbg!(temp);
    if let Some(item) = temp {
      result.push(item);
      target_path = item;
    } else {
      break;
    }
  };
  result.reverse();
  result
}

fn dir_logic(sess: &Session, sftp: &Sftp, local_target_folder: &Path, current_remote_folder_path: &Path, remote_folder_path: &Path, is_first: bool) {
  let next_path_buf = if is_first == false {
    let name = local_target_folder.file_name().unwrap().to_str().unwrap();
    // let temp = target_path.join(name);
    current_remote_folder_path.join(name)
  } else {
    current_remote_folder_path.to_path_buf()
  };
  let next_path: &Path = next_path_buf.as_path();
  match sftp.mkdir(next_path, 0o755) {
    Ok(_) => {},
    Err(_) => {},
  }

  let list = get_dir_entry_list(local_target_folder).unwrap();
  for entry in list {
    if entry.metadata().unwrap().is_dir() {
      dir_logic(sess, sftp, entry.path().as_path(), next_path, remote_folder_path, false);
    } else {
      file_logic(sess, sftp, entry.path().as_path(), next_path.join(entry.file_name().to_str().unwrap()).as_path());
    }
  }
}

fn file_logic(sess: &Session, _: &Sftp, local_file_path: &Path, remote_file_path: &Path) {
  upload_file(sess, local_file_path, remote_file_path).unwrap();
}