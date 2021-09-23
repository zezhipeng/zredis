pub mod net;
pub use crate::net::new_server;
pub mod protocol;

fn main() {
  let port: u16 = 6379;
  let server = new_server("127.0.0.1", &port);
  server.start();
}
