use sqlx::PgPool;
use std::{io::Result, net::TcpListener};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use ztpir::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<()> {
    LogTracer::init().expect("Failed to set logger.");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber.");

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
