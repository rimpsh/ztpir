use env_logger::Env;
use sqlx::PgPool;
use std::{io::Result, net::TcpListener};
use ztpir::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = get_configuration().expect("Failed to read configuration file");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let address = format!("{}:{}", config.application_host, config.application_port);
    let listener = TcpListener::bind(address)?;

    // equivalent to run(listener, connection)?.await
    match run(listener, connection_pool) {
        Ok(server) => server.await,
        Err(err) => Err(err),
    }
}
