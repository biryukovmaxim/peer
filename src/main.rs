use crate::tcp_server::Server;
use clap::{App, Arg};
use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::task;
use tokio::task::JoinHandle;

mod tcp_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let join = {
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
        matches.value_of("port")
            .map(|port| port.to_string())
            .ok_or("port is required".to_string())
    };

    let port = join.unwrap();
    let peers: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

    tokio::spawn(async move {
        let server = Server::new(port.to_string(), peers.clone());
        server.run().await;
    });
    Ok(())
}
