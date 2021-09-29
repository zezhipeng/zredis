use super::protocol::parse;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

// use super::protocol::parse;

pub struct Client {
  pub stream: TcpStream,
}

pub struct Server {
  pub ip: String,
  pub port: u16,
}

impl Client {
  pub fn new(stream: TcpStream) -> Client {
    Client { stream }
  }
  pub fn read(&mut self) {
    let mut buffer = [0u8; 512];

    loop {
      let result = self.stream.read(&mut buffer);
      if result.is_err() {
        break;
      }
      let len = result.unwrap();
      println!("gonna print {}", len);
      if len == 0 {
        break;
      }
      let try_parser = parse(&buffer, len);
      if try_parser.is_err() {
        break;
      }
      let parser = try_parser.unwrap();
      println!("{}", parser.argc);
      for i in 0..parser.argc {
        println!("{}", parser.get_str(i).unwrap())
      }
    }
  }
}

impl Server {
  pub fn new(ip: &str, port: &u16) -> Server {
    Server {
      ip: ip.to_string(),
      port: *port,
    }
  }

  pub fn handle_client(&self, stream: TcpStream) {
    thread::spawn(move || {
      let mut client = Client::new(stream);
      client.read();
    });
  }

  pub fn start(&self) {
    let addr: String = format!("{0}:{1}", self.ip, self.port);
    let listener = TcpListener::bind(&*addr).unwrap();
    for stream in listener.incoming() {
      match stream {
        Ok(stream) => self.handle_client(stream),
        Err(e) => {
          println!("{}", e);
        }
      }
    }
    drop(listener);
  }
}

pub fn new_server(ip: &str, port: &u16) -> Server {
  Server::new(ip, port)
}
