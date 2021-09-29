extern crate zredis;

use zredis::net::Server;

use std::io::Write;
use std::net::TcpStream;
use std::thread;

#[test]
fn parse_ping() {
  let port: u16 = 6379;
  thread::spawn(move || {
    let server = Server::new("127.0.0.1", &port);
    server.start();
  });
  thread::sleep_ms(120);
  let addr = format!("127.0.0.1:{}", port);
  println!("addraddraddraddr {}", addr);
  // let streamres = TcpStream::connect(&*addr);
  // assert!(streamres.is_ok());
  // let mut stream = streamres.unwrap();
  // let message = b"*2\r\n$4\r\nping\r\n$4\r\npong\r\n";
  // assert!(stream.write(message).is_ok());
}
