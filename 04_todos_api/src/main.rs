#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use dropshot::{
    ApiDescription, ConfigDropshot, ConfigLogging, ConfigLoggingLevel, HttpServerStarter,
};
use dotenv::dotenv;
use std::env;

mod todos;
mod db;
mod health;
mod schema;

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();

    db::init();

    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let config: ConfigDropshot = ConfigDropshot {
        bind_address: format!("127.0.0.1:{}", port).parse().unwrap(),
        ..Default::default()
    };

    let log_config = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    };
    let log = log_config
        .to_logger("todos_api")
        .map_err(|error| format!("Failed to create logger: {}", error))?;

    let mut api = ApiDescription::new();
    todos::register(&mut api);
    health::register(&mut api);

    HttpServerStarter::new(&config, api, (), &log)
        .map_err(|error| format!("Failed to create server: {}", error))?
        .start()
        .await
}
