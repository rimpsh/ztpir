use ztpir::run;
use std::{io::Result, net::TcpListener};

static PORT: &u16 = &8000;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT))
        .expect("Failed to bind random port");

    // equivalent to run()?.await
    match run(listener) {
        Ok(server) => server.await,
        Err(err) => Err(err)
    }
}