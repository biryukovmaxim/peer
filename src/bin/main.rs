use crate::peer::PeerService;
use config::Config;
use std::error::Error;
use std::sync::{Arc};
use tonic::transport::Server;
use crate::peer::peer_server::PeerServer;

#[path = "../config.rs"]
mod config;
#[path = "../errors.rs"]
mod errors;
#[path = "../peer.rs"]
mod peer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::read_config()?;
    let (tx, _) = tokio::sync::broadcast::channel(16);
    let sender = Arc::new(tx);
    let peer = PeerService::new(format!("localhost:{}", config.port), sender);
    peer.run().await?;
    Ok(())
}
