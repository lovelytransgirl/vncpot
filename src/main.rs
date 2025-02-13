use std::process;
mod config;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = config::Config::new().unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = server::run_servers(config).await {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}