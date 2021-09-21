use std::net::{TcpListener, TcpStream};

pub struct Server {
  pub ip: String,
  pub port: u16,
  pub clients: Vec<TcpStream>,
}

impl Server {
  pub fn new(ip: &str, port: &u16) -> Server {
    return Server {
      ip: ip.to_string(),
      port: *port,
      clients: Vec::new(),
    };
  }

  pub fn handle_client(&mut self, stream: TcpStream) {
    self.clients.push(stream);
    println!("Hello, client!");
  }

  pub fn start(&mut self) {
    let addr: String = format!("{0}:{1}", self.ip, self.port);
    let listener = TcpListener::bind(&*addr).unwrap();
    for stream in listener.incoming() {
      match stream {
        Ok(stream) => self.handle_client(stream),
        Err(e) => { /* 连接失败 */ }
      }
    }
    drop(listener);
  }
}

pub fn new_server(ip: &str, port: &u16) -> Server {
  return Server::new(ip, port);
}
