use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

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
      let usize = self.stream.read(&mut buffer).unwrap();
      if usize == 0 {
        break;
      }
      println!("{} {}", from_utf8(&buffer).unwrap(), usize);
    }
  }
}

impl Server {
  pub fn new(ip: &str, port: &u16) -> Server {
    return Server {
      ip: ip.to_string(),
      port: *port,
    };
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
  return Server::new(ip, port);
}

#[cfg(test)]
mod tests {
  use std::net::TcpStream;

  #[test]
  fn it_works() {
    let mut stream = TcpStream::connect("127.0.0.1:6379");

    // stream.write(&[1]);
    // stream.read(&mut [0; 128]);
    // Ok(());
  }
}
