use std::{io::Result, net::TcpListener};
use ztpir::{configuration::get_configuration, startup::run};

static PORT: &u16 = &8000;

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_configuration().expect("Failed to read configuration file");
    let address = format!("{}:{}", config.application_host, config.application_port);

    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    // equivalent to run()?.await
    match run(listener) {
        Ok(server) => server.await,
        Err(err) => Err(err),
    }
}
