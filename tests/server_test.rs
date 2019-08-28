extern crate futures;
extern crate rsocket;
extern crate tokio;

use rsocket::frame::Frame;
use rsocket::prelude::*;
use rsocket::transport::Context;
use rsocket::transport::FrameCodec;

use futures::future::Future;
use futures::sync::mpsc::{self, Receiver, Sender};
use futures::{Sink, Stream};
use std::io;
use std::net::SocketAddr;
use tokio::codec::Framed;
use tokio::net::{TcpListener, TcpStream};

#[test]
fn test_serve() {
  let addr = "127.0.0.1:7878".parse().unwrap();
  serve(&addr);
}

fn serve(addr: &SocketAddr) {
  let listener = TcpListener::bind(addr).unwrap();
  let server = listener
    .incoming()
    .map_err(|e| println!("listen error: {}", e))
    .for_each(move |socket| {
      let sk = DuplexSocket::builder()
        .set_acceptor(Box::new(MockResponder))
        .from_socket(socket);
      Ok(())
    });
  tokio::run(server);
}