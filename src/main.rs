use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::{io::Result, net::TcpListener};
use ztpir::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<()> {
    let macgyver = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(macgyver);

    let config = get_configuration().expect("Failed to read configuration file");
    let connection_pool =
        PgPool::connect_lazy(&config.database.connection_string().expose_secret())
            .expect("Failed to connect to database");

    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;

    // equivalent to run(listener, connection)?.await
    match run(listener, connection_pool) {
        Ok(server) => server.await,
        Err(err) => Err(err),
    }
}
