extern crate env_logger;
extern crate futures;
extern crate log;
extern crate rsocket_rust;
extern crate tokio;

use futures::executor::block_on;
use rsocket_rust::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::builder().format_timestamp_millis().init();

    RSocketFactory::receive()
        .acceptor(|setup, _sending_socket| {
            println!("incoming socket: setup={:?}", setup);
            Ok(Box::new(block_on(async move {
                RSocketFactory::connect()
                    .acceptor(|| Box::new(EchoRSocket))
                    .setup(Payload::from("I'm Rust!"))
                    .transport("tcp://127.0.0.1:7878")
                    .start()
                    .await
                    .unwrap()
            })))
        })
        .transport("tcp://127.0.0.1:7979")
        .serve()
        .await
}
