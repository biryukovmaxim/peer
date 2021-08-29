use clap::{App, Arg};
use crate::errors::PeerError;

pub struct Config {
    pub port: String,
    pub known_peer_address: Option<String>,
    pub period: usize,
}

impl Config {
    pub fn read_config() -> Result<Self, PeerError> {
        let mut config = Config {
            port: "".to_string(),
            known_peer_address: None,
            period: 0,
        };
        let matches = App::new("")
            .about("simple p2p app")
            .arg(Arg::with_name("port")
                .long("port")
                .help("port on which the application will run")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("period")
                .long("period")
                .value_name("X")
                .help("every X second the peer will send gossip message to another known peers")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("connect")
                .long("connect")
                .short("c")
                .value_name("ipaddress>:<port")
                .help("connect to first known peer")
                .takes_value(true))
            .get_matches();
        match matches.value_of("port") {
            Some(port) => config.port = port.to_string(),
            None => return Err(PeerError::ReadConfig("port is required".to_string())),
        }
        match matches.value_of("connect") {
            Some(peer) => config.known_peer_address = Some(peer.to_string()),
            None => config.known_peer_address = None
        }

        Ok(config)
    }
}