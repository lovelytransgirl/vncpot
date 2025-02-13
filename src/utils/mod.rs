use std::time::{SystemTime, UNIX_EPOCH};
use std::net::SocketAddr;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn log_to_file(addr: SocketAddr, port: u16) -> std::io::Result<()> {
    let now = Local::now();
    let log_entry = format!(
        "[{}] Connection attempt from {} on port {}\n",
        now.format("%Y-%m-%d %H:%M:%S"),
        addr,
        port
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("connection_log.txt")?;

    file.write_all(log_entry.as_bytes())?;
    Ok(())
}