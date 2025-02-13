use clap::{Command, Arg};

pub struct Config {
    pub port_range: (u16, u16),
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let matches = Command::new("vncpot")
            .version("1.0")
            .author("notmycode-labs")
            .about("vnc honeypot")
            .arg(
                Arg::new("start-port")
                    .short('s')
                    .long("start-port")
                    .value_name("PORT")
                    .help("Starting port number")
                    .required(true),
            )
            .arg(
                Arg::new("end-port")
                    .short('e')
                    .long("end-port")
                    .value_name("PORT")
                    .help("Ending port number")
                    .required(true),
            )
            .get_matches();

        let start_port: u16 = matches
            .get_one::<String>("start-port")
            .unwrap()
            .parse()
            .map_err(|_| "Invalid start port number".to_string())?;

        let end_port: u16 = matches
            .get_one::<String>("end-port")
            .unwrap()
            .parse()
            .map_err(|_| "Invalid end port number".to_string())?;

        if start_port > end_port {
            return Err("Start port must be less than or equal to end port".to_string());
        }

        Ok(Config {
            port_range: (start_port, end_port),
        })
    }
}