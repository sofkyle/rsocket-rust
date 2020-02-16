extern crate env_logger;
extern crate futures;
extern crate log;
extern crate rsocket_rust;
extern crate tokio;

use rsocket_rust::prelude::*;

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp_millis().init();

    let cli = RSocketFactory::connect()
        .acceptor(|| Box::new(EchoRSocket))
        .transport("tcp://127.0.0.1:7878")
        .setup(Payload::from("READY!"))
        .mime_type("text/plain", "text/plain")
        .start()
        .await
        .unwrap();
    let req = Payload::builder()
        .set_data_utf8("Hello World!")
        .set_metadata_utf8("Rust")
        .build();
    let res = cli.request_response(req).await.unwrap();
    println!("got: {:?}", res);
    cli.close();
}
