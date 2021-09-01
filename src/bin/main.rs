use config::Config;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use crate::peer::peer::Peer;
use crate::peer::server::Server;

#[path = "../peer/mod.rs"]
mod peer;
#[path = "../config.rs"]
mod config;
#[path = "../errors.rs"]
mod errors;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::read_config()?;
    let (tx, _) = broadcast::channel(16);
    let sender = Arc::new(tx);

    let sender_to_server_conns = sender.clone();
    let server = Server::new(Arc::new(Mutex::new(Default::default())), config.address.clone(), sender_to_server_conns);
    let peer = Peer::new(server,config.address, config.known_peer_address);
    peer.run().await?;
    Ok(())
}
