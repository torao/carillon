use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::Result;

pub fn http_server() -> Result<()> {
  let port = 7878;
  let address = format!("localhost:{}", port);
  let listener = TcpListener::bind(&address)?;
  log::info!("Test HTTP server is listening on http://{}", &address);

  for stream in listener.incoming() {
    let stream = stream?;
    handle_connection(stream)?;
  }
  Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
  log::info!("Accept connection: {}", stream.peer_addr()?);
  let mut buffer = [0; 1024];

  stream.read(&mut buffer).unwrap();

  let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nhello, world";

  stream.write(response.as_bytes())?;
  stream.flush()?;

  Ok(())
}