use self::net::Server;

pub mod net;

fn main() {
  println!("redis start");
  let port: u16 = 6379;
  let mut server = Server::new("127.0.0.1", &port);
  server.start();
}
