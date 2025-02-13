use crate::config::Config;
use log::{error, info};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;

pub async fn run_servers(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let (start_port, end_port) = config.port_range;
    
    for port in start_port..=end_port {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        match TcpListener::bind(&addr).await {
            Ok(listener) => {
                info!("VNC honeypot listening on port {}", port);
                tokio::spawn(async move {
                    handle_connections(listener).await;
                });
            }
            Err(e) => {
                error!("Failed to bind to port {}: {}", port, e);
            }
        }
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}

async fn handle_connections(listener: TcpListener) {
    while let Ok((mut socket, addr)) = listener.accept().await {
        info!("New connection from: {}", addr);
        
        let version = b"RFB 003.008\n";
        
        if let Err(e) = socket.write_all(version).await {
            error!("Failed to send version to {}: {}", addr, e);
        }
        
        if let Err(e) = crate::utils::log_to_file(addr, socket.local_addr().unwrap().port()) {
            error!("Failed to log connection: {}", e);
        }
    }
}